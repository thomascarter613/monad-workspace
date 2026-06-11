#!/usr/bin/env bash
set -euo pipefail

# Complete Epic E36 — GitHub Integration and PR Workflow Foundation
#
# Safety:
# - local-first and deterministic
# - no GitHub/API calls
# - no git branch creation
# - no PR creation
# - no issue closure
# - no remote state mutation
# - no network access
# - no package-manager invocation
# - generated evidence writes only under .monad/reports through E19 approval gates
# - backs up touched files under .monad/script-backups/...

if [[ ! -f "Cargo.toml" || ! -d "crates/monad-core/src" || ! -d "crates/monad-cli/src" ]]; then
  echo "Run this script from the monad-workspace repository root." >&2
  exit 1
fi

BACKUP_DIR=".monad/script-backups/complete-epic-e36-github-workflow-foundation-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$BACKUP_DIR"

backup_if_exists() {
  local path="$1"
  if [[ -e "$path" ]]; then
    mkdir -p "$BACKUP_DIR/$(dirname "$path")"
    cp -a "$path" "$BACKUP_DIR/$path"
  fi
}

backup_if_exists "crates/monad-core/src/lib.rs"
backup_if_exists "crates/monad-cli/src/main.rs"
backup_if_exists "crates/monad-core/src/github_workflow.rs"
backup_if_exists "docs/github-workflow/README.md"
backup_if_exists "docs/roadmap/epic-36-github-integration-pr-workflow.md"
backup_if_exists "tools/scripts/verify-github-workflow.sh"
backup_if_exists "tools/scripts/verify-e36.sh"

mkdir -p crates/monad-core/src docs/github-workflow docs/roadmap tools/scripts

cat > crates/monad-core/src/github_workflow.rs <<'RS'
//! GitHub Integration and PR Workflow Foundation.
//!
//! E36 defines a local, supervised GitHub workflow planning foundation. It
//! models issue export/sync, branch and PR planning, review-pack generation,
//! and issue closeout evidence without calling GitHub, creating branches,
//! opening PRs, closing issues, accessing the network, or mutating remote state.

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{gated_generated_write, GatedWriteRequest, GatedWriteResult};

/// GitHub integration boundary decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GithubWorkflowBoundaryDecision {
    LocalPlanOnly,
    ReviewRequired,
    RemoteMutationBlocked,
}

impl GithubWorkflowBoundaryDecision {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalPlanOnly => "local-plan-only",
            Self::ReviewRequired => "review-required",
            Self::RemoteMutationBlocked => "remote-mutation-blocked",
        }
    }
}

/// Local GitHub integration boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GithubIntegrationBoundary {
    repository_full_name: String,
    remote_api_disabled: bool,
    remote_mutation_disabled: bool,
    network_disabled: bool,
    branch_creation_disabled: bool,
    pr_creation_disabled: bool,
    issue_closure_disabled: bool,
    rules: Vec<String>,
}

impl GithubIntegrationBoundary {
    #[must_use]
    pub fn default_boundary() -> Self {
        Self {
            repository_full_name: "thomascarter613/monad-workspace".to_string(),
            remote_api_disabled: true,
            remote_mutation_disabled: true,
            network_disabled: true,
            branch_creation_disabled: true,
            pr_creation_disabled: true,
            issue_closure_disabled: true,
            rules: vec![
                "E36 produces local GitHub workflow plans only.".to_string(),
                "No GitHub API calls are made by Monad.".to_string(),
                "No branches or pull requests are created by Monad.".to_string(),
                "No GitHub issues are closed by Monad.".to_string(),
                "Review packs and closeout records are generated as evidence.".to_string(),
            ],
        }
    }

    #[must_use]
    pub fn repository_full_name(&self) -> &str {
        &self.repository_full_name
    }

    #[must_use]
    pub const fn remote_api_disabled(&self) -> bool {
        self.remote_api_disabled
    }

    #[must_use]
    pub const fn remote_mutation_disabled(&self) -> bool {
        self.remote_mutation_disabled
    }

    #[must_use]
    pub const fn network_disabled(&self) -> bool {
        self.network_disabled
    }

    #[must_use]
    pub const fn branch_creation_disabled(&self) -> bool {
        self.branch_creation_disabled
    }

    #[must_use]
    pub const fn pr_creation_disabled(&self) -> bool {
        self.pr_creation_disabled
    }

    #[must_use]
    pub const fn issue_closure_disabled(&self) -> bool {
        self.issue_closure_disabled
    }

    #[must_use]
    pub fn rules(&self) -> &[String] {
        &self.rules
    }
}

/// Issue sync/export record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GithubIssueExportRecord {
    issue_number: u32,
    title: String,
    work_packet: String,
    export_path: PathBuf,
    decision: GithubWorkflowBoundaryDecision,
}

impl GithubIssueExportRecord {
    #[must_use]
    pub fn new(
        issue_number: u32,
        title: impl Into<String>,
        work_packet: impl Into<String>,
        export_path: impl Into<PathBuf>,
        decision: GithubWorkflowBoundaryDecision,
    ) -> Self {
        Self {
            issue_number,
            title: title.into(),
            work_packet: work_packet.into(),
            export_path: export_path.into(),
            decision,
        }
    }

    #[must_use]
    pub const fn issue_number(&self) -> u32 {
        self.issue_number
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub fn work_packet(&self) -> &str {
        &self.work_packet
    }

    #[must_use]
    pub fn export_path(&self) -> &Path {
        &self.export_path
    }

    #[must_use]
    pub const fn decision(&self) -> GithubWorkflowBoundaryDecision {
        self.decision
    }
}

/// Branch and PR plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GithubBranchPrPlan {
    branch_name: String,
    base_branch: String,
    pr_title: String,
    labels: Vec<String>,
    reviewers: Vec<String>,
    creation_disabled: bool,
}

impl GithubBranchPrPlan {
    #[must_use]
    pub fn new(
        branch_name: impl Into<String>,
        base_branch: impl Into<String>,
        pr_title: impl Into<String>,
        mut labels: Vec<String>,
        mut reviewers: Vec<String>,
    ) -> Self {
        labels.sort();
        labels.dedup();
        reviewers.sort();
        reviewers.dedup();

        Self {
            branch_name: branch_name.into(),
            base_branch: base_branch.into(),
            pr_title: pr_title.into(),
            labels,
            reviewers,
            creation_disabled: true,
        }
    }

    #[must_use]
    pub fn branch_name(&self) -> &str {
        &self.branch_name
    }

    #[must_use]
    pub fn base_branch(&self) -> &str {
        &self.base_branch
    }

    #[must_use]
    pub fn pr_title(&self) -> &str {
        &self.pr_title
    }

    #[must_use]
    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    #[must_use]
    pub fn reviewers(&self) -> &[String] {
        &self.reviewers
    }

    #[must_use]
    pub const fn creation_disabled(&self) -> bool {
        self.creation_disabled
    }
}

/// PR description and review pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GithubPrReviewPack {
    summary: String,
    changed_paths: Vec<PathBuf>,
    verification_commands: Vec<String>,
    evidence_paths: Vec<PathBuf>,
    generated_only: bool,
}

impl GithubPrReviewPack {
    #[must_use]
    pub fn new(
        summary: impl Into<String>,
        mut changed_paths: Vec<PathBuf>,
        mut verification_commands: Vec<String>,
        mut evidence_paths: Vec<PathBuf>,
    ) -> Self {
        changed_paths.sort();
        changed_paths.dedup();
        verification_commands.sort();
        verification_commands.dedup();
        evidence_paths.sort();
        evidence_paths.dedup();

        Self {
            summary: summary.into(),
            changed_paths,
            verification_commands,
            evidence_paths,
            generated_only: true,
        }
    }

    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    #[must_use]
    pub fn changed_paths(&self) -> &[PathBuf] {
        &self.changed_paths
    }

    #[must_use]
    pub fn verification_commands(&self) -> &[String] {
        &self.verification_commands
    }

    #[must_use]
    pub fn evidence_paths(&self) -> &[PathBuf] {
        &self.evidence_paths
    }

    #[must_use]
    pub const fn generated_only(&self) -> bool {
        self.generated_only
    }
}

/// Issue closeout/evidence helper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GithubIssueCloseoutEvidence {
    issue_number: u32,
    closeout_note_path: PathBuf,
    required_evidence: Vec<String>,
    closure_disabled: bool,
}

impl GithubIssueCloseoutEvidence {
    #[must_use]
    pub fn new(
        issue_number: u32,
        closeout_note_path: impl Into<PathBuf>,
        mut required_evidence: Vec<String>,
    ) -> Self {
        required_evidence.sort();
        required_evidence.dedup();

        Self {
            issue_number,
            closeout_note_path: closeout_note_path.into(),
            required_evidence,
            closure_disabled: true,
        }
    }

    #[must_use]
    pub const fn issue_number(&self) -> u32 {
        self.issue_number
    }

    #[must_use]
    pub fn closeout_note_path(&self) -> &Path {
        &self.closeout_note_path
    }

    #[must_use]
    pub fn required_evidence(&self) -> &[String] {
        &self.required_evidence
    }

    #[must_use]
    pub const fn closure_disabled(&self) -> bool {
        self.closure_disabled
    }
}

/// Full GitHub workflow plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GithubWorkflowPlan {
    command: String,
    boundary: GithubIntegrationBoundary,
    issue_exports: Vec<GithubIssueExportRecord>,
    branch_pr_plan: GithubBranchPrPlan,
    review_pack: GithubPrReviewPack,
    closeout_evidence: Vec<GithubIssueCloseoutEvidence>,
    evidence_paths: Vec<PathBuf>,
    safety_notes: Vec<String>,
}

impl GithubWorkflowPlan {
    #[must_use]
    pub fn new(
        boundary: GithubIntegrationBoundary,
        mut issue_exports: Vec<GithubIssueExportRecord>,
        branch_pr_plan: GithubBranchPrPlan,
        review_pack: GithubPrReviewPack,
        mut closeout_evidence: Vec<GithubIssueCloseoutEvidence>,
    ) -> Self {
        issue_exports.sort_by(|left, right| left.issue_number().cmp(&right.issue_number()));
        closeout_evidence.sort_by(|left, right| left.issue_number().cmp(&right.issue_number()));

        Self {
            command: "github-plan".to_string(),
            boundary,
            issue_exports,
            branch_pr_plan,
            review_pack,
            closeout_evidence,
            evidence_paths: vec![
                PathBuf::from(".monad/reports/github-workflow-plan.md"),
                PathBuf::from(".monad/reports/github-workflow-plan.json"),
                PathBuf::from(".monad/reports/github-pr-review-pack.md"),
                PathBuf::from(".monad/reports/github-issue-closeout.md"),
            ],
            safety_notes: vec![
                "No GitHub API calls are made by Monad.".to_string(),
                "No remote state is mutated by Monad.".to_string(),
                "No git branches are created by Monad.".to_string(),
                "No pull requests are opened by Monad.".to_string(),
                "No GitHub issues are closed by Monad.".to_string(),
            ],
        }
    }

    #[must_use]
    pub fn boundary(&self) -> &GithubIntegrationBoundary {
        &self.boundary
    }

    #[must_use]
    pub fn issue_exports(&self) -> &[GithubIssueExportRecord] {
        &self.issue_exports
    }

    #[must_use]
    pub fn branch_pr_plan(&self) -> &GithubBranchPrPlan {
        &self.branch_pr_plan
    }

    #[must_use]
    pub fn review_pack(&self) -> &GithubPrReviewPack {
        &self.review_pack
    }

    #[must_use]
    pub fn closeout_evidence(&self) -> &[GithubIssueCloseoutEvidence] {
        &self.closeout_evidence
    }

    #[must_use]
    pub fn evidence_paths(&self) -> &[PathBuf] {
        &self.evidence_paths
    }

    #[must_use]
    pub fn safety_notes(&self) -> &[String] {
        &self.safety_notes
    }
}

/// Apply result for generated GitHub workflow evidence writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GithubWorkflowApplyResult {
    write_results: Vec<GatedWriteResult>,
}

impl GithubWorkflowApplyResult {
    #[must_use]
    pub fn new(write_results: Vec<GatedWriteResult>) -> Self {
        Self { write_results }
    }

    #[must_use]
    pub fn write_results(&self) -> &[GatedWriteResult] {
        &self.write_results
    }
}

/// Builds the E36 GitHub workflow foundation plan.
#[must_use]
pub fn build_github_workflow_plan(_root: impl AsRef<Path>) -> GithubWorkflowPlan {
    let boundary = GithubIntegrationBoundary::default_boundary();

    let issue_exports = vec![
        GithubIssueExportRecord::new(
            176,
            "E36 — GitHub Integration and PR Workflow Foundation",
            "E36",
            ".monad/reports/github/issues/176.json",
            GithubWorkflowBoundaryDecision::LocalPlanOnly,
        ),
        GithubIssueExportRecord::new(
            282,
            "WP-E36-001 — Define GitHub integration boundary",
            "WP-E36-001",
            ".monad/reports/github/issues/282.json",
            GithubWorkflowBoundaryDecision::LocalPlanOnly,
        ),
        GithubIssueExportRecord::new(
            285,
            "WP-E36-004 — Add PR description and review-pack generation",
            "WP-E36-004",
            ".monad/reports/github/issues/285.json",
            GithubWorkflowBoundaryDecision::LocalPlanOnly,
        ),
    ];

    let branch_pr_plan = GithubBranchPrPlan::new(
        "feat/e36-github-pr-workflow-foundation",
        "main",
        "feat: add E36 GitHub integration PR workflow foundation",
        vec![
            "epic:e36".to_string(),
            "area:github-workflow".to_string(),
            "type:foundation".to_string(),
        ],
        vec!["human-review".to_string()],
    );

    let review_pack = GithubPrReviewPack::new(
        "Adds a local, supervised GitHub workflow planning foundation.",
        vec![
            PathBuf::from("crates/monad-core/src/github_workflow.rs"),
            PathBuf::from("crates/monad-cli/src/main.rs"),
            PathBuf::from("docs/github-workflow/README.md"),
            PathBuf::from("tools/scripts/verify-github-workflow.sh"),
        ],
        vec![
            "cargo fmt --check".to_string(),
            "cargo test".to_string(),
            "cargo clippy --all-targets --all-features -- -D warnings".to_string(),
            "tools/scripts/verify-github-workflow.sh".to_string(),
            "tools/scripts/verify-e36.sh".to_string(),
        ],
        vec![
            PathBuf::from(".monad/reports/github-workflow-plan.md"),
            PathBuf::from(".monad/reports/github-pr-review-pack.md"),
        ],
    );

    let closeout_evidence = vec![
        GithubIssueCloseoutEvidence::new(
            176,
            ".monad/reports/github-issue-closeout.md",
            vec![
                "verification commands passed".to_string(),
                "commit recorded".to_string(),
                "closeout note reviewed".to_string(),
            ],
        ),
        GithubIssueCloseoutEvidence::new(
            282,
            ".monad/reports/github-issue-closeout.md",
            vec!["boundary documented".to_string(), "tests passed".to_string()],
        ),
    ];

    GithubWorkflowPlan::new(
        boundary,
        issue_exports,
        branch_pr_plan,
        review_pack,
        closeout_evidence,
    )
}

/// Writes generated GitHub workflow evidence.
pub fn write_github_workflow_evidence(
    root: impl AsRef<Path>,
) -> Result<GithubWorkflowApplyResult, String> {
    let root = root.as_ref();
    let plan = build_github_workflow_plan(root);
    let requests = [
        GatedWriteRequest::new(
            ".monad/reports/github-workflow-plan.md",
            render_github_workflow_plan(&plan),
            true,
        ),
        GatedWriteRequest::new(
            ".monad/reports/github-workflow-plan.json",
            render_github_workflow_plan_json(&plan),
            true,
        ),
        GatedWriteRequest::new(
            ".monad/reports/github-pr-review-pack.md",
            render_github_pr_review_pack(&plan),
            true,
        ),
        GatedWriteRequest::new(
            ".monad/reports/github-issue-closeout.md",
            render_github_issue_closeout_evidence(&plan),
            true,
        ),
    ];

    let write_results = requests
        .iter()
        .map(|request| gated_generated_write(root, request))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(GithubWorkflowApplyResult::new(write_results))
}

/// Renders a text GitHub workflow plan.
#[must_use]
pub fn render_github_workflow_plan(plan: &GithubWorkflowPlan) -> String {
    let mut lines = vec![
        "Monad GitHub integration and PR workflow foundation plan".to_string(),
        String::new(),
        "GitHub integration boundary:".to_string(),
        format!("  repository: {}", plan.boundary().repository_full_name()),
        format!("  remote_api_disabled: {}", plan.boundary().remote_api_disabled()),
        format!(
            "  remote_mutation_disabled: {}",
            plan.boundary().remote_mutation_disabled()
        ),
        format!("  network_disabled: {}", plan.boundary().network_disabled()),
        format!(
            "  branch_creation_disabled: {}",
            plan.boundary().branch_creation_disabled()
        ),
        format!(
            "  pr_creation_disabled: {}",
            plan.boundary().pr_creation_disabled()
        ),
        format!(
            "  issue_closure_disabled: {}",
            plan.boundary().issue_closure_disabled()
        ),
        String::new(),
        "Boundary rules:".to_string(),
    ];

    for rule in plan.boundary().rules() {
        lines.push(format!("  - {rule}"));
    }

    lines.push(String::new());
    lines.push("Issue sync/export model:".to_string());
    for record in plan.issue_exports() {
        lines.push(format!(
            "  - #{} {} work_packet={} decision={} export={}",
            record.issue_number(),
            record.title(),
            record.work_packet(),
            record.decision().as_str(),
            record.export_path().display()
        ));
    }

    lines.push(String::new());
    lines.push("Branch and PR planning model:".to_string());
    lines.push(format!("  branch: {}", plan.branch_pr_plan().branch_name()));
    lines.push(format!("  base: {}", plan.branch_pr_plan().base_branch()));
    lines.push(format!("  title: {}", plan.branch_pr_plan().pr_title()));
    lines.push(format!(
        "  creation_disabled: {}",
        plan.branch_pr_plan().creation_disabled()
    ));
    for label in plan.branch_pr_plan().labels() {
        lines.push(format!("  - label: {label}"));
    }
    for reviewer in plan.branch_pr_plan().reviewers() {
        lines.push(format!("  - reviewer: {reviewer}"));
    }

    lines.push(String::new());
    lines.push("PR description and review-pack generation:".to_string());
    lines.push(format!("  summary: {}", plan.review_pack().summary()));
    for command in plan.review_pack().verification_commands() {
        lines.push(format!("  - verify: {command}"));
    }

    lines.push(String::new());
    lines.push("Issue closeout/evidence helpers:".to_string());
    for evidence in plan.closeout_evidence() {
        lines.push(format!(
            "  - #{} closure_disabled={} note={}",
            evidence.issue_number(),
            evidence.closure_disabled(),
            evidence.closeout_note_path().display()
        ));
    }

    lines.push(String::new());
    lines.push("Evidence outputs:".to_string());
    for path in plan.evidence_paths() {
        lines.push(format!("  - {}", path.display()));
    }

    lines.push(String::new());
    lines.push("Safety notes:".to_string());
    for note in plan.safety_notes() {
        lines.push(format!("  - {note}"));
    }

    lines.join("\n")
}

/// Renders a JSON GitHub workflow plan.
#[must_use]
pub fn render_github_workflow_plan_json(plan: &GithubWorkflowPlan) -> String {
    serde_json::to_string_pretty(plan).unwrap_or_else(|_| {
        "{\n  \"command\": \"github-plan\",\n  \"error\": \"github workflow plan serialization failed\"\n}".to_string()
    })
}

/// Renders a markdown PR review pack.
#[must_use]
pub fn render_github_pr_review_pack(plan: &GithubWorkflowPlan) -> String {
    let mut lines = vec![
        "# GitHub PR Review Pack".to_string(),
        String::new(),
        format!("## Summary\n\n{}", plan.review_pack().summary()),
        String::new(),
        "## Changed Paths".to_string(),
    ];

    for path in plan.review_pack().changed_paths() {
        lines.push(format!("- {}", path.display()));
    }

    lines.push(String::new());
    lines.push("## Verification".to_string());
    for command in plan.review_pack().verification_commands() {
        lines.push(format!("- `{command}`"));
    }

    lines.push(String::new());
    lines.push("## Evidence".to_string());
    for path in plan.review_pack().evidence_paths() {
        lines.push(format!("- {}", path.display()));
    }

    lines.push(String::new());
    lines.push("No GitHub API calls, branch creation, or PR creation were performed.".to_string());

    lines.join("\n")
}

/// Renders issue closeout/evidence helper text.
#[must_use]
pub fn render_github_issue_closeout_evidence(plan: &GithubWorkflowPlan) -> String {
    let mut lines = vec![
        "# GitHub Issue Closeout Evidence".to_string(),
        String::new(),
        "Issue closure is disabled in E36. Use this as a review checklist only.".to_string(),
        String::new(),
    ];

    for evidence in plan.closeout_evidence() {
        lines.push(format!("## Issue #{}", evidence.issue_number()));
        lines.push(format!(
            "- closeout_note_path: {}",
            evidence.closeout_note_path().display()
        ));
        lines.push(format!("- closure_disabled: {}", evidence.closure_disabled()));
        for item in evidence.required_evidence() {
            lines.push(format!("- required: {item}"));
        }
        lines.push(String::new());
    }

    lines.join("\n")
}

/// Renders generated evidence write results.
#[must_use]
pub fn render_github_workflow_apply_result(result: &GithubWorkflowApplyResult) -> String {
    let mut lines = vec![
        "Monad GitHub workflow evidence write result".to_string(),
        String::new(),
        "Results:".to_string(),
    ];

    for write_result in result.write_results() {
        match write_result {
            GatedWriteResult::Written(path) | GatedWriteResult::SkippedIdentical(path) => {
                lines.push(format!("  - [{}] {}", write_result.as_str(), path.display()));
            }
            GatedWriteResult::ApprovalRequired(message) | GatedWriteResult::Blocked(message) => {
                lines.push(format!("  - [{}] {message}", write_result.as_str()));
            }
        }
    }

    lines.push(String::new());
    lines.push("No GitHub API calls were made.".to_string());
    lines.push("No pull request was opened.".to_string());
    lines.push("No issue was closed.".to_string());

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundary_blocks_remote_mutation() {
        let boundary = GithubIntegrationBoundary::default_boundary();

        assert!(boundary.remote_api_disabled());
        assert!(boundary.remote_mutation_disabled());
        assert!(boundary.network_disabled());
        assert!(boundary.pr_creation_disabled());
        assert!(boundary.issue_closure_disabled());
    }

    #[test]
    fn issue_export_model_contains_e36_records() {
        let plan = build_github_workflow_plan(".");

        assert!(plan.issue_exports().iter().any(|record| record.issue_number() == 176));
        assert!(plan
            .issue_exports()
            .iter()
            .all(|record| record.decision() == GithubWorkflowBoundaryDecision::LocalPlanOnly));
    }

    #[test]
    fn branch_and_pr_plan_is_creation_disabled() {
        let plan = build_github_workflow_plan(".");

        assert_eq!(
            plan.branch_pr_plan().branch_name(),
            "feat/e36-github-pr-workflow-foundation"
        );
        assert!(plan.branch_pr_plan().creation_disabled());
    }

    #[test]
    fn review_pack_contains_verification_commands() {
        let plan = build_github_workflow_plan(".");

        assert!(plan
            .review_pack()
            .verification_commands()
            .contains(&"cargo test".to_string()));
        assert!(plan.review_pack().generated_only());
    }

    #[test]
    fn issue_closeout_is_disabled() {
        let plan = build_github_workflow_plan(".");

        assert!(plan
            .closeout_evidence()
            .iter()
            .all(GithubIssueCloseoutEvidence::closure_disabled));
    }

    #[test]
    fn text_render_mentions_github_sections() {
        let plan = build_github_workflow_plan(".");
        let text = render_github_workflow_plan(&plan);

        assert!(text.contains("Monad GitHub integration and PR workflow foundation plan"));
        assert!(text.contains("Issue sync/export model"));
        assert!(text.contains("PR description and review-pack generation"));
    }

    #[test]
    fn json_render_contains_github_plan_command() {
        let plan = build_github_workflow_plan(".");
        let json = render_github_workflow_plan_json(&plan);

        assert!(json.contains("\"command\": \"github-plan\""));
        assert!(json.contains("thomascarter613/monad-workspace"));
    }
}
RS

cat > docs/github-workflow/README.md <<'MD'
# GitHub Integration and PR Workflow Foundation

E36 adds Monad's GitHub Integration and PR Workflow Foundation.

## Command surface

```bash
monad github-plan --dry-run
monad github-plan --dry-run --format=json
monad github-plan --yes
monad github-workflow --dry-run
monad pr-plan --dry-run
```

## What this foundation models

- GitHub integration boundary
- GitHub issue sync/export model
- Branch and PR planning model
- PR description and review-pack generation
- Issue closeout/evidence helpers
- GitHub workflow smoke tests

## Safety boundaries

This foundation does **not** call GitHub APIs, create branches, open pull
requests, close issues, access networks, invoke package managers, or mutate
remote state.

`--yes` writes generated GitHub workflow evidence only under `.monad/reports`.
MD

cat > docs/roadmap/epic-36-github-integration-pr-workflow.md <<'MD'
# E36 — GitHub Integration and PR Workflow Foundation

## Product Area

GitHub Integration and PR Workflow Foundation

## Objective

Add Monad's deterministic, supervised GitHub workflow foundation: integration
boundary, issue sync/export model, branch and PR planning, PR review-pack
generation, closeout/evidence helpers, and smoke tests.

## Work Packets

- WP-E36-001 — Define GitHub integration boundary
- WP-E36-002 — Add GitHub issue sync/export model
- WP-E36-003 — Add branch and PR planning model
- WP-E36-004 — Add PR description and review-pack generation
- WP-E36-005 — Add issue closeout/evidence helpers
- WP-E36-006 — Add GitHub workflow smoke tests

## Safety

E36 is local and supervised. Monad models GitHub workflow behavior and renders
evidence, but it does not call GitHub, create branches, open PRs, close issues,
or mutate remote state.
MD

cat > tools/scripts/verify-github-workflow.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib github_workflow
cargo test -p monad-cli github_plan

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- github-plan --dry-run > "$text_output"
grep -q "Monad GitHub integration and PR workflow foundation plan" "$text_output"
grep -q "Issue sync/export model" "$text_output"
grep -q "PR description and review-pack generation" "$text_output"

cargo run -p monad-cli -- github-plan --dry-run --format=json > "$json_output"
grep -q '"command": "github-plan"' "$json_output"
grep -q 'thomascarter613/monad-workspace' "$json_output"

echo "GitHub workflow verification passed."
SH
chmod +x tools/scripts/verify-github-workflow.sh

cat > tools/scripts/verify-e36.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-github-workflow.sh

test -f crates/monad-core/src/github_workflow.rs
test -f docs/github-workflow/README.md
test -f docs/roadmap/epic-36-github-integration-pr-workflow.md

grep -q "GitHub Integration and PR Workflow Foundation" docs/github-workflow/README.md
grep -q "WP-E36-001" docs/roadmap/epic-36-github-integration-pr-workflow.md
grep -q "WP-E36-006" docs/roadmap/epic-36-github-integration-pr-workflow.md

echo "E36 verification passed."
SH
chmod +x tools/scripts/verify-e36.sh

python3 - <<'PY'
from pathlib import Path

lib = Path("crates/monad-core/src/lib.rs")
text = lib.read_text()

if "pub mod github_workflow;" not in text:
    anchors = [
        "pub mod web_workbench;\n",
        "pub mod interactive_workbench;\n",
        "pub mod agent_sandbox;\n",
        "pub mod local_ai_retrieval;\n",
        "pub mod mcp_integration;\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, anchor + "pub mod github_workflow;\n", 1)
            break
    else:
        raise SystemExit("Could not find lib.rs module insertion point for github_workflow.")

pub_use = """pub use github_workflow::{
    GithubBranchPrPlan, GithubIntegrationBoundary, GithubIssueCloseoutEvidence,
    GithubIssueExportRecord, GithubPrReviewPack, GithubWorkflowApplyResult,
    GithubWorkflowBoundaryDecision, GithubWorkflowPlan, build_github_workflow_plan,
    render_github_issue_closeout_evidence, render_github_pr_review_pack,
    render_github_workflow_apply_result, render_github_workflow_plan,
    render_github_workflow_plan_json, write_github_workflow_evidence,
};
"""
if "pub use github_workflow::" not in text:
    anchors = [
        "pub use web_workbench::{\n",
        "pub use interactive_workbench::{\n",
        "pub use agent_sandbox::{\n",
        "pub use local_ai_retrieval::{\n",
        "pub use mcp_integration::{\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, pub_use + anchor, 1)
            break
    else:
        raise SystemExit("Could not find lib.rs pub use insertion point for github_workflow.")

lib.write_text(text)

main = Path("crates/monad-cli/src/main.rs")
text = main.read_text()

variant = """    /// Plan GitHub issue/branch/PR workflow without remote mutation.
    GithubPlan {
        /// Whether to run in dry-run mode.
        dry_run: bool,

        /// Whether to write generated GitHub workflow evidence.
        yes: bool,

        /// Requested output format.
        output_format: OutputFormat,
    },

"""
if "GithubPlan {" not in text:
    anchors = [
        "    /// Plan local web workbench behavior without starting a server.\n    WebWorkbenchPlan {\n",
        "    /// Plan interactive workbench/TUI behavior without starting an event loop.\n    WorkbenchPlan {\n",
        "    /// Plan agent workflow sandbox behavior without executing actions.\n    SandboxPlan {\n",
        "    /// Plan local AI retrieval and vector memory without provider calls.\n    RetrievalPlan {\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, variant + anchor, 1)
            break
    else:
        raise SystemExit("Could not find CliCommand enum insertion point for GithubPlan.")

if 'Some("github-plan")' not in text:
    anchor = '            && parts.first().copied() != Some("sync")'
    if anchor in text:
        text = text.replace(
            anchor,
            anchor
            + '\n            && parts.first().copied() != Some("github-plan")'
            + '\n            && parts.first().copied() != Some("github-workflow")'
            + '\n            && parts.first().copied() != Some("pr-plan")',
            1,
        )

parse_arm = """            ["github-plan"] | ["github-workflow"] | ["pr-plan"] => {
                reject_write_for_non_context(write)?;
                require_github_plan_mode(dry_run, yes)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::GithubPlan {
                    dry_run,
                    yes,
                    output_format,
                })
            }
            ["github-plan", other, ..]
            | ["github-workflow", other, ..]
            | ["pr-plan", other, ..] => {
                reject_write_for_non_context(write)?;
                Err(format!("unknown github-plan argument: {other}"))
            }
"""
if '["github-plan"] | ["github-workflow"] | ["pr-plan"]' not in text:
    anchors = [
        '            ["web-workbench-plan"] | ["web-workbench"] | ["web-ui"] => {\n',
        '            ["workbench-plan"] | ["workbench"] | ["tui"] => {\n',
        '            ["sandbox-plan"] | ["agent-sandbox"] | ["sandbox-verify"] => {\n',
        '            ["retrieval-plan"] | ["local-retrieval"] | ["vector-memory"] => {\n',
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, parse_arm + anchor, 1)
            break
    else:
        raise SystemExit("Could not find command parse insertion point for github-plan.")

run_arm = """        CliCommand::GithubPlan {
            dry_run,
            yes,
            output_format,
        } => render_github_plan(dry_run, yes, output_format),
"""
if "render_github_plan(dry_run, yes, output_format)" not in text:
    anchors = [
        "        CliCommand::WebWorkbenchPlan {\n",
        "        CliCommand::WorkbenchPlan {\n",
        "        CliCommand::SandboxPlan {\n",
        "        CliCommand::RetrievalPlan {\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, run_arm + anchor, 1)
            break
    else:
        raise SystemExit("Could not find run match insertion point for github-plan.")

helper = """/// Requires exactly one github-plan mode.
fn require_github_plan_mode(dry_run: bool, yes: bool) -> Result<(), String> {
    match (dry_run, yes) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => Err(
            "github-plan currently requires either --dry-run to preview or --yes to write generated GitHub workflow evidence".to_string(),
        ),
        (true, true) => Err("github-plan accepts either --dry-run or --yes, not both".to_string()),
    }
}

"""
if "fn require_github_plan_mode" not in text:
    anchors = [
        "/// Requires exactly one web-workbench-plan mode.\n",
        "/// Requires exactly one workbench-plan mode.\n",
        "/// Requires exactly one sandbox-plan mode.\n",
        "/// Requires exactly one retrieval-plan mode.\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, helper + anchor, 1)
            break
    else:
        raise SystemExit("Could not find helper insertion point for github-plan.")

render_fn = """/// Renders or writes local GitHub workflow evidence.
fn render_github_plan(
    dry_run: bool,
    yes: bool,
    output_format: OutputFormat,
) -> Result<String, String> {
    let root = std::env::current_dir().map_err(|error| error.to_string())?;

    if dry_run {
        let plan = monad_core::build_github_workflow_plan(&root);
        return match output_format {
            OutputFormat::Text => Ok(monad_core::render_github_workflow_plan(&plan)),
            OutputFormat::Json => Ok(monad_core::render_github_workflow_plan_json(&plan)),
        };
    }

    if yes {
        let result =
            monad_core::write_github_workflow_evidence(&root).map_err(|error| error.to_string())?;
        return Ok(monad_core::render_github_workflow_apply_result(&result));
    }

    Err("github-plan currently requires either --dry-run to preview or --yes to write generated GitHub workflow evidence".to_string())
}

"""
if "fn render_github_plan(" not in text:
    anchors = [
        "/// Renders or writes local web workbench evidence.\n",
        "/// Renders or writes local interactive workbench evidence.\n",
        "/// Renders or writes local agent sandbox evidence.\n",
        "/// Renders or writes local AI retrieval evidence.\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, render_fn + anchor, 1)
            break
    else:
        raise SystemExit("Could not find render function insertion point for github-plan.")

if "  github-plan --dry-run" not in text:
    anchors = [
        '        "  web-workbench-plan --yes                 Write generated web workbench evidence",\n',
        '        "  workbench-plan --yes                     Write generated workbench evidence",\n',
        '        "  sandbox-plan --yes                      Write generated sandbox evidence",\n',
        '        "  retrieval-plan --yes                     Write generated retrieval evidence",\n',
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(
                anchor,
                anchor
                + '        "  github-plan --dry-run                   Preview GitHub issue/PR workflow plan",\n'
                + '        "  github-plan --dry-run --format=json     Preview GitHub workflow plan as JSON",\n'
                + '        "  github-plan --yes                       Write generated GitHub workflow evidence",\n',
                1,
            )
            break

if "monad pr-plan --dry-run" not in text:
    anchors = [
        '        "  monad web-ui --dry-run",\n',
        '        "  monad tui --dry-run",\n',
        '        "  monad sandbox-verify --dry-run",\n',
        '        "  monad vector-memory --dry-run",\n',
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(
                anchor,
                anchor
                + '        "  monad github-plan --dry-run",\n'
                + '        "  monad github-plan --dry-run --format=json",\n'
                + '        "  monad github-workflow --dry-run",\n'
                + '        "  monad pr-plan --dry-run",\n',
                1,
            )
            break

if "github-plan writes generated evidence only" not in text:
    anchors = [
        '        "  web-workbench-plan writes generated evidence only and does not start an HTTP server.",\n',
        '        "  workbench-plan writes generated evidence only and does not start a TUI event loop.",\n',
        '        "  sandbox-plan writes generated evidence only and does not execute agent actions.",\n',
        '        "  retrieval-plan writes generated evidence only and does not call model providers.",\n',
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(
                anchor,
                anchor
                + '        "  github-plan writes generated evidence only and does not call GitHub APIs.",\n',
                1,
            )
            break

test_block = """    #[test]
    fn github_plan_dry_run_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "github-plan", "--dry-run"]),
            Ok(CliCommand::GithubPlan {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            })
        );

        assert_eq!(
            parse_arguments(&["monad", "github-plan", "--dry-run", "--format=json"]),
            Ok(CliCommand::GithubPlan {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Json,
            })
        );
    }

    #[test]
    fn github_plan_aliases_parse() {
        assert_eq!(
            parse_arguments(&["monad", "github-workflow", "--dry-run"]),
            Ok(CliCommand::GithubPlan {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            })
        );

        assert_eq!(
            parse_arguments(&["monad", "pr-plan", "--dry-run"]),
            Ok(CliCommand::GithubPlan {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            })
        );
    }

    #[test]
    fn github_plan_yes_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "github-plan", "--yes"]),
            Ok(CliCommand::GithubPlan {
                dry_run: false,
                yes: true,
                output_format: OutputFormat::Text,
            })
        );
    }

    #[test]
    fn github_plan_requires_mode() {
        let result = parse_arguments(&["monad", "github-plan"]);
        assert!(
            matches!(
                result,
                Err(ref error)
                    if error.contains("github-plan currently requires either --dry-run")
            ),
            "expected github-plan to require --dry-run or --yes; got {result:?}"
        );
    }

"""
if "fn github_plan_dry_run_command_parses" not in text:
    anchors = [
        "    #[test]\n    fn web_workbench_plan_dry_run_command_parses() {\n",
        "    #[test]\n    fn workbench_plan_dry_run_command_parses() {\n",
        "    #[test]\n    fn sandbox_plan_dry_run_command_parses() {\n",
        "    #[test]\n    fn retrieval_plan_dry_run_command_parses() {\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, test_block + anchor, 1)
            break
    else:
        raise SystemExit("Could not find CLI test insertion point for github-plan.")

main.write_text(text)
PY

cargo fmt

echo "Applied E36 GitHub integration and PR workflow foundation."
echo "Backups written under: $BACKUP_DIR"
echo
echo "Run verification:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-github-workflow.sh"
echo "  tools/scripts/verify-e36.sh"
