//! Aurora chat orchestration. Model execution remains outside the ShivaCore TCB.
use crate::{AuroraError, ChatModel, ChatRequest, ChatResponse, ContextBuilder, ChatTool, ToolCallRequest};

#[derive(Debug)]
pub struct AuroraOrchestrator<M> { model: M, context: ContextBuilder }

impl<M: ChatModel> AuroraOrchestrator<M> {
    pub fn new(model: M, context: ContextBuilder) -> Self { Self { model, context } }

    pub fn chat(&self, request: &ChatRequest) -> Result<ChatResponse, AuroraError> {
        let messages = self.context.build(request)?;
        let bounded = ChatRequest { messages, ..request.clone() };
        self.model.complete(&bounded)
    }

    pub fn stream(
        &self,
        request: &ChatRequest,
        sink: &mut dyn FnMut(crate::ChatChunk),
    ) -> Result<(), AuroraError> {
        let messages = self.context.build(request)?;
        let bounded = ChatRequest { messages, ..request.clone() };
        self.model.stream(&bounded, sink)
    }
}

pub struct AuthorizedTool<T> { tool: T }

impl<T: ChatTool> AuthorizedTool<T> {
    pub fn new(tool: T) -> Self { Self { tool } }
    pub fn call<F>(&self, request: &ToolCallRequest, authorize: F) -> Result<String, AuroraError>
    where F: FnOnce(&ToolCallRequest) -> Result<(), AuroraError> {
        authorize(request)?;
        self.tool.call(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AgentId, ModelId, RequestId, SessionId, ChatMessage, MessageRole};
    struct Echo;
    impl ChatModel for Echo {
        fn complete(&self, r: &ChatRequest) -> Result<ChatResponse, AuroraError> {
            Ok(ChatResponse { request_id: r.request_id.clone(), message: ChatMessage::new(MessageRole::Assistant, r.messages.last().unwrap().content.clone())? })
        }
    }
    #[test]
    fn orchestration_uses_bounded_context() {
        let r=ChatRequest{request_id:RequestId::new("r").unwrap(),session_id:SessionId::new("s").unwrap(),agent_id:AgentId::new("a").unwrap(),model_id:ModelId::new("m").unwrap(),messages:vec![ChatMessage::new(MessageRole::User,"hello").unwrap()]};
        let o=AuroraOrchestrator::new(Echo, ContextBuilder::new(Default::default()).unwrap());
        assert_eq!(o.chat(&r).unwrap().message.content,"hello");
    }
    #[test]
    fn tool_authorization_runs_before_execution() {
        struct T;
        impl ChatTool for T { fn call(&self,_:&ToolCallRequest)->Result<String,AuroraError>{Ok("executed".into())} }
        let tool=AuthorizedTool::new(T);
        let req=ToolCallRequest{request_id:RequestId::new("r").unwrap(),tool_id:crate::ToolId::new("t").unwrap(),input:"x".into()};
        assert_eq!(tool.call(&req, |_| Err(AuroraError::CapabilityDenied)).unwrap_err(), AuroraError::CapabilityDenied);
    }
}
