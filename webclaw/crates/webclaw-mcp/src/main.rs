/// webclaw-mcp: MCP (Model Context Protocol) server for webclaw.
/// Exposes web extraction tools over stdio transport for AI agents
/// like Claude Desktop, Claude Code, and other MCP clients.
mod cloud;
mod server;
mod tools;

use rmcp::ServiceExt;
use rmcp::transport::stdio;

use server::WebclawMcp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Log to stderr -- stdout is the MCP transport channel
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let service = WebclawMcp::new().await.serve(stdio()).await?;

    service.waiting().await?;
    Ok(())
}
