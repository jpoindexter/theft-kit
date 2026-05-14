use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;
use webclaw_core::ExtractionOptions;
use webclaw_fetch::{FetchClient, FetchConfig};

struct AppState {
    auth_key: Option<String>,
}

#[derive(Deserialize)]
struct ScrapeRequest {
    url: String,
    #[serde(default = "default_formats")]
    formats: Vec<String>,
    #[serde(default)]
    include_selectors: Vec<String>,
    #[serde(default)]
    exclude_selectors: Vec<String>,
    #[serde(default)]
    only_main_content: bool,
    #[serde(default)]
    timeout: Option<u64>,
}

fn default_formats() -> Vec<String> {
    vec!["markdown".to_string()]
}

#[derive(Serialize)]
struct ScrapeResponse {
    success: bool,
    data: ScrapeData,
}

#[derive(Serialize)]
struct ScrapeData {
    #[serde(skip_serializing_if = "Option::is_none")]
    markdown: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    llm: Option<String>,
    metadata: ScrapeMetadata,
}

#[derive(Serialize)]
struct ScrapeMetadata {
    title: Option<String>,
    description: Option<String>,
    #[serde(rename = "sourceURL")]
    source_url: String,
    word_count: usize,
    elapsed_ms: u128,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: String,
}

fn check_auth(state: &AppState, headers: &HeaderMap) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    let Some(ref expected) = state.auth_key else {
        return Ok(());
    };

    let auth = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    match auth {
        Some(token) if token == expected => Ok(()),
        _ => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                error: "Invalid or missing API key".to_string(),
            }),
        )),
    }
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn scrape(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<ScrapeRequest>,
) -> impl IntoResponse {
    if let Err(e) = check_auth(&state, &headers) {
        return e.into_response();
    }

    let start = Instant::now();
    let timeout_secs = req.timeout.unwrap_or(30);

    let config = FetchConfig {
        timeout: Duration::from_secs(timeout_secs),
        ..Default::default()
    };

    let client = match FetchClient::new(config) {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    success: false,
                    error: format!("Client init failed: {e}"),
                }),
            )
                .into_response();
        }
    };

    let options = ExtractionOptions {
        include_selectors: req.include_selectors,
        exclude_selectors: req.exclude_selectors,
        only_main_content: req.only_main_content,
        include_raw_html: req.formats.iter().any(|f| f == "html"),
    };

    // Try HTTP fetch first
    let extraction = match client
        .fetch_and_extract_with_options(&req.url, &options)
        .await
    {
        Ok(result) => {
            // Check if it's an SPA / empty page -- try browser fallback
            if result.metadata.word_count < 50 && result.content.links.is_empty() {
                info!(url = %req.url, "SPA detected, trying browser fallback");
                match webclaw_browser::render(
                    &req.url,
                    Duration::from_secs(timeout_secs.max(30)),
                )
                .await
                {
                    Ok(browser_result) => {
                        match webclaw_core::extract_with_options(
                            &browser_result.html,
                            Some(&req.url),
                            &options,
                        ) {
                            Ok(ext) if ext.metadata.word_count > 10 => ext,
                            _ => result, // browser didn't help, return HTTP result
                        }
                    }
                    Err(e) => {
                        warn!(url = %req.url, error = %e, "browser fallback failed");
                        result
                    }
                }
            } else {
                result
            }
        }
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse {
                    success: false,
                    error: format!("Fetch failed: {e}"),
                }),
            )
                .into_response();
        }
    };

    let elapsed = start.elapsed();

    let markdown = if req.formats.iter().any(|f| f == "markdown" || f == "llm") {
        Some(if req.formats.contains(&"llm".to_string()) {
            webclaw_core::to_llm_text(&extraction, Some(&req.url))
        } else {
            extraction.content.markdown.clone()
        })
    } else {
        None
    };

    let html = if req.formats.iter().any(|f| f == "html") {
        extraction.content.raw_html.clone()
    } else {
        None
    };

    let text = if req.formats.iter().any(|f| f == "text") {
        Some(extraction.content.plain_text.clone())
    } else {
        None
    };

    let llm = if req.formats.iter().any(|f| f == "llm") {
        Some(webclaw_core::to_llm_text(&extraction, Some(&req.url)))
    } else {
        None
    };

    (
        StatusCode::OK,
        Json(ScrapeResponse {
            success: true,
            data: ScrapeData {
                markdown,
                html,
                text,
                llm,
                metadata: ScrapeMetadata {
                    title: extraction.metadata.title,
                    description: extraction.metadata.description,
                    source_url: req.url,
                    word_count: extraction.metadata.word_count,
                    elapsed_ms: elapsed.as_millis(),
                },
            },
        }),
    )
        .into_response()
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let filter = EnvFilter::try_from_env("WEBCLAW_LOG")
        .unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let auth_key = std::env::var("WEBCLAW_AUTH_KEY").ok().filter(|k| !k.is_empty());
    if auth_key.is_none() {
        warn!("WEBCLAW_AUTH_KEY not set -- API is unauthenticated");
    }

    let state = Arc::new(AppState { auth_key });

    let host = std::env::var("WEBCLAW_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("WEBCLAW_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    // Serve web UI from ../web/ (relative to exe) or ./web/ (relative to cwd)
    let web_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("../../web")))
        .filter(|p| p.exists())
        .unwrap_or_else(|| std::path::PathBuf::from("web"));

    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/scrape", post(scrape))
        .fallback_service(ServeDir::new(&web_dir))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = format!("{host}:{port}");
    info!("webclaw-api listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
