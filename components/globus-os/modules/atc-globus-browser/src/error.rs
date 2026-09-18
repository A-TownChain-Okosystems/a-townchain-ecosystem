use std::fmt;

/// Errors returned by the GlobusOS browser boundary.
#[derive(Debug)]
pub enum BrowserError {
    InvalidUrl(String),
    UnsupportedScheme(String),
    BlockedHost(String),
    InvalidPort,
    ResourceLimit(String),
    Network(reqwest::Error),
}

impl fmt::Display for BrowserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUrl(v) => write!(f, "invalid URL: {v}"),
            Self::UnsupportedScheme(v) => write!(f, "unsupported URL scheme: {v}"),
            Self::BlockedHost(v) => write!(f, "host blocked by browser policy: {v}"),
            Self::InvalidPort => write!(f, "invalid URL port"),
            Self::ResourceLimit(v) => write!(f, "browser resource limit: {v}"),
            Self::Network(e) => write!(f, "network error: {e}"),
        }
    }
}

impl std::error::Error for BrowserError {}

impl From<reqwest::Error> for BrowserError {
    fn from(value: reqwest::Error) -> Self {
        Self::Network(value)
    }
}
