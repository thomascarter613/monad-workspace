//! Diagnostic types for Monad.
//!
//! A diagnostic is a structured message produced by Monad while inspecting,
//! verifying, or evolving a repository.
//!
//! This module is intentionally small for the first runtime slice. It gives the
//! rest of the system a durable vocabulary for reporting information, warnings,
//! and errors without forcing every command to invent its own output format.

/// The seriousness of a diagnostic message.
///
/// An `enum` is a Rust type where the value must be one of a fixed set of
/// variants. Here, a diagnostic can be informational, a warning, or an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Informational output. This does not mean anything is wrong.
    Info,

    /// A warning. Work can continue, but the user should pay attention.
    Warning,

    /// An error. The requested operation should be considered unsuccessful.
    Error,
}

impl Severity {
    /// Returns a stable uppercase label for display and reports.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Warning => "WARNING",
            Self::Error => "ERROR",
        }
    }
}

/// A structured message produced by Monad.
///
/// This is deliberately simple:
///
/// - `severity` tells the caller how serious the diagnostic is.
/// - `code` gives the message a stable machine-readable identifier.
/// - `message` gives humans a readable explanation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// The seriousness of the diagnostic.
    pub severity: Severity,

    /// A stable diagnostic code, such as `MONAD0001`.
    pub code: &'static str,

    /// A human-readable diagnostic message.
    pub message: String,
}

impl Diagnostic {
    /// Creates a new diagnostic.
    ///
    /// The `impl Into<String>` parameter means callers may pass either a
    /// `String` or a string slice like `"hello"`, and Rust will convert it into
    /// an owned `String` for storage.
    #[must_use]
    pub fn new(severity: Severity, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity,
            code,
            message: message.into(),
        }
    }

    /// Creates an informational diagnostic.
    #[must_use]
    pub fn info(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(Severity::Info, code, message)
    }

    /// Creates a warning diagnostic.
    #[must_use]
    pub fn warning(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(Severity::Warning, code, message)
    }

    /// Creates an error diagnostic.
    #[must_use]
    pub fn error(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(Severity::Error, code, message)
    }

    /// Returns true when this diagnostic is an error.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self.severity, Severity::Error)
    }

    /// Renders the diagnostic as a stable single-line message.
    #[must_use]
    pub fn render(&self) -> String {
        format!(
            "[{}] {}: {}",
            self.severity.as_str(),
            self.code,
            self.message
        )
    }
}

/// A small collection of diagnostics.
///
/// This type gives future commands one object to return when they need to
/// report multiple findings.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DiagnosticReport {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticReport {
    /// Creates an empty diagnostic report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Adds one diagnostic to the report.
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Returns the number of diagnostics in the report.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns true when the report has no diagnostics.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Returns true when the report contains at least one error.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.is_error())
    }

    /// Returns the diagnostics as a read-only slice.
    ///
    /// A slice lets callers inspect the diagnostics without taking ownership of
    /// the internal `Vec`.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Renders all diagnostics as lines.
    #[must_use]
    pub fn render_lines(&self) -> Vec<String> {
        self.diagnostics.iter().map(Diagnostic::render).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_labels_are_stable() {
        assert_eq!(Severity::Info.as_str(), "INFO");
        assert_eq!(Severity::Warning.as_str(), "WARNING");
        assert_eq!(Severity::Error.as_str(), "ERROR");
    }

    #[test]
    fn diagnostic_renders_as_single_line_message() {
        let diagnostic = Diagnostic::info("MONAD0001", "runtime ready");

        assert_eq!(diagnostic.render(), "[INFO] MONAD0001: runtime ready");
    }

    #[test]
    fn report_knows_when_it_contains_errors() {
        let mut report = DiagnosticReport::new();

        assert!(report.is_empty());
        assert!(!report.has_errors());

        report.push(Diagnostic::warning("MONAD1000", "check this later"));
        assert_eq!(report.len(), 1);
        assert!(!report.has_errors());

        report.push(Diagnostic::error("MONAD9000", "operation failed"));
        assert_eq!(report.len(), 2);
        assert!(report.has_errors());
    }
}
