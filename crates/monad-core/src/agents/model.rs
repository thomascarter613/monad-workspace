//! Provider-agnostic model request and response types.
//!
//! These types intentionally avoid vendor-specific concepts. They are small
//! enough to replace later if real provider integrations prove different needs.

/// Role for a model message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModelMessageRole {
    /// System/developer instruction.
    System,

    /// User-provided input.
    User,

    /// Assistant/model output.
    Assistant,
}

impl ModelMessageRole {
    /// Returns a stable role label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::Assistant => "assistant",
        }
    }
}

/// One provider-agnostic message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelMessage {
    role: ModelMessageRole,
    content: String,
}

impl ModelMessage {
    /// Creates a model message.
    #[must_use]
    pub fn new(role: ModelMessageRole, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    /// Creates a system message.
    #[must_use]
    pub fn system(content: impl Into<String>) -> Self {
        Self::new(ModelMessageRole::System, content)
    }

    /// Creates a user message.
    #[must_use]
    pub fn user(content: impl Into<String>) -> Self {
        Self::new(ModelMessageRole::User, content)
    }

    /// Creates an assistant message.
    #[must_use]
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(ModelMessageRole::Assistant, content)
    }

    /// Returns the message role.
    #[must_use]
    pub const fn role(&self) -> ModelMessageRole {
        self.role
    }

    /// Returns the message content.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Provider-agnostic model request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelRequest {
    purpose: String,
    messages: Vec<ModelMessage>,
}

impl ModelRequest {
    /// Creates a model request.
    #[must_use]
    pub fn new(purpose: impl Into<String>, messages: Vec<ModelMessage>) -> Self {
        Self {
            purpose: purpose.into(),
            messages,
        }
    }

    /// Creates a simple single-prompt request.
    #[must_use]
    pub fn from_user_prompt(purpose: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self::new(purpose, vec![ModelMessage::user(prompt)])
    }

    /// Returns the request purpose.
    #[must_use]
    pub fn purpose(&self) -> &str {
        &self.purpose
    }

    /// Returns request messages.
    #[must_use]
    pub fn messages(&self) -> &[ModelMessage] {
        &self.messages
    }

    /// Returns true when the request has no messages.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Returns the number of messages.
    #[must_use]
    pub fn len(&self) -> usize {
        self.messages.len()
    }
}

/// Provider-agnostic model response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelResponse {
    provider_id: String,
    model_id: Option<String>,
    content: String,
}

impl ModelResponse {
    /// Creates a model response.
    #[must_use]
    pub fn new(
        provider_id: impl Into<String>,
        model_id: Option<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            provider_id: provider_id.into(),
            model_id,
            content: content.into(),
        }
    }

    /// Returns the provider ID that produced this response.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    /// Returns the model ID, if available.
    #[must_use]
    pub fn model_id(&self) -> Option<&str> {
        self.model_id.as_deref()
    }

    /// Returns response content.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_role_labels_are_stable() {
        assert_eq!(ModelMessageRole::System.as_str(), "system");
        assert_eq!(ModelMessageRole::User.as_str(), "user");
        assert_eq!(ModelMessageRole::Assistant.as_str(), "assistant");
    }

    #[test]
    fn model_request_preserves_purpose_and_messages() {
        let request = ModelRequest::new(
            "plan",
            vec![
                ModelMessage::system("Follow repo policy."),
                ModelMessage::user("Plan this work packet."),
            ],
        );

        assert_eq!(request.purpose(), "plan");
        assert_eq!(request.len(), 2);
        assert_eq!(request.messages()[0].role(), ModelMessageRole::System);
        assert_eq!(request.messages()[1].content(), "Plan this work packet.");
    }

    #[test]
    fn user_prompt_request_contains_one_user_message() {
        let request = ModelRequest::from_user_prompt("explain", "Explain the repository.");

        assert_eq!(request.purpose(), "explain");
        assert_eq!(request.len(), 1);
        assert_eq!(request.messages()[0].role(), ModelMessageRole::User);
        assert_eq!(request.messages()[0].content(), "Explain the repository.");
    }

    #[test]
    fn model_response_preserves_provider_model_and_content() {
        let response =
            ModelResponse::new("mock", Some("mock-model".to_string()), "Planned response.");

        assert_eq!(response.provider_id(), "mock");
        assert_eq!(response.model_id(), Some("mock-model"));
        assert_eq!(response.content(), "Planned response.");
    }
}
