//! User-controlled bounded conversation memory.
use crate::{AuroraError, ChatMessage, SessionId};

#[derive(Debug, Clone)]
pub struct ConversationMemory { session_id: SessionId, messages: Vec<ChatMessage>, max_messages: usize }

impl ConversationMemory {
    pub fn new(session_id: SessionId, max_messages: usize) -> Result<Self, AuroraError> {
        if max_messages == 0 { return Err(AuroraError::InvalidRequest); }
        Ok(Self { session_id, messages: Vec::new(), max_messages })
    }
    pub fn session_id(&self) -> &SessionId { &self.session_id }
    pub fn append(&mut self, message: ChatMessage) {
        self.messages.push(message);
        if self.messages.len() > self.max_messages {
            let excess = self.messages.len() - self.max_messages;
            self.messages.drain(..excess);
        }
    }
    pub fn messages(&self) -> &[ChatMessage] { &self.messages }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn memory_is_bounded() {
        let mut m=ConversationMemory::new(SessionId::new("s").unwrap(),2).unwrap();
        m.append(ChatMessage::new(crate::MessageRole::User,"a").unwrap());
        m.append(ChatMessage::new(crate::MessageRole::Assistant,"b").unwrap());
        m.append(ChatMessage::new(crate::MessageRole::User,"c").unwrap());
        assert_eq!(m.messages().len(),2); assert_eq!(m.messages()[0].content,"b");
    }
}
