//! Internet-browser foundation for GlobusOS.
mod browser;
mod error;
mod history;
mod policy;
#[allow(missing_docs)]
mod tabs;

pub use browser::{Browser, BrowserConfig, BrowserResponse};
pub use error::BrowserError;
pub use history::{History, HistoryEntry};
pub use policy::BrowserPolicy;
pub use tabs::{Tab, TabId, TabManager};
