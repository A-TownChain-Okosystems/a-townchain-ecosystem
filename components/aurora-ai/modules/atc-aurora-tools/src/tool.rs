use atc_aurora_policy::CapabilityRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolRequest {
    pub agent_id: String,
    pub capability: CapabilityRequest,
}

impl ToolRequest {
    pub fn new(agent_id: impl Into<String>, capability: CapabilityRequest) -> Self {
        Self { agent_id: agent_id.into(), capability }
    }
}

pub trait Tool: Send + Sync {
    fn id(&self) -> &str;
    fn execute(&self, request: &ToolRequest) -> Result<ToolResult, ToolError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolResult {
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolError {
    ExecutionDenied(String),
    ExecutionFailed(String),
}
