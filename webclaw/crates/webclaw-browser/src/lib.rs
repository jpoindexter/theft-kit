//! Playwright sidecar for headless browser rendering.
//! Spawns a Node.js process running the sidecar script (sidecar/render.mjs)
//! and communicates via JSON over stdin/stdout.
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::process::Command;
use tracing::debug;

#[derive(Error, Debug)]
pub enum BrowserError {
    #[error("sidecar not found at {0}")]
    SidecarNotFound(PathBuf),
    #[error("node not found in PATH")]
    NodeNotFound,
    #[error("sidecar process failed: {0}")]
    ProcessFailed(String),
    #[error("sidecar returned error: {0}")]
    RenderError(String),
    #[error("invalid sidecar response: {0}")]
    InvalidResponse(String),
    #[error("sidecar timed out after {0}s")]
    Timeout(u64),
}

#[derive(Serialize)]
struct RenderRequest {
    url: String,
    timeout: u64,
    wait_until: String,
}

#[derive(Deserialize)]
struct RenderResponse {
    html: Option<String>,
    url: Option<String>,
    status: Option<u16>,
    error: Option<String>,
}

pub struct BrowserResult {
    pub html: String,
    pub url: String,
    pub status: u16,
}

/// Locate the sidecar script relative to the webclaw binary or workspace root.
fn find_sidecar() -> Result<PathBuf, BrowserError> {
    // 1. Check relative to the current executable (for release builds)
    if let Ok(exe) = std::env::current_exe() {
        let candidates = [
            exe.parent().map(|p| p.join("../sidecar/render.mjs")),
            exe.parent().map(|p| p.join("../../sidecar/render.mjs")),
            exe.parent()
                .map(|p| p.join("../../../sidecar/render.mjs")),
        ];
        for candidate in candidates.into_iter().flatten() {
            let resolved = candidate.canonicalize().unwrap_or(candidate);
            if resolved.exists() {
                return Ok(resolved);
            }
        }
    }

    // 2. Check WEBCLAW_SIDECAR env var
    if let Ok(path) = std::env::var("WEBCLAW_SIDECAR") {
        let p = PathBuf::from(&path);
        if p.exists() {
            return Ok(p);
        }
    }

    // 3. Check relative to CWD (for development)
    let cwd_relative = Path::new("sidecar/render.mjs");
    if cwd_relative.exists() {
        return Ok(cwd_relative.canonicalize().unwrap_or(cwd_relative.to_path_buf()));
    }

    Err(BrowserError::SidecarNotFound(PathBuf::from(
        "sidecar/render.mjs",
    )))
}

/// Render a URL using headless Chromium via the Playwright sidecar.
pub async fn render(url: &str, timeout: Duration) -> Result<BrowserResult, BrowserError> {
    let sidecar = find_sidecar()?;

    debug!(url, sidecar = %sidecar.display(), "spawning browser sidecar");

    let request = RenderRequest {
        url: url.to_string(),
        timeout: timeout.as_millis() as u64,
        wait_until: "domcontentloaded".to_string(),
    };
    let input = serde_json::to_string(&request)
        .map_err(|e| BrowserError::ProcessFailed(e.to_string()))?;

    let mut child = Command::new("node")
        .arg(&sidecar)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                BrowserError::NodeNotFound
            } else {
                BrowserError::ProcessFailed(e.to_string())
            }
        })?;

    {
        use tokio::io::AsyncWriteExt;
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(input.as_bytes())
                .await
                .map_err(|e| BrowserError::ProcessFailed(format!("stdin write: {e}")))?;
        }
    }

    let output = tokio::time::timeout(
        timeout + Duration::from_secs(10),
        child.wait_with_output(),
    )
    .await
    .map_err(|_| BrowserError::Timeout(timeout.as_secs()))?
    .map_err(|e| BrowserError::ProcessFailed(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.trim().is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(BrowserError::ProcessFailed(format!(
            "empty stdout, stderr: {stderr}"
        )));
    }

    let resp: RenderResponse = serde_json::from_str(stdout.trim())
        .map_err(|e| BrowserError::InvalidResponse(format!("{e}: {}", &stdout[..stdout.len().min(200)])))?;

    if let Some(err) = resp.error {
        return Err(BrowserError::RenderError(err));
    }

    Ok(BrowserResult {
        html: resp.html.unwrap_or_default(),
        url: resp.url.unwrap_or_else(|| url.to_string()),
        status: resp.status.unwrap_or(0),
    })
}
