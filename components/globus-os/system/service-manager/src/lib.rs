//! Dependency-aware userspace service supervision.
//!
//! The manager owns lifecycle policy only. Actual process creation, signaling,
//! and resource control remain capability-gated platform operations.

use std::collections::{HashMap, HashSet, VecDeque};

/// Lifecycle state tracked by the supervisor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Defined,
    Starting,
    Ready,
    Failed,
    Stopped,
}

/// Declarative service definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceSpec {
    pub name: String,
    pub dependencies: Vec<String>,
    pub critical: bool,
    pub restart_limit: u32,
}

/// Runtime health report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HealthReport {
    pub healthy: bool,
    pub consecutive_failures: u32,
}

/// Service-manager validation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceError {
    DuplicateService(String),
    MissingDependency { service: String, dependency: String },
    DependencyCycle,
    UnknownService(String),
    RestartLimitExceeded(String),
}

/// Dependency-aware service supervisor.
#[derive(Debug, Default)]
pub struct ServiceManager {
    specs: HashMap<String, ServiceSpec>,
    states: HashMap<String, ServiceState>,
    failures: HashMap<String, u32>,
}

impl ServiceManager {
    /// Creates an empty service manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a service definition.
    pub fn register(&mut self, spec: ServiceSpec) -> Result<(), ServiceError> {
        if self.specs.contains_key(&spec.name) {
            return Err(ServiceError::DuplicateService(spec.name));
        }
        self.states.insert(spec.name.clone(), ServiceState::Defined);
        self.failures.insert(spec.name.clone(), 0);
        self.specs.insert(spec.name.clone(), spec);
        Ok(())
    }

    /// Validates dependency references and detects cycles.
    pub fn validate(&self) -> Result<(), ServiceError> {
        for spec in self.specs.values() {
            for dependency in &spec.dependencies {
                if !self.specs.contains_key(dependency) {
                    return Err(ServiceError::MissingDependency {
                        service: spec.name.clone(),
                        dependency: dependency.clone(),
                    });
                }
            }
        }

        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        for name in self.specs.keys() {
            self.visit(name, &mut visiting, &mut visited)?;
        }
        Ok(())
    }

    fn visit(
        &self,
        name: &str,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
    ) -> Result<(), ServiceError> {
        if visited.contains(name) {
            return Ok(());
        }
        if !visiting.insert(name.to_owned()) {
            return Err(ServiceError::DependencyCycle);
        }
        let spec = self.specs.get(name).ok_or_else(|| ServiceError::UnknownService(name.into()))?;
        for dependency in &spec.dependencies {
            self.visit(dependency, visiting, visited)?;
        }
        visiting.remove(name);
        visited.insert(name.to_owned());
        Ok(())
    }

    /// Produces a deterministic dependency-first startup order.
    pub fn startup_order(&self) -> Result<Vec<String>, ServiceError> {
        self.validate()?;
        let mut order = Vec::with_capacity(self.specs.len());
        let mut queued = VecDeque::new();
        let mut seen = HashSet::new();

        let mut names: Vec<_> = self.specs.keys().cloned().collect();
        names.sort();
        for name in names {
            queued.push_back(name);
        }

        while let Some(name) = queued.pop_front() {
            if !seen.insert(name.clone()) {
                continue;
            }
            let spec = &self.specs[&name];
            for dependency in &spec.dependencies {
                if !seen.contains(dependency) {
                    queued.push_front(name.clone());
                    queued.push_front(dependency.clone());
                    break;
                }
            }
            if seen.contains(&name) {
                order.push(name);
            }
        }

        // The queue algorithm above can revisit a service after its dependency;
        // remove duplicates while preserving first dependency-first occurrence.
        let mut unique = Vec::with_capacity(order.len());
        let mut emitted = HashSet::new();
        for name in order {
            if emitted.insert(name.clone()) {
                unique.push(name);
            }
        }
        Ok(unique)
    }

    /// Records a service state transition.
    pub fn set_state(&mut self, name: &str, state: ServiceState) -> Result<(), ServiceError> {
        if !self.specs.contains_key(name) {
            return Err(ServiceError::UnknownService(name.into()));
        }
        self.states.insert(name.into(), state);
        Ok(())
    }

    /// Records a health result and enforces the configured restart budget.
    pub fn record_health(&mut self, name: &str, report: HealthReport) -> Result<bool, ServiceError> {
        let spec = self.specs.get(name).ok_or_else(|| ServiceError::UnknownService(name.into()))?;
        if report.healthy {
            self.failures.insert(name.into(), 0);
            self.states.insert(name.into(), ServiceState::Ready);
            return Ok(false);
        }

        let failures = self.failures.entry(name.into()).or_default();
        *failures = failures.saturating_add(1);
        self.states.insert(name.into(), ServiceState::Failed);
        if *failures > spec.restart_limit {
            return Err(ServiceError::RestartLimitExceeded(name.into()));
        }
        Ok(true)
    }

    /// Returns the tracked state of a service.
    pub fn state(&self, name: &str) -> Option<ServiceState> {
        self.states.get(name).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(name: &str, dependencies: &[&str]) -> ServiceSpec {
        ServiceSpec {
            name: name.into(),
            dependencies: dependencies.iter().map(|v| (*v).into()).collect(),
            critical: true,
            restart_limit: 2,
        }
    }

    #[test]
    fn validates_dependencies_and_detects_cycles() {
        let mut manager = ServiceManager::new();
        manager.register(spec("network", &["security"])).unwrap();
        manager.register(spec("security", &[])).unwrap();
        assert!(manager.validate().is_ok());

        let mut cyclic = ServiceManager::new();
        cyclic.register(spec("a", &["b"])).unwrap();
        cyclic.register(spec("b", &["a"])).unwrap();
        assert_eq!(cyclic.validate(), Err(ServiceError::DependencyCycle));
    }

    #[test]
    fn health_failures_are_bounded() {
        let mut manager = ServiceManager::new();
        manager.register(spec("network", &[])).unwrap();
        assert_eq!(
            manager.record_health("network", HealthReport { healthy: false, consecutive_failures: 1 }),
            Ok(true)
        );
        assert_eq!(
            manager.record_health("network", HealthReport { healthy: false, consecutive_failures: 2 }),
            Ok(true)
        );
        assert_eq!(
            manager.record_health("network", HealthReport { healthy: false, consecutive_failures: 3 }),
            Ok(false)
        );
        assert_eq!(manager.state("network"), Some(ServiceState::Failed));
    }
}
