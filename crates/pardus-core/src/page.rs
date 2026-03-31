use scraper::{Html, Selector};
use std::sync::Arc;
use url::Url;

use crate::app::App;
use crate::semantic::tree::SemanticTree;
use crate::navigation::graph::NavigationGraph;

/// A parsed web page with its DOM and metadata.
pub struct Page {
    pub url: String,
    pub status: u16,
    pub content_type: Option<String>,
    pub html: Html,
    pub base_url: String,
}

impl Page {
    /// Fetch a URL and parse it into a Page.
    pub async fn from_url(app: &Arc<App>, url: &str) -> anyhow::Result<Self> {
        let response = app.http_client.get(url).send().await?;
        let status = response.status().as_u16();
        let final_url = response.url().to_string();
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let body = response.text().await?;

        let html = Html::parse_document(&body);

        // Resolve base URL from <base> tag or use final URL
        let base_url = Self::extract_base_url(&html, &final_url);

        Ok(Self {
            url: final_url,
            status,
            content_type,
            html,
            base_url,
        })
    }

    /// Parse an HTML string (useful for testing).
    pub fn from_html(html_str: &str, url: &str) -> Self {
        let html = Html::parse_document(html_str);
        let base_url = Self::extract_base_url(&html, url);
        Self {
            url: url.to_string(),
            status: 200,
            content_type: Some("text/html".to_string()),
            html,
            base_url,
        }
    }

    /// Extract the page title.
    pub fn title(&self) -> Option<String> {
        let selector = Selector::parse("title").ok()?;
        self.html
            .select(&selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
    }

    /// Build the semantic tree from this page's DOM.
    pub fn semantic_tree(&self) -> SemanticTree {
        SemanticTree::build(&self.html, &self.base_url)
    }

    /// Build the navigation graph from this page.
    pub fn navigation_graph(&self) -> NavigationGraph {
        NavigationGraph::build(&self.html, &self.url)
    }

    fn extract_base_url(html: &Html, fallback: &str) -> String {
        if let Ok(selector) = Selector::parse("base[href]") {
            if let Some(base_el) = html.select(&selector).next() {
                if let Some(href) = base_el.value().attr("href") {
                    if let Ok(resolved) = Url::parse(fallback)
                        .and_then(|base| base.join(href))
                    {
                        return resolved.to_string();
                    }
                }
            }
        }
        fallback.to_string()
    }
}
