/// Schema-based and prompt-based LLM extraction.
/// Both functions build a system prompt, send content to the LLM, and parse JSON back.
use crate::clean::strip_thinking_tags;
use crate::error::LlmError;
use crate::provider::{CompletionRequest, LlmProvider, Message};

/// Extract structured JSON from content using a JSON schema.
/// The schema tells the LLM exactly what fields to extract and their types.
pub async fn extract_json(
    content: &str,
    schema: &serde_json::Value,
    provider: &dyn LlmProvider,
    model: Option<&str>,
) -> Result<serde_json::Value, LlmError> {
    let system = format!(
        "You are a JSON extraction engine. Extract data from the content according to this schema.\n\
         Return ONLY valid JSON matching the schema. No explanations, no markdown, no commentary.\n\n\
         Schema:\n```json\n{}\n```",
        serde_json::to_string_pretty(schema).unwrap_or_else(|_| schema.to_string())
    );

    let request = CompletionRequest {
        model: model.unwrap_or_default().to_string(),
        messages: vec![
            Message {
                role: "system".into(),
                content: system,
            },
            Message {
                role: "user".into(),
                content: content.to_string(),
            },
        ],
        temperature: Some(0.0),
        max_tokens: None,
        json_mode: true,
    };

    let response = provider.complete(&request).await?;
    parse_json_response(&response)
}

/// Extract information using a natural language prompt.
/// More flexible than schema extraction — the user describes what they want.
pub async fn extract_with_prompt(
    content: &str,
    prompt: &str,
    provider: &dyn LlmProvider,
    model: Option<&str>,
) -> Result<serde_json::Value, LlmError> {
    let system = format!(
        "You are a JSON extraction engine. Extract information from the content based on these instructions.\n\
         Return ONLY valid JSON. No explanations, no markdown, no commentary.\n\n\
         Instructions: {prompt}"
    );

    let request = CompletionRequest {
        model: model.unwrap_or_default().to_string(),
        messages: vec![
            Message {
                role: "system".into(),
                content: system,
            },
            Message {
                role: "user".into(),
                content: content.to_string(),
            },
        ],
        temperature: Some(0.0),
        max_tokens: None,
        json_mode: true,
    };

    let response = provider.complete(&request).await?;
    parse_json_response(&response)
}

/// Parse an LLM response string as JSON. Handles common edge cases:
/// - Thinking tags (`<think>...</think>`)
/// - Markdown code fences (```json ... ```)
/// - Leading/trailing whitespace
fn parse_json_response(response: &str) -> Result<serde_json::Value, LlmError> {
    // Strip thinking tags before any JSON parsing — providers already do this,
    // but defense in depth for any caller that bypasses the provider layer
    let cleaned = strip_thinking_tags(response);
    let trimmed = cleaned.trim();

    // Strip markdown code fences if present
    let json_str = if trimmed.starts_with("```") {
        let without_opener = trimmed
            .strip_prefix("```json")
            .or_else(|| trimmed.strip_prefix("```"))
            .unwrap_or(trimmed);
        without_opener
            .strip_suffix("```")
            .unwrap_or(without_opener)
            .trim()
    } else {
        trimmed
    };

    serde_json::from_str(json_str)
        .map_err(|e| LlmError::InvalidJson(format!("{e} — raw response: {response}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::mock::MockProvider;

    #[test]
    fn parse_clean_json() {
        let result = parse_json_response(r#"{"name": "Rust", "version": 2024}"#).unwrap();
        assert_eq!(result["name"], "Rust");
        assert_eq!(result["version"], 2024);
    }

    #[test]
    fn parse_json_with_code_fence() {
        let response = "```json\n{\"key\": \"value\"}\n```";
        let result = parse_json_response(response).unwrap();
        assert_eq!(result["key"], "value");
    }

    #[test]
    fn parse_json_with_whitespace() {
        let response = "  \n  {\"ok\": true}  \n  ";
        let result = parse_json_response(response).unwrap();
        assert_eq!(result["ok"], true);
    }

    #[test]
    fn parse_invalid_json() {
        let result = parse_json_response("not json at all");
        assert!(matches!(result, Err(LlmError::InvalidJson(_))));
    }

    #[test]
    fn parse_json_with_thinking_tags() {
        let response = "<think>analyzing the content</think>{\"title\": \"Hello\"}";
        let result = parse_json_response(response).unwrap();
        assert_eq!(result["title"], "Hello");
    }

    #[test]
    fn parse_json_with_thinking_and_code_fence() {
        let response = "<think>let me think</think>\n```json\n{\"key\": \"value\"}\n```";
        let result = parse_json_response(response).unwrap();
        assert_eq!(result["key"], "value");
    }

    #[tokio::test]
    async fn extract_json_uses_schema_in_prompt() {
        let mock = MockProvider::ok(r#"{"title": "Test Article", "author": "Jane"}"#);

        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "title": { "type": "string" },
                "author": { "type": "string" }
            }
        });

        let result = extract_json("Some article content by Jane", &schema, &mock, None)
            .await
            .unwrap();

        assert_eq!(result["title"], "Test Article");
        assert_eq!(result["author"], "Jane");
    }

    #[tokio::test]
    async fn extract_with_prompt_returns_json() {
        let mock = MockProvider::ok(r#"{"emails": ["test@example.com"]}"#);

        let result = extract_with_prompt(
            "Contact us at test@example.com",
            "Find all email addresses",
            &mock,
            None,
        )
        .await
        .unwrap();

        assert_eq!(result["emails"][0], "test@example.com");
    }
}
