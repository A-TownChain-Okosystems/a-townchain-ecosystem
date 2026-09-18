use crate::BrowserError;
use url::Url;

#[derive(Debug, Clone)]
pub struct BrowserPolicy {
    pub allow_http: bool,
    pub allow_https: bool,
    pub max_response_bytes: usize,
}
impl Default for BrowserPolicy {
    fn default() -> Self {
        Self { allow_http: true, allow_https: true, max_response_bytes: 16 * 1024 * 1024 }
    }
}
impl BrowserPolicy {
    pub fn validate(&self, url: &Url) -> Result<(), BrowserError> {
        match url.scheme() {
            "http" if self.allow_http => {}
            "https" if self.allow_https => {}
            scheme => return Err(BrowserError::UnsupportedScheme(scheme.to_owned())),
        }
        if url.host_str().is_none() {
            return Err(BrowserError::InvalidUrl(url.to_string()));
        }
        if url.port().is_some() && url.port().unwrap_or_default() == 0 {
            return Err(BrowserError::InvalidPort);
        }
        Ok(())
    }
}
