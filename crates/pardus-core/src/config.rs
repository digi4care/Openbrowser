use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BrowserConfig {
    pub cache_dir: PathBuf,
    pub user_agent: String,
    pub timeout_ms: u32,
    pub wait_ms: u32,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            cache_dir: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join("pardus-browser"),
            user_agent: format!("PardusBrowser/{}", env!("CARGO_PKG_VERSION")),
            timeout_ms: 10_000,
            wait_ms: 3_000,
        }
    }
}
