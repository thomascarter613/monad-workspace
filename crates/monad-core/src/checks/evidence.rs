//! Evidence packet model for Monad verification.
//!
//! An evidence packet is a reviewable explanation of what Monad checked,
//! what passed, what failed, what commands ran, and what limitations remain.
//!
//! WP-E4-004 intentionally keeps this human-readable and lightweight. It is not
//! a signed attestation, SBOM, SLSA provenance document, compliance report, or
//! machine-readable evidence schema.

use crate::{CheckResult, CheckRunReport, CommandResult};

/// Human-readable evidence packet for one verification run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidencePacket {
    schema_version: u16,
    title: String,
    generated_by: String,
    summary: EvidenceSummary,
    check_results: Vec<EvidenceCheckResult>,
    command_summaries: Vec<EvidenceCommandSummary>,
    risks: Vec<String>,
}

impl EvidencePacket {
    /// Creates an evidence packet.
    #[must_use]
    pub fn new(
        schema_version: u16,
        title: impl Into<String>,
        generated_by: impl Into<String>,
        summary: EvidenceSummary,
        check_results: Vec<EvidenceCheckResult>,
        command_summaries: Vec<EvidenceCommandSummary>,
        risks: Vec<String>,
    ) -> Self {
        Self {
            schema_version,
            title: title.into(),
            generated_by: generated_by.into(),
            summary,
            check_results,
            command_summaries,
            risks,
        }
    }

    /// Builds an evidence packet from a check run report.
    #[must_use]
    pub fn from_check_run_report(report: &CheckRunReport) -> Self {
        let check_results = report
            .results()
            .iter()
            .map(EvidenceCheckResult::from_check_result)
            .collect();

        let command_summaries = report
            .command_results()
            .iter()
            .map(EvidenceCommandSummary::from_command_result)
            .collect();

        let mut risks = vec![
            "This evidence packet is unsigned and is intended for human review only.".to_string(),
            "This packet does not provide SLSA provenance, SBOM output, or compliance attestation."
                .to_string(),
        ];

        if report.has_failures() {
            risks.push(
                "One or more checks failed; review failed check details before trusting the repository state."
                    .to_string(),
            );
        }

        Self::new(
            1,
            "Monad check evidence packet",
            "monad-core",
            EvidenceSummary::from_check_run_report(report),
            check_results,
            command_summaries,
            risks,
        )
    }

    /// Returns the evidence schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the packet title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the generator name.
    #[must_use]
    pub fn generated_by(&self) -> &str {
        &self.generated_by
    }

    /// Returns the summary.
    #[must_use]
    pub const fn summary(&self) -> &EvidenceSummary {
        &self.summary
    }

    /// Returns check results.
    #[must_use]
    pub fn check_results(&self) -> &[EvidenceCheckResult] {
        &self.check_results
    }

    /// Returns command summaries.
    #[must_use]
    pub fn command_summaries(&self) -> &[EvidenceCommandSummary] {
        &self.command_summaries
    }

    /// Returns known limitations or risks.
    #[must_use]
    pub fn risks(&self) -> &[String] {
        &self.risks
    }
}

/// Summary counts for an evidence packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceSummary {
    check_count: usize,
    passed_count: usize,
    failed_count: usize,
    warning_count: usize,
    skipped_count: usize,
    command_count: usize,
    passed: bool,
}

impl EvidenceSummary {
    /// Creates an evidence summary.
    #[must_use]
    pub const fn new(
        check_count: usize,
        passed_count: usize,
        failed_count: usize,
        warning_count: usize,
        skipped_count: usize,
        command_count: usize,
        passed: bool,
    ) -> Self {
        Self {
            check_count,
            passed_count,
            failed_count,
            warning_count,
            skipped_count,
            command_count,
            passed,
        }
    }

    /// Builds a summary from a check run report.
    #[must_use]
    pub fn from_check_run_report(report: &CheckRunReport) -> Self {
        Self::new(
            report.result_count(),
            report.passed_count(),
            report.failed_count(),
            report.warning_count(),
            report.skipped_count(),
            report.command_results().len(),
            !report.has_failures(),
        )
    }

    /// Returns the number of checks.
    #[must_use]
    pub const fn check_count(&self) -> usize {
        self.check_count
    }

    /// Returns the number of passed checks.
    #[must_use]
    pub const fn passed_count(&self) -> usize {
        self.passed_count
    }

    /// Returns the number of failed checks.
    #[must_use]
    pub const fn failed_count(&self) -> usize {
        self.failed_count
    }

    /// Returns the number of warning checks.
    #[must_use]
    pub const fn warning_count(&self) -> usize {
        self.warning_count
    }

    /// Returns the number of skipped checks.
    #[must_use]
    pub const fn skipped_count(&self) -> usize {
        self.skipped_count
    }

    /// Returns the number of command summaries.
    #[must_use]
    pub const fn command_count(&self) -> usize {
        self.command_count
    }

    /// Returns true when the verification run passed.
    #[must_use]
    pub const fn passed(&self) -> bool {
        self.passed
    }
}

/// Evidence representation of one check result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceCheckResult {
    check_id: String,
    status: String,
    message: String,
}

impl EvidenceCheckResult {
    /// Creates an evidence check result.
    #[must_use]
    pub fn new(
        check_id: impl Into<String>,
        status: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            status: status.into(),
            message: message.into(),
        }
    }

    /// Builds an evidence check result from a core check result.
    #[must_use]
    pub fn from_check_result(result: &CheckResult) -> Self {
        Self::new(
            result.check_id().as_str(),
            result.status().as_str(),
            result.message(),
        )
    }

    /// Returns the check ID.
    #[must_use]
    pub fn check_id(&self) -> &str {
        &self.check_id
    }

    /// Returns the check status.
    #[must_use]
    pub fn status(&self) -> &str {
        &self.status
    }

    /// Returns the check message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Evidence representation of one command execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceCommandSummary {
    command: String,
    working_directory: String,
    exit_code: Option<i32>,
    success: bool,
    stdout_summary: Option<String>,
    stderr_summary: Option<String>,
}

impl EvidenceCommandSummary {
    /// Creates a command evidence summary.
    #[must_use]
    pub fn new(
        command: impl Into<String>,
        working_directory: impl Into<String>,
        exit_code: Option<i32>,
        success: bool,
        stdout_summary: Option<String>,
        stderr_summary: Option<String>,
    ) -> Self {
        Self {
            command: command.into(),
            working_directory: working_directory.into(),
            exit_code,
            success,
            stdout_summary,
            stderr_summary,
        }
    }

    /// Builds a command evidence summary from a command result.
    #[must_use]
    pub fn from_command_result(result: &CommandResult) -> Self {
        Self::new(
            result.command(),
            result.working_directory(),
            result.exit_code(),
            result.success(),
            first_non_empty_line(result.stdout()),
            first_non_empty_line(result.stderr()),
        )
    }

    /// Returns the command display string.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Returns the working directory.
    #[must_use]
    pub fn working_directory(&self) -> &str {
        &self.working_directory
    }

    /// Returns the exit code.
    #[must_use]
    pub const fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    /// Returns true when the command succeeded.
    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    /// Returns the stdout summary.
    #[must_use]
    pub fn stdout_summary(&self) -> Option<&str> {
        self.stdout_summary.as_deref()
    }

    /// Returns the stderr summary.
    #[must_use]
    pub fn stderr_summary(&self) -> Option<&str> {
        self.stderr_summary.as_deref()
    }
}

fn first_non_empty_line(text: &str) -> Option<String> {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToOwned::to_owned)
}

#[cfg(test)]
mod tests {
    use crate::{CheckId, CheckResult, CheckRunReport, CheckStatus};

    use super::*;

    #[test]
    fn evidence_packet_summarizes_check_report() {
        let report = CheckRunReport::new(
            crate::initial_workspace_check_registry(),
            vec![
                CheckResult::passed(CheckId::new("MONAD-CHECK-0001"), "passed"),
                CheckResult::failed(CheckId::new("MONAD-CHECK-0002"), "failed"),
            ],
        );

        let packet = EvidencePacket::from_check_run_report(&report);

        assert_eq!(packet.schema_version(), 1);
        assert_eq!(packet.title(), "Monad check evidence packet");
        assert_eq!(packet.summary().check_count(), 2);
        assert_eq!(packet.summary().passed_count(), 1);
        assert_eq!(packet.summary().failed_count(), 1);
        assert!(!packet.summary().passed());
        assert_eq!(packet.check_results().len(), 2);
        assert!(packet.risks().iter().any(|risk| risk.contains("failed")));
    }

    #[test]
    fn evidence_check_result_preserves_check_status() {
        let result = CheckResult::warning(CheckId::new("MONAD-CHECK-0003"), "warning");
        let evidence = EvidenceCheckResult::from_check_result(&result);

        assert_eq!(evidence.check_id(), "MONAD-CHECK-0003");
        assert_eq!(evidence.status(), CheckStatus::Warning.as_str());
        assert_eq!(evidence.message(), "warning");
    }

    #[test]
    fn evidence_command_summary_captures_first_output_lines() {
        let result = CommandResult::new(
            "cargo --version",
            ".",
            Some(0),
            true,
            "cargo 1.95.0\nextra\n",
            "",
        );

        let summary = EvidenceCommandSummary::from_command_result(&result);

        assert_eq!(summary.command(), "cargo --version");
        assert_eq!(summary.exit_code(), Some(0));
        assert!(summary.success());
        assert_eq!(summary.stdout_summary(), Some("cargo 1.95.0"));
        assert_eq!(summary.stderr_summary(), None);
    }
}
