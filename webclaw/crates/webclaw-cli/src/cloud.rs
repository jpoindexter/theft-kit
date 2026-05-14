/// Cloud API client for automatic fallback when local extraction fails.
///
/// When WEBCLAW_API_KEY is set (or --api-key is passed), the CLI can fall back
/// to api.webclaw.io for bot-protected or JS-rendered sites. With --cloud flag,
/// all requests go through the cloud API directly.
///
/// NOTE: The canonical, full-featured cloud module lives in webclaw-mcp/src/cloud.rs
/// (smart_fetch, bot detection, JS rendering checks). This is the minimal subset
/// needed by the CLI. Kept separate to avoid pulling in rmcp via webclaw-mcp.
/// and adding webclaw-mcp as a dependency would pull in rmcp.
use serde_json::{Value, json};

const API_BASE: &str = "https://api.webclaw.io/v1";

pub struct CloudClient {
    api_key: String,
    http: reqwest::Client,
}

impl CloudClient {
    /// Create from explicit key or WEBCLAW_API_KEY env var.
    pub fn new(explicit_key: Option<&str>) -> Option<Self> {
        let key = explicit_key
            .map(String::from)
            .or_else(|| std::env::var("WEBCLAW_API_KEY").ok())
            .filter(|k| !k.is_empty())?;

        Some(Self {
            api_key: key,
            http: reqwest::Client::new(),
        })
    }

    /// Scrape via the cloud API.
    pub async fn scrape(
        &self,
        url: &str,
        formats: &[&str],
        include_selectors: &[String],
        exclude_selectors: &[String],
        only_main_content: bool,
    ) -> Result<Value, String> {
        let mut body = json!({
            "url": url,
            "formats": formats,
        });
        if only_main_content {
            body["only_main_content"] = json!(true);
        }
        if !include_selectors.is_empty() {
            body["include_selectors"] = json!(include_selectors);
        }
        if !exclude_selectors.is_empty() {
            body["exclude_selectors"] = json!(exclude_selectors);
        }
        self.post("scrape", body).await
    }

    async fn post(&self, endpoint: &str, body: Value) -> Result<Value, String> {
        let resp = self
            .http
            .post(format!("{API_BASE}/{endpoint}"))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .await
            .map_err(|e| format!("cloud API request failed: {e}"))?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("cloud API error {status}: {text}"));
        }

        resp.json::<Value>()
            .await
            .map_err(|e| format!("cloud API response parse failed: {e}"))
    }
}
