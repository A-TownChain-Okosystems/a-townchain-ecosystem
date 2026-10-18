// Copyright (c) 2026 A-TownChain-Okosystems. All Rights Reserved.
//! Transactional trial execution with mandatory rollback on rejected results.
//!
//! This is a platform-neutral contract only. Hardware/runtime adapters must
//! implement authorization, parameter access, and trustworthy measurements.

use crate::performance_optimizer::{
    MetricKind, OptimizationProposal, PerformanceOptimizer, ProposalError, TrialResult,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrialError {
    ReadParameterFailed,
    BaselineMeasurementFailed,
    ApplyFailed,
    CandidateMeasurementFailed,
    RollbackFailed,
    InvalidProposal(ProposalError),
}

pub trait OptimizationAdapter {
    /// Read the exact current value that will be restored if the trial fails.
    fn read_parameter(&mut self, proposal: OptimizationProposal) -> Result<u32, ()>;
    /// Measure the requested metric under the current workload.
    fn measure(&mut self, metric: MetricKind) -> Result<u64, ()>;
    /// Apply one value. Implementations must enforce platform-specific safety
    /// checks and authorization before mutating system state.
    fn apply(&mut self, proposal: OptimizationProposal, value: u32) -> Result<(), ()>;
    /// Restore the exact value returned by read_parameter.
    fn restore(&mut self, proposal: OptimizationProposal, previous_value: u32) -> Result<(), ()>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrialReport {
    pub result: TrialResult,
    pub before: u64,
    pub after: Option<u64>,
    pub previous_value: u32,
    pub candidate_value: u32,
    pub rolled_back: bool,
}

/// Apply a candidate, measure its effect, and restore the exact prior value
/// unless the optimizer accepts it. Dry-run and exhausted change budgets never
/// mutate the system. Any measurement or apply failure triggers restoration
/// when mutation may have occurred.
pub fn execute_trial<A: OptimizationAdapter>(
    optimizer: &mut PerformanceOptimizer,
    adapter: &mut A,
    proposal: OptimizationProposal,
) -> Result<TrialReport, TrialError> {
    proposal.validate().map_err(TrialError::InvalidProposal)?;
    let policy = optimizer.policy();

    let previous_value = adapter
        .read_parameter(proposal)
        .map_err(|_| TrialError::ReadParameterFailed)?;
    let before = adapter
        .measure(proposal.target_metric)
        .map_err(|_| TrialError::BaselineMeasurementFailed)?;

    if policy.dry_run {
        return Ok(TrialReport {
            result: TrialResult::DryRunOnly,
            before,
            after: None,
            previous_value,
            candidate_value: proposal.candidate_value,
            rolled_back: false,
        });
    }

    if optimizer.accepted_changes() >= policy.max_changes_per_boot {
        return Ok(TrialReport {
            result: TrialResult::ChangeLimitReached,
            before,
            after: None,
            previous_value,
            candidate_value: proposal.candidate_value,
            rolled_back: false,
        });
    }

    if adapter.apply(proposal, proposal.candidate_value).is_err() {
        // Apply may have partially changed state before reporting failure.
        adapter
            .restore(proposal, previous_value)
            .map_err(|_| TrialError::RollbackFailed)?;
        return Err(TrialError::ApplyFailed);
    }

    let after = match adapter.measure(proposal.target_metric) {
        Ok(value) => value,
        Err(_) => {
            adapter
                .restore(proposal, previous_value)
                .map_err(|_| TrialError::RollbackFailed)?;
            return Err(TrialError::CandidateMeasurementFailed);
        }
    };

    let result = match optimizer.assess(proposal, before, after) {
        Ok(result) => result,
        Err(error) => {
            adapter
                .restore(proposal, previous_value)
                .map_err(|_| TrialError::RollbackFailed)?;
            return Err(TrialError::InvalidProposal(error));
        }
    };

    if result == TrialResult::Accepted {
        Ok(TrialReport {
            result,
            before,
            after: Some(after),
            previous_value,
            candidate_value: proposal.candidate_value,
            rolled_back: false,
        })
    } else {
        adapter
            .restore(proposal, previous_value)
            .map_err(|_| TrialError::RollbackFailed)?;
        Ok(TrialReport {
            result,
            before,
            after: Some(after),
            previous_value,
            candidate_value: proposal.candidate_value,
            rolled_back: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::performance_optimizer::{Goal, OptimizationPolicy, TunableParameter};

    struct FakeAdapter {
        current: u32,
        before: u64,
        after: u64,
        fail_after_measurement: bool,
        fail_restore: bool,
        measurements: usize,
    }

    impl OptimizationAdapter for FakeAdapter {
        fn read_parameter(&mut self, _: OptimizationProposal) -> Result<u32, ()> {
            Ok(self.current)
        }
        fn measure(&mut self, _: MetricKind) -> Result<u64, ()> {
            self.measurements += 1;
            if self.measurements == 1 {
                Ok(self.before)
            } else if self.fail_after_measurement {
                Err(())
            } else {
                Ok(self.after)
            }
        }
        fn apply(&mut self, _: OptimizationProposal, value: u32) -> Result<(), ()> {
            self.current = value;
            Ok(())
        }
        fn restore(&mut self, _: OptimizationProposal, value: u32) -> Result<(), ()> {
            if self.fail_restore {
                return Err(());
            }
            self.current = value;
            Ok(())
        }
    }

    fn proposal() -> OptimizationProposal {
        OptimizationProposal {
            parameter: TunableParameter::SchedulerQuantumNs,
            candidate_value: 2_000_000,
            target_metric: MetricKind::SchedulerLatencyNs,
            goal: Goal::LowerIsBetter,
        }
    }

    fn adapter(after: u64) -> FakeAdapter {
        FakeAdapter {
            current: 1_000_000,
            before: 1_000,
            after,
            fail_after_measurement: false,
            fail_restore: false,
            measurements: 0,
        }
    }

    fn active_optimizer() -> PerformanceOptimizer {
        PerformanceOptimizer::new(OptimizationPolicy {
            dry_run: false,
            ..OptimizationPolicy::default()
        })
    }

    #[test]
    fn keeps_candidate_only_when_improvement_is_accepted() {
        let mut optimizer = active_optimizer();
        let mut system = adapter(800);
        let report = execute_trial(&mut optimizer, &mut system, proposal()).unwrap();
        assert_eq!(report.result, TrialResult::Accepted);
        assert!(!report.rolled_back);
        assert_eq!(system.current, 2_000_000);
    }

    #[test]
    fn rolls_back_when_performance_regresses() {
        let mut optimizer = active_optimizer();
        let mut system = adapter(1_200);
        let report = execute_trial(&mut optimizer, &mut system, proposal()).unwrap();
        assert_eq!(report.result, TrialResult::RejectedRegression);
        assert!(report.rolled_back);
        assert_eq!(system.current, 1_000_000);
    }

    #[test]
    fn rolls_back_when_post_change_measurement_fails() {
        let mut optimizer = active_optimizer();
        let mut system = adapter(800);
        system.fail_after_measurement = true;
        assert_eq!(
            execute_trial(&mut optimizer, &mut system, proposal()),
            Err(TrialError::CandidateMeasurementFailed)
        );
        assert_eq!(system.current, 1_000_000);
    }

    #[test]
    fn dry_run_never_applies_candidate() {
        let mut optimizer = PerformanceOptimizer::new(OptimizationPolicy::default());
        let mut system = adapter(800);
        let report = execute_trial(&mut optimizer, &mut system, proposal()).unwrap();
        assert_eq!(report.result, TrialResult::DryRunOnly);
        assert_eq!(system.current, 1_000_000);
        assert_eq!(report.after, None);
    }

    #[test]
    fn reports_rollback_failure_explicitly() {
        let mut optimizer = active_optimizer();
        let mut system = adapter(1_200);
        system.fail_restore = true;
        assert_eq!(
            execute_trial(&mut optimizer, &mut system, proposal()),
            Err(TrialError::RollbackFailed)
        );
    }
}
