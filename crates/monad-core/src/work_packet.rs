//! Work-packet execution workflow foundation.
//!
//! E21 turns Monad work packets into deterministic local workflow records. It
//! models work-packet metadata, parses simple Markdown packet records, generates
//! implementation plans, renders verification/evidence checklists, and writes
//! generated closeout/handoff evidence under explicit E19 generated-write gates.
//! It does not contact GitHub, run issue automation, execute arbitrary commands,
//! mutate user source, or autonomously complete work.

use crate::policy::{GatedWriteRequest, GatedWriteResult, gated_generated_write};
use std::path::{Path, PathBuf};

/// Markdown evidence report path for the work-packet plan.
pub const WORK_PACKET_PLAN_MARKDOWN_PATH: &str = ".monad/reports/work-packet-plan.md";

/// JSON evidence report path for the work-packet plan.
pub const WORK_PACKET_PLAN_JSON_PATH: &str = ".monad/reports/work-packet-plan.json";

/// Generated handoff record path for the work-packet workflow foundation.
pub const WORK_PACKET_HANDOFF_PATH: &str = ".monad/work-packets/e21-closeout-handoff.md";

/// Work-packet lifecycle status tracked by the workflow foundation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorkPacketStatus {
    /// Work packet is known but not yet started.
    Planned,

    /// Work packet is being implemented.
    Active,

    /// Work packet is ready for verification.
    Verification,

    /// Work packet is ready for closeout/handoff.
    Closeout,
}

impl WorkPacketStatus {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Active => "active",
            Self::Verification => "verification",
            Self::Closeout => "closeout",
        }
    }
}

/// Parsed or generated work-packet metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkPacketMetadata {
    id: String,
    title: String,
    epic: String,
    product_area: String,
    objective: String,
    verification_commands: Vec<String>,
    evidence_paths: Vec<PathBuf>,
    status: WorkPacketStatus,
}

impl WorkPacketMetadata {
    /// Creates a work-packet metadata record.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        epic: impl Into<String>,
        objective: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            epic: epic.into(),
            product_area: "Work Packet Execution Workflow Foundation".to_string(),
            objective: objective.into(),
            verification_commands: Vec::new(),
            evidence_paths: Vec::new(),
            status: WorkPacketStatus::Planned,
        }
    }

    /// Sets product area.
    #[must_use]
    pub fn with_product_area(mut self, product_area: impl Into<String>) -> Self {
        self.product_area = product_area.into();
        self
    }

    /// Sets verification commands.
    #[must_use]
    pub fn with_verification_commands(mut self, commands: Vec<String>) -> Self {
        self.verification_commands = commands;
        self
    }

    /// Sets evidence paths.
    #[must_use]
    pub fn with_evidence_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.evidence_paths = paths;
        self
    }

    /// Sets status.
    #[must_use]
    pub fn with_status(mut self, status: WorkPacketStatus) -> Self {
        self.status = status;
        self
    }

    /// Work-packet ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Parent epic.
    #[must_use]
    pub fn epic(&self) -> &str {
        &self.epic
    }

    /// Product area.
    #[must_use]
    pub fn product_area(&self) -> &str {
        &self.product_area
    }

    /// Objective.
    #[must_use]
    pub fn objective(&self) -> &str {
        &self.objective
    }

    /// Verification commands.
    #[must_use]
    pub fn verification_commands(&self) -> &[String] {
        &self.verification_commands
    }

    /// Evidence paths.
    #[must_use]
    pub fn evidence_paths(&self) -> &[PathBuf] {
        &self.evidence_paths
    }

    /// Status.
    #[must_use]
    pub const fn status(&self) -> WorkPacketStatus {
        self.status
    }
}

/// One generated implementation-plan step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkPacketImplementationStep {
    order: usize,
    work_packet_id: String,
    action: String,
    rationale: String,
}

impl WorkPacketImplementationStep {
    /// Creates an implementation-plan step.
    #[must_use]
    pub fn new(
        order: usize,
        work_packet_id: impl Into<String>,
        action: impl Into<String>,
        rationale: impl Into<String>,
    ) -> Self {
        Self {
            order,
            work_packet_id: work_packet_id.into(),
            action: action.into(),
            rationale: rationale.into(),
        }
    }

    /// Step order.
    #[must_use]
    pub const fn order(&self) -> usize {
        self.order
    }

    /// Associated work-packet ID.
    #[must_use]
    pub fn work_packet_id(&self) -> &str {
        &self.work_packet_id
    }

    /// Action.
    #[must_use]
    pub fn action(&self) -> &str {
        &self.action
    }

    /// Rationale.
    #[must_use]
    pub fn rationale(&self) -> &str {
        &self.rationale
    }
}

/// Expected evidence item for verification/closeout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkPacketEvidenceItem {
    path: PathBuf,
    description: String,
    required: bool,
}

impl WorkPacketEvidenceItem {
    /// Creates an evidence item.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>, description: impl Into<String>, required: bool) -> Self {
        Self {
            path: path.into(),
            description: description.into(),
            required,
        }
    }

    /// Evidence path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Whether this evidence item is required.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

/// Full deterministic work-packet execution plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkPacketExecutionPlan {
    epic_id: String,
    epic_title: String,
    work_packets: Vec<WorkPacketMetadata>,
    implementation_steps: Vec<WorkPacketImplementationStep>,
    verification_commands: Vec<String>,
    evidence_items: Vec<WorkPacketEvidenceItem>,
    handoff_notes: Vec<String>,
    safety_boundaries: Vec<String>,
}

impl WorkPacketExecutionPlan {
    /// Creates a deterministic execution plan.
    #[must_use]
    pub fn new(
        epic_id: impl Into<String>,
        epic_title: impl Into<String>,
        mut work_packets: Vec<WorkPacketMetadata>,
    ) -> Self {
        work_packets.sort_by(|left, right| left.id().cmp(right.id()));
        let implementation_steps = implementation_steps_from_packets(&work_packets);
        let verification_commands = default_verification_commands();
        let evidence_items = default_evidence_items();
        let handoff_notes = default_handoff_notes();
        let safety_boundaries = default_safety_boundaries();

        Self {
            epic_id: epic_id.into(),
            epic_title: epic_title.into(),
            work_packets,
            implementation_steps,
            verification_commands,
            evidence_items,
            handoff_notes,
            safety_boundaries,
        }
    }

    /// Epic ID.
    #[must_use]
    pub fn epic_id(&self) -> &str {
        &self.epic_id
    }

    /// Epic title.
    #[must_use]
    pub fn epic_title(&self) -> &str {
        &self.epic_title
    }

    /// Work packets.
    #[must_use]
    pub fn work_packets(&self) -> &[WorkPacketMetadata] {
        &self.work_packets
    }

    /// Implementation steps.
    #[must_use]
    pub fn implementation_steps(&self) -> &[WorkPacketImplementationStep] {
        &self.implementation_steps
    }

    /// Verification commands.
    #[must_use]
    pub fn verification_commands(&self) -> &[String] {
        &self.verification_commands
    }

    /// Evidence items.
    #[must_use]
    pub fn evidence_items(&self) -> &[WorkPacketEvidenceItem] {
        &self.evidence_items
    }

    /// Handoff notes.
    #[must_use]
    pub fn handoff_notes(&self) -> &[String] {
        &self.handoff_notes
    }

    /// Safety boundaries.
    #[must_use]
    pub fn safety_boundaries(&self) -> &[String] {
        &self.safety_boundaries
    }
}

/// Result from supervised work-packet evidence apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkPacketApplyResult {
    plan: WorkPacketExecutionPlan,
    write_results: Vec<GatedWriteResult>,
}

impl WorkPacketApplyResult {
    /// Creates an apply result.
    #[must_use]
    pub fn new(plan: WorkPacketExecutionPlan, write_results: Vec<GatedWriteResult>) -> Self {
        Self {
            plan,
            write_results,
        }
    }

    /// Execution plan.
    #[must_use]
    pub const fn plan(&self) -> &WorkPacketExecutionPlan {
        &self.plan
    }

    /// Write results.
    #[must_use]
    pub fn write_results(&self) -> &[GatedWriteResult] {
        &self.write_results
    }
}

/// Parses a simple Markdown work-packet metadata document.
///
/// The parser intentionally accepts a small stable subset: a top-level `#`
/// title plus optional `## Product Area`, `## Objective`, `## Parent Epic`,
/// `## Verification`, and `## Evidence` sections. It is deterministic and does
/// not fetch GitHub or inspect remote issue state.
pub fn parse_work_packet_metadata(markdown: &str) -> Result<WorkPacketMetadata, String> {
    let title = parse_title(markdown).ok_or_else(|| "missing work-packet title".to_string())?;
    let id = parse_work_packet_id(&title)
        .ok_or_else(|| "work-packet title must begin with an ID such as WP-E21-001".to_string())?;
    let objective = first_section_value(markdown, "Objective")
        .unwrap_or_else(|| format!("Complete {id} as an atomic, reviewable work packet."));
    let epic = first_section_value(markdown, "Parent Epic").unwrap_or_else(|| "E21".to_string());
    let product_area = first_section_value(markdown, "Product Area")
        .unwrap_or_else(|| "Work Packet Execution Workflow Foundation".to_string());
    let verification_commands = section_list_values(markdown, "Verification");
    let evidence_paths = section_list_values(markdown, "Evidence")
        .into_iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>();

    Ok(WorkPacketMetadata::new(id, title, epic, objective)
        .with_product_area(product_area)
        .with_verification_commands(verification_commands)
        .with_evidence_paths(evidence_paths))
}

/// Builds the default E21 work-packet execution plan.
#[must_use]
pub fn build_work_packet_execution_plan(_root: impl AsRef<Path>) -> WorkPacketExecutionPlan {
    WorkPacketExecutionPlan::new(
        "E21",
        "Work Packet Execution Workflow Foundation",
        default_e21_work_packets(),
    )
}

/// Applies generated work-packet workflow evidence under E19 generated-write gates.
pub fn apply_work_packet_execution_plan(
    root: impl AsRef<Path>,
) -> Result<WorkPacketApplyResult, String> {
    let plan = build_work_packet_execution_plan(root.as_ref());
    let requests = work_packet_evidence_write_requests(&plan);
    let mut write_results = Vec::new();

    for request in requests {
        write_results.push(gated_generated_write(root.as_ref(), &request)?);
    }

    Ok(WorkPacketApplyResult::new(plan, write_results))
}

/// Renders the work-packet execution plan as deterministic text.
#[must_use]
pub fn render_work_packet_execution_plan(plan: &WorkPacketExecutionPlan) -> String {
    let mut lines = vec![
        "Monad work-packet execution workflow plan".to_string(),
        String::new(),
        "Summary:".to_string(),
        format!("  epic_id: {}", plan.epic_id()),
        format!("  epic_title: {}", plan.epic_title()),
        format!("  work_packets: {}", plan.work_packets().len()),
        format!(
            "  implementation_steps: {}",
            plan.implementation_steps().len()
        ),
        format!(
            "  verification_commands: {}",
            plan.verification_commands().len()
        ),
        format!("  evidence_items: {}", plan.evidence_items().len()),
        String::new(),
        "Work packets:".to_string(),
    ];

    for packet in plan.work_packets() {
        lines.push(format!(
            "  - {} — {} [{}]",
            packet.id(),
            packet.title(),
            packet.status().as_str()
        ));
        lines.push(format!("    product_area: {}", packet.product_area()));
        lines.push(format!("    objective: {}", packet.objective()));
    }

    lines.push(String::new());
    lines.push("Implementation plan:".to_string());
    for step in plan.implementation_steps() {
        lines.push(format!(
            "  {}. {} — {}",
            step.order(),
            step.work_packet_id(),
            step.action()
        ));
        lines.push(format!("     rationale: {}", step.rationale()));
    }

    lines.push(String::new());
    lines.push("Verification checklist:".to_string());
    for command in plan.verification_commands() {
        lines.push(format!("  - `{command}`"));
    }

    lines.push(String::new());
    lines.push("Evidence checklist:".to_string());
    for evidence in plan.evidence_items() {
        lines.push(format!(
            "  - [{}] {} — {}",
            if evidence.required() {
                "required"
            } else {
                "optional"
            },
            evidence.path().display(),
            evidence.description()
        ));
    }

    lines.push(String::new());
    lines.push("Closeout and handoff notes:".to_string());
    for note in plan.handoff_notes() {
        lines.push(format!("  - {note}"));
    }

    lines.push(String::new());
    lines.push("Safety boundaries:".to_string());
    for boundary in plan.safety_boundaries() {
        lines.push(format!("  - {boundary}"));
    }

    lines.push(String::new());
    lines.push("No commands were executed by this planner.".to_string());
    lines.push("No GitHub issues were modified.".to_string());
    lines.push("No user-owned source files were rewritten.".to_string());
    lines.push("No remote services were contacted.".to_string());

    lines.join("\n")
}

/// Renders the work-packet execution plan as deterministic JSON.
#[must_use]
pub fn render_work_packet_execution_plan_json(plan: &WorkPacketExecutionPlan) -> String {
    let packets = plan
        .work_packets()
        .iter()
        .map(render_packet_json)
        .collect::<Vec<_>>()
        .join(",");
    let steps = plan
        .implementation_steps()
        .iter()
        .map(render_step_json)
        .collect::<Vec<_>>()
        .join(",");
    let verification = json_array(plan.verification_commands().iter().map(String::as_str));
    let evidence = plan
        .evidence_items()
        .iter()
        .map(render_evidence_json)
        .collect::<Vec<_>>()
        .join(",");
    let handoff = json_array(plan.handoff_notes().iter().map(String::as_str));
    let safety = json_array(plan.safety_boundaries().iter().map(String::as_str));

    format!(
        "{{\"command\":\"work-packet\",\"mode\":\"dry-run\",\"epic_id\":\"{}\",\"epic_title\":\"{}\",\"work_packet_count\":{},\"implementation_step_count\":{},\"work_packets\":[{}],\"implementation_steps\":[{}],\"verification_commands\":{},\"evidence_items\":[{}],\"handoff_notes\":{},\"safety_boundaries\":{}}}",
        json_escape(plan.epic_id()),
        json_escape(plan.epic_title()),
        plan.work_packets().len(),
        plan.implementation_steps().len(),
        packets,
        steps,
        verification,
        evidence,
        handoff,
        safety,
    )
}

/// Renders supervised evidence apply result as text.
#[must_use]
pub fn render_work_packet_apply_result(result: &WorkPacketApplyResult) -> String {
    let mut lines = vec![
        "Monad work-packet workflow evidence write result".to_string(),
        String::new(),
        "Plan summary:".to_string(),
        format!("  epic_id: {}", result.plan().epic_id()),
        format!("  work_packets: {}", result.plan().work_packets().len()),
        String::new(),
        "Write results:".to_string(),
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
    lines.push("Safety notes:".to_string());
    lines.push("  - Writes are limited to generated Monad workflow evidence.".to_string());
    lines.push("  - Existing files with different content are not overwritten.".to_string());
    lines.push("  - No issues are closed or modified automatically.".to_string());
    lines.push("  - No implementation commands are executed automatically.".to_string());

    lines.join("\n")
}

/// Renders supervised evidence apply result as JSON.
#[must_use]
pub fn render_work_packet_apply_result_json(result: &WorkPacketApplyResult) -> String {
    let write_results = result
        .write_results()
        .iter()
        .map(render_gated_write_result_json)
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"command\":\"work-packet\",\"mode\":\"apply\",\"epic_id\":\"{}\",\"work_packet_count\":{},\"write_results\":[{}]}}",
        json_escape(result.plan().epic_id()),
        result.plan().work_packets().len(),
        write_results,
    )
}

fn default_e21_work_packets() -> Vec<WorkPacketMetadata> {
    let commands = default_verification_commands();
    let evidence_paths = vec![
        PathBuf::from(WORK_PACKET_PLAN_MARKDOWN_PATH),
        PathBuf::from(WORK_PACKET_PLAN_JSON_PATH),
        PathBuf::from(WORK_PACKET_HANDOFF_PATH),
    ];

    [
        (
            "WP-E21-001",
            "Define work-packet execution model",
            "Define the local lifecycle contract for planned, active, verification, closeout, and handoff states.",
        ),
        (
            "WP-E21-002",
            "Add work-packet metadata parser",
            "Parse stable Markdown work-packet metadata into deterministic core records.",
        ),
        (
            "WP-E21-003",
            "Add work-packet implementation plan generator",
            "Generate an implementation sequence from work-packet metadata without executing commands.",
        ),
        (
            "WP-E21-004",
            "Add verification and evidence checklist automation",
            "Render deterministic verification commands and expected evidence paths.",
        ),
        (
            "WP-E21-005",
            "Add closeout and handoff record generation",
            "Generate local closeout and handoff records for reviewable work-packet completion.",
        ),
        (
            "WP-E21-006",
            "Add work-packet workflow smoke tests",
            "Add smoke coverage for parsing, rendering, evidence generation, and CLI behavior.",
        ),
    ]
    .into_iter()
    .map(|(id, title, objective)| {
        WorkPacketMetadata::new(id, title, "E21", objective)
            .with_verification_commands(commands.clone())
            .with_evidence_paths(evidence_paths.clone())
    })
    .collect()
}

fn implementation_steps_from_packets(
    packets: &[WorkPacketMetadata],
) -> Vec<WorkPacketImplementationStep> {
    packets
        .iter()
        .enumerate()
        .map(|(index, packet)| {
            WorkPacketImplementationStep::new(
                index + 1,
                packet.id(),
                format!("Implement {} as a reviewable local workflow slice", packet.title()),
                "keep domain behavior in monad-core, keep CLI thin, and preserve generated evidence",
            )
        })
        .collect()
}

fn default_verification_commands() -> Vec<String> {
    vec![
        "cargo fmt --check".to_string(),
        "cargo test".to_string(),
        "cargo clippy --all-targets --all-features -- -D warnings".to_string(),
        "tools/scripts/verify-work-packet.sh".to_string(),
        "tools/scripts/verify-e21.sh".to_string(),
    ]
}

fn default_evidence_items() -> Vec<WorkPacketEvidenceItem> {
    vec![
        WorkPacketEvidenceItem::new(
            WORK_PACKET_PLAN_MARKDOWN_PATH,
            "deterministic Markdown work-packet execution plan",
            true,
        ),
        WorkPacketEvidenceItem::new(
            WORK_PACKET_PLAN_JSON_PATH,
            "deterministic JSON work-packet execution plan",
            true,
        ),
        WorkPacketEvidenceItem::new(
            WORK_PACKET_HANDOFF_PATH,
            "generated closeout and handoff record",
            true,
        ),
    ]
}

fn default_handoff_notes() -> Vec<String> {
    vec![
        "Record verification commands before closing a work packet.".to_string(),
        "Attach or reference generated evidence reports during closeout.".to_string(),
        "Capture follow-up risks before proceeding to the next work packet.".to_string(),
        "Do not close GitHub issues automatically from the local CLI.".to_string(),
    ]
}

fn default_safety_boundaries() -> Vec<String> {
    vec![
        "No autonomous work-packet execution.".to_string(),
        "No GitHub issue mutation or closeout automation.".to_string(),
        "No arbitrary command execution.".to_string(),
        "No user-owned source rewrites.".to_string(),
        "No remote service calls.".to_string(),
        "Generated writes require explicit --yes and E19 approval gates.".to_string(),
    ]
}

fn work_packet_evidence_write_requests(plan: &WorkPacketExecutionPlan) -> Vec<GatedWriteRequest> {
    vec![
        GatedWriteRequest::new(
            WORK_PACKET_PLAN_MARKDOWN_PATH,
            render_work_packet_execution_plan(plan),
            true,
        ),
        GatedWriteRequest::new(
            WORK_PACKET_PLAN_JSON_PATH,
            render_work_packet_execution_plan_json(plan),
            true,
        ),
        GatedWriteRequest::new(WORK_PACKET_HANDOFF_PATH, render_handoff_record(plan), true),
    ]
}

fn render_handoff_record(plan: &WorkPacketExecutionPlan) -> String {
    let mut lines = vec![
        "# E21 Work-Packet Workflow Closeout Handoff".to_string(),
        String::new(),
        format!("Epic: {} — {}", plan.epic_id(), plan.epic_title()),
        String::new(),
        "## Work packets".to_string(),
    ];

    for packet in plan.work_packets() {
        lines.push(format!("- {} — {}", packet.id(), packet.title()));
    }

    lines.push(String::new());
    lines.push("## Verification".to_string());
    for command in plan.verification_commands() {
        lines.push(format!("- `{command}`"));
    }

    lines.push(String::new());
    lines.push("## Handoff notes".to_string());
    for note in plan.handoff_notes() {
        lines.push(format!("- {note}"));
    }

    lines.push(String::new());
    lines.push("## Safety".to_string());
    for boundary in plan.safety_boundaries() {
        lines.push(format!("- {boundary}"));
    }

    lines.push(String::new());
    lines.join("\n")
}

fn parse_title(markdown: &str) -> Option<String> {
    markdown.lines().find_map(|line| {
        let trimmed = line.trim();
        trimmed
            .strip_prefix("# ")
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn parse_work_packet_id(title: &str) -> Option<String> {
    let candidate = title
        .split_once('—')
        .map(|(left, _)| left)
        .or_else(|| title.split_once("--").map(|(left, _)| left))
        .or_else(|| title.split_once('-').map(|(left, _)| left))
        .unwrap_or(title)
        .trim();

    if candidate.starts_with("WP-E") {
        Some(candidate.to_string())
    } else {
        None
    }
}

fn first_section_value(markdown: &str, heading: &str) -> Option<String> {
    section_lines(markdown, heading)
        .into_iter()
        .map(|line| clean_list_line(&line))
        .find(|line| !line.is_empty())
}

fn section_list_values(markdown: &str, heading: &str) -> Vec<String> {
    section_lines(markdown, heading)
        .into_iter()
        .map(|line| clean_list_line(&line))
        .filter(|line| !line.is_empty())
        .collect()
}

fn section_lines(markdown: &str, heading: &str) -> Vec<String> {
    let target = format!("## {heading}");
    let mut in_section = false;
    let mut values = Vec::new();

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed == target {
            in_section = true;
            continue;
        }

        if in_section && trimmed.starts_with("## ") {
            break;
        }

        if in_section {
            values.push(trimmed.to_string());
        }
    }

    values
}

fn clean_list_line(line: &str) -> String {
    line.trim()
        .trim_start_matches('-')
        .trim_start_matches('*')
        .trim()
        .trim_matches('`')
        .trim()
        .to_string()
}

fn render_packet_json(packet: &WorkPacketMetadata) -> String {
    let commands = json_array(packet.verification_commands().iter().map(String::as_str));
    let evidence_paths = json_array(
        packet
            .evidence_paths()
            .iter()
            .map(|path| path.display().to_string()),
    );

    format!(
        "{{\"id\":\"{}\",\"title\":\"{}\",\"epic\":\"{}\",\"product_area\":\"{}\",\"objective\":\"{}\",\"status\":\"{}\",\"verification_commands\":{},\"evidence_paths\":{}}}",
        json_escape(packet.id()),
        json_escape(packet.title()),
        json_escape(packet.epic()),
        json_escape(packet.product_area()),
        json_escape(packet.objective()),
        packet.status().as_str(),
        commands,
        evidence_paths,
    )
}

fn render_step_json(step: &WorkPacketImplementationStep) -> String {
    format!(
        "{{\"order\":{},\"work_packet_id\":\"{}\",\"action\":\"{}\",\"rationale\":\"{}\"}}",
        step.order(),
        json_escape(step.work_packet_id()),
        json_escape(step.action()),
        json_escape(step.rationale()),
    )
}

fn render_evidence_json(evidence: &WorkPacketEvidenceItem) -> String {
    format!(
        "{{\"path\":\"{}\",\"description\":\"{}\",\"required\":{}}}",
        json_escape(&evidence.path().display().to_string()),
        json_escape(evidence.description()),
        evidence.required(),
    )
}

fn render_gated_write_result_json(result: &GatedWriteResult) -> String {
    match result {
        GatedWriteResult::Written(path) | GatedWriteResult::SkippedIdentical(path) => {
            format!(
                "{{\"status\":\"{}\",\"path\":\"{}\"}}",
                result.as_str(),
                json_escape(&path.display().to_string())
            )
        }
        GatedWriteResult::ApprovalRequired(message) | GatedWriteResult::Blocked(message) => {
            format!(
                "{{\"status\":\"{}\",\"message\":\"{}\"}}",
                result.as_str(),
                json_escape(message)
            )
        }
    }
}

fn json_array<I, S>(values: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let rendered = values
        .into_iter()
        .map(|value| format!("\"{}\"", json_escape(value.as_ref())))
        .collect::<Vec<_>>()
        .join(",");

    format!("[{rendered}]")
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
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn work_packet_default_plan_contains_six_packets() {
        let root = unique_test_root("plan");
        let plan = build_work_packet_execution_plan(&root);

        assert_eq!(plan.epic_id(), "E21");
        assert_eq!(plan.work_packets().len(), 6);
        assert_eq!(plan.implementation_steps().len(), 6);
        assert_eq!(plan.work_packets()[0].id(), "WP-E21-001");
        assert_eq!(plan.work_packets()[5].id(), "WP-E21-006");
    }

    #[test]
    fn work_packet_metadata_parser_extracts_core_fields() {
        let markdown = r#"# WP-E21-002 — Add work-packet metadata parser

## Product Area

Work Packet Execution Workflow Foundation

## Objective

Parse stable packet records.

## Parent Epic

E21

## Verification

- cargo test
- tools/scripts/verify-e21.sh

## Evidence

- .monad/reports/work-packet-plan.md
"#;

        let packet = parse_work_packet_metadata(markdown).expect("metadata should parse");

        assert_eq!(packet.id(), "WP-E21-002");
        assert_eq!(packet.epic(), "E21");
        assert_eq!(packet.objective(), "Parse stable packet records.");
        assert_eq!(packet.verification_commands().len(), 2);
        assert_eq!(packet.evidence_paths().len(), 1);
    }

    #[test]
    fn work_packet_metadata_parser_requires_title() {
        let error = parse_work_packet_metadata("## Objective\n\nMissing title")
            .expect_err("missing title should fail");

        assert!(error.contains("missing work-packet title"));
    }

    #[test]
    fn work_packet_plan_renders_checklists_and_safety() {
        let root = unique_test_root("render");
        let plan = build_work_packet_execution_plan(&root);
        let rendered = render_work_packet_execution_plan(&plan);

        assert!(rendered.contains("Monad work-packet execution workflow plan"));
        assert!(rendered.contains("Verification checklist"));
        assert!(rendered.contains("Evidence checklist"));
        assert!(rendered.contains("No GitHub issues were modified"));
    }

    #[test]
    fn work_packet_plan_json_is_command_tagged() {
        let root = unique_test_root("json");
        let plan = build_work_packet_execution_plan(&root);
        let rendered = render_work_packet_execution_plan_json(&plan);

        assert!(rendered.contains("\"command\":\"work-packet\""));
        assert!(rendered.contains("\"mode\":\"dry-run\""));
        assert!(rendered.contains("\"work_packet_count\":6"));
    }

    #[test]
    fn work_packet_apply_writes_generated_evidence() {
        let root = unique_test_root("apply");
        let result = apply_work_packet_execution_plan(&root).expect("apply should complete");

        assert_eq!(result.write_results().len(), 3);
        assert!(root.join(WORK_PACKET_PLAN_MARKDOWN_PATH).exists());
        assert!(root.join(WORK_PACKET_PLAN_JSON_PATH).exists());
        assert!(root.join(WORK_PACKET_HANDOFF_PATH).exists());
    }

    #[test]
    fn work_packet_apply_result_json_is_command_tagged() {
        let root = unique_test_root("apply-json");
        let result = apply_work_packet_execution_plan(&root).expect("apply should complete");
        let rendered = render_work_packet_apply_result_json(&result);

        assert!(rendered.contains("\"command\":\"work-packet\""));
        assert!(rendered.contains("\"mode\":\"apply\""));
        assert!(rendered.contains("write_results"));
    }

    #[test]
    fn work_packet_status_labels_are_stable() {
        assert_eq!(WorkPacketStatus::Planned.as_str(), "planned");
        assert_eq!(WorkPacketStatus::Active.as_str(), "active");
        assert_eq!(WorkPacketStatus::Verification.as_str(), "verification");
        assert_eq!(WorkPacketStatus::Closeout.as_str(), "closeout");
    }

    fn unique_test_root(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after Unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "monad-work-packet-{name}-{}-{nanos}",
            std::process::id()
        ));
        fs::remove_dir_all(&root).ok();
        root
    }
}
