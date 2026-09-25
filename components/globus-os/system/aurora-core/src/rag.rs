//! Bounded RAG/knowledge contracts. Storage and embedding engines are pluggable.

use crate::{AuroraError, ChatMessage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeDocument { pub id: String, pub content: String, pub source: String }

pub trait Retriever {
    fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<KnowledgeDocument>, AuroraError>;
}

pub struct ContextAugmenter<R> { retriever: R, max_documents: usize }

impl<R: Retriever> ContextAugmenter<R> {
    pub fn new(retriever: R, max_documents: usize) -> Result<Self, AuroraError> {
        if max_documents == 0 { return Err(AuroraError::InvalidRequest); }
        Ok(Self { retriever, max_documents })
    }
    pub fn augment(&self, messages: &[ChatMessage]) -> Result<Vec<ChatMessage>, AuroraError> {
        let query = messages.iter().rev().find(|m| matches!(m.role, crate::MessageRole::User))
            .ok_or(AuroraError::InvalidRequest)?.content.clone();
        let docs = self.retriever.retrieve(&query, self.max_documents)?;
        if docs.is_empty() { return Ok(messages.to_vec()); }
        let knowledge = docs.into_iter().map(|d| format!("[{}] {}\nsource: {}", d.id, d.content, d.source)).collect::<Vec<_>>().join("\n");
        let mut out=Vec::with_capacity(messages.len()+1);
        out.push(ChatMessage::new(crate::MessageRole::System, format!("Retrieved knowledge:\n{knowledge}"))?);
        out.extend_from_slice(messages);
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct R;
    impl Retriever for R { fn retrieve(&self,_:&str,limit:usize)->Result<Vec<KnowledgeDocument>,AuroraError>{ Ok((0..limit.min(1)).map(|i|KnowledgeDocument{id:i.to_string(),content:"fact".into(),source:"test".into()}).collect())}}
    #[test]
    fn retrieval_is_bounded_and_augments_context() {
        let a=ContextAugmenter::new(R,2).unwrap();
        let m=vec![ChatMessage::new(crate::MessageRole::User,"question").unwrap()];
        let out=a.augment(&m).unwrap();
        assert_eq!(out.len(),2);
        assert!(matches!(out[0].role,crate::MessageRole::System));
    }
}
