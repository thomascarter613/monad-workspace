//! Interactive Workbench / TUI foundation.
//!
//! E34 defines the deterministic planning model for Monad's future interactive
//! workbench. This is not a real TUI runtime yet: it does not enter raw terminal
//! mode, start an event loop, read keyboard input, call GitHub, access the
//! network, or mutate source files.

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{GatedWriteRequest, GatedWriteResult, gated_generated_write};

/// Workbench route/screen kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkbenchScreenKind {
    Dashboard,
    Issues,
    WorkPackets,
    Plans,
    Reports,
    Context,
    Approvals,
    Help,
}

impl WorkbenchScreenKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dashboard => "dashboard",
            Self::Issues => "issues",
            Self::WorkPackets => "work-packets",
            Self::Plans => "plans",
            Self::Reports => "reports",
            Self::Context => "context",
            Self::Approvals => "approvals",
            Self::Help => "help",
        }
    }
}

/// Workbench navigation item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkbenchNavigationItem {
    id: String,
    label: String,
    screen: WorkbenchScreenKind,
    shortcut: String,
    description: String,
}

impl WorkbenchNavigationItem {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        screen: WorkbenchScreenKind,
        shortcut: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            screen,
            shortcut: shortcut.into(),
            description: description.into(),
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub const fn screen(&self) -> WorkbenchScreenKind {
        self.screen
    }

    #[must_use]
    pub fn shortcut(&self) -> &str {
        &self.shortcut
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Workbench navigation model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkbenchNavigationModel {
    default_screen: WorkbenchScreenKind,
    items: Vec<WorkbenchNavigationItem>,
}

impl WorkbenchNavigationModel {
    #[must_use]
    pub fn new(
        default_screen: WorkbenchScreenKind,
        mut items: Vec<WorkbenchNavigationItem>,
    ) -> Self {
        items.sort_by(|left, right| left.id().cmp(right.id()));
        items.dedup_by(|left, right| left.id() == right.id());

        Self {
            default_screen,
            items,
        }
    }

    #[must_use]
    pub const fn default_screen(&self) -> WorkbenchScreenKind {
        self.default_screen
    }

    #[must_use]
    pub fn items(&self) -> &[WorkbenchNavigationItem] {
        &self.items
    }
}

/// TUI shell proof-of-concept frame.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkbenchShellFrame {
    title: String,
    width: u16,
    height: u16,
    panels: Vec<String>,
    interaction_disabled: bool,
}

impl WorkbenchShellFrame {
    #[must_use]
    pub fn new(title: impl Into<String>, width: u16, height: u16, mut panels: Vec<String>) -> Self {
        panels.sort();
        panels.dedup();

        Self {
            title: title.into(),
            width,
            height,
            panels,
            interaction_disabled: true,
        }
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> u16 {
        self.height
    }

    #[must_use]
    pub fn panels(&self) -> &[String] {
        &self.panels
    }

    #[must_use]
    pub const fn interaction_disabled(&self) -> bool {
        self.interaction_disabled
    }
}

/// Issue/work-packet view foundation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkbenchIssueView {
    title: String,
    parent_epic: String,
    work_packets: Vec<String>,
    offline_only: bool,
}

impl WorkbenchIssueView {
    #[must_use]
    pub fn new(
        title: impl Into<String>,
        parent_epic: impl Into<String>,
        mut work_packets: Vec<String>,
    ) -> Self {
        work_packets.sort();

        Self {
            title: title.into(),
            parent_epic: parent_epic.into(),
            work_packets,
            offline_only: true,
        }
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub fn parent_epic(&self) -> &str {
        &self.parent_epic
    }

    #[must_use]
    pub fn work_packets(&self) -> &[String] {
        &self.work_packets
    }

    #[must_use]
    pub const fn offline_only(&self) -> bool {
        self.offline_only
    }
}

/// Plan/report/context viewer foundation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkbenchViewerPane {
    id: String,
    label: String,
    source_path: PathBuf,
    read_only: bool,
}

impl WorkbenchViewerPane {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        source_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            source_path: source_path.into(),
            read_only: true,
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub fn source_path(&self) -> &Path {
        &self.source_path
    }

    #[must_use]
    pub const fn read_only(&self) -> bool {
        self.read_only
    }
}

/// Approval review screen foundation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkbenchApprovalReview {
    id: String,
    title: String,
    approval_required: bool,
    mutation_disabled: bool,
    notes: Vec<String>,
}

impl WorkbenchApprovalReview {
    #[must_use]
    pub fn new(id: impl Into<String>, title: impl Into<String>, mut notes: Vec<String>) -> Self {
        notes.sort();
        notes.dedup();

        Self {
            id: id.into(),
            title: title.into(),
            approval_required: true,
            mutation_disabled: true,
            notes,
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub const fn approval_required(&self) -> bool {
        self.approval_required
    }

    #[must_use]
    pub const fn mutation_disabled(&self) -> bool {
        self.mutation_disabled
    }

    #[must_use]
    pub fn notes(&self) -> &[String] {
        &self.notes
    }
}

/// Full interactive workbench plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InteractiveWorkbenchPlan {
    command: String,
    navigation: WorkbenchNavigationModel,
    shell_frame: WorkbenchShellFrame,
    issue_view: WorkbenchIssueView,
    viewer_panes: Vec<WorkbenchViewerPane>,
    approval_review: WorkbenchApprovalReview,
    evidence_paths: Vec<PathBuf>,
    safety_notes: Vec<String>,
}

impl InteractiveWorkbenchPlan {
    #[must_use]
    pub fn new(
        navigation: WorkbenchNavigationModel,
        shell_frame: WorkbenchShellFrame,
        issue_view: WorkbenchIssueView,
        mut viewer_panes: Vec<WorkbenchViewerPane>,
        approval_review: WorkbenchApprovalReview,
    ) -> Self {
        viewer_panes.sort_by(|left, right| left.id().cmp(right.id()));

        Self {
            command: "workbench-plan".to_string(),
            navigation,
            shell_frame,
            issue_view,
            viewer_panes,
            approval_review,
            evidence_paths: vec![
                PathBuf::from(".monad/reports/interactive-workbench-plan.md"),
                PathBuf::from(".monad/reports/interactive-workbench-plan.json"),
            ],
            safety_notes: vec![
                "No interactive terminal event loop is started by Monad.".to_string(),
                "No raw terminal mode is entered by Monad.".to_string(),
                "No network calls are performed by Monad.".to_string(),
                "No source files are mutated by the workbench plan.".to_string(),
                "Generated workbench evidence is written only when --yes is used.".to_string(),
            ],
        }
    }

    #[must_use]
    pub const fn navigation(&self) -> &WorkbenchNavigationModel {
        &self.navigation
    }

    #[must_use]
    pub const fn shell_frame(&self) -> &WorkbenchShellFrame {
        &self.shell_frame
    }

    #[must_use]
    pub const fn issue_view(&self) -> &WorkbenchIssueView {
        &self.issue_view
    }

    #[must_use]
    pub fn viewer_panes(&self) -> &[WorkbenchViewerPane] {
        &self.viewer_panes
    }

    #[must_use]
    pub const fn approval_review(&self) -> &WorkbenchApprovalReview {
        &self.approval_review
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

/// Apply result for generated workbench evidence writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteractiveWorkbenchApplyResult {
    write_results: Vec<GatedWriteResult>,
}

impl InteractiveWorkbenchApplyResult {
    #[must_use]
    pub fn new(write_results: Vec<GatedWriteResult>) -> Self {
        Self { write_results }
    }

    #[must_use]
    pub fn write_results(&self) -> &[GatedWriteResult] {
        &self.write_results
    }
}

/// Builds the E34 interactive workbench foundation plan.
#[must_use]
pub fn build_interactive_workbench_plan(_root: impl AsRef<Path>) -> InteractiveWorkbenchPlan {
    let navigation = WorkbenchNavigationModel::new(
        WorkbenchScreenKind::Dashboard,
        vec![
            WorkbenchNavigationItem::new(
                "nav:dashboard",
                "Dashboard",
                WorkbenchScreenKind::Dashboard,
                "d",
                "Show project status and next action.",
            ),
            WorkbenchNavigationItem::new(
                "nav:issues",
                "Issues",
                WorkbenchScreenKind::Issues,
                "i",
                "Show issue and epic/work-packet summary.",
            ),
            WorkbenchNavigationItem::new(
                "nav:plans",
                "Plans",
                WorkbenchScreenKind::Plans,
                "p",
                "Show generated plans and roadmaps.",
            ),
            WorkbenchNavigationItem::new(
                "nav:reports",
                "Reports",
                WorkbenchScreenKind::Reports,
                "r",
                "Show local reports and verification evidence.",
            ),
            WorkbenchNavigationItem::new(
                "nav:approvals",
                "Approvals",
                WorkbenchScreenKind::Approvals,
                "a",
                "Review approval-required generated actions.",
            ),
        ],
    );

    let shell_frame = WorkbenchShellFrame::new(
        "Monad Interactive Workbench",
        100,
        32,
        vec![
            "navigation".to_string(),
            "detail".to_string(),
            "status".to_string(),
            "approval-review".to_string(),
        ],
    );

    let issue_view = WorkbenchIssueView::new(
        "E34 — Interactive Workbench / TUI Foundation",
        "#174",
        vec![
            "WP-E34-001 — Define TUI navigation model".to_string(),
            "WP-E34-002 — Add TUI shell proof of concept".to_string(),
            "WP-E34-003 — Add issue/work-packet view".to_string(),
            "WP-E34-004 — Add plan/report/context viewer".to_string(),
            "WP-E34-005 — Add approval review screen foundation".to_string(),
            "WP-E34-006 — Add TUI smoke tests".to_string(),
        ],
    );

    let viewer_panes = vec![
        WorkbenchViewerPane::new(
            "viewer:plan",
            "Plan Viewer",
            ".monad/reports/interactive-workbench-plan.md",
        ),
        WorkbenchViewerPane::new("viewer:report", "Report Viewer", ".monad/reports"),
        WorkbenchViewerPane::new("viewer:context", "Context Viewer", "docs"),
    ];

    let approval_review = WorkbenchApprovalReview::new(
        "approval:generated-action",
        "Generated Action Review",
        vec![
            "Approval screen is read-only in E34.".to_string(),
            "No mutations are applied from the workbench foundation.".to_string(),
            "Future interactive approvals must produce explicit evidence.".to_string(),
        ],
    );

    InteractiveWorkbenchPlan::new(
        navigation,
        shell_frame,
        issue_view,
        viewer_panes,
        approval_review,
    )
}

/// Writes generated interactive workbench evidence.
pub fn write_interactive_workbench_evidence(
    root: impl AsRef<Path>,
) -> Result<InteractiveWorkbenchApplyResult, String> {
    let root = root.as_ref();
    let plan = build_interactive_workbench_plan(root);
    let requests = [
        GatedWriteRequest::new(
            ".monad/reports/interactive-workbench-plan.md",
            render_interactive_workbench_plan(&plan),
            true,
        ),
        GatedWriteRequest::new(
            ".monad/reports/interactive-workbench-plan.json",
            render_interactive_workbench_plan_json(&plan),
            true,
        ),
    ];

    let write_results = requests
        .iter()
        .map(|request| gated_generated_write(root, request))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(InteractiveWorkbenchApplyResult::new(write_results))
}

/// Renders a text workbench foundation plan.
#[must_use]
pub fn render_interactive_workbench_plan(plan: &InteractiveWorkbenchPlan) -> String {
    let mut lines = vec![
        "Monad interactive workbench / TUI foundation plan".to_string(),
        String::new(),
        "Navigation model:".to_string(),
        format!(
            "  default_screen: {}",
            plan.navigation().default_screen().as_str()
        ),
    ];

    for item in plan.navigation().items() {
        lines.push(format!(
            "  - {} label={} screen={} shortcut={} description={}",
            item.id(),
            item.label(),
            item.screen().as_str(),
            item.shortcut(),
            item.description()
        ));
    }

    lines.push(String::new());
    lines.push("TUI shell proof of concept:".to_string());
    lines.push(format!("  title: {}", plan.shell_frame().title()));
    lines.push(format!(
        "  size: {}x{}",
        plan.shell_frame().width(),
        plan.shell_frame().height()
    ));
    lines.push(format!(
        "  interaction_disabled: {}",
        plan.shell_frame().interaction_disabled()
    ));
    for panel in plan.shell_frame().panels() {
        lines.push(format!("  - panel: {panel}"));
    }

    lines.push(String::new());
    lines.push("Issue/work-packet view:".to_string());
    lines.push(format!("  title: {}", plan.issue_view().title()));
    lines.push(format!(
        "  parent_epic: {}",
        plan.issue_view().parent_epic()
    ));
    for packet in plan.issue_view().work_packets() {
        lines.push(format!("  - {packet}"));
    }

    lines.push(String::new());
    lines.push("Plan/report/context viewer:".to_string());
    for pane in plan.viewer_panes() {
        lines.push(format!(
            "  - {} label={} read_only={} source={}",
            pane.id(),
            pane.label(),
            pane.read_only(),
            pane.source_path().display()
        ));
    }

    lines.push(String::new());
    lines.push("Approval review screen:".to_string());
    lines.push(format!("  id: {}", plan.approval_review().id()));
    lines.push(format!("  title: {}", plan.approval_review().title()));
    lines.push(format!(
        "  approval_required: {}",
        plan.approval_review().approval_required()
    ));
    lines.push(format!(
        "  mutation_disabled: {}",
        plan.approval_review().mutation_disabled()
    ));
    for note in plan.approval_review().notes() {
        lines.push(format!("  - {note}"));
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

/// Renders a JSON workbench plan.
#[must_use]
pub fn render_interactive_workbench_plan_json(plan: &InteractiveWorkbenchPlan) -> String {
    serde_json::to_string_pretty(plan).unwrap_or_else(|_| {
        "{\n  \"command\": \"workbench-plan\",\n  \"error\": \"interactive workbench plan serialization failed\"\n}".to_string()
    })
}

/// Renders generated evidence write results.
#[must_use]
pub fn render_interactive_workbench_apply_result(
    result: &InteractiveWorkbenchApplyResult,
) -> String {
    let mut lines = vec![
        "Monad interactive workbench evidence write result".to_string(),
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
    lines.push("No interactive terminal event loop was started.".to_string());
    lines.push("No raw terminal mode was entered.".to_string());
    lines.push("No source files were mutated by the workbench.".to_string());

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_model_contains_core_screens() {
        let plan = build_interactive_workbench_plan(".");

        assert_eq!(
            plan.navigation().default_screen(),
            WorkbenchScreenKind::Dashboard
        );
        assert!(
            plan.navigation()
                .items()
                .iter()
                .any(|item| item.screen() == WorkbenchScreenKind::Approvals)
        );
    }

    #[test]
    fn shell_proof_of_concept_is_non_interactive() {
        let plan = build_interactive_workbench_plan(".");

        assert!(plan.shell_frame().interaction_disabled());
        assert!(
            plan.shell_frame()
                .panels()
                .contains(&"navigation".to_string())
        );
    }

    #[test]
    fn issue_view_contains_e34_work_packets() {
        let plan = build_interactive_workbench_plan(".");

        assert_eq!(plan.issue_view().parent_epic(), "#174");
        assert_eq!(plan.issue_view().work_packets().len(), 6);
    }

    #[test]
    fn viewer_panes_are_read_only() {
        let plan = build_interactive_workbench_plan(".");

        assert!(
            plan.viewer_panes()
                .iter()
                .all(WorkbenchViewerPane::read_only)
        );
    }

    #[test]
    fn approval_review_requires_approval_and_blocks_mutation() {
        let plan = build_interactive_workbench_plan(".");

        assert!(plan.approval_review().approval_required());
        assert!(plan.approval_review().mutation_disabled());
    }

    #[test]
    fn text_render_mentions_tui_foundation() {
        let plan = build_interactive_workbench_plan(".");
        let text = render_interactive_workbench_plan(&plan);

        assert!(text.contains("Monad interactive workbench / TUI foundation plan"));
        assert!(text.contains("TUI shell proof of concept"));
        assert!(text.contains("Approval review screen"));
    }

    #[test]
    fn json_render_contains_workbench_command() {
        let plan = build_interactive_workbench_plan(".");
        let json = render_interactive_workbench_plan_json(&plan);

        assert!(json.contains("\"command\": \"workbench-plan\""));
        assert!(json.contains("nav:dashboard"));
    }
}
