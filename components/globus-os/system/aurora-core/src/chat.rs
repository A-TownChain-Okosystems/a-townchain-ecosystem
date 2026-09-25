//! ChatGPT-like Aurora conversation primitives.
//! This crate layer is model-provider neutral: no vendor/model is trusted implicitly.

use crate::{AgentId, AuroraError, ModelId, RequestId, SessionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageRole { System, User, Assistant, Tool }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

impl ChatMessage {
    pub fn new(role: MessageRole, content: impl Into<String>) -> Result<Self, AuroraError> {
        let content = content.into();
        if content.trim().is_empty() { return Err(AuroraError::InvalidRequest); }
        Ok(Self { role, content })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatRequest {
    pub request_id: RequestId,
    pub session_id: SessionId,
    pub agent_id: AgentId,
    pub model_id: ModelId,
    pub messages: Vec<ChatMessage>,
}

impl ChatRequest {
    pub fn validate(&self) -> Result<(), AuroraError> {
        if self.messages.is_empty() { return Err(AuroraError::InvalidRequest); }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatResponse {
    pub request_id: RequestId,
    pub message: ChatMessage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatChunk {
    pub request_id: RequestId,
    pub sequence: u64,
    pub delta: String,
    pub done: bool,
}

pub trait ChatModel {
    fn complete(&self, request: &ChatRequest) -> Result<ChatResponse, AuroraError>;

    fn stream(
        &self,
        request: &ChatRequest,
        sink: &mut dyn FnMut(ChatChunk),
    ) -> Result<(), AuroraError> {
        let response = self.complete(request)?;
        sink(ChatChunk {
            request_id: response.request_id,
            sequence: 0,
            delta: response.message.content,
            done: true,
        });
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallRequest {
    pub request_id: RequestId,
    pub tool_id: crate::ToolId,
    pub input: String,
}

pub trait ChatTool {
    fn call(&self, request: &ToolCallRequest) -> Result<String, AuroraError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Echo;
    impl ChatModel for Echo {
        fn complete(&self, request: &ChatRequest) -> Result<ChatResponse, AuroraError> {
            let msg = request.messages.last().unwrap();
            Ok(ChatResponse {
                request_id: request.request_id.clone(),
                message: ChatMessage::new(MessageRole::Assistant, msg.content.clone())?,
            })
        }
    }

    #[test]
    fn rejects_empty_chat() {
        let r = ChatRequest {
            request_id: RequestId::new("r").unwrap(),
            session_id: SessionId::new("s").unwrap(),
            agent_id: AgentId::new("a").unwrap(),
            model_id: ModelId::new("m").unwrap(),
            messages: Vec::new(),
        };
        assert!(r.validate().is_err());
    }

    #[test]
    fn model_is_provider_neutral() {
        let request = ChatRequest {
            request_id: RequestId::new("r").unwrap(),
            session_id: SessionId::new("s").unwrap(),
            agent_id: AgentId::new("a").unwrap(),
            model_id: ModelId::new("m").unwrap(),
            messages: vec![ChatMessage::new(MessageRole::User, "hello").unwrap()],
        };
        let response = Echo.complete(&request).unwrap();
        assert_eq!(response.message.role, MessageRole::Assistant);
    }

    #[test]
    fn default_stream_emits_terminal_chunk() {
        let request = ChatRequest {
            request_id: RequestId::new("r").unwrap(),
            session_id: SessionId::new("s").unwrap(),
            agent_id: AgentId::new("a").unwrap(),
            model_id: ModelId::new("m").unwrap(),
            messages: vec![ChatMessage::new(MessageRole::User, "hello").unwrap()],
        };
        let mut chunks = Vec::new();
        Echo.stream(&request, &mut |c| chunks.push(c)).unwrap();
        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].done);
    }
}
