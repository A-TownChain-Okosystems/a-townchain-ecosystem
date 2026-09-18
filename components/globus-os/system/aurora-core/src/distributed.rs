//! Distributed inference contracts. Networking, transport and consensus remain external.

use crate::{AgentId, AuroraError, ModelId, RequestId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferenceTask {
    pub request_id: RequestId,
    pub agent_id: AgentId,
    pub model_id: ModelId,
    pub prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerDescriptor { pub worker_id: String, pub model_id: ModelId, pub endpoint: String }

pub trait InferenceWorker {
    fn descriptor(&self) -> &WorkerDescriptor;
    fn execute(&self, task: &InferenceTask) -> Result<String, AuroraError>;
}

pub trait WorkerRouter {
    fn route(&self, task: &InferenceTask) -> Result<String, AuroraError>;
}

pub struct LocalWorker<W> { worker: W }

impl<W: InferenceWorker> LocalWorker<W> {
    pub fn new(worker: W) -> Self { Self { worker } }
}

impl<W: InferenceWorker> WorkerRouter for LocalWorker<W> {
    fn route(&self, task: &InferenceTask) -> Result<String, AuroraError> {
        self.worker.execute(task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct W(WorkerDescriptor);
    impl InferenceWorker for W {
        fn descriptor(&self)->&WorkerDescriptor { &self.0 }
        fn execute(&self,_:&InferenceTask)->Result<String,AuroraError>{Ok("worker-result".into())}
    }
    #[test]
    fn local_worker_routes_task() {
        let model=ModelId::new("m").unwrap();
        let w=LocalWorker::new(W(WorkerDescriptor{worker_id:"w".into(),model_id:model.clone(),endpoint:"local".into()}));
        let t=InferenceTask{request_id:RequestId::new("r").unwrap(),agent_id:AgentId::new("a").unwrap(),model_id:model,prompt:"x".into()};
        assert_eq!(w.route(&t).unwrap(),"worker-result");
    }
}
