//! Model provider abstraction.
//!
//! This file defines the first provider-agnostic interface used by future
//! supervised agent workflows. It intentionally does not implement real API
//! calls or provider-specific configuration.

use crate::{ModelRequest, ModelResponse, MonadError, MonadResult};

/// Capability metadata for a model provider.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ModelProviderCapabilities {
    supports_chat: bool,
    supports_streaming: bool,
    supports_tool_calls: bool,
    supports_local_execution: bool,
}

impl ModelProviderCapabilities {
    /// Creates provider capability metadata.
    #[must_use]
    pub const fn new(
        supports_chat: bool,
        supports_streaming: bool,
        supports_tool_calls: bool,
        supports_local_execution: bool,
    ) -> Self {
        Self {
            supports_chat,
            supports_streaming,
            supports_tool_calls,
            supports_local_execution,
        }
    }

    /// Capabilities for the built-in mock provider.
    #[must_use]
    pub const fn mock() -> Self {
        Self::new(true, false, false, true)
    }

    /// Returns true when chat-style requests are supported.
    #[must_use]
    pub const fn supports_chat(&self) -> bool {
        self.supports_chat
    }

    /// Returns true when streaming responses are supported.
    #[must_use]
    pub const fn supports_streaming(&self) -> bool {
        self.supports_streaming
    }

    /// Returns true when tool calls are supported.
    #[must_use]
    pub const fn supports_tool_calls(&self) -> bool {
        self.supports_tool_calls
    }

    /// Returns true when execution is local/self-contained.
    #[must_use]
    pub const fn supports_local_execution(&self) -> bool {
        self.supports_local_execution
    }
}

/// Stable provider metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelProviderMetadata {
    provider_id: String,
    display_name: String,
    default_model_id: Option<String>,
    capabilities: ModelProviderCapabilities,
}

impl ModelProviderMetadata {
    /// Creates provider metadata.
    #[must_use]
    pub fn new(
        provider_id: impl Into<String>,
        display_name: impl Into<String>,
        default_model_id: Option<String>,
        capabilities: ModelProviderCapabilities,
    ) -> Self {
        Self {
            provider_id: provider_id.into(),
            display_name: display_name.into(),
            default_model_id,
            capabilities,
        }
    }

    /// Returns the stable provider ID.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    /// Returns the human-readable provider name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Returns the default model ID, if known.
    #[must_use]
    pub fn default_model_id(&self) -> Option<&str> {
        self.default_model_id.as_deref()
    }

    /// Returns provider capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> ModelProviderCapabilities {
        self.capabilities
    }
}

/// Provider abstraction for model-assisted output.
///
/// The trait is synchronous for now to avoid premature async/provider
/// complexity. Real providers can introduce async adapters later if justified.
pub trait ModelProvider {
    /// Returns provider metadata.
    fn metadata(&self) -> &ModelProviderMetadata;

    /// Requests model-assisted output.
    fn complete(&self, request: &ModelRequest) -> MonadResult<ModelResponse>;
}

/// No-network mock provider for tests and early local workflows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockModelProvider {
    metadata: ModelProviderMetadata,
    response_content: String,
}

impl Default for MockModelProvider {
    fn default() -> Self {
        Self::new("mock response")
    }
}

impl MockModelProvider {
    /// Creates a mock provider with fixed response content.
    #[must_use]
    pub fn new(response_content: impl Into<String>) -> Self {
        Self {
            metadata: ModelProviderMetadata::new(
                "mock",
                "Mock Model Provider",
                Some("mock-model".to_string()),
                ModelProviderCapabilities::mock(),
            ),
            response_content: response_content.into(),
        }
    }

    /// Creates a mock provider with explicit metadata.
    #[must_use]
    pub fn with_metadata(
        metadata: ModelProviderMetadata,
        response_content: impl Into<String>,
    ) -> Self {
        Self {
            metadata,
            response_content: response_content.into(),
        }
    }
}

impl ModelProvider for MockModelProvider {
    fn metadata(&self) -> &ModelProviderMetadata {
        &self.metadata
    }

    fn complete(&self, request: &ModelRequest) -> MonadResult<ModelResponse> {
        if request.is_empty() {
            return Err(MonadError::invalid_input(
                "model request must contain at least one message",
            ));
        }

        Ok(ModelResponse::new(
            self.metadata.provider_id(),
            self.metadata.default_model_id().map(ToOwned::to_owned),
            self.response_content.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{ModelMessage, ModelMessageRole};

    #[test]
    fn capability_metadata_describes_mock_provider() {
        let capabilities = ModelProviderCapabilities::mock();

        assert!(capabilities.supports_chat());
        assert!(!capabilities.supports_streaming());
        assert!(!capabilities.supports_tool_calls());
        assert!(capabilities.supports_local_execution());
    }

    #[test]
    fn provider_metadata_exposes_stable_fields() {
        let metadata = ModelProviderMetadata::new(
            "local-test",
            "Local Test Provider",
            Some("test-model".to_string()),
            ModelProviderCapabilities::new(true, false, false, true),
        );

        assert_eq!(metadata.provider_id(), "local-test");
        assert_eq!(metadata.display_name(), "Local Test Provider");
        assert_eq!(metadata.default_model_id(), Some("test-model"));
        assert!(metadata.capabilities().supports_local_execution());
    }

    #[test]
    fn mock_provider_returns_fixed_response_without_external_api() -> MonadResult<()> {
        let provider = MockModelProvider::new("structured plan");
        let request = ModelRequest::from_user_prompt("plan", "Plan this work packet.");

        let response = provider.complete(&request)?;

        assert_eq!(provider.metadata().provider_id(), "mock");
        assert_eq!(response.provider_id(), "mock");
        assert_eq!(response.model_id(), Some("mock-model"));
        assert_eq!(response.content(), "structured plan");

        Ok(())
    }

    #[test]
    fn mock_provider_rejects_empty_requests() {
        let provider = MockModelProvider::default();
        let request = ModelRequest::new("plan", Vec::new());

        let error = provider
            .complete(&request)
            .expect_err("empty model requests should fail");

        assert_eq!(error.code(), "MONAD2001");
        assert!(error.message().contains("at least one message"));
    }

    #[test]
    fn provider_trait_can_be_used_through_trait_object() -> MonadResult<()> {
        let provider: Box<dyn ModelProvider> = Box::new(MockModelProvider::new("review result"));
        let request = ModelRequest::new(
            "review",
            vec![ModelMessage::new(
                ModelMessageRole::User,
                "Review this proposed change.",
            )],
        );

        let response = provider.complete(&request)?;

        assert_eq!(response.provider_id(), "mock");
        assert_eq!(response.content(), "review result");

        Ok(())
    }
}
