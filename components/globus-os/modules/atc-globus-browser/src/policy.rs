use crate::BrowserError;
use url::Url;

/// URL and response-size policy enforced before every browser request.
#[derive(Debug, Clone)]
pub struct BrowserPolicy {
    /// Whether clear-text HTTP navigation is permitted.
    pub allow_http: bool,
    /// Whether HTTPS navigation is permitted.
    pub allow_https: bool,
    /// Maximum response body size retained in memory.
    pub max_response_bytes: usize,
}

impl Default for BrowserPolicy {
    fn default() -> Self {
        Self { allow_http: true, allow_https: true, max_response_bytes: 16 * 1024 * 1024 }
    }
}

impl BrowserPolicy {
    /// Validates a navigation target against the browser's URL policy.
    pub fn validate(&self, url: &Url) -> Result<(), BrowserError> {
        match url.scheme() {
            "http" if self.allow_http => {}
            "https" if self.allow_https => {}
            scheme => return Err(BrowserError::UnsupportedScheme(scheme.to_owned())),
        }
        if url.host_str().is_none() {
            return Err(BrowserError::InvalidUrl(url.to_string()));
        }
        if url.port() == Some(0) {
            return Err(BrowserError::InvalidPort);
        }
        if self.max_response_bytes == 0 {
            return Err(BrowserError::ResourceLimit("maximum response size must be greater than zero".into()));
        }
        Ok(())
    }
}
