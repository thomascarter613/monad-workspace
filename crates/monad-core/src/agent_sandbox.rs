//! Agent workflow sandbox foundation.
//!
//! E33 defines a local, review-only sandbox boundary for agent-generated draft
//! operations. It plans isolated draft operations, verification command paths,
//! and promotion approval records. It does not execute agent actions, apply
//! patches, run verification commands, or promote sandbox changes.

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{GatedWriteRequest, GatedWriteResult, gated_generated_write};

/// Agent action allowedness inside the sandbox foundation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentSandboxActionDecision {
    AllowedDraftOnly,
    ReviewRequired,
    Blocked,
}

impl AgentSandboxActionDecision {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowedDraftOnly => "allowed-draft-only",
            Self::ReviewRequired => "review-required",
            Self::Blocked => "blocked",
        }
    }
}

/// Promotion state for a sandbox output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AgentSandboxPromotionState {
    NotRequested,
    AwaitingApproval,
    Blocked,
}

impl AgentSandboxPromotionState {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequested => "not-requested",
            Self::AwaitingApproval => "awaiting-approval",
            Self::Blocked => "blocked",
        }
    }
}

/// Local sandbox boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentSandboxBoundary {
    sandbox_root: PathBuf,
    drafts_root: PathBuf,
    reports_root: PathBuf,
    source_mutation_disabled: bool,
    command_execution_disabled: bool,
    promotion_requires_approval: bool,
    rules: Vec<String>,
}

impl AgentSandboxBoundary {
    #[must_use]
    pub fn default_boundary() -> Self {
        Self {
            sandbox_root: PathBuf::from(".monad/sandboxes/agent-workflow"),
            drafts_root: PathBuf::from(".monad/sandboxes/agent-workflow/drafts"),
            reports_root: PathBuf::from(".monad/reports"),
            source_mutation_disabled: true,
            command_execution_disabled: true,
            promotion_requires_approval: true,
            rules: vec![
                "Agent sandbox planning is local-only.".to_string(),
                "Draft operations are planned but not applied.".to_string(),
                "Verification commands are listed but not executed.".to_string(),
                "Promotion requires explicit human approval.".to_string(),
            ],
        }
    }

    #[must_use]
    pub fn sandbox_root(&self) -> &Path {
        &self.sandbox_root
    }

    #[must_use]
    pub fn drafts_root(&self) -> &Path {
        &self.drafts_root
    }

    #[must_use]
    pub fn reports_root(&self) -> &Path {
        &self.reports_root
    }

    #[must_use]
    pub const fn source_mutation_disabled(&self) -> bool {
        self.source_mutation_disabled
    }

    #[must_use]
    pub const fn command_execution_disabled(&self) -> bool {
        self.command_execution_disabled
    }

    #[must_use]
    pub const fn promotion_requires_approval(&self) -> bool {
        self.promotion_requires_approval
    }

    #[must_use]
    pub fn rules(&self) -> &[String] {
        &self.rules
    }
}

/// Sandbox workspace model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentSandboxWorkspace {
    id: String,
    root: PathBuf,
    draft_operations_path: PathBuf,
    verification_plan_path: PathBuf,
    promotion_record_path: PathBuf,
}

impl AgentSandboxWorkspace {
    #[must_use]
    pub fn new(boundary: &AgentSandboxBoundary) -> Self {
        Self {
            id: "agent-workflow-default".to_string(),
            root: boundary.sandbox_root().to_path_buf(),
            draft_operations_path: boundary.drafts_root().join("draft-operations.json"),
            verification_plan_path: boundary.sandbox_root().join("verification-plan.json"),
            promotion_record_path: boundary.sandbox_root().join("promotion-approval.json"),
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[must_use]
    pub fn draft_operations_path(&self) -> &Path {
        &self.draft_operations_path
    }

    #[must_use]
    pub fn verification_plan_path(&self) -> &Path {
        &self.verification_plan_path
    }

    #[must_use]
    pub fn promotion_record_path(&self) -> &Path {
        &self.promotion_record_path
    }
}

/// Isolated draft operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentDraftOperation {
    id: String,
    target_path: PathBuf,
    description: String,
    decision: AgentSandboxActionDecision,
    reason: String,
}

impl AgentDraftOperation {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        target_path: impl Into<PathBuf>,
        description: impl Into<String>,
        decision: AgentSandboxActionDecision,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            target_path: target_path.into(),
            description: description.into(),
            decision,
            reason: reason.into(),
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn target_path(&self) -> &Path {
        &self.target_path
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub const fn decision(&self) -> AgentSandboxActionDecision {
        self.decision
    }

    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// Verification command path planned for review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentSandboxVerificationCommand {
    id: String,
    command: String,
    execution_disabled: bool,
    reason: String,
}

impl AgentSandboxVerificationCommand {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        command: impl Into<String>,
        execution_disabled: bool,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            command: command.into(),
            execution_disabled,
            reason: reason.into(),
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    #[must_use]
    pub const fn execution_disabled(&self) -> bool {
        self.execution_disabled
    }

    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// Promotion/approval record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentSandboxPromotionApproval {
    state: AgentSandboxPromotionState,
    required_approvals: Vec<String>,
    blocked_reasons: Vec<String>,
}

impl AgentSandboxPromotionApproval {
    #[must_use]
    pub fn new(
        state: AgentSandboxPromotionState,
        mut required_approvals: Vec<String>,
        mut blocked_reasons: Vec<String>,
    ) -> Self {
        required_approvals.sort();
        required_approvals.dedup();
        blocked_reasons.sort();
        blocked_reasons.dedup();

        Self {
            state,
            required_approvals,
            blocked_reasons,
        }
    }

    #[must_use]
    pub const fn state(&self) -> AgentSandboxPromotionState {
        self.state
    }

    #[must_use]
    pub fn required_approvals(&self) -> &[String] {
        &self.required_approvals
    }

    #[must_use]
    pub fn blocked_reasons(&self) -> &[String] {
        &self.blocked_reasons
    }
}

/// Full sandbox plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentSandboxPlan {
    command: String,
    boundary: AgentSandboxBoundary,
    workspace: AgentSandboxWorkspace,
    draft_operations: Vec<AgentDraftOperation>,
    verification_commands: Vec<AgentSandboxVerificationCommand>,
    promotion_approval: AgentSandboxPromotionApproval,
    evidence_paths: Vec<PathBuf>,
    safety_notes: Vec<String>,
}

impl AgentSandboxPlan {
    #[must_use]
    pub fn new(
        boundary: AgentSandboxBoundary,
        workspace: AgentSandboxWorkspace,
        mut draft_operations: Vec<AgentDraftOperation>,
        mut verification_commands: Vec<AgentSandboxVerificationCommand>,
        promotion_approval: AgentSandboxPromotionApproval,
    ) -> Self {
        draft_operations.sort_by(|left, right| left.id().cmp(right.id()));
        verification_commands.sort_by(|left, right| left.id().cmp(right.id()));

        Self {
            command: "sandbox-plan".to_string(),
            boundary,
            workspace,
            draft_operations,
            verification_commands,
            promotion_approval,
            evidence_paths: vec![
                PathBuf::from(".monad/reports/agent-sandbox-plan.md"),
                PathBuf::from(".monad/reports/agent-sandbox-plan.json"),
            ],
            safety_notes: vec![
                "No agent actions are executed by Monad.".to_string(),
                "No source files are mutated by Monad.".to_string(),
                "No patches are applied by Monad.".to_string(),
                "No verification commands are executed by Monad.".to_string(),
                "No sandbox promotion is performed by Monad.".to_string(),
            ],
        }
    }

    #[must_use]
    pub const fn boundary(&self) -> &AgentSandboxBoundary {
        &self.boundary
    }

    #[must_use]
    pub const fn workspace(&self) -> &AgentSandboxWorkspace {
        &self.workspace
    }

    #[must_use]
    pub fn draft_operations(&self) -> &[AgentDraftOperation] {
        &self.draft_operations
    }

    #[must_use]
    pub fn verification_commands(&self) -> &[AgentSandboxVerificationCommand] {
        &self.verification_commands
    }

    #[must_use]
    pub const fn promotion_approval(&self) -> &AgentSandboxPromotionApproval {
        &self.promotion_approval
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

/// Apply result for generated sandbox evidence writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSandboxApplyResult {
    write_results: Vec<GatedWriteResult>,
}

impl AgentSandboxApplyResult {
    #[must_use]
    pub fn new(write_results: Vec<GatedWriteResult>) -> Self {
        Self { write_results }
    }

    #[must_use]
    pub fn write_results(&self) -> &[GatedWriteResult] {
        &self.write_results
    }
}

/// Builds the E33 sandbox plan.
#[must_use]
pub fn build_agent_sandbox_plan(_root: impl AsRef<Path>) -> AgentSandboxPlan {
    let boundary = AgentSandboxBoundary::default_boundary();
    let workspace = AgentSandboxWorkspace::new(&boundary);
    let draft_operations = vec![
        AgentDraftOperation::new(
            "draft:docs-note",
            workspace.draft_operations_path(),
            "Prepare a generated draft operation record for human review.",
            AgentSandboxActionDecision::AllowedDraftOnly,
            "draft operation is isolated under the sandbox workspace",
        ),
        AgentDraftOperation::new(
            "draft:source-mutation",
            "src/example.rs",
            "Attempted direct source mutation placeholder.",
            AgentSandboxActionDecision::Blocked,
            "direct source mutation is disabled in E33",
        ),
    ];
    let verification_commands = vec![
        AgentSandboxVerificationCommand::new(
            "verify:fmt",
            "cargo fmt --check",
            true,
            "verification commands are planned but not executed",
        ),
        AgentSandboxVerificationCommand::new(
            "verify:test",
            "cargo test",
            true,
            "verification commands are planned but not executed",
        ),
    ];
    let promotion_approval = AgentSandboxPromotionApproval::new(
        AgentSandboxPromotionState::AwaitingApproval,
        vec![
            "human-review".to_string(),
            "verification-evidence".to_string(),
        ],
        vec!["automatic promotion is disabled".to_string()],
    );

    AgentSandboxPlan::new(
        boundary,
        workspace,
        draft_operations,
        verification_commands,
        promotion_approval,
    )
}

/// Writes generated sandbox evidence.
pub fn write_agent_sandbox_evidence(
    root: impl AsRef<Path>,
) -> Result<AgentSandboxApplyResult, String> {
    let root = root.as_ref();
    let plan = build_agent_sandbox_plan(root);
    let requests = [
        GatedWriteRequest::new(
            ".monad/reports/agent-sandbox-plan.md",
            render_agent_sandbox_plan(&plan),
            true,
        ),
        GatedWriteRequest::new(
            ".monad/reports/agent-sandbox-plan.json",
            render_agent_sandbox_plan_json(&plan),
            true,
        ),
    ];

    let write_results = requests
        .iter()
        .map(|request| gated_generated_write(root, request))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(AgentSandboxApplyResult::new(write_results))
}

/// Renders a text sandbox plan.
#[must_use]
pub fn render_agent_sandbox_plan(plan: &AgentSandboxPlan) -> String {
    let mut lines = vec![
        "Monad agent workflow sandbox plan".to_string(),
        String::new(),
        "Boundary:".to_string(),
        format!(
            "  sandbox_root: {}",
            plan.boundary().sandbox_root().display()
        ),
        format!("  drafts_root: {}", plan.boundary().drafts_root().display()),
        format!(
            "  source_mutation_disabled: {}",
            plan.boundary().source_mutation_disabled()
        ),
        format!(
            "  command_execution_disabled: {}",
            plan.boundary().command_execution_disabled()
        ),
        format!(
            "  promotion_requires_approval: {}",
            plan.boundary().promotion_requires_approval()
        ),
        String::new(),
        "Rules:".to_string(),
    ];

    for rule in plan.boundary().rules() {
        lines.push(format!("  - {rule}"));
    }

    lines.push(String::new());
    lines.push("Workspace:".to_string());
    lines.push(format!("  id: {}", plan.workspace().id()));
    lines.push(format!("  root: {}", plan.workspace().root().display()));
    lines.push(format!(
        "  draft_operations_path: {}",
        plan.workspace().draft_operations_path().display()
    ));
    lines.push(format!(
        "  verification_plan_path: {}",
        plan.workspace().verification_plan_path().display()
    ));
    lines.push(format!(
        "  promotion_record_path: {}",
        plan.workspace().promotion_record_path().display()
    ));

    lines.push(String::new());
    lines.push("Draft operations:".to_string());
    for operation in plan.draft_operations() {
        lines.push(format!(
            "  - {} decision={} target={} reason={}",
            operation.id(),
            operation.decision().as_str(),
            operation.target_path().display(),
            operation.reason()
        ));
    }

    lines.push(String::new());
    lines.push("Verification command path:".to_string());
    for command in plan.verification_commands() {
        lines.push(format!(
            "  - {} execution_disabled={} command={} reason={}",
            command.id(),
            command.execution_disabled(),
            command.command(),
            command.reason()
        ));
    }

    lines.push(String::new());
    lines.push(format!(
        "Promotion state: {}",
        plan.promotion_approval().state().as_str()
    ));
    for approval in plan.promotion_approval().required_approvals() {
        lines.push(format!("  - required: {approval}"));
    }
    for reason in plan.promotion_approval().blocked_reasons() {
        lines.push(format!("  - blocked: {reason}"));
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

/// Renders a JSON sandbox plan.
#[must_use]
pub fn render_agent_sandbox_plan_json(plan: &AgentSandboxPlan) -> String {
    serde_json::to_string_pretty(plan).unwrap_or_else(|_| {
        "{\n  \"command\": \"sandbox-plan\",\n  \"error\": \"agent sandbox plan serialization failed\"\n}".to_string()
    })
}

/// Renders generated evidence write results.
#[must_use]
pub fn render_agent_sandbox_apply_result(result: &AgentSandboxApplyResult) -> String {
    let mut lines = vec![
        "Monad agent sandbox evidence write result".to_string(),
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
    lines.push("No agent actions were executed.".to_string());
    lines.push("No patches were applied.".to_string());
    lines.push("No sandbox promotion was performed.".to_string());

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundary_disables_mutation_and_execution() {
        let boundary = AgentSandboxBoundary::default_boundary();

        assert!(boundary.source_mutation_disabled());
        assert!(boundary.command_execution_disabled());
        assert!(boundary.promotion_requires_approval());
    }

    #[test]
    fn plan_contains_blocked_source_mutation() {
        let plan = build_agent_sandbox_plan(".");

        assert!(plan.draft_operations().iter().any(|operation| {
            operation.id() == "draft:source-mutation"
                && operation.decision() == AgentSandboxActionDecision::Blocked
        }));
    }

    #[test]
    fn verification_commands_are_not_executed() {
        let plan = build_agent_sandbox_plan(".");

        assert!(
            plan.verification_commands()
                .iter()
                .all(AgentSandboxVerificationCommand::execution_disabled)
        );
    }

    #[test]
    fn promotion_requires_approval() {
        let plan = build_agent_sandbox_plan(".");

        assert_eq!(
            plan.promotion_approval().state(),
            AgentSandboxPromotionState::AwaitingApproval
        );
        assert!(
            plan.promotion_approval()
                .required_approvals()
                .contains(&"human-review".to_string())
        );
    }

    #[test]
    fn text_render_mentions_safety_boundaries() {
        let plan = build_agent_sandbox_plan(".");
        let text = render_agent_sandbox_plan(&plan);

        assert!(text.contains("Monad agent workflow sandbox plan"));
        assert!(text.contains("No agent actions are executed by Monad"));
        assert!(text.contains("Verification command path"));
    }

    #[test]
    fn json_render_contains_sandbox_plan_command() {
        let plan = build_agent_sandbox_plan(".");
        let json = render_agent_sandbox_plan_json(&plan);

        assert!(json.contains("\"command\": \"sandbox-plan\""));
        assert!(json.contains("agent-workflow-default"));
    }
}
