use std::sync::Arc;
use crate::config::BrowserConfig;

/// Shared application context passed through the browser session.
pub struct App {
    pub http_client: reqwest::Client,
    pub config: BrowserConfig,
}

impl App {
    pub fn new(config: BrowserConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .user_agent(&config.user_agent)
            .timeout(std::time::Duration::from_millis(config.timeout_ms as u64))
            .cookie_store(true)
            .build()
            .expect("failed to build HTTP client");

        Self { http_client, config }
    }

    pub fn default() -> Arc<Self> {
        Arc::new(Self::new(BrowserConfig::default()))
    }
}
