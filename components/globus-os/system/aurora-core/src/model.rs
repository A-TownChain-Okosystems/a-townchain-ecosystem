//! Provider-neutral model runtime boundary.
//! Concrete inference engines (local, remote, distributed) implement this trait.
//! The deterministic Aurora control plane never embeds vendor-specific inference code.

use crate::{AgentId, AuroraError, ChatModel, ChatRequest, ChatResponse, ModelId};

pub trait InferenceEngine {
    fn model_id(&self) -> &ModelId;
    fn infer(&self, agent: &AgentId, prompt: &str) -> Result<String, AuroraError>;
}

pub struct EngineBackedModel<E> { engine: E }

impl<E: InferenceEngine> EngineBackedModel<E> {
    pub fn new(engine: E) -> Self { Self { engine } }
    pub fn engine(&self) -> &E { &self.engine }
}

impl<E: InferenceEngine> ChatModel for EngineBackedModel<E> {
    fn complete(&self, request: &ChatRequest) -> Result<ChatResponse, AuroraError> {
        if request.model_id != *self.engine.model_id() {
            return Err(AuroraError::InvalidRequest);
        }
        let prompt = request.messages.iter()
            .map(|m| format!("{:?}: {}", m.role, m.content))
            .collect::<Vec<_>>().join("\n");
        let output = self.engine.infer(&request.agent_id, &prompt)?;
        Ok(ChatResponse {
            request_id: request.request_id.clone(),
            message: crate::ChatMessage::new(crate::MessageRole::Assistant, output)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicTestEngine { id: ModelId }

impl DeterministicTestEngine {
    pub fn new(id: ModelId) -> Self { Self { id } }
}

impl InferenceEngine for DeterministicTestEngine {
    fn model_id(&self) -> &ModelId { &self.id }
    fn infer(&self, _agent: &AgentId, prompt: &str) -> Result<String, AuroraError> {
        Ok(format!("inference:{prompt}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ChatMessage, MessageRole, RequestId, SessionId};
    #[test]
    fn engine_is_reached_through_chat_model() {
        let id=ModelId::new("test-model").unwrap();
        let model=EngineBackedModel::new(DeterministicTestEngine::new(id.clone()));
        let request=ChatRequest {
            request_id:RequestId::new("r").unwrap(),
            session_id:SessionId::new("s").unwrap(),
            agent_id:AgentId::new("a").unwrap(),
            model_id:id,
            messages:vec![ChatMessage::new(MessageRole::User,"hello").unwrap()],
        };
        assert!(model.complete(&request).unwrap().message.content.starts_with("inference:"));
    }
}
