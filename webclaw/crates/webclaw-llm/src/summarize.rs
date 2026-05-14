/// LLM-powered content summarization. Keeps it simple: one function, one prompt.
use crate::clean::strip_thinking_tags;
use crate::error::LlmError;
use crate::provider::{CompletionRequest, LlmProvider, Message};

/// Summarize content using an LLM.
/// Returns plain text (not JSON). Default is 3 sentences.
pub async fn summarize(
    content: &str,
    max_sentences: Option<usize>,
    provider: &dyn LlmProvider,
    model: Option<&str>,
) -> Result<String, LlmError> {
    let n = max_sentences.unwrap_or(3);

    let system = format!(
        "You are a summarization engine. Summarize the following content in exactly {n} sentences. \
         Output ONLY the summary, nothing else. No introductions, no questions, no formatting, no preamble."
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
        temperature: Some(0.3),
        max_tokens: None,
        json_mode: false,
    };

    let response = provider.complete(&request).await?;

    // Providers already strip thinking tags, but defense in depth for summarize
    // since its output goes directly to the user as plain text
    Ok(strip_thinking_tags(&response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockSummarizer;

    #[async_trait]
    impl LlmProvider for MockSummarizer {
        async fn complete(&self, req: &CompletionRequest) -> Result<String, LlmError> {
            // Verify the prompt is well-formed
            let system = &req.messages[0].content;
            assert!(system.contains("sentences"));
            assert!(system.contains("summarization engine"));
            assert!(!req.json_mode, "summarize should not use json_mode");
            Ok("This is a test summary.".into())
        }
        async fn is_available(&self) -> bool {
            true
        }
        fn name(&self) -> &str {
            "mock"
        }
    }

    #[tokio::test]
    async fn summarize_returns_text() {
        let result = summarize("Long article content...", None, &MockSummarizer, None)
            .await
            .unwrap();
        assert_eq!(result, "This is a test summary.");
    }

    #[tokio::test]
    async fn summarize_custom_sentence_count() {
        // Verify custom count is passed through
        struct CountChecker;

        #[async_trait]
        impl LlmProvider for CountChecker {
            async fn complete(&self, req: &CompletionRequest) -> Result<String, LlmError> {
                assert!(req.messages[0].content.contains("5 sentences"));
                Ok("Summary.".into())
            }
            async fn is_available(&self) -> bool {
                true
            }
            fn name(&self) -> &str {
                "count_checker"
            }
        }

        summarize("Content", Some(5), &CountChecker, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn summarize_strips_thinking_tags() {
        struct ThinkingMock;

        #[async_trait]
        impl LlmProvider for ThinkingMock {
            async fn complete(&self, _req: &CompletionRequest) -> Result<String, LlmError> {
                Ok("<think>let me analyze this</think>This is the clean summary.".into())
            }
            async fn is_available(&self) -> bool {
                true
            }
            fn name(&self) -> &str {
                "thinking_mock"
            }
        }

        let result = summarize("Some content", None, &ThinkingMock, None)
            .await
            .unwrap();
        assert_eq!(result, "This is the clean summary.");
    }
}
