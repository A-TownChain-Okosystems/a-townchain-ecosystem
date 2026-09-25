use atc_aurora_policy::{Approval, AuditEvent, PolicyDecision, PolicyEngine};

use crate::{ToolError, ToolRegistry, ToolRequest, ToolResult};

pub struct ToolExecutor {
    pub policy: PolicyEngine,
    pub registry: ToolRegistry,
}

impl ToolExecutor {
    pub fn new(policy: PolicyEngine, registry: ToolRegistry) -> Self {
        Self { policy, registry }
    }

    pub fn execute(
        &self,
        request: &ToolRequest,
        approval: Option<&Approval>,
    ) -> Result<(ToolResult, AuditEvent), ToolError> {
        let decision = self
            .policy
            .evaluate(&request.agent_id, &request.capability, approval);

        let audit = match decision {
            PolicyDecision::Deny { audit } => {
                return Err(ToolError::ExecutionDenied(audit.reason.clone()));
            }
            PolicyDecision::ApprovalRequired { audit } => {
                return Err(ToolError::ExecutionDenied(audit.reason.clone()));
            }
            PolicyDecision::Allow { audit } => audit,
        };

        let tool = self.registry.require(request.capability.id.as_str())?;
        let result = tool.execute(request)?;
        let mut audit = audit;
        audit.outcome = atc_aurora_policy::AuditOutcome::Executed;
        Ok((result, audit))
    }
}
