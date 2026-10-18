// Copyright (c) 2026 A-TownChain-Okosystems. All Rights Reserved.
//! Guarded policy primitives for automatic performance optimization.
//!
//! This module deliberately separates recommendations from hardware/runtime
//! changes. A platform adapter must apply a validated proposal, measure its
//! effect, and roll it back when the measured result violates policy.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetricKind {
    BootDurationNs,
    SchedulerLatencyNs,
    QueueDepth,
    MemoryPressurePermille,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Goal {
    LowerIsBetter,
    HigherIsBetter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TunableParameter {
    SchedulerQuantumTicks,
    BackgroundWorkBudgetPermille,
    CacheCapacityPages,
    IoBatchSize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParameterBounds {
    pub min: u32,
    pub max: u32,
}

impl TunableParameter {
    /// Conservative bounds. Platform adapters may impose stricter bounds,
    /// but must never widen these without an explicit reviewed change.
    pub const fn bounds(self) -> ParameterBounds {
        match self {
            Self::SchedulerQuantumTicks => ParameterBounds { min: 2, max: 50 },
            Self::BackgroundWorkBudgetPermille => ParameterBounds { min: 10, max: 500 },
            Self::CacheCapacityPages => ParameterBounds { min: 0, max: 65_536 },
            Self::IoBatchSize => ParameterBounds { min: 1, max: 1_024 },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OptimizationProposal {
    pub parameter: TunableParameter,
    pub candidate_value: u32,
    pub target_metric: MetricKind,
    pub goal: Goal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProposalError {
    OutOfBounds,
    InvalidMetricGoal,
}

impl OptimizationProposal {
    pub fn validate(self) -> Result<Self, ProposalError> {
        let bounds = self.parameter.bounds();
        if self.candidate_value < bounds.min || self.candidate_value > bounds.max {
            return Err(ProposalError::OutOfBounds);
        }
        if matches!(self.target_metric, MetricKind::MemoryPressurePermille)
            && self.goal == Goal::HigherIsBetter
        {
            return Err(ProposalError::InvalidMetricGoal);
        }
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OptimizationPolicy {
    /// Required improvement to consider a trial useful (10 permille = 1%).
    pub min_improvement_permille: u16,
    /// Maximum accepted regression in the target metric (permille).
    pub max_regression_permille: u16,
    /// Maximum accepted automatic changes during one boot session.
    pub max_changes_per_boot: u16,
    /// When true, the engine reports proposals but never applies them.
    pub dry_run: bool,
}

impl Default for OptimizationPolicy {
    fn default() -> Self {
        Self {
            min_improvement_permille: 10,
            max_regression_permille: 50,
            max_changes_per_boot: 3,
            dry_run: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrialResult {
    DryRunOnly,
    RejectedRegression,
    RejectedInsufficientImprovement,
    Accepted,
    ChangeLimitReached,
}

#[derive(Debug)]
pub struct PerformanceOptimizer {
    policy: OptimizationPolicy,
    accepted_changes: u16,
}

impl PerformanceOptimizer {
    pub const fn new(policy: OptimizationPolicy) -> Self {
        Self { policy, accepted_changes: 0 }
    }

    pub const fn policy(&self) -> OptimizationPolicy {
        self.policy
    }

    pub const fn accepted_changes(&self) -> u16 {
        self.accepted_changes
    }

    /// Compare a before/after measurement for one validated proposal.
    ///
    /// The caller must only supply measurements from the same workload,
    /// hardware profile, and measurement window. This function never applies
    /// a setting; a platform adapter must own apply/rollback and authorization.
    pub fn assess(
        &mut self,
        proposal: OptimizationProposal,
        before: u64,
        after: u64,
    ) -> Result<TrialResult, ProposalError> {
        proposal.validate()?;
        if self.policy.dry_run {
            return Ok(TrialResult::DryRunOnly);
        }
        if self.accepted_changes >= self.policy.max_changes_per_boot {
            return Ok(TrialResult::ChangeLimitReached);
        }

        let improved = match proposal.goal {
            Goal::LowerIsBetter => after < before,
            Goal::HigherIsBetter => after > before,
        };
        let (delta, denominator) = if improved {
            (before.abs_diff(after), before)
        } else {
            (before.abs_diff(after), before)
        };

        // A zero baseline cannot produce a meaningful relative comparison.
        if denominator == 0 {
            return Ok(TrialResult::RejectedInsufficientImprovement);
        }
        let permille = delta.saturating_mul(1_000) / denominator;

        if improved && permille >= self.policy.min_improvement_permille as u64 {
            self.accepted_changes = self.accepted_changes.saturating_add(1);
            return Ok(TrialResult::Accepted);
        }

        if !improved && permille > self.policy.max_regression_permille as u64 {
            return Ok(TrialResult::RejectedRegression);
        }
        Ok(TrialResult::RejectedInsufficientImprovement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proposal(value: u32) -> OptimizationProposal {
        OptimizationProposal {
            parameter: TunableParameter::SchedulerQuantumTicks,
            candidate_value: value,
            target_metric: MetricKind::SchedulerLatencyNs,
            goal: Goal::LowerIsBetter,
        }
    }

    #[test]
    fn defaults_to_dry_run_and_never_counts_a_change() {
        let mut optimizer = PerformanceOptimizer::new(OptimizationPolicy::default());
        assert_eq!(optimizer.assess(proposal(1_000_000), 1_000, 800), Ok(TrialResult::DryRunOnly));
        assert_eq!(optimizer.accepted_changes(), 0);
    }

    #[test]
    fn rejects_values_outside_conservative_bounds() {
        let mut optimizer = PerformanceOptimizer::new(OptimizationPolicy { dry_run: false, ..OptimizationPolicy::default() });
        assert_eq!(optimizer.assess(proposal(1), 1_000, 800), Err(ProposalError::OutOfBounds));
    }

    #[test]
    fn accepts_only_measurable_improvement() {
        let mut optimizer = PerformanceOptimizer::new(OptimizationPolicy { dry_run: false, ..OptimizationPolicy::default() });
        assert_eq!(optimizer.assess(proposal(1_000_000), 1_000, 900), Ok(TrialResult::Accepted));
        assert_eq!(optimizer.accepted_changes(), 1);
        assert_eq!(optimizer.assess(proposal(1_000_000), 1_000, 999), Ok(TrialResult::RejectedInsufficientImprovement));
    }

    #[test]
    fn rejects_large_regression_and_enforces_change_limit() {
        let mut optimizer = PerformanceOptimizer::new(OptimizationPolicy {
            dry_run: false,
            max_changes_per_boot: 1,
            ..OptimizationPolicy::default()
        });
        assert_eq!(optimizer.assess(proposal(1_000_000), 1_000, 1_200), Ok(TrialResult::RejectedRegression));
        assert_eq!(optimizer.assess(proposal(1_000_000), 1_000, 900), Ok(TrialResult::Accepted));
        assert_eq!(optimizer.assess(proposal(1_000_000), 1_000, 800), Ok(TrialResult::ChangeLimitReached));
    }

    #[test]
    fn rejects_higher_is_better_for_memory_pressure() {
        let p = OptimizationProposal {
            parameter: TunableParameter::CacheCapacityPages,
            candidate_value: 64,
            target_metric: MetricKind::MemoryPressurePermille,
            goal: Goal::HigherIsBetter,
        };
        assert_eq!(p.validate(), Err(ProposalError::InvalidMetricGoal));
    }
}
