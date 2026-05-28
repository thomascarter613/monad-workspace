//! Draft sandbox workflow model.
//!
//! A draft sandbox is a reviewable proposal for future repository changes. It
//! connects supervised agent planning to the safe file operation model without
//! writing files, mutating Git state, creating branches, creating worktrees, or
//! applying changes.

use std::path::{Path, PathBuf};

use crate::{FileOperationKind, FileOperationPlan, PlannedFileOperation};

/// Stable identifier for a draft.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DraftId(String);

impl DraftId {
    /// Creates a draft ID.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the draft ID as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Draft lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DraftState {
    /// Draft has been proposed but not reviewed.
    Proposed,

    /// Draft has been reviewed.
    Reviewed,

    /// Draft has been approved for a specific next action.
    Approved,

    /// Draft has been rejected.
    Rejected,

    /// Draft has been replaced by another draft.
    Superseded,

    /// Draft has been applied through a future safe apply workflow.
    Applied,
}

impl DraftState {
    /// Returns a stable draft-state label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::Reviewed => "reviewed",
            Self::Approved => "approved",
            Self::Rejected => "rejected",
            Self::Superseded => "superseded",
            Self::Applied => "applied",
        }
    }

    /// Returns true when this state is terminal for planning purposes.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Rejected | Self::Superseded | Self::Applied)
    }
}

/// Draft sandbox storage/isolation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DraftSandboxKind {
    /// Draft exists only as in-memory data.
    InMemory,

    /// Future draft may be stored under `.monad/drafts`.
    FileBacked,

    /// Future draft may be isolated on a branch.
    BranchBacked,

    /// Future draft may be isolated in a Git worktree.
    WorktreeBacked,
}

impl DraftSandboxKind {
    /// Returns a stable sandbox-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InMemory => "in_memory",
            Self::FileBacked => "file_backed",
            Self::BranchBacked => "branch_backed",
            Self::WorktreeBacked => "worktree_backed",
        }
    }

    /// Returns true when this sandbox kind implies Git isolation.
    #[must_use]
    pub const fn uses_git_isolation(self) -> bool {
        matches!(self, Self::BranchBacked | Self::WorktreeBacked)
    }
}

/// Reviewable draft representation of a planned file operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DraftFileOperation {
    operation_kind: FileOperationKind,
    target_path: PathBuf,
    explanation: String,
    requires_approval: bool,
}

impl DraftFileOperation {
    /// Creates a draft file operation.
    #[must_use]
    pub fn new(
        operation_kind: FileOperationKind,
        target_path: impl Into<PathBuf>,
        explanation: impl Into<String>,
        requires_approval: bool,
    ) -> Self {
        Self {
            operation_kind,
            target_path: target_path.into(),
            explanation: explanation.into(),
            requires_approval,
        }
    }

    /// Converts a planned file operation into a reviewable draft operation.
    #[must_use]
    pub fn from_planned_operation(operation: &PlannedFileOperation) -> Self {
        let operation_kind = operation.kind();

        Self::new(
            operation_kind,
            operation.target().path().to_path_buf(),
            operation.explanation().to_string(),
            draft_operation_requires_approval(operation_kind),
        )
    }

    /// Returns the operation kind.
    #[must_use]
    pub const fn operation_kind(&self) -> FileOperationKind {
        self.operation_kind
    }

    /// Returns the target path.
    #[must_use]
    pub fn target_path(&self) -> &Path {
        &self.target_path
    }

    /// Returns the target path for display.
    #[must_use]
    pub fn display_path(&self) -> String {
        self.target_path.display().to_string()
    }

    /// Returns the explanation.
    #[must_use]
    pub fn explanation(&self) -> &str {
        &self.explanation
    }

    /// Returns true when explicit approval is required.
    #[must_use]
    pub const fn requires_approval(&self) -> bool {
        self.requires_approval
    }
}

fn draft_operation_requires_approval(operation_kind: FileOperationKind) -> bool {
    operation_kind.writes_content()
        || operation_kind.is_destructive()
        || operation_kind.is_blocking()
}

/// Reviewable draft sandbox object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DraftSandbox {
    id: DraftId,
    title: String,
    summary: String,
    state: DraftState,
    sandbox_kind: DraftSandboxKind,
    operations: Vec<DraftFileOperation>,
    verification_notes: Vec<String>,
    risks: Vec<String>,
}

impl DraftSandbox {
    /// Creates a new proposed draft sandbox.
    #[must_use]
    pub fn new(
        id: DraftId,
        title: impl Into<String>,
        summary: impl Into<String>,
        sandbox_kind: DraftSandboxKind,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            summary: summary.into(),
            state: DraftState::Proposed,
            sandbox_kind,
            operations: Vec::new(),
            verification_notes: Vec::new(),
            risks: Vec::new(),
        }
    }

    /// Creates a draft sandbox from a safe file operation plan.
    #[must_use]
    pub fn from_file_operation_plan(
        id: DraftId,
        title: impl Into<String>,
        summary: impl Into<String>,
        sandbox_kind: DraftSandboxKind,
        plan: &FileOperationPlan,
    ) -> Self {
        let operations = plan
            .operations()
            .iter()
            .map(DraftFileOperation::from_planned_operation)
            .collect();

        let mut draft = Self::new(id, title, summary, sandbox_kind);
        draft.operations = operations;
        draft
    }

    /// Returns draft ID.
    #[must_use]
    pub const fn id(&self) -> &DraftId {
        &self.id
    }

    /// Returns draft title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns draft summary.
    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Returns draft state.
    #[must_use]
    pub const fn state(&self) -> DraftState {
        self.state
    }

    /// Returns sandbox kind.
    #[must_use]
    pub const fn sandbox_kind(&self) -> DraftSandboxKind {
        self.sandbox_kind
    }

    /// Returns draft file operations.
    #[must_use]
    pub fn operations(&self) -> &[DraftFileOperation] {
        &self.operations
    }

    /// Returns verification notes.
    #[must_use]
    pub fn verification_notes(&self) -> &[String] {
        &self.verification_notes
    }

    /// Returns risks.
    #[must_use]
    pub fn risks(&self) -> &[String] {
        &self.risks
    }

    /// Returns the number of draft operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns true when there are no draft operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns true when any draft operation requires explicit approval.
    #[must_use]
    pub fn requires_approval(&self) -> bool {
        self.operations()
            .iter()
            .any(DraftFileOperation::requires_approval)
    }

    /// Adds a draft file operation.
    pub fn push_operation(&mut self, operation: DraftFileOperation) {
        self.operations.push(operation);
    }

    /// Adds a verification note.
    pub fn push_verification_note(&mut self, note: impl Into<String>) {
        self.verification_notes.push(note.into());
    }

    /// Adds a risk note.
    pub fn push_risk(&mut self, risk: impl Into<String>) {
        self.risks.push(risk.into());
    }

    /// Marks a draft reviewed.
    pub fn mark_reviewed(&mut self) {
        if !self.state.is_terminal() {
            self.state = DraftState::Reviewed;
        }
    }

    /// Marks a draft approved.
    pub fn mark_approved(&mut self) {
        if !self.state.is_terminal() {
            self.state = DraftState::Approved;
        }
    }

    /// Marks a draft rejected.
    pub fn mark_rejected(&mut self) {
        self.state = DraftState::Rejected;
    }

    /// Marks a draft superseded.
    pub fn mark_superseded(&mut self) {
        self.state = DraftState::Superseded;
    }
}

/// Renders a draft sandbox as reviewable text.
#[must_use]
pub fn render_draft_sandbox(draft: &DraftSandbox) -> String {
    let mut lines = Vec::new();

    lines.push("Monad draft sandbox".to_string());
    lines.push(String::new());
    lines.push(format!("Draft ID: {}", draft.id().as_str()));
    lines.push(format!("Title: {}", draft.title()));
    lines.push(format!("State: {}", draft.state().as_str()));
    lines.push(format!("Sandbox kind: {}", draft.sandbox_kind().as_str()));
    lines.push(format!("Summary: {}", draft.summary()));
    lines.push(String::new());

    lines.push("Draft operations:".to_string());
    if draft.operations().is_empty() {
        lines.push("- No operations are currently proposed.".to_string());
    } else {
        for operation in draft.operations() {
            lines.push(format!(
                "- [{}] {}: {}{}",
                operation.operation_kind().as_str().to_uppercase(),
                operation.display_path(),
                operation.explanation(),
                if operation.requires_approval() {
                    " [approval required]"
                } else {
                    ""
                }
            ));
        }
    }

    lines.push(String::new());
    lines.push("Verification notes:".to_string());
    if draft.verification_notes().is_empty() {
        lines.push("- Verification has not been run for this draft.".to_string());
    } else {
        for note in draft.verification_notes() {
            lines.push(format!("- {note}"));
        }
    }

    lines.push(String::new());
    lines.push("Risks:".to_string());
    if draft.risks().is_empty() {
        lines.push("- No draft-specific risks recorded yet.".to_string());
    } else {
        for risk in draft.risks() {
            lines.push(format!("- {risk}"));
        }
    }

    lines.push(String::new());
    lines.push(
        "Status: draft only; no files were written and no Git state was changed.".to_string(),
    );

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::{FileOperationKind, FileOperationPlan, PlannedFileOperation};

    use super::*;

    #[test]
    fn draft_state_labels_are_stable() {
        assert_eq!(DraftState::Proposed.as_str(), "proposed");
        assert_eq!(DraftState::Reviewed.as_str(), "reviewed");
        assert_eq!(DraftState::Approved.as_str(), "approved");
        assert_eq!(DraftState::Rejected.as_str(), "rejected");
        assert_eq!(DraftState::Superseded.as_str(), "superseded");
        assert_eq!(DraftState::Applied.as_str(), "applied");
    }

    #[test]
    fn sandbox_kind_identifies_git_isolation() {
        assert!(!DraftSandboxKind::InMemory.uses_git_isolation());
        assert!(!DraftSandboxKind::FileBacked.uses_git_isolation());
        assert!(DraftSandboxKind::BranchBacked.uses_git_isolation());
        assert!(DraftSandboxKind::WorktreeBacked.uses_git_isolation());
    }

    #[test]
    fn draft_operation_can_be_created_from_planned_operation() {
        let planned =
            PlannedFileOperation::create("docs/example.md", "create example documentation");

        let draft_operation = DraftFileOperation::from_planned_operation(&planned);

        assert_eq!(draft_operation.operation_kind(), FileOperationKind::Create);
        assert_eq!(draft_operation.display_path(), "docs/example.md");
        assert_eq!(
            draft_operation.explanation(),
            "create example documentation"
        );
        assert!(draft_operation.requires_approval());
    }

    #[test]
    fn draft_can_be_created_from_file_operation_plan() {
        let plan = FileOperationPlan::from_operations(vec![
            PlannedFileOperation::create("docs/a.md", "create A"),
            PlannedFileOperation::update("docs/b.md", "update B"),
        ]);

        let draft = DraftSandbox::from_file_operation_plan(
            DraftId::new("draft-1"),
            "Example Draft",
            "Draft generated from a safe file operation plan.",
            DraftSandboxKind::InMemory,
            &plan,
        );

        assert_eq!(draft.id().as_str(), "draft-1");
        assert_eq!(draft.title(), "Example Draft");
        assert_eq!(draft.state(), DraftState::Proposed);
        assert_eq!(draft.sandbox_kind(), DraftSandboxKind::InMemory);
        assert_eq!(draft.len(), 2);
        assert!(draft.requires_approval());
    }

    #[test]
    fn terminal_draft_state_cannot_be_reopened_by_review_or_approval() {
        let mut draft = DraftSandbox::new(
            DraftId::new("draft-terminal"),
            "Terminal Draft",
            "Rejected drafts stay rejected.",
            DraftSandboxKind::InMemory,
        );

        draft.mark_rejected();
        draft.mark_reviewed();
        draft.mark_approved();

        assert_eq!(draft.state(), DraftState::Rejected);
    }

    #[test]
    fn rendered_draft_is_reviewable_and_non_mutating() {
        let plan = FileOperationPlan::from_operations(vec![PlannedFileOperation::create(
            "docs/example.md",
            "create example documentation",
        )]);

        let mut draft = DraftSandbox::from_file_operation_plan(
            DraftId::new("draft-render"),
            "Rendered Draft",
            "Draft used to test rendering.",
            DraftSandboxKind::InMemory,
            &plan,
        );
        draft.push_verification_note("Run cargo test after approved apply.");
        draft.push_risk("Draft has not been applied or verified.");

        let rendered = render_draft_sandbox(&draft);

        assert!(rendered.contains("Monad draft sandbox"));
        assert!(rendered.contains("Draft ID: draft-render"));
        assert!(rendered.contains("[CREATE] docs/example.md"));
        assert!(rendered.contains("Run cargo test"));
        assert!(rendered.contains("no files were written"));
        assert!(rendered.contains("no Git state was changed"));
    }
}
