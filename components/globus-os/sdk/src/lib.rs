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
