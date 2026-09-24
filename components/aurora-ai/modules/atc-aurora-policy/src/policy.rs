use crate::{Approval, ApprovalRequirement, AuditEvent, AuditOutcome, Capability, CapabilityRequest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow { audit: AuditEvent },
    Deny { audit: AuditEvent },
    ApprovalRequired { audit: AuditEvent },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyError {
    CapabilityDenied,
    ApprovalRequired,
}

#[derive(Default)]
pub struct PolicyEngine {
    capabilities: Vec<Capability>,
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, capability: Capability) {
        self.capabilities.push(capability);
    }

    pub fn evaluate(
        &self,
        agent_id: &str,
        request: &CapabilityRequest,
        approval: Option<&Approval>,
    ) -> PolicyDecision {
        let Some(capability) = self.capabilities.iter().find(|c| c.permits(request)) else {
            return PolicyDecision::Deny {
                audit: AuditEvent::new(
                    agent_id,
                    request.capability.as_str(),
                    &request.operation,
                    &request.resource,
                    AuditOutcome::Denied,
                    "capability not registered or resource not permitted",
                ),
            };
        };

        if capability.approval == ApprovalRequirement::ExplicitUser {
            if approval.map(|a| a.is_satisfied()).unwrap_or(false) == false {
                return PolicyDecision::ApprovalRequired {
                    audit: AuditEvent::new(
                        agent_id,
                        request.capability.as_str(),
                        &request.operation,
                        &request.resource,
                        AuditOutcome::Denied,
                        "explicit user approval required",
                    ),
                };
            }
        }

        PolicyDecision::Allow {
            audit: AuditEvent::new(
                agent_id,
                request.capability.as_str(),
                &request.operation,
                &request.resource,
                AuditOutcome::Allowed,
                "capability and policy checks passed",
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undeclared_capability_is_denied() {
        let engine = PolicyEngine::new();
        let decision = engine.evaluate(
            "agent",
            &CapabilityRequest::new("filesystem.read", "read", "/workspace/a"),
            None,
        );
        assert!(matches!(decision, PolicyDecision::Deny { .. }));
    }

    #[test]
    fn explicit_approval_is_required() {
        let mut engine = PolicyEngine::new();
        engine.register(Capability::new(
            "wallet.transfer",
            "transfer",
            "wallet/*",
            "wallet.transfer",
            ApprovalRequirement::ExplicitUser,
            true,
        ));

        let request = CapabilityRequest::new("wallet.transfer", "transfer", "wallet/main");
        assert!(matches!(
            engine.evaluate("agent", &request, None),
            PolicyDecision::ApprovalRequired { .. }
        ));
        assert!(matches!(
            engine.evaluate("agent", &request, Some(&Approval::explicit_user(true))),
            PolicyDecision::Allow { .. }
        ));
    }
}
