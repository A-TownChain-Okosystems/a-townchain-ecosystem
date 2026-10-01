use atc_aurora_core::AuroraCore;
use atc_aurora_memory::{ContextWindow, MemoryIndex};

/// Userspace runtime boundary connecting Aurora Core with memory/context services.
pub struct AuroraRuntime {
    core: AuroraCore,
    memory: MemoryIndex,
    context: ContextWindow,
}

impl AuroraRuntime {
    pub fn new(core: AuroraCore, context_limit: usize) -> Self {
        Self {
            core,
            memory: MemoryIndex::new(),
            context: ContextWindow::new(context_limit),
        }
    }

    pub fn core(&self) -> &AuroraCore {
        &self.core
    }

    pub fn core_mut(&mut self) -> &mut AuroraCore {
        &mut self.core
    }

    pub fn remember(&mut self, content: &str, tags: Vec<String>, timestamp: u64) -> u64 {
        self.memory.add(content, tags, timestamp)
    }

    pub fn context_push(&mut self, role: &str, content: &str) {
        self.context.add(role, content);
    }

    pub fn memory_count(&self) -> usize {
        self.memory.count()
    }

    pub fn context(&self) -> String {
        self.context.get_context()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atc_aurora_core::EchoBackend;

    #[test]
    fn runtime_connects_core_and_memory() {
        let mut core = AuroraCore::new();
        core.model_hub.register_backend("shiva-1.0", EchoBackend);
        core.start().unwrap();
        core.agent_registry
            .register("test", "Test Agent", "test", vec![]);

        let mut runtime = AuroraRuntime::new(core, 1024);
        runtime.remember("hello", vec!["test".into()], 1);
        runtime.context_push("user", "hello");

        assert_eq!(runtime.memory_count(), 1);
        assert!(runtime.context().contains("user: hello"));
    }
}
