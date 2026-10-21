// Copyright (c) 2026 A-TownChain-Okosystems. All Rights Reserved.
//! Low-overhead, clock-agnostic boot and component timing primitives.
//!
//! Callers must supply timestamps from a real monotonic clock. This module
//! deliberately does not fabricate timestamps or select an uncalibrated clock.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetricStatus {
    Started,
    Ready,
    Failed,
    TimedOut,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BootMetric {
    pub component: String,
    pub start_ns: u64,
    pub end_ns: Option<u64>,
    pub duration_ns: Option<u64>,
    pub status: MetricStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetricError {
    AlreadyRunning,
    NotRunning,
    ClockWentBackwards,
}

/// Collects component spans. Timestamps are monotonic nanoseconds in one
/// clock domain; callers decide when a component is truly ready.
#[derive(Debug, Default)]
pub struct BootMetrics {
    completed: Vec<BootMetric>,
    running: Vec<BootMetric>,
}

impl BootMetrics {
    pub const fn new() -> Self {
        Self { completed: Vec::new(), running: Vec::new() }
    }

    pub fn begin(&mut self, component: &str, now_ns: u64) -> Result<(), MetricError> {
        if self.running.iter().any(|m| m.component == component) {
            return Err(MetricError::AlreadyRunning);
        }
        self.running.push(BootMetric {
            component: String::from(component),
            start_ns: now_ns,
            end_ns: None,
            duration_ns: None,
            status: MetricStatus::Started,
        });
        Ok(())
    }

    pub fn ready(&mut self, component: &str, now_ns: u64) -> Result<u64, MetricError> {
        self.finish(component, now_ns, MetricStatus::Ready)
    }

    pub fn fail(&mut self, component: &str, now_ns: u64) -> Result<u64, MetricError> {
        self.finish(component, now_ns, MetricStatus::Failed)
    }

    pub fn timeout(&mut self, component: &str, now_ns: u64) -> Result<u64, MetricError> {
        self.finish(component, now_ns, MetricStatus::TimedOut)
    }

    fn finish(
        &mut self,
        component: &str,
        now_ns: u64,
        status: MetricStatus,
    ) -> Result<u64, MetricError> {
        let Some(index) = self.running.iter().position(|m| m.component == component) else {
            return Err(MetricError::NotRunning);
        };
        if now_ns < self.running[index].start_ns {
            return Err(MetricError::ClockWentBackwards);
        }
        let mut metric = self.running.remove(index);
        let duration = now_ns - metric.start_ns;
        metric.end_ns = Some(now_ns);
        metric.duration_ns = Some(duration);
        metric.status = status;
        self.completed.push(metric);
        Ok(duration)
    }

    pub fn completed(&self) -> &[BootMetric] {
        &self.completed
    }

    pub fn running(&self) -> &[BootMetric] {
        &self.running
    }

    /// Returns true only when no span remains unfinished.
    pub fn is_complete(&self) -> bool {
        self.running.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_monotonic_duration_and_ready_status() {
        let mut metrics = BootMetrics::new();
        metrics.begin("scheduler", 100).unwrap();
        assert_eq!(metrics.ready("scheduler", 1_100), Ok(1_000));
        assert_eq!(metrics.completed()[0].status, MetricStatus::Ready);
        assert_eq!(metrics.completed()[0].duration_ns, Some(1_000));
        assert!(metrics.is_complete());
    }

    #[test]
    fn rejects_duplicate_start_and_missing_component() {
        let mut metrics = BootMetrics::new();
        metrics.begin("ipc", 10).unwrap();
        assert_eq!(metrics.begin("ipc", 20), Err(MetricError::AlreadyRunning));
        assert_eq!(metrics.ready("unknown", 30), Err(MetricError::NotRunning));
    }

    #[test]
    fn rejects_clock_regression_without_losing_running_span() {
        let mut metrics = BootMetrics::new();
        metrics.begin("memory", 50).unwrap();
        assert_eq!(metrics.ready("memory", 49), Err(MetricError::ClockWentBackwards));
        assert_eq!(metrics.running().len(), 1);
    }

    #[test]
    fn captures_failure_and_timeout_as_non_ready_states() {
        let mut metrics = BootMetrics::new();
        metrics.begin("network", 0).unwrap();
        metrics.fail("network", 12).unwrap();
        metrics.begin("filesystem", 20).unwrap();
        metrics.timeout("filesystem", 40).unwrap();
        assert_eq!(metrics.completed()[0].status, MetricStatus::Failed);
        assert_eq!(metrics.completed()[1].status, MetricStatus::TimedOut);
    }
}
