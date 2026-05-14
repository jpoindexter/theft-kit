/// Core LLM abstraction. Every backend (Ollama, OpenAI, Anthropic) implements `LlmProvider`.
/// The trait is intentionally minimal — just completion and availability check.
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::LlmError;

#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    /// When true, instruct the provider to return valid JSON.
    pub json_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Send a completion request and return the assistant's text response.
    async fn complete(&self, request: &CompletionRequest) -> Result<String, LlmError>;

    /// Quick health check — is this provider reachable / configured?
    async fn is_available(&self) -> bool;

    /// Human-readable name for logging.
    fn name(&self) -> &str;
}
