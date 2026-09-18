use std::fmt;

#[derive(Debug)]
pub enum BrowserError {
    InvalidUrl(String),
    UnsupportedScheme(String),
    BlockedHost(String),
    InvalidPort,
    Network(reqwest::Error),
}

impl fmt::Display for BrowserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUrl(value) => write!(f, "invalid URL: {value}"),
            Self::UnsupportedScheme(value) => write!(f, "unsupported URL scheme: {value}"),
            Self::BlockedHost(value) => write!(f, "host blocked by browser policy: {value}"),
            Self::InvalidPort => write!(f, "invalid URL port"),
            Self::Network(error) => write!(f, "network error: {error}"),
        }
    }
}
impl std::error::Error for BrowserError {}
impl From<reqwest::Error> for BrowserError {
    fn from(value: reqwest::Error) -> Self { Self::Network(value) }
}
