use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditOutcome {
    Allowed,
    Denied,
    Executed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    pub agent_id: String,
    pub capability: String,
    pub operation: String,
    pub resource: String,
    pub outcome: AuditOutcome,
    pub timestamp: u64,
    pub reason: String,
}

impl AuditEvent {
    pub fn new(
        agent_id: impl Into<String>,
        capability: impl Into<String>,
        operation: impl Into<String>,
        resource: impl Into<String>,
        outcome: AuditOutcome,
        reason: impl Into<String>,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            agent_id: agent_id.into(),
            capability: capability.into(),
            operation: operation.into(),
            resource: resource.into(),
            outcome,
            timestamp,
            reason: reason.into(),
        }
    }
}
