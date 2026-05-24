//! Error types for Monad.
//!
//! Diagnostics describe findings that Monad reports to users.
//! Errors describe failures that prevent an operation from completing normally.
//!
//! This module gives future Monad runtime modules one shared error vocabulary
//! instead of each module returning arbitrary strings.

use std::fmt;

use crate::Diagnostic;

/// Monad's core error type.
///
/// An `enum` is useful here because a Monad operation can fail for different
/// known reasons, but each failure still belongs to one shared type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonadError {
    /// The caller provided invalid input.
    InvalidInput {
        /// Human-readable explanation of what was invalid.
        message: String,
    },

    /// A required resource could not be found.
    NotFound {
        /// The resource that was expected but not found.
        resource: String,
    },

    /// Verification failed.
    VerificationFailed {
        /// Human-readable explanation of the verification failure.
        message: String,
    },

    /// An unexpected internal failure occurred.
    Internal {
        /// Human-readable explanation of the internal failure.
        message: String,
    },
}

/// Standard result type for Monad core operations.
///
/// `Result<T, E>` is Rust's standard success-or-failure type:
///
/// - `Ok(value)` means the operation succeeded and produced `value`.
/// - `Err(error)` means the operation failed and produced `error`.
///
/// This alias means future core functions can return `MonadResult<T>` instead
/// of repeatedly writing `Result<T, MonadError>`.
pub type MonadResult<T> = Result<T, MonadError>;

impl MonadError {
    /// Creates an invalid input error.
    #[must_use]
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    /// Creates a not found error.
    #[must_use]
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound {
            resource: resource.into(),
        }
    }

    /// Creates a verification failed error.
    #[must_use]
    pub fn verification_failed(message: impl Into<String>) -> Self {
        Self::VerificationFailed {
            message: message.into(),
        }
    }

    /// Creates an internal error.
    #[must_use]
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Returns a stable machine-readable error code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidInput { .. } => "MONAD2001",
            Self::NotFound { .. } => "MONAD2002",
            Self::VerificationFailed { .. } => "MONAD2003",
            Self::Internal { .. } => "MONAD9000",
        }
    }

    /// Returns a human-readable error message.
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            Self::InvalidInput { message } => message.clone(),
            Self::NotFound { resource } => format!("required resource not found: {resource}"),
            Self::VerificationFailed { message } => message.clone(),
            Self::Internal { message } => message.clone(),
        }
    }

    /// Converts this error into a diagnostic.
    ///
    /// This connects the error model to the diagnostics model from WP-E1-002.
    #[must_use]
    pub fn to_diagnostic(&self) -> Diagnostic {
        Diagnostic::error(self.code(), self.message())
    }
}

impl fmt::Display for MonadError {
    /// Formats the error for humans.
    ///
    /// `Display` is the Rust trait used by `{}` formatting.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.message())
    }
}

impl std::error::Error for MonadError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_input_has_stable_code_and_message() {
        let error = MonadError::invalid_input("workspace path is empty");

        assert_eq!(error.code(), "MONAD2001");
        assert_eq!(error.message(), "workspace path is empty");
        assert_eq!(error.to_string(), "MONAD2001: workspace path is empty");
    }

    #[test]
    fn not_found_names_missing_resource() {
        let error = MonadError::not_found("monad.toml");

        assert_eq!(error.code(), "MONAD2002");
        assert_eq!(error.message(), "required resource not found: monad.toml");
    }

    #[test]
    fn verification_failed_converts_to_error_diagnostic() {
        let error = MonadError::verification_failed("frontmatter check failed");
        let diagnostic = error.to_diagnostic();

        assert_eq!(diagnostic.code, "MONAD2003");
        assert!(diagnostic.is_error());
        assert_eq!(
            diagnostic.render(),
            "[ERROR] MONAD2003: frontmatter check failed"
        );
    }

    #[test]
    fn monad_result_alias_can_return_success_or_error() {
        fn succeeds() -> MonadResult<&'static str> {
            Ok("ready")
        }

        fn fails() -> MonadResult<&'static str> {
            Err(MonadError::internal("unexpected runtime state"))
        }

        assert_eq!(succeeds(), Ok("ready"));

        let error = fails().expect_err("the helper should fail");
        assert_eq!(error.code(), "MONAD9000");
    }
}
