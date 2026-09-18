//! Local model inference adapter.
//! Runs a locally installed inference executable without granting it ShivaCore authority.
//! The executable receives the prompt on stdin and returns the generated response on stdout.

use crate::{AgentId, AuroraError, InferenceEngine, ModelId};
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalInferenceConfig {
    pub model_id: ModelId,
    pub executable: String,
    pub arguments: Vec<String>,
}

impl LocalInferenceConfig {
    pub fn new(model_id: ModelId, executable: impl Into<String>) -> Result<Self, AuroraError> {
        let executable = executable.into();
        if executable.trim().is_empty() {
            return Err(AuroraError::InvalidRequest);
        }
        Ok(Self { model_id, executable, arguments: Vec::new() })
    }

    pub fn with_arguments(mut self, arguments: Vec<String>) -> Self {
        self.arguments = arguments;
        self
    }
}

pub struct LocalInferenceEngine {
    config: LocalInferenceConfig,
}

impl LocalInferenceEngine {
    pub fn new(config: LocalInferenceConfig) -> Self { Self { config } }

    pub fn config(&self) -> &LocalInferenceConfig { &self.config }
}

impl InferenceEngine for LocalInferenceEngine {
    fn model_id(&self) -> &ModelId { &self.config.model_id }

    fn infer(&self, _agent: &AgentId, prompt: &str) -> Result<String, AuroraError> {
        if prompt.trim().is_empty() {
            return Err(AuroraError::InvalidRequest);
        }

        let mut child = Command::new(&self.config.executable)
            .args(&self.config.arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| AuroraError::ResourceExhausted)?;

        child.stdin.take()
            .ok_or(AuroraError::ResourceExhausted)?
            .write_all(prompt.as_bytes())
            .map_err(|_| AuroraError::ResourceExhausted)?;

        let output = child.wait_with_output()
            .map_err(|_| AuroraError::ResourceExhausted)?;

        if !output.status.success() {
            return Err(AuroraError::ResourceExhausted);
        }

        let response = String::from_utf8(output.stdout)
            .map_err(|_| AuroraError::InvalidRequest)?;
        if response.trim().is_empty() {
            return Err(AuroraError::InvalidRequest);
        }
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_executable() {
        assert!(LocalInferenceConfig::new(ModelId::new("local").unwrap(), "").is_err());
    }

    #[cfg(unix)]
    #[test]
    fn executes_a_local_process() {
        let cfg = LocalInferenceConfig::new(ModelId::new("local-test").unwrap(), "cat").unwrap();
        let engine = LocalInferenceEngine::new(cfg);
        let output = engine.infer(&AgentId::new("agent").unwrap(), "hello-local").unwrap();
        assert_eq!(output, "hello-local");
    }
}
