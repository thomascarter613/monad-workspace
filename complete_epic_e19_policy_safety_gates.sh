#!/usr/bin/env bash
set -euo pipefail

# Epic E19 — Policy, Safety, and Approval Gate Foundation
#
# Implements the first MVP-safe policy and approval-gate foundation:
#
#   monad policy --dry-run
#   monad policy --dry-run --format=json
#   monad policy --yes
#
# Safety:
# - Does not execute commands.
# - Does not rewrite user source files.
# - Does not publish releases.
# - Does not call AI providers.
# - Does not approve risky operations automatically.
# - Does not integrate remote/cloud policy services.
# - Does not claim OPA/Rego support.

echo "==> Epic E19: Policy, Safety, and Approval Gate Foundation"

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

cd "$REPO_ROOT"

echo "==> Repo root: $REPO_ROOT"

POLICY_FILE="crates/monad-core/src/policy.rs"
LIB_FILE="crates/monad-core/src/lib.rs"
CLI_FILE="crates/monad-cli/src/main.rs"

for required in "$LIB_FILE" "$CLI_FILE"; do
  if [ ! -f "$required" ]; then
    echo "ERROR: expected file not found: $required" >&2
    echo "Run this from the Monad repository root." >&2
    exit 1
  fi
done

mkdir -p \
  docs/architecture \
  docs/commands \
  docs/workflows \
  docs/verification \
  tools/scripts \
  work/learning/E19 \
  work/deliverables/E19 \
  .monad/script-backups/E19/EPIC-E19

BACKUP_STAMP="$(date +%Y%m%d%H%M%S)"
[ -f "$POLICY_FILE" ] && cp "$POLICY_FILE" ".monad/script-backups/E19/EPIC-E19/policy.rs.$BACKUP_STAMP.bak"
cp "$LIB_FILE" ".monad/script-backups/E19/EPIC-E19/lib.rs.$BACKUP_STAMP.bak"
cp "$CLI_FILE" ".monad/script-backups/E19/EPIC-E19/main.rs.$BACKUP_STAMP.bak"

cat > "$POLICY_FILE" <<'EOF'
//! Policy, safety, and approval gate foundation.
//!
//! E19 provides the first MVP-safe policy model for classifying operations,
//! generating approval plans, checking basic file/command risks, and writing
//! policy evidence reports. It does not execute commands, rewrite user source,
//! publish releases, call AI providers, or approve risky operations
//! automatically.

use std::fs;
use std::path::{Path, PathBuf};

/// High-level operation category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperationKind {
    /// Repository initialization.
    Init,

    /// Component scaffolding.
    Add,

    /// Native command execution.
    Run,

    /// Repository contract synchronization.
    Sync,

    /// Diagnostics.
    Doctor,

    /// Release planning/distribution.
    Release,

    /// Repository upgrade/evolution.
    Upgrade,

    /// AI context export/handoff.
    AiContext,

    /// Future patch/apply workflow.
    Patch,

    /// Generic file write.
    FileWrite,
}

impl OperationKind {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Add => "add",
            Self::Run => "run",
            Self::Sync => "sync",
            Self::Doctor => "doctor",
            Self::Release => "release",
            Self::Upgrade => "upgrade",
            Self::AiContext => "ai-context",
            Self::Patch => "patch",
            Self::FileWrite => "file-write",
        }
    }
}

/// Operation mutability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperationMutability {
    /// Read-only operation.
    ReadOnly,

    /// Writes generated artifacts only.
    GeneratedWrite,

    /// Mutates source/user-owned files.
    SourceMutation,

    /// Executes commands with side effects.
    ExternalSideEffect,
}

impl OperationMutability {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read-only",
            Self::GeneratedWrite => "generated-write",
            Self::SourceMutation => "source-mutation",
            Self::ExternalSideEffect => "external-side-effect",
        }
    }
}

/// Risk level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    /// Safe read-only behavior.
    Low,

    /// Writes generated local evidence/metadata.
    Medium,

    /// Can affect user source, releases, commands, or external systems.
    High,

    /// Forbidden in MVP.
    Forbidden,
}

impl RiskLevel {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Forbidden => "forbidden",
        }
    }
}

/// Required approval gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApprovalGate {
    /// No approval beyond command invocation.
    None,

    /// Dry-run review is required before apply.
    DryRunReview,

    /// Explicit `--yes` style approval is required.
    ExplicitYes,

    /// Operation is blocked in MVP.
    Forbidden,
}

impl ApprovalGate {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DryRunReview => "dry-run-review",
            Self::ExplicitYes => "explicit-yes",
            Self::Forbidden => "forbidden",
        }
    }

    /// Whether explicit caller approval is required.
    #[must_use]
    pub const fn requires_explicit_approval(self) -> bool {
        matches!(self, Self::ExplicitYes)
    }

    /// Whether the operation is forbidden.
    #[must_use]
    pub const fn is_forbidden(self) -> bool {
        matches!(self, Self::Forbidden)
    }
}

/// One operation classification record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationClassification {
    kind: OperationKind,
    mutability: OperationMutability,
    risk: RiskLevel,
    approval_gate: ApprovalGate,
    rationale: String,
}

impl OperationClassification {
    /// Creates a classification.
    #[must_use]
    pub fn new(
        kind: OperationKind,
        mutability: OperationMutability,
        risk: RiskLevel,
        approval_gate: ApprovalGate,
        rationale: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            mutability,
            risk,
            approval_gate,
            rationale: rationale.into(),
        }
    }

    /// Operation kind.
    #[must_use]
    pub const fn kind(&self) -> OperationKind {
        self.kind
    }

    /// Mutability.
    #[must_use]
    pub const fn mutability(&self) -> OperationMutability {
        self.mutability
    }

    /// Risk.
    #[must_use]
    pub const fn risk(&self) -> RiskLevel {
        self.risk
    }

    /// Approval gate.
    #[must_use]
    pub const fn approval_gate(&self) -> ApprovalGate {
        self.approval_gate
    }

    /// Rationale.
    #[must_use]
    pub fn rationale(&self) -> &str {
        &self.rationale
    }
}

/// Policy finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PolicyFindingSeverity {
    /// Informational finding.
    Info,

    /// Requires human review.
    ApprovalRequired,

    /// Operation is blocked.
    Blocked,
}

impl PolicyFindingSeverity {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::ApprovalRequired => "approval-required",
            Self::Blocked => "blocked",
        }
    }
}

/// Policy finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyFinding {
    id: String,
    severity: PolicyFindingSeverity,
    subject: String,
    message: String,
}

impl PolicyFinding {
    /// Creates a finding.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        severity: PolicyFindingSeverity,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            severity,
            subject: subject.into(),
            message: message.into(),
        }
    }

    /// Finding ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Finding severity.
    #[must_use]
    pub const fn severity(&self) -> PolicyFindingSeverity {
        self.severity
    }

    /// Subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Approval plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalPlan {
    operation: OperationClassification,
    findings: Vec<PolicyFinding>,
}

impl ApprovalPlan {
    /// Creates a deterministic approval plan.
    #[must_use]
    pub fn new(operation: OperationClassification, mut findings: Vec<PolicyFinding>) -> Self {
        findings.sort_by(|left, right| {
            left.severity()
                .cmp(&right.severity())
                .then(left.id().cmp(right.id()))
        });

        Self {
            operation,
            findings,
        }
    }

    /// Operation classification.
    #[must_use]
    pub const fn operation(&self) -> &OperationClassification {
        &self.operation
    }

    /// Findings.
    #[must_use]
    pub fn findings(&self) -> &[PolicyFinding] {
        &self.findings
    }

    /// Whether explicit approval is required.
    #[must_use]
    pub fn requires_approval(&self) -> bool {
        self.operation.approval_gate().requires_explicit_approval()
            || self
                .findings
                .iter()
                .any(|finding| finding.severity() == PolicyFindingSeverity::ApprovalRequired)
    }

    /// Whether the plan is blocked.
    #[must_use]
    pub fn is_blocked(&self) -> bool {
        self.operation.approval_gate().is_forbidden()
            || self
                .findings
                .iter()
                .any(|finding| finding.severity() == PolicyFindingSeverity::Blocked)
    }
}

/// File operation intent for policy checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileOperationIntent {
    /// Read a file.
    Read,

    /// Write generated metadata/evidence.
    WriteGenerated,

    /// Write source or user-owned files.
    WriteUserOwned,

    /// Delete files.
    Delete,
}

impl FileOperationIntent {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::WriteGenerated => "write-generated",
            Self::WriteUserOwned => "write-user-owned",
            Self::Delete => "delete",
        }
    }
}

/// Command execution intent for policy checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandExecutionIntent {
    /// Read-only inspection command.
    Inspect,

    /// Local verification command.
    Verify,

    /// Build/test command.
    Build,

    /// Publish/release command.
    Publish,

    /// Remote/network-affecting command.
    Remote,

    /// Arbitrary shell command.
    Arbitrary,
}

impl CommandExecutionIntent {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inspect => "inspect",
            Self::Verify => "verify",
            Self::Build => "build",
            Self::Publish => "publish",
            Self::Remote => "remote",
            Self::Arbitrary => "arbitrary",
        }
    }
}

/// Gated write request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatedWriteRequest {
    relative_path: PathBuf,
    content: String,
    caller_approved: bool,
}

impl GatedWriteRequest {
    /// Creates a gated generated-write request.
    #[must_use]
    pub fn new(
        relative_path: impl Into<PathBuf>,
        content: impl Into<String>,
        caller_approved: bool,
    ) -> Self {
        Self {
            relative_path: relative_path.into(),
            content: content.into(),
            caller_approved,
        }
    }

    /// Relative path.
    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// Content.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Whether explicit caller approval marker was provided.
    #[must_use]
    pub const fn caller_approved(&self) -> bool {
        self.caller_approved
    }
}

/// Gated write result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatedWriteResult {
    /// File was written.
    Written(PathBuf),

    /// File already existed with identical content.
    SkippedIdentical(PathBuf),

    /// Approval was required but missing.
    ApprovalRequired(String),

    /// Operation was blocked.
    Blocked(String),
}

impl GatedWriteResult {
    /// Stable result label.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Written(_) => "written",
            Self::SkippedIdentical(_) => "skipped-identical",
            Self::ApprovalRequired(_) => "approval-required",
            Self::Blocked(_) => "blocked",
        }
    }
}

/// Full policy report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyReport {
    plans: Vec<ApprovalPlan>,
}

impl PolicyReport {
    /// Creates a deterministic report.
    #[must_use]
    pub fn new(mut plans: Vec<ApprovalPlan>) -> Self {
        plans.sort_by(|left, right| {
            left.operation()
                .kind()
                .cmp(&right.operation().kind())
                .then(left.operation().risk().cmp(&right.operation().risk()))
        });

        Self { plans }
    }

    /// Plans.
    #[must_use]
    pub fn plans(&self) -> &[ApprovalPlan] {
        &self.plans
    }

    /// Plan count.
    #[must_use]
    pub fn plan_count(&self) -> usize {
        self.plans.len()
    }

    /// Approval-required count.
    #[must_use]
    pub fn approval_required_count(&self) -> usize {
        self.plans.iter().filter(|plan| plan.requires_approval()).count()
    }

    /// Blocked count.
    #[must_use]
    pub fn blocked_count(&self) -> usize {
        self.plans.iter().filter(|plan| plan.is_blocked()).count()
    }
}

/// Classifies a Monad operation.
#[must_use]
pub fn classify_operation(kind: OperationKind) -> OperationClassification {
    match kind {
        OperationKind::Doctor => OperationClassification::new(
            kind,
            OperationMutability::ReadOnly,
            RiskLevel::Low,
            ApprovalGate::None,
            "diagnostics inspect local state without mutation",
        ),
        OperationKind::Init
        | OperationKind::Add
        | OperationKind::Sync
        | OperationKind::Upgrade
        | OperationKind::AiContext
        | OperationKind::FileWrite => OperationClassification::new(
            kind,
            OperationMutability::GeneratedWrite,
            RiskLevel::Medium,
            ApprovalGate::ExplicitYes,
            "generated local writes require explicit user approval",
        ),
        OperationKind::Release => OperationClassification::new(
            kind,
            OperationMutability::ExternalSideEffect,
            RiskLevel::High,
            ApprovalGate::DryRunReview,
            "release work must remain dry-run unless publishing is explicitly implemented later",
        ),
        OperationKind::Run => OperationClassification::new(
            kind,
            OperationMutability::ExternalSideEffect,
            RiskLevel::High,
            ApprovalGate::ExplicitYes,
            "native command execution can have side effects",
        ),
        OperationKind::Patch => OperationClassification::new(
            kind,
            OperationMutability::SourceMutation,
            RiskLevel::Forbidden,
            ApprovalGate::Forbidden,
            "automatic source patch application is forbidden in the MVP policy foundation",
        ),
    }
}

/// Builds an approval plan for an operation.
#[must_use]
pub fn build_approval_plan(kind: OperationKind) -> ApprovalPlan {
    let operation = classify_operation(kind);
    let mut findings = Vec::new();

    findings.push(PolicyFinding::new(
        format!("policy.operation.{}.classified", kind.as_str()),
        PolicyFindingSeverity::Info,
        kind.as_str(),
        format!(
            "operation is classified as mutability={}, risk={}, gate={}",
            operation.mutability().as_str(),
            operation.risk().as_str(),
            operation.approval_gate().as_str()
        ),
    ));

    if operation.approval_gate().requires_explicit_approval() {
        findings.push(PolicyFinding::new(
            format!("policy.operation.{}.approval-required", kind.as_str()),
            PolicyFindingSeverity::ApprovalRequired,
            kind.as_str(),
            "explicit caller approval marker is required before mutation",
        ));
    }

    if operation.approval_gate().is_forbidden() {
        findings.push(PolicyFinding::new(
            format!("policy.operation.{}.blocked", kind.as_str()),
            PolicyFindingSeverity::Blocked,
            kind.as_str(),
            "operation is forbidden in the MVP policy foundation",
        ));
    }

    ApprovalPlan::new(operation, findings)
}

/// Checks a file operation against MVP policy.
#[must_use]
pub fn check_file_operation(path: &Path, intent: FileOperationIntent) -> ApprovalPlan {
    let mut plan = build_approval_plan(match intent {
        FileOperationIntent::Read => OperationKind::Doctor,
        FileOperationIntent::WriteGenerated => OperationKind::FileWrite,
        FileOperationIntent::WriteUserOwned | FileOperationIntent::Delete => OperationKind::Patch,
    });

    let mut findings = plan.findings().to_vec();

    if path.is_absolute() {
        findings.push(PolicyFinding::new(
            "policy.file.absolute-path-blocked",
            PolicyFindingSeverity::Blocked,
            path.display().to_string(),
            "absolute paths are blocked for MVP repository mutation",
        ));
    }

    if matches!(intent, FileOperationIntent::Delete) {
        findings.push(PolicyFinding::new(
            "policy.file.delete-blocked",
            PolicyFindingSeverity::Blocked,
            path.display().to_string(),
            "delete operations are blocked in the MVP policy foundation",
        ));
    }

    if matches!(intent, FileOperationIntent::WriteUserOwned) {
        findings.push(PolicyFinding::new(
            "policy.file.user-owned-write-blocked",
            PolicyFindingSeverity::Blocked,
            path.display().to_string(),
            "user-owned source writes are blocked by the MVP policy foundation",
        ));
    }

    plan = ApprovalPlan::new(plan.operation().clone(), findings);
    plan
}

/// Checks command execution intent against MVP policy.
#[must_use]
pub fn check_command_execution(intent: CommandExecutionIntent) -> ApprovalPlan {
    let mut plan = build_approval_plan(match intent {
        CommandExecutionIntent::Inspect | CommandExecutionIntent::Verify | CommandExecutionIntent::Build => {
            OperationKind::Run
        }
        CommandExecutionIntent::Publish | CommandExecutionIntent::Remote | CommandExecutionIntent::Arbitrary => {
            OperationKind::Patch
        }
    });

    let mut findings = plan.findings().to_vec();

    if matches!(
        intent,
        CommandExecutionIntent::Publish
            | CommandExecutionIntent::Remote
            | CommandExecutionIntent::Arbitrary
    ) {
        findings.push(PolicyFinding::new(
            format!("policy.command.{}-blocked", intent.as_str()),
            PolicyFindingSeverity::Blocked,
            intent.as_str(),
            "command execution intent is blocked by the MVP policy foundation",
        ));
    }

    plan = ApprovalPlan::new(plan.operation().clone(), findings);
    plan
}

/// Builds the default policy report.
#[must_use]
pub fn build_policy_report() -> PolicyReport {
    PolicyReport::new(vec![
        build_approval_plan(OperationKind::Init),
        build_approval_plan(OperationKind::Add),
        build_approval_plan(OperationKind::Run),
        build_approval_plan(OperationKind::Sync),
        build_approval_plan(OperationKind::Doctor),
        build_approval_plan(OperationKind::Release),
        build_approval_plan(OperationKind::Upgrade),
        build_approval_plan(OperationKind::AiContext),
        build_approval_plan(OperationKind::Patch),
    ])
}

/// Performs a gated generated write.
pub fn gated_generated_write(
    root: impl AsRef<Path>,
    request: &GatedWriteRequest,
) -> Result<GatedWriteResult, String> {
    let plan = check_file_operation(request.relative_path(), FileOperationIntent::WriteGenerated);

    if plan.is_blocked() {
        return Ok(GatedWriteResult::Blocked(render_blocked_summary(&plan)));
    }

    if plan.requires_approval() && !request.caller_approved() {
        return Ok(GatedWriteResult::ApprovalRequired(
            "explicit caller approval marker is required before writing generated content"
                .to_string(),
        ));
    }

    let absolute = root.as_ref().join(request.relative_path());

    if let Some(parent) = absolute.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    if absolute.exists() {
        let existing = fs::read_to_string(&absolute).map_err(|error| error.to_string())?;
        if existing == request.content() {
            return Ok(GatedWriteResult::SkippedIdentical(
                request.relative_path().to_path_buf(),
            ));
        }

        return Ok(GatedWriteResult::Blocked(
            "refusing unsafe overwrite of existing file with different content".to_string(),
        ));
    }

    fs::write(&absolute, request.content()).map_err(|error| error.to_string())?;

    Ok(GatedWriteResult::Written(
        request.relative_path().to_path_buf(),
    ))
}

/// Writes policy evidence reports under `.monad/reports`.
pub fn write_policy_evidence(root: impl AsRef<Path>) -> Result<Vec<GatedWriteResult>, String> {
    let report = build_policy_report();
    let markdown = render_policy_report(&report);
    let json = render_policy_report_json(&report);

    let requests = [
        GatedWriteRequest::new(".monad/reports/policy-report.md", markdown, true),
        GatedWriteRequest::new(".monad/reports/policy-report.json", json, true),
    ];

    requests
        .iter()
        .map(|request| gated_generated_write(root.as_ref(), request))
        .collect()
}

fn render_blocked_summary(plan: &ApprovalPlan) -> String {
    plan.findings()
        .iter()
        .filter(|finding| finding.severity() == PolicyFindingSeverity::Blocked)
        .map(PolicyFinding::message)
        .collect::<Vec<_>>()
        .join("; ")
}

/// Renders policy report text.
#[must_use]
pub fn render_policy_report(report: &PolicyReport) -> String {
    let mut lines = vec![
        "Monad policy and approval-gate report".to_string(),
        String::new(),
        "Summary:".to_string(),
        format!("  plans: {}", report.plan_count()),
        format!("  approval_required: {}", report.approval_required_count()),
        format!("  blocked: {}", report.blocked_count()),
        String::new(),
        "Operations:".to_string(),
    ];

    for plan in report.plans() {
        let operation = plan.operation();

        lines.push(format!(
            "  - {} mutability={} risk={} gate={}",
            operation.kind().as_str(),
            operation.mutability().as_str(),
            operation.risk().as_str(),
            operation.approval_gate().as_str()
        ));
        lines.push(format!("    rationale: {}", operation.rationale()));
        lines.push(format!("    requires_approval: {}", plan.requires_approval()));
        lines.push(format!("    blocked: {}", plan.is_blocked()));

        for finding in plan.findings() {
            lines.push(format!(
                "    finding [{}] {}: {}",
                finding.severity().as_str(),
                finding.id(),
                finding.message()
            ));
        }
    }

    lines.push(String::new());
    lines.push("No commands were executed.".to_string());
    lines.push("No user source files were rewritten.".to_string());
    lines.push("No releases were published.".to_string());
    lines.push("No AI providers were called.".to_string());

    lines.join("\n")
}

/// Renders policy report JSON.
#[must_use]
pub fn render_policy_report_json(report: &PolicyReport) -> String {
    let plans = report
        .plans()
        .iter()
        .map(|plan| {
            let operation = plan.operation();
            let findings = plan
                .findings()
                .iter()
                .map(|finding| {
                    format!(
                        "{{\"id\":\"{}\",\"severity\":\"{}\",\"subject\":\"{}\",\"message\":\"{}\"}}",
                        json_escape(finding.id()),
                        finding.severity().as_str(),
                        json_escape(finding.subject()),
                        json_escape(finding.message())
                    )
                })
                .collect::<Vec<_>>()
                .join(",");

            format!(
                "{{\"operation\":\"{}\",\"mutability\":\"{}\",\"risk\":\"{}\",\"approval_gate\":\"{}\",\"requires_approval\":{},\"blocked\":{},\"findings\":[{}]}}",
                operation.kind().as_str(),
                operation.mutability().as_str(),
                operation.risk().as_str(),
                operation.approval_gate().as_str(),
                plan.requires_approval(),
                plan.is_blocked(),
                findings
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"command\":\"policy\",\"plans\":{},\"approval_required\":{},\"blocked\":{},\"items\":[{}]}}",
        report.plan_count(),
        report.approval_required_count(),
        report.blocked_count(),
        plans
    )
}

/// Renders policy evidence write results.
#[must_use]
pub fn render_policy_evidence_results(results: &[GatedWriteResult]) -> String {
    let mut lines = vec![
        "Monad policy evidence write result".to_string(),
        String::new(),
        "Results:".to_string(),
    ];

    for result in results {
        match result {
            GatedWriteResult::Written(path) | GatedWriteResult::SkippedIdentical(path) => {
                lines.push(format!("  - [{}] {}", result.as_str(), path.display()));
            }
            GatedWriteResult::ApprovalRequired(message) | GatedWriteResult::Blocked(message) => {
                lines.push(format!("  - [{}] {message}", result.as_str()));
            }
        }
    }

    lines.push(String::new());
    lines.push("No commands were executed.".to_string());
    lines.push("No user source files were rewritten.".to_string());
    lines.push("No releases were published.".to_string());
    lines.push("No AI providers were called.".to_string());

    lines.join("\n")
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doctor_is_low_risk_read_only() {
        let classification = classify_operation(OperationKind::Doctor);

        assert_eq!(classification.mutability(), OperationMutability::ReadOnly);
        assert_eq!(classification.risk(), RiskLevel::Low);
        assert_eq!(classification.approval_gate(), ApprovalGate::None);
    }

    #[test]
    fn generated_writes_require_explicit_approval() {
        let plan = build_approval_plan(OperationKind::Upgrade);

        assert!(plan.requires_approval());
        assert!(!plan.is_blocked());
    }

    #[test]
    fn patch_is_blocked_in_mvp() {
        let plan = build_approval_plan(OperationKind::Patch);

        assert!(plan.is_blocked());
    }

    #[test]
    fn user_owned_file_write_is_blocked() {
        let plan = check_file_operation(Path::new("src/main.rs"), FileOperationIntent::WriteUserOwned);

        assert!(plan.is_blocked());
    }

    #[test]
    fn absolute_file_path_is_blocked() {
        let plan = check_file_operation(Path::new("/tmp/example"), FileOperationIntent::WriteGenerated);

        assert!(plan.is_blocked());
    }

    #[test]
    fn publish_command_is_blocked() {
        let plan = check_command_execution(CommandExecutionIntent::Publish);

        assert!(plan.is_blocked());
    }

    #[test]
    fn gated_write_requires_approval() {
        let root = std::env::temp_dir().join("monad-policy-approval-required");
        fs::remove_dir_all(&root).ok();

        let request = GatedWriteRequest::new(".monad/reports/example.md", "hello\n", false);
        let result = gated_generated_write(&root, &request).expect("policy check should succeed");

        assert!(matches!(result, GatedWriteResult::ApprovalRequired(_)));
        assert!(!root.join(".monad/reports/example.md").exists());

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn gated_write_refuses_unsafe_overwrite() {
        let root = std::env::temp_dir().join("monad-policy-overwrite");
        fs::remove_dir_all(&root).ok();
        fs::create_dir_all(root.join(".monad/reports")).expect("reports dir should be created");
        fs::write(root.join(".monad/reports/example.md"), "user-owned\n")
            .expect("file should be written");

        let request = GatedWriteRequest::new(".monad/reports/example.md", "generated\n", true);
        let result = gated_generated_write(&root, &request).expect("policy check should succeed");

        assert!(matches!(result, GatedWriteResult::Blocked(_)));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn policy_report_json_contains_core_fields() {
        let report = build_policy_report();
        let output = render_policy_report_json(&report);

        assert!(output.contains("\"command\":\"policy\""));
        assert!(output.contains("\"approval_required\""));
        assert!(output.contains("\"blocked\""));
    }
}
EOF

python3 <<'PY'
from pathlib import Path
import re

LIB = Path("crates/monad-core/src/lib.rs")
CLI = Path("crates/monad-cli/src/main.rs")


def insert_before_first(text: str, markers: list[str], insertion: str, label: str) -> str:
    if insertion.strip() in text:
        return text

    for marker in markers:
        index = text.find(marker)
        if index != -1:
            return text[:index] + insertion + text[index:]

    raise SystemExit(f"ERROR: could not find insertion point for {label}")


def add_imports_to_monad_core_use(text: str, names: list[str]) -> str:
    match = re.search(r"use monad_core::\{(?P<body>.*?)\};", text, re.DOTALL)
    if not match:
        raise SystemExit("ERROR: could not find monad_core import block in CLI")

    body = match.group("body")
    missing = [name for name in names if name not in body]

    if not missing:
        return text

    addition = "\n    " + ", ".join(missing) + ","
    return text[:match.start("body")] + body + addition + text[match.end("body"):]


def ensure_cli_enum_variant_commas(text: str) -> str:
    enum_start = text.find("enum CliCommand {")
    if enum_start == -1:
        return text

    brace_start = text.find("{", enum_start)
    if brace_start == -1:
        return text

    depth = 0
    enum_end = None
    for index in range(brace_start, len(text)):
        char = text[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                enum_end = index
                break

    if enum_end is None:
        return text

    before = text[:brace_start + 1]
    body = text[brace_start + 1:enum_end]
    after = text[enum_end:]

    lines = body.splitlines(keepends=True)
    fixed = []
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped == "}":
            next_non_empty = ""
            for future in lines[i + 1:]:
                if future.strip():
                    next_non_empty = future.strip()
                    break
            if next_non_empty.startswith("///") or next_non_empty.startswith("#[") or next_non_empty[:1].isupper():
                line = line.rstrip("\n") + ",\n"
        fixed.append(line)

    return before + "".join(fixed) + after


# Patch lib.rs.
lib = LIB.read_text()

if "pub mod policy;" not in lib:
    lib = insert_before_first(
        lib,
        ["pub mod release;", "pub mod repository_context_pack;", "pub mod workspace;"],
        "pub mod policy;\n",
        "policy module declaration",
    )

if "pub use policy::" not in lib:
    export = """pub use policy::{
    ApprovalGate, ApprovalPlan, CommandExecutionIntent, FileOperationIntent, GatedWriteRequest,
    GatedWriteResult, OperationClassification, OperationKind, OperationMutability, PolicyFinding,
    PolicyFindingSeverity, PolicyReport, RiskLevel, build_approval_plan, build_policy_report,
    check_command_execution, check_file_operation, classify_operation, gated_generated_write,
    render_policy_evidence_results, render_policy_report, render_policy_report_json,
    write_policy_evidence,
};
"""
    lib = insert_before_first(
        lib,
        ["pub use release::{", "pub use repository_context_pack::{", "pub use workspace::{"],
        export,
        "policy public exports",
    )

LIB.write_text(lib)


# Patch CLI.
cli = CLI.read_text()

cli = add_imports_to_monad_core_use(
    cli,
    [
        "build_policy_report",
        "render_policy_evidence_results",
        "render_policy_report",
        "render_policy_report_json",
        "write_policy_evidence",
    ],
)

if "Policy {" not in cli:
    variant = """    /// Render or write policy and approval-gate evidence.
    Policy {
        /// Whether to run in dry-run mode.
        dry_run: bool,

        /// Whether to write generated policy evidence.
        yes: bool,

        /// Requested output format.
        output_format: OutputFormat,
    }

"""
    cli = insert_before_first(
        cli,
        [
            "    /// Generate provider-agnostic AI context and memory artifacts.\n",
            "    AiContext {\n",
            "    /// Plan or apply safe repository upgrades.\n",
            "    Upgrade {\n",
            "    /// Plan release readiness without publishing.\n",
            "    Release {\n",
        ],
        variant,
        "CliCommand::Policy variant",
    )

# Allow --yes for policy.
if 'Some("policy")' not in cli:
    cli = cli.replace(
        'parts.first().copied() != Some("ai-context")',
        'parts.first().copied() != Some("ai-context")\n            && parts.first().copied() != Some("policy")',
        1,
    )
    cli = cli.replace(
        "--yes is only supported for init, add, sync, upgrade, and ai-context commands",
        "--yes is only supported for init, add, sync, upgrade, ai-context, and policy commands",
    )

if '["policy"] => {' not in cli:
    parse_arm = """            ["policy"] => {
                reject_write_for_non_context(write)?;
                require_policy_mode(dry_run, yes)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::Policy {
                    dry_run,
                    yes,
                    output_format,
                })
            }
            ["policy", other, ..] => {
                reject_write_for_non_context(write)?;
                Err(format!("unknown policy argument: {other}"))
            }
"""
    cli = insert_before_first(
        cli,
        [
            '            ["ai-context"] => {\n',
            '            ["upgrade"] => {\n',
            '            ["release"] => {\n',
            '            ["doctor"] => {\n',
        ],
        parse_arm,
        "policy parse arm",
    )

if "CliCommand::Policy" not in cli.split("match command", 1)[-1]:
    run_arm = """        CliCommand::Policy {
            dry_run,
            yes,
            output_format,
        } => render_policy(dry_run, yes, output_format),
"""
    cli = insert_before_first(
        cli,
        [
            "        CliCommand::AiContext {\n",
            "        CliCommand::Upgrade {\n",
            "        CliCommand::Release {\n",
            "        CliCommand::Doctor { output_format } => render_doctor(output_format),\n",
        ],
        run_arm,
        "policy run arm",
    )

if "fn require_policy_mode" not in cli:
    helper = """/// Requires exactly one policy mode.
fn require_policy_mode(dry_run: bool, yes: bool) -> Result<(), String> {
    match (dry_run, yes) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => {
            Err("policy currently requires either --dry-run to preview or --yes to write generated policy evidence".to_string())
        }
        (true, true) => Err("policy accepts either --dry-run or --yes, not both".to_string()),
    }
}

"""
    cli = insert_before_first(
        cli,
        [
            "/// Requires exactly one AI context mode.\n",
            "fn require_ai_context_mode",
            "/// Requires exactly one upgrade mode.\n",
            "fn require_upgrade_mode",
            "/// Requires dry-run mode for the first release foundation.\n",
            "fn require_release_mode",
        ],
        helper,
        "require_policy_mode helper",
    )

if "fn render_policy(" not in cli:
    renderer = """/// Renders or writes policy evidence.
fn render_policy(dry_run: bool, yes: bool, output_format: OutputFormat) -> Result<String, String> {
    if dry_run {
        let report = build_policy_report();
        return match output_format {
            OutputFormat::Text => Ok(render_policy_report(&report)),
            OutputFormat::Json => Ok(render_policy_report_json(&report)),
        };
    }

    if yes {
        let root = std::env::current_dir().map_err(|error| error.to_string())?;
        let results = write_policy_evidence(root)?;
        return Ok(render_policy_evidence_results(&results));
    }

    Err("policy currently requires either --dry-run to preview or --yes to write generated policy evidence".to_string())
}

"""
    cli = insert_before_first(
        cli,
        [
            "/// Renders or writes AI context artifacts.\n",
            "fn render_ai_context",
            "/// Renders or applies repository upgrade output.\n",
            "fn render_upgrade",
            "/// Renders release readiness planning output.\n",
            "fn render_release",
        ],
        renderer,
        "render_policy helper",
    )

if "policy --dry-run" not in cli:
    cli = cli.replace(
        '        "  ai-context --dry-run                    Preview AI context/memory artifacts",\n',
        '        "  ai-context --dry-run                    Preview AI context/memory artifacts",\n        "  policy --dry-run                        Preview policy and approval gates",\n        "  policy --dry-run --format=json          Preview policy report as JSON",\n        "  policy --yes                            Write generated policy evidence",\n',
        1,
    )
    cli = cli.replace(
        '        "  monad ai-context --dry-run",\n',
        '        "  monad ai-context --dry-run",\n        "  monad policy --dry-run",\n        "  monad policy --dry-run --format=json",\n',
        1,
    )
    cli = cli.replace(
        '        "  ai-context never calls providers or sends repo data remotely.",\n',
        '        "  ai-context never calls providers or sends repo data remotely.",\n        "  policy writes generated evidence only and never approves risky work automatically.",\n',
        1,
    )

if "fn policy_dry_run_command_parses" not in cli and "parse_arguments(&" in cli:
    tests = """    #[test]
    fn policy_dry_run_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "policy", "--dry-run"])
                .expect("policy dry-run should parse"),
            CliCommand::Policy {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "policy", "--dry-run", "--format=json"])
                .expect("policy json should parse"),
            CliCommand::Policy {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Json,
            }
        );
    }

    #[test]
    fn policy_yes_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "policy", "--yes"]).expect("policy yes should parse"),
            CliCommand::Policy {
                dry_run: false,
                yes: true,
                output_format: OutputFormat::Text,
            }
        );
    }

    #[test]
    fn policy_requires_mode() {
        let error = parse_arguments(&["monad", "policy"]).expect_err("policy should require mode");

        assert!(error.contains("policy currently requires either --dry-run"));
    }

"""
    cli = insert_before_first(
        cli,
        [
            "    #[test]\n    fn ai_context_dry_run_command_parses()",
            "    #[test]\n    fn upgrade_dry_run_command_parses()",
            "    #[test]\n    fn release_command_parses_text_and_json_formats()",
        ],
        tests,
        "policy parser tests",
    )

cli = ensure_cli_enum_variant_commas(cli)
CLI.write_text(cli)
PY

cat > docs/commands/POLICY.md <<'EOF'
---
title: monad policy
status: complete
epic: E19
---

# `monad policy`

`monad policy` previews or writes generated policy/approval-gate evidence.

## Commands

```bash
monad policy --dry-run
monad policy --dry-run --format=json
monad policy --yes
```

## Safety contract

`policy` does not:

- execute commands;
- rewrite user source files;
- publish releases;
- call AI providers;
- automatically approve risky operations;
- use a remote policy service;
- claim OPA/Rego support.

## Generated evidence

`monad policy --yes` writes generated evidence only:

```text
.monad/reports/policy-report.md
.monad/reports/policy-report.json
```
EOF

cat > docs/architecture/POLICY-MODEL.md <<'EOF'
---
title: Policy Model
status: complete
epic: E19
---

# Policy Model

E19 defines Monad's first policy and approval-gate foundation.

## Core concepts

```text
OperationKind
OperationMutability
RiskLevel
ApprovalGate
OperationClassification
ApprovalPlan
PolicyFinding
PolicyReport
GatedWriteRequest
GatedWriteResult
```

## Approval gates

- `none`
- `dry-run-review`
- `explicit-yes`
- `forbidden`

## Principles

- read-only operations are low risk;
- generated writes require explicit approval;
- source mutations are blocked in MVP policy;
- publishing and remote side effects remain dry-run or blocked;
- risky operations are never approved automatically.
EOF

cat > docs/architecture/APPROVAL-GATES.md <<'EOF'
---
title: Approval Gates
status: complete
epic: E19
---

# Approval Gates

Approval gates are the boundary between planning and mutation.

## Gate types

| Gate | Meaning |
|---|---|
| none | Safe read-only operation |
| dry-run-review | Must be reviewed before any future apply path |
| explicit-yes | Requires explicit caller approval marker |
| forbidden | Blocked in the MVP policy foundation |

## Examples

- `doctor` uses `none`.
- `sync --yes`, `upgrade --yes`, and `ai-context --yes` require `explicit-yes`.
- release publishing remains outside MVP.
- patch/source mutation is `forbidden`.
EOF

cat > docs/workflows/POLICY-WORKFLOW.md <<'EOF'
---
title: Policy Workflow
status: complete
epic: E19
---

# Policy Workflow

## Preview policy

```bash
monad policy --dry-run
```

## Preview JSON

```bash
monad policy --dry-run --format=json
```

## Write generated evidence

```bash
monad policy --yes
```

## Review evidence

```text
.monad/reports/policy-report.md
.monad/reports/policy-report.json
```
EOF

cat > docs/verification/POLICY-SMOKE-TESTS.md <<'EOF'
---
title: Policy Smoke Tests
status: complete
epic: E19
---

# Policy Smoke Tests

Run:

```bash
tools/scripts/verify-policy.sh
```

This verifies:

- policy dry-run output;
- policy JSON output;
- generated policy evidence writes;
- dry-run writes no files;
- blocked and approval-required examples are represented.
EOF

cat > docs/verification/E19-CLOSEOUT.md <<'EOF'
---
title: E19 Closeout
status: complete
epic: E19
---

# E19 Closeout — Policy, Safety, and Approval Gate Foundation

E19 is complete when:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-policy.sh
tools/scripts/verify-e19.sh
```

## Completed capability

```bash
monad policy --dry-run
monad policy --dry-run --format=json
monad policy --yes
```

## Safety retained

No command execution.

No user source rewrites.

No release publishing.

No AI provider calls.

No automatic risky approval.
EOF

cat > tools/scripts/verify-policy.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

echo "==> verify-policy: repo root: $REPO_ROOT"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

(
  cd "$tmpdir"

  echo "==> verify policy dry-run writes no files"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- policy --dry-run >/tmp/monad-e19-policy.out
  grep -q "Monad policy and approval-gate report" /tmp/monad-e19-policy.out
  grep -q "approval_required:" /tmp/monad-e19-policy.out
  grep -q "blocked:" /tmp/monad-e19-policy.out
  grep -q "No commands were executed." /tmp/monad-e19-policy.out
  test ! -e .monad/reports/policy-report.md

  echo "==> verify policy json"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- policy --dry-run --format=json >/tmp/monad-e19-policy.json
  grep -q '"command":"policy"' /tmp/monad-e19-policy.json
  grep -q '"approval_required"' /tmp/monad-e19-policy.json
  grep -q '"blocked"' /tmp/monad-e19-policy.json

  echo "==> verify policy evidence write"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- policy --yes >/tmp/monad-e19-policy-apply.out
  grep -q "Monad policy evidence write result" /tmp/monad-e19-policy-apply.out
  grep -q "No user source files were rewritten." /tmp/monad-e19-policy-apply.out
  test -f .monad/reports/policy-report.md
  test -f .monad/reports/policy-report.json
  grep -q "patch" .monad/reports/policy-report.md
  grep -q "forbidden" .monad/reports/policy-report.md
)

echo "verify-policy: PASS"
EOF
chmod +x tools/scripts/verify-policy.sh

cat > tools/scripts/verify-e19.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

echo "==> E19 verification"

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-policy.sh

if [ -x tools/scripts/verify.sh ]; then
  tools/scripts/verify.sh
fi

echo "verify-e19: PASS"
EOF
chmod +x tools/scripts/verify-e19.sh

cat > work/learning/E19/EPIC-E19-policy-safety-gates.md <<'EOF'
---
title: Epic E19 Learning Note
epic: E19
---

# Epic E19 Learning Note: Policy and Approval Gates

E19 establishes a safety layer.

The core pattern is:

```text
classify operation → evaluate risk → require approval or block → write evidence
```

This gives future commands a shared policy vocabulary before deeper enforcement is added.
EOF

cat > work/deliverables/E19/EPIC-E19-policy-safety-gates.md <<'EOF'
---
title: Epic E19 Deliverable Record
epic: E19
status: complete
---

# Epic E19 Deliverable Record

## Epic

E19 — Policy, Safety, and Approval Gate Foundation.

## Completed work packets

- WP-E19-001 — Define policy and approval-gate contract
- WP-E19-002 — Add operation classification and risk model
- WP-E19-003 — Add approval plan and approval evidence model
- WP-E19-004 — Add policy checks for file operations and command execution
- WP-E19-005 — Add gated write/apply foundation
- WP-E19-006 — Add policy reports and smoke tests

## Implementation files

```text
crates/monad-core/src/policy.rs
crates/monad-core/src/lib.rs
crates/monad-cli/src/main.rs
tools/scripts/verify-policy.sh
tools/scripts/verify-e19.sh
```

## Verification command

```bash
tools/scripts/verify-e19.sh
```
EOF

echo "==> Formatting Rust code"
cargo fmt

echo
echo "==> Epic E19 patch complete."
echo
echo "Recommended inspection:"
echo "  git diff -- crates/monad-core/src/policy.rs"
echo "  git diff -- crates/monad-core/src/lib.rs"
echo "  git diff -- crates/monad-cli/src/main.rs"
echo "  git diff -- docs/commands/POLICY.md"
echo "  git diff -- tools/scripts/verify-policy.sh"
echo
echo "Recommended verification:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-policy.sh"
echo "  tools/scripts/verify-e19.sh"
echo
echo "Commit:"
echo "  git add crates/monad-core/src/policy.rs crates/monad-core/src/lib.rs crates/monad-cli/src/main.rs docs/commands/POLICY.md docs/architecture/POLICY-MODEL.md docs/architecture/APPROVAL-GATES.md docs/workflows/POLICY-WORKFLOW.md docs/verification/POLICY-SMOKE-TESTS.md docs/verification/E19-CLOSEOUT.md tools/scripts/verify-policy.sh tools/scripts/verify-e19.sh work/learning/E19/EPIC-E19-policy-safety-gates.md work/deliverables/E19/EPIC-E19-policy-safety-gates.md"
echo "  git commit -m \"feat(policy): add approval gate foundation\""
echo
echo "Done."
