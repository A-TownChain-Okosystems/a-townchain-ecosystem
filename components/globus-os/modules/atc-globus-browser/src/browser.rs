use crate::{BrowserError, BrowserPolicy, History};
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use url::Url;

#[derive(Debug, Clone)]
pub struct BrowserConfig {
    pub policy: BrowserPolicy,
    pub user_agent: String,
    pub timeout_seconds: u64,
}
impl Default for BrowserConfig {
    fn default() -> Self {
        Self { policy: BrowserPolicy::default(), user_agent: "GlobusOS-Browser/0.1".into(), timeout_seconds: 20 }
    }
}
#[derive(Debug, Clone)]
pub struct BrowserResponse {
    pub url: String,
    pub status: u16,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
}
pub struct Browser {
    client: Client,
    config: BrowserConfig,
    history: History,
}
impl Browser {
    pub fn new(config: BrowserConfig) -> Result<Self, BrowserError> {
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::limited(10))
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .user_agent(config.user_agent.clone())
            .build()?;
        Ok(Self { client, config, history: History::new() })
    }
    pub fn navigate(&mut self, raw_url: &str) -> Result<BrowserResponse, BrowserError> {
        let url = Url::parse(raw_url).map_err(|_| BrowserError::InvalidUrl(raw_url.into()))?;
        self.config.policy.validate(&url)?;
        let response = self.client.get(url).header(USER_AGENT, self.config.user_agent.clone()).send()?;
        let final_url = response.url().clone();
        self.config.policy.validate(&final_url)?;
        let body = response.bytes()?.to_vec();
        if body.len() > self.config.policy.max_response_bytes {
            return Err(BrowserError::InvalidUrl("response exceeds configured size limit".into()));
        }
        let content_type = response.headers().get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok()).map(str::to_owned);
        let status = response.status().as_u16();
        self.history.push(final_url.to_string());
        Ok(BrowserResponse { url: final_url.to_string(), status, content_type, body })
    }
    pub fn back(&mut self) -> Option<&str> { self.history.back().map(|e| e.url.as_str()) }
    pub fn forward(&mut self) -> Option<&str> { self.history.forward().map(|e| e.url.as_str()) }
    pub fn current_url(&self) -> Option<&str> { self.history.current().map(|e| e.url.as_str()) }
    pub fn history(&self) -> &History { &self.history }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_non_web_scheme() {
        let policy = BrowserPolicy::default();
        let url = Url::parse("file:///etc/passwd").expect("parse");
        assert!(policy.validate(&url).is_err());
    }
    #[test]
    fn rejects_hostless_url() {
        let policy = BrowserPolicy::default();
        let url = Url::parse("https:///missing-host").expect("parse");
        assert!(policy.validate(&url).is_err());
    }
}
