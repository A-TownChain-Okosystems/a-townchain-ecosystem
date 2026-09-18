#![doc = "Aurora AI control-plane contracts for GlobusOS."]

pub mod contracts;
pub mod bridge;
pub use bridge::{BridgeDecision, GlobusPolicyEndpoint, PolicyRequest, PolicyResponse};
pub use identity::{AgentIdentity, IdentityVerifier};
pub use attestation::{sign_attestation, verify_attestation, AttestationSigner, AttestationVerifier, ExecutionAttestation};
pub use chat::{ChatChunk, ChatMessage, ChatModel, ChatRequest, ChatResponse, ChatTool, MessageRole, ToolCallRequest};
pub use context::{ContextBuilder, ContextLimits};
pub use memory::ConversationMemory;
pub use orchestrator::{AuroraOrchestrator, AuthorizedTool};
pub use model::{DeterministicTestEngine, EngineBackedModel, InferenceEngine};
pub use rag::{ContextAugmenter, KnowledgeDocument, Retriever};
pub use multimodal::{MediaInput, Modality, MultimodalInput};
pub use distributed::{InferenceTask, InferenceWorker, LocalWorker, WorkerDescriptor, WorkerRouter};
pub use local::{LocalInferenceConfig, LocalInferenceEngine};\npub use genesis::GenesisEngineControl;
pub mod errors;
pub mod state;
pub mod types;
pub mod identity;
pub mod attestation;
pub mod chat;
pub mod context;
pub mod memory;
pub mod orchestrator;
pub mod model;
pub mod rag;
pub mod multimodal;
pub mod distributed;
pub mod local;\npub mod genesis;

pub use contracts::*;
pub use errors::AuroraError;
pub use state::StateMachine;
pub use types::{
    AgentId, ApprovalId, AuroraRequest, AuroraResponse, CapabilityId, EventId, ModelId, PolicyId,
    RequestId, SessionId, SkillId, ToolId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Deny,
    ApprovalRequired,
}

impl Decision {
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Deny)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalState {
    NotRequired,
    Pending,
    Granted,
    Denied,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestStatus {
    Created,
    Queued,
    Running,
    WaitingApproval,
    WaitingResource,
    Paused,
    Verifying,
    Completed,
    Failed,
    Cancelled,
    Denied,
    Timeout,
    ResourceExhausted,
    SecurityViolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    L0,
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deny_is_terminal() {
        assert!(Decision::Deny.is_terminal());
        assert!(!Decision::ApprovalRequired.is_terminal());
    }
}
