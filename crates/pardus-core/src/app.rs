use crate::config::BrowserConfig;
use pardus_debug::NetworkLog;
use std::sync::Arc;
use std::sync::Mutex;
use url::Url;

pub struct App {
    pub http_client: reqwest::Client,
    pub config: BrowserConfig,
    pub network_log: Arc<Mutex<NetworkLog>>,
}

impl App {
    pub fn new(config: BrowserConfig) -> Self {
        let mut client_builder = reqwest::Client::builder()
            .user_agent(&config.user_agent)
            .timeout(std::time::Duration::from_millis(config.timeout_ms as u64));

        // Sandbox: disable cookie store for ephemeral sessions
        if !config.sandbox.ephemeral_session {
            client_builder = client_builder.cookie_store(true);
        }

        let http_client = client_builder
            .build()
            .expect("failed to build HTTP client");

        Self {
            http_client,
            config,
            network_log: Arc::new(Mutex::new(NetworkLog::new())),
        }
    }

    /// Validate a URL against the configured security policy.
    ///
    /// Returns an parsed URL if valid, or an error if the URL violates the policy.
    pub fn validate_url(&self, url: &str) -> anyhow::Result<Url> {
        self.config.url_policy.validate(url)
    }
}
