// atc-aurora-core — Central AI Engine, Model Hub, LLM Router
// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.

pub mod agent_registry;
pub mod aurora_core;
pub mod config_manager;
pub mod llm_router;
pub mod model_hub;

pub use agent_registry::AgentRegistry;
pub use aurora_core::AuroraCore;
pub use config_manager::ConfigManager;
pub use llm_router::LlmRouter;
pub use model_hub::{EchoBackend, InferenceBackend, InferenceError, ModelHub, ModelInfo};
