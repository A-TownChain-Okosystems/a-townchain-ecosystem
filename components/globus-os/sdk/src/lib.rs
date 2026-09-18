//! GlobusOS application SDK facade.

pub mod abi;
pub use abi::{compatible, AbiHeader, ApiClass, ABI_MAJOR, ABI_MINOR, ABI_VERSION};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WalletHandle {
    pub address: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityHandle {
    pub id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServiceHandle {
    pub name: String,
}
pub trait IdentityProvider {
    fn identity(&self) -> IdentityHandle;
}
pub trait WalletProvider {
    fn wallet(&self) -> WalletHandle;
}
pub trait ServiceProvider {
    fn service(&self, name: &str) -> Option<ServiceHandle>;
}


/// Capability-scoped application context exposed by the SDK.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppContext {
    pub app_id: u64,
    pub capabilities: Vec<String>,
}

impl AppContext {
    pub fn new(app_id: u64) -> Self { Self { app_id, capabilities: Vec::new() } }
    pub fn with_capability(mut self, capability: impl Into<String>) -> Self {
        let capability = capability.into();
        if !capability.trim().is_empty() && !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
            self.capabilities.sort();
        }
        self
    }
    pub fn has_capability(&self, capability: &str) -> bool { self.capabilities.iter().any(|c| c == capability) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capability_context_is_deterministic() {
        let context = AppContext::new(7).with_capability("network").with_capability("identity").with_capability("network");
        assert_eq!(context.capabilities, vec!["identity", "network"]);
        assert!(context.has_capability("network"));
    }
}
