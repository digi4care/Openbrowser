use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchRequest {
    pub url: String,
    #[serde(default = "default_method")]
    pub method: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub body: Option<String>,
}

fn default_method() -> String { "GET".to_string() }

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub ok: bool,
}

/// Execute a fetch request via reqwest.
pub async fn execute_fetch(client: reqwest::Client, request: FetchRequest) -> anyhow::Result<FetchResponse> {
    let mut builder = client.request(
        reqwest::Method::from_bytes(request.method.as_bytes())
            .unwrap_or(reqwest::Method::GET),
        &request.url,
    );

    for (k, v) in &request.headers {
        builder = builder.header(k.as_str(), v.as_str());
    }
    if let Some(body) = &request.body {
        builder = builder.body(body.clone());
    }

    let response = builder.send().await?;
    let status = response.status().as_u16();
    let status_text = response.status().canonical_reason().unwrap_or("").to_string();
    let ok = response.status().is_success();

    let mut headers = HashMap::new();
    for (k, v) in response.headers() {
        if let Ok(v_str) = v.to_str() {
            headers.insert(k.to_string(), v_str.to_string());
        }
    }

    let body = response.text().await.unwrap_or_default();

    Ok(FetchResponse {
        status,
        status_text,
        headers,
        body,
        ok,
    })
}
