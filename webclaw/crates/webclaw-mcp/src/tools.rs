/// Tool parameter structs for MCP tool inputs.
/// Each struct derives JsonSchema for automatic schema generation,
/// and Deserialize for parsing from MCP tool call arguments.
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScrapeParams {
    /// URL to scrape
    pub url: String,
    /// Output format: "markdown" (default), "llm", "text", or "json"
    pub format: Option<String>,
    /// CSS selectors to include (only extract matching elements)
    pub include_selectors: Option<Vec<String>>,
    /// CSS selectors to exclude from output
    pub exclude_selectors: Option<Vec<String>>,
    /// If true, extract only the main content (article/main element)
    pub only_main_content: Option<bool>,
    /// Browser profile: "chrome" (default), "firefox", or "random"
    pub browser: Option<String>,
    /// Cookies to send with the request (e.g. ["name=value", "session=abc123"])
    pub cookies: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CrawlParams {
    /// Seed URL to start crawling from
    pub url: String,
    /// Maximum link depth to follow (default: 2)
    pub depth: Option<u32>,
    /// Maximum number of pages to crawl (default: 50)
    pub max_pages: Option<usize>,
    /// Number of concurrent requests (default: 5)
    pub concurrency: Option<usize>,
    /// Seed the frontier from sitemap discovery before crawling
    pub use_sitemap: Option<bool>,
    /// Output format for each page: "markdown" (default), "llm", "text"
    pub format: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MapParams {
    /// Base URL to discover sitemaps from (e.g. `<https://example.com>`)
    pub url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BatchParams {
    /// List of URLs to extract content from
    pub urls: Vec<String>,
    /// Output format: "markdown" (default), "llm", "text"
    pub format: Option<String>,
    /// Number of concurrent requests (default: 5)
    pub concurrency: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExtractParams {
    /// URL to fetch and extract structured data from
    pub url: String,
    /// Natural language prompt describing what to extract
    pub prompt: Option<String>,
    /// JSON schema describing the structure to extract
    pub schema: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SummarizeParams {
    /// URL to fetch and summarize
    pub url: String,
    /// Number of sentences in the summary (default: 3)
    pub max_sentences: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DiffParams {
    /// URL to fetch current content from
    pub url: String,
    /// Previous extraction snapshot as a JSON string (ExtractionResult)
    pub previous_snapshot: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BrandParams {
    /// URL to extract brand identity from
    pub url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResearchParams {
    /// Research query or question to investigate
    pub query: String,
    /// Enable deep research mode for more thorough investigation (default: false)
    pub deep: Option<bool>,
    /// Topic hint to guide research focus (e.g. "technology", "finance", "science")
    pub topic: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchParams {
    /// Search query
    pub query: String,
    /// Number of results to return (default: 10)
    pub num_results: Option<u32>,
}
