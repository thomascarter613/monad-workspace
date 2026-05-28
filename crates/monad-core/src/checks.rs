//! Workspace and verification checks for Monad.
//!
//! This module is the public boundary for Monad's verification foundation.
//!
//! E1 introduced the first simple workspace checks. E4 starts turning checks
//! into a reusable verification model with check definitions, statuses,
//! results, and a registry.

pub mod evidence;
pub mod model;
pub mod registry;
pub mod report;
pub mod run;

pub use evidence::{EvidenceCheckResult, EvidenceCommandSummary, EvidencePacket, EvidenceSummary};
pub use model::{CheckDefinition, CheckId, CheckResult, CheckSeverity, CheckStatus};
pub use registry::CheckRegistry;
pub use report::{
    DEFAULT_EVIDENCE_REPORT_PATH, evidence_packet_from_check_run_report,
    render_evidence_packet_markdown, write_check_evidence_packet,
};
pub use run::{
    CheckRunReport, initial_workspace_check_registry, render_check_run_report,
    run_monad_workspace_checks,
};

use crate::{Diagnostic, DiagnosticReport, Severity, WorkspaceContext};

/// Runs the current workspace checks.
///
/// This function preserves the existing E1 behavior while E4 introduces the
/// richer verification model beside it.
#[must_use]
pub fn run_workspace_checks(context: &WorkspaceContext) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    if context.root().join("monad.toml").exists() {
        report.push(Diagnostic::new(
            Severity::Info,
            "MONAD1001",
            "monad.toml found",
        ));
    } else {
        report.push(Diagnostic::new(
            Severity::Warning,
            "MONAD1002",
            "monad.toml not found",
        ));
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_model_exports_are_usable_from_checks_boundary() {
        let definition = CheckDefinition::new(
            CheckId::new("MONAD-CHECK-0001"),
            "Example check",
            CheckSeverity::Info,
            "Example check description",
        );

        let result = CheckResult::passed(definition.id().clone(), "Example check passed");

        assert_eq!(definition.id().as_str(), "MONAD-CHECK-0001");
        assert_eq!(result.status(), CheckStatus::Passed);
    }
}
