// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Model Hub — model registry and pluggable inference backends
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub version: String,
    pub context_window: usize,
    pub is_loaded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferenceError {
    ModelNotFound(String),
    BackendNotConfigured(String),
    Backend(String),
}

impl fmt::Display for InferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ModelNotFound(name) => write!(f, "model '{}' not found", name),
            Self::BackendNotConfigured(name) => write!(f, "no inference backend configured for '{}'", name),
            Self::Backend(message) => write!(f, "inference backend error: {}", message),
        }
    }
}

impl Error for InferenceError {}

/// Backend boundary for real model providers.
///
/// Implementations may connect to local inference, a userspace model server,
/// or an explicitly authorized remote provider. The core never grants a backend
/// kernel or chain authority implicitly.
pub trait InferenceBackend: Send + Sync {
    fn infer(&self, model: &ModelInfo, prompt: &str) -> Result<String, InferenceError>;
}

/// Explicit development/test backend. It is not a model implementation.
#[derive(Debug, Clone, Copy, Default)]
pub struct EchoBackend;

impl InferenceBackend for EchoBackend {
    fn infer(&self, model: &ModelInfo, prompt: &str) -> Result<String, InferenceError> {
        let preview: String = prompt.chars().take(40).collect();
        Ok(format!(
            "[{}: {} → response ({} chars)]",
            model.name,
            preview,
            prompt.chars().count() * 2
        ))
    }
}

pub struct ModelHub {
    models: HashMap<String, ModelInfo>,
    backends: HashMap<String, Arc<dyn InferenceBackend>>,
    default_model: String,
}

impl Default for ModelHub {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelHub {
    pub fn new() -> Self {
        let mut hub = Self {
            models: HashMap::new(),
            backends: HashMap::new(),
            default_model: String::new(),
        };
        hub.register("shiva-1.0", "1.0.0", 8192);
        hub.set_default("shiva-1.0").expect("built-in model must exist");
        hub
    }

    pub fn register(&mut self, name: &str, version: &str, ctx: usize) {
        self.models.insert(
            name.into(),
            ModelInfo {
                name: name.into(),
                version: version.into(),
                context_window: ctx,
                is_loaded: false,
            },
        );
    }

    pub fn register_backend<B>(&mut self, model: &str, backend: B)
    where
        B: InferenceBackend + 'static,
    {
        self.backends.insert(model.into(), Arc::new(backend));
        if let Some(info) = self.models.get_mut(model) {
            info.is_loaded = true;
        }
    }

    pub fn set_default(&mut self, name: &str) -> Result<(), InferenceError> {
        if !self.models.contains_key(name) {
            return Err(InferenceError::ModelNotFound(name.into()));
        }
        self.default_model = name.into();
        Ok(())
    }

    pub fn get_default(&self) -> &str {
        &self.default_model
    }

    pub fn inference(&self, model: &str, prompt: &str) -> Result<String, InferenceError> {
        let info = self
            .models
            .get(model)
            .ok_or_else(|| InferenceError::ModelNotFound(model.into()))?;

        if prompt.chars().count() > info.context_window {
            return Err(InferenceError::Backend(format!(
                "prompt exceeds context window of {}",
                info.context_window
            )));
        }

        let backend = self
            .backends
            .get(model)
            .ok_or_else(|| InferenceError::BackendNotConfigured(model.into()))?;

        backend.infer(info, prompt)
    }

    pub fn list_models(&self) -> Vec<&ModelInfo> {
        self.models.values().collect()
    }

    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    pub fn backend_configured(&self, model: &str) -> bool {
        self.backends.contains_key(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_hub_requires_backend() {
        let hub = ModelHub::new();
        assert!(!hub.backend_configured("shiva-1.0"));
        assert_eq!(
            hub.inference("shiva-1.0", "test prompt").unwrap_err(),
            InferenceError::BackendNotConfigured("shiva-1.0".into())
        );
    }

    #[test]
    fn test_model_hub() {
        let mut hub = ModelHub::new();
        hub.register("shiva-2.0", "2.0.0", 32768);
        hub.register_backend("shiva-1.0", EchoBackend);
        assert_eq!(hub.model_count(), 2);
        let r = hub.inference("shiva-1.0", "test prompt").unwrap();
        assert!(r.contains("shiva-1.0"));
    }

    #[test]
    fn test_inference_accepts_unicode_without_panicking() {
        let mut hub = ModelHub::new();
        hub.register_backend("shiva-1.0", EchoBackend);
        let prompt = "Ä".repeat(100);
        let response = hub.inference("shiva-1.0", &prompt).unwrap();
        assert!(response.contains(&"Ä".repeat(40)));
    }
}
