//! Verification check model for Monad.
//!
//! This file defines the small vocabulary Monad uses to describe checks and
//! check results. It does not run commands. It only models verification facts.

/// Stable identifier for a Monad verification check.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CheckId(String);

impl CheckId {
    /// Creates a check identifier.
    ///
    /// The caller provides the stable ID string. Later slices can add stronger
    /// validation rules if needed.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the check identifier as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Severity describes the importance of a check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CheckSeverity {
    /// Informational check.
    Info,

    /// Advisory check.
    Advisory,

    /// Warning-level check.
    Warning,

    /// Error-level check.
    Error,
}

impl CheckSeverity {
    /// Returns a stable severity label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Advisory => "advisory",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

/// Status describes what happened when a check was evaluated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CheckStatus {
    /// The check passed.
    Passed,

    /// The check failed.
    Failed,

    /// The check was skipped.
    Skipped,

    /// The check produced a warning.
    Warning,
}

impl CheckStatus {
    /// Returns a stable status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
            Self::Warning => "warning",
        }
    }

    /// Returns true when this status represents a failed check.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::Failed)
    }
}

/// Metadata describing an available verification check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckDefinition {
    id: CheckId,
    name: String,
    severity: CheckSeverity,
    description: String,
}

impl CheckDefinition {
    /// Creates a check definition.
    #[must_use]
    pub fn new(
        id: CheckId,
        name: impl Into<String>,
        severity: CheckSeverity,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            severity,
            description: description.into(),
        }
    }

    /// Returns the check identifier.
    #[must_use]
    pub fn id(&self) -> &CheckId {
        &self.id
    }

    /// Returns the check name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the check severity.
    #[must_use]
    pub const fn severity(&self) -> CheckSeverity {
        self.severity
    }

    /// Returns the check description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Result produced by evaluating one check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResult {
    check_id: CheckId,
    status: CheckStatus,
    message: String,
}

impl CheckResult {
    /// Creates a check result.
    #[must_use]
    pub fn new(check_id: CheckId, status: CheckStatus, message: impl Into<String>) -> Self {
        Self {
            check_id,
            status,
            message: message.into(),
        }
    }

    /// Creates a passed check result.
    #[must_use]
    pub fn passed(check_id: CheckId, message: impl Into<String>) -> Self {
        Self::new(check_id, CheckStatus::Passed, message)
    }

    /// Creates a failed check result.
    #[must_use]
    pub fn failed(check_id: CheckId, message: impl Into<String>) -> Self {
        Self::new(check_id, CheckStatus::Failed, message)
    }

    /// Creates a skipped check result.
    #[must_use]
    pub fn skipped(check_id: CheckId, message: impl Into<String>) -> Self {
        Self::new(check_id, CheckStatus::Skipped, message)
    }

    /// Creates a warning check result.
    #[must_use]
    pub fn warning(check_id: CheckId, message: impl Into<String>) -> Self {
        Self::new(check_id, CheckStatus::Warning, message)
    }

    /// Returns the check identifier.
    #[must_use]
    pub fn check_id(&self) -> &CheckId {
        &self.check_id
    }

    /// Returns the check status.
    #[must_use]
    pub const fn status(&self) -> CheckStatus {
        self.status
    }

    /// Returns the result message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns true when this result represents a failure.
    #[must_use]
    pub const fn is_failure(&self) -> bool {
        self.status.is_failure()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_id_preserves_stable_text() {
        let id = CheckId::new("MONAD-CHECK-0001");

        assert_eq!(id.as_str(), "MONAD-CHECK-0001");
    }

    #[test]
    fn severity_labels_are_stable() {
        assert_eq!(CheckSeverity::Info.as_str(), "info");
        assert_eq!(CheckSeverity::Advisory.as_str(), "advisory");
        assert_eq!(CheckSeverity::Warning.as_str(), "warning");
        assert_eq!(CheckSeverity::Error.as_str(), "error");
    }

    #[test]
    fn status_labels_are_stable() {
        assert_eq!(CheckStatus::Passed.as_str(), "passed");
        assert_eq!(CheckStatus::Failed.as_str(), "failed");
        assert_eq!(CheckStatus::Skipped.as_str(), "skipped");
        assert_eq!(CheckStatus::Warning.as_str(), "warning");
    }

    #[test]
    fn check_definition_exposes_metadata() {
        let definition = CheckDefinition::new(
            CheckId::new("MONAD-CHECK-0002"),
            "Required file exists",
            CheckSeverity::Warning,
            "Checks whether a required repository file exists.",
        );

        assert_eq!(definition.id().as_str(), "MONAD-CHECK-0002");
        assert_eq!(definition.name(), "Required file exists");
        assert_eq!(definition.severity(), CheckSeverity::Warning);
        assert!(
            definition
                .description()
                .contains("required repository file")
        );
    }

    #[test]
    fn check_result_constructors_set_status() {
        let passed = CheckResult::passed(CheckId::new("A"), "passed");
        let failed = CheckResult::failed(CheckId::new("B"), "failed");
        let skipped = CheckResult::skipped(CheckId::new("C"), "skipped");
        let warning = CheckResult::warning(CheckId::new("D"), "warning");

        assert_eq!(passed.status(), CheckStatus::Passed);
        assert_eq!(failed.status(), CheckStatus::Failed);
        assert_eq!(skipped.status(), CheckStatus::Skipped);
        assert_eq!(warning.status(), CheckStatus::Warning);

        assert!(!passed.is_failure());
        assert!(failed.is_failure());
        assert!(!skipped.is_failure());
        assert!(!warning.is_failure());
    }
}
