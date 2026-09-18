//! Bounded conversation context construction.
use crate::{AuroraError, ChatMessage, ChatRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextLimits { pub max_messages: usize, pub max_chars: usize }

impl Default for ContextLimits {
    fn default() -> Self { Self { max_messages: 32, max_chars: 32_000 } }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextBuilder { limits: ContextLimits }

impl ContextBuilder {
    pub fn new(limits: ContextLimits) -> Result<Self, AuroraError> {
        if limits.max_messages == 0 || limits.max_chars == 0 { return Err(AuroraError::InvalidRequest); }
        Ok(Self { limits })
    }

    pub fn build(&self, request: &ChatRequest) -> Result<Vec<ChatMessage>, AuroraError> {
        request.validate()?;
        let mut out = Vec::new();
        let mut chars = 0usize;
        for msg in request.messages.iter().rev() {
            if out.len() >= self.limits.max_messages { break; }
            if chars.saturating_add(msg.content.len()) > self.limits.max_chars { break; }
            chars += msg.content.len();
            out.push(msg.clone());
        }
        out.reverse();
        if out.is_empty() { return Err(AuroraError::ResourceExhausted); }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AgentId, ModelId, RequestId, SessionId};
    #[test]
    fn context_is_bounded_and_keeps_recent_messages() {
        let msgs = (0..4).map(|i| ChatMessage::new(crate::MessageRole::User, format!("m{i}")).unwrap()).collect();
        let r = ChatRequest { request_id: RequestId::new("r").unwrap(), session_id: SessionId::new("s").unwrap(), agent_id: AgentId::new("a").unwrap(), model_id: ModelId::new("m").unwrap(), messages: msgs };
        let c = ContextBuilder::new(ContextLimits { max_messages: 2, max_chars: 100 }).unwrap().build(&r).unwrap();
        assert_eq!(c.len(), 2); assert_eq!(c[0].content, "m2");
    }
}
