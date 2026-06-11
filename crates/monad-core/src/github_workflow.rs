//! GitHub Integration and PR Workflow Foundation.
//!
//! E36 defines a local, supervised GitHub workflow planning foundation. It
//! models issue export/sync, branch and PR planning, review-pack generation,
//! and issue closeout evidence without calling GitHub, creating branches,
//! opening PRs, closing issues, accessing the network, or mutating remote state.

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{GatedWriteRequest, GatedWriteResult, gated_generated_write};

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
        issue_exports.sort_by_key(GithubIssueExportRecord::issue_number);
        closeout_evidence.sort_by_key(GithubIssueCloseoutEvidence::issue_number);

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
            vec![
                "boundary documented".to_string(),
                "tests passed".to_string(),
            ],
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
        format!(
            "  remote_api_disabled: {}",
            plan.boundary().remote_api_disabled()
        ),
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
        lines.push(format!(
            "- closure_disabled: {}",
            evidence.closure_disabled()
        ));
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
                lines.push(format!(
                    "  - [{}] {}",
                    write_result.as_str(),
                    path.display()
                ));
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

        assert!(
            plan.issue_exports()
                .iter()
                .any(|record| record.issue_number() == 176)
        );
        assert!(
            plan.issue_exports()
                .iter()
                .all(|record| record.decision() == GithubWorkflowBoundaryDecision::LocalPlanOnly)
        );
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

        assert!(
            plan.review_pack()
                .verification_commands()
                .contains(&"cargo test".to_string())
        );
        assert!(plan.review_pack().generated_only());
    }

    #[test]
    fn issue_closeout_is_disabled() {
        let plan = build_github_workflow_plan(".");

        assert!(
            plan.closeout_evidence()
                .iter()
                .all(GithubIssueCloseoutEvidence::closure_disabled)
        );
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
