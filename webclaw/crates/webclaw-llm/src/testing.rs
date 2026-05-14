/// Shared test utilities for webclaw-llm.
///
/// Provides a configurable mock LLM provider for unit tests across
/// extract, chain, and other modules that need a fake LLM backend.
#[cfg(test)]
pub(crate) mod mock {
    use async_trait::async_trait;

    use crate::error::LlmError;
    use crate::provider::{CompletionRequest, LlmProvider};

    /// A mock LLM provider that returns a canned response or error.
    /// Covers the common test cases: success, failure, and availability.
    pub struct MockProvider {
        pub name: &'static str,
        pub response: Result<String, String>,
        pub available: bool,
    }

    impl MockProvider {
        /// Shorthand for a mock that always succeeds with the given response.
        pub fn ok(response: &str) -> Self {
            Self {
                name: "mock",
                response: Ok(response.to_string()),
                available: true,
            }
        }
    }

    #[async_trait]
    impl LlmProvider for MockProvider {
        async fn complete(&self, _request: &CompletionRequest) -> Result<String, LlmError> {
            match &self.response {
                Ok(text) => Ok(text.clone()),
                Err(msg) => Err(LlmError::ProviderError(msg.clone())),
            }
        }

        async fn is_available(&self) -> bool {
            self.available
        }

        fn name(&self) -> &str {
            self.name
        }
    }
}
