#!/usr/bin/env bash
set -euo pipefail

# complete_epic_e21_work_packet_execution_workflow.sh
#
# Implements Epic 21 — Work Packet Execution Workflow Foundation.
# This script is local-first, deterministic, supervised, and non-destructive.
# It backs up touched files before editing and refuses to run outside a Monad repo.

SCRIPT_NAME="complete_epic_e21_work_packet_execution_workflow"
BACKUP_ROOT=".monad/script-backups/${SCRIPT_NAME}-$(date -u +%Y%m%dT%H%M%SZ)"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "ERROR: required file not found: $path" >&2
    exit 1
  fi
}

require_dir() {
  local path="$1"
  if [[ ! -d "$path" ]]; then
    echo "ERROR: required directory not found: $path" >&2
    exit 1
  fi
}

backup_path() {
  local path="$1"
  if [[ -e "$path" ]]; then
    mkdir -p "${BACKUP_ROOT}/$(dirname "$path")"
    cp -a "$path" "${BACKUP_ROOT}/$path"
  fi
}

write_executable() {
  local path="$1"
  chmod +x "$path"
}

if [[ ! -f Cargo.toml ]]; then
  echo "ERROR: run this script from the repository root." >&2
  exit 1
fi

require_dir crates/monad-core/src
require_dir crates/monad-cli/src
require_file crates/monad-core/src/lib.rs
require_file crates/monad-core/src/policy.rs
require_file crates/monad-cli/src/main.rs

mkdir -p "$BACKUP_ROOT"
backup_path crates/monad-core/src/lib.rs
backup_path crates/monad-cli/src/main.rs
backup_path crates/monad-core/src/work_packet.rs
backup_path docs/work-packets/README.md
backup_path docs/roadmap/epic-21-work-packet-execution-workflow.md
backup_path tools/scripts/verify-work-packet.sh
backup_path tools/scripts/verify-e21.sh

mkdir -p docs/work-packets docs/roadmap tools/scripts crates/monad-core/src

cat > crates/monad-core/src/work_packet.rs <<'RS'
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
    let id = parse_work_packet_id(&title).ok_or_else(|| {
        "work-packet title must begin with an ID such as WP-E21-001".to_string()
    })?;
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
        format!("  implementation_steps: {}", plan.implementation_steps().len()),
        format!("  verification_commands: {}", plan.verification_commands().len()),
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
            if evidence.required() { "required" } else { "optional" },
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

fn work_packet_evidence_write_requests(
    plan: &WorkPacketExecutionPlan,
) -> Vec<GatedWriteRequest> {
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
RS

python3 - <<'PY'
from pathlib import Path

lib_path = Path("crates/monad-core/src/lib.rs")
lib = lib_path.read_text()

if "pub mod work_packet;" not in lib:
    marker = "pub mod upgrade;\n"
    if marker not in lib:
        raise SystemExit("Could not find insertion point for pub mod work_packet in monad-core lib.rs")
    lib = lib.replace(marker, marker + "pub mod work_packet;\n", 1)

work_packet_pub_use = '''pub use work_packet::{
    WORK_PACKET_HANDOFF_PATH, WORK_PACKET_PLAN_JSON_PATH, WORK_PACKET_PLAN_MARKDOWN_PATH,
    WorkPacketApplyResult, WorkPacketEvidenceItem, WorkPacketExecutionPlan,
    WorkPacketImplementationStep, WorkPacketMetadata, WorkPacketStatus,
    apply_work_packet_execution_plan, build_work_packet_execution_plan,
    parse_work_packet_metadata, render_work_packet_apply_result,
    render_work_packet_apply_result_json, render_work_packet_execution_plan,
    render_work_packet_execution_plan_json,
};
'''

if "pub use work_packet::{" not in lib:
    marker = "pub use workspace::{WorkspaceContext, discover_workspace_root, is_workspace_root};\n"
    if marker not in lib:
        raise SystemExit("Could not find insertion point for work_packet exports in monad-core lib.rs")
    lib = lib.replace(marker, work_packet_pub_use + marker, 1)

lib_path.write_text(lib)

main_path = Path("crates/monad-cli/src/main.rs")
main = main_path.read_text()

replacements = [
    (
        "apply_add_plan, apply_ai_context_plan, apply_init_plan, apply_patch_plan, apply_sync_plan,",
        "apply_add_plan, apply_ai_context_plan, apply_init_plan, apply_patch_plan, apply_sync_plan,\n    apply_work_packet_execution_plan,",
    ),
    (
        "build_ai_context_plan, build_local_agent_plan, build_patch_plan, build_policy_report,",
        "build_ai_context_plan, build_local_agent_plan, build_patch_plan, build_policy_report,\n    build_work_packet_execution_plan,",
    ),
    (
        "render_patch_apply_result, render_patch_apply_result_json, render_patch_plan,",
        "render_patch_apply_result, render_patch_apply_result_json, render_patch_plan,",
    ),
]

for before, after in replacements:
    if after not in main:
        if before not in main:
            raise SystemExit(f"Could not find import insertion point: {before}")
        main = main.replace(before, after, 1)

render_import_before = "render_patch_plan_json, render_policy_evidence_results, render_policy_report,"
render_import_after = (
    "render_patch_plan_json, render_policy_evidence_results, render_policy_report,\n"
    "    render_work_packet_apply_result, render_work_packet_apply_result_json,\n"
    "    render_work_packet_execution_plan, render_work_packet_execution_plan_json,"
)
if "render_work_packet_execution_plan_json" not in main:
    if render_import_before not in main:
        raise SystemExit("Could not find render import insertion point for work_packet")
    main = main.replace(render_import_before, render_import_after, 1)

work_packet_variant = '''
    /// Plan or write generated work-packet workflow evidence.
    WorkPacket {
        /// Whether to run in dry-run mode.
        dry_run: bool,

        /// Whether to write generated workflow evidence after explicit approval.
        yes: bool,

        /// Requested output format.
        output_format: OutputFormat,
    },
'''

if "WorkPacket {" not in main:
    marker = "\n    /// Plan or apply generated patch artifacts under E19 approval gates.\n"
    if marker not in main:
        raise SystemExit("Could not find enum insertion point for WorkPacket command")
    main = main.replace(marker, work_packet_variant + marker, 1)

if '&& parts.first().copied() != Some("work-packet")' not in main:
    before = '''            && parts.first().copied() != Some("patch")
            && parts.first().copied() != Some("sync")'''
    after = '''            && parts.first().copied() != Some("patch")
            && parts.first().copied() != Some("work-packet")
            && parts.first().copied() != Some("sync")'''
    if before not in main:
        raise SystemExit("Could not find --yes allow-list insertion point for work-packet")
    main = main.replace(before, after, 1)

main = main.replace(
    "--yes is only supported for init, add, sync, upgrade, ai-context, policy, and patch commands",
    "--yes is only supported for init, add, sync, upgrade, ai-context, policy, patch, and work-packet commands",
)

work_packet_parse = '''
            ["work-packet"] => {
                reject_write_for_non_context(write)?;
                require_work_packet_mode(dry_run, yes)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::WorkPacket {
                    dry_run,
                    yes,
                    output_format,
                })
            }
            ["work-packet", other, ..] => {
                reject_write_for_non_context(write)?;
                Err(format!("unknown work-packet argument: {other}"))
            }
'''

if '["work-packet"] =>' not in main:
    marker = '''            ["patch"] => {
'''
    if marker not in main:
        raise SystemExit("Could not find parse insertion point for work-packet command")
    main = main.replace(marker, work_packet_parse + marker, 1)

work_packet_run = '''
        CliCommand::WorkPacket {
            dry_run,
            yes,
            output_format,
        } => render_work_packet(dry_run, yes, output_format),
'''

if "=> render_work_packet(dry_run, yes, output_format)," not in main:
    marker = '''        CliCommand::Patch {
'''
    if marker not in main:
        raise SystemExit("Could not find run insertion point for work-packet command")
    main = main.replace(marker, work_packet_run + marker, 1)

require_work_packet = '''
/// Requires exactly one work-packet mode.
fn require_work_packet_mode(dry_run: bool, yes: bool) -> Result<(), String> {
    match (dry_run, yes) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => Err(
            "work-packet currently requires either --dry-run to preview or --yes to write generated workflow evidence"
                .to_string(),
        ),
        (true, true) => Err("work-packet accepts either --dry-run or --yes, not both".to_string()),
    }
}

'''

if "fn require_work_packet_mode" not in main:
    marker = "/// Requires exactly one patch mode.\n"
    if marker not in main:
        raise SystemExit("Could not find require_work_packet_mode insertion point")
    main = main.replace(marker, require_work_packet + marker, 1)

render_work_packet = '''
/// Renders or writes generated work-packet workflow evidence.
fn render_work_packet(dry_run: bool, yes: bool, output_format: OutputFormat) -> Result<String, String> {
    let root = std::env::current_dir().map_err(|error| error.to_string())?;

    if dry_run {
        let plan = build_work_packet_execution_plan(&root);
        return match output_format {
            OutputFormat::Text => Ok(render_work_packet_execution_plan(&plan)),
            OutputFormat::Json => Ok(render_work_packet_execution_plan_json(&plan)),
        };
    }

    if yes {
        let result = apply_work_packet_execution_plan(&root)?;
        return match output_format {
            OutputFormat::Text => Ok(render_work_packet_apply_result(&result)),
            OutputFormat::Json => Ok(render_work_packet_apply_result_json(&result)),
        };
    }

    Err("work-packet currently requires either --dry-run to preview or --yes to write generated workflow evidence".to_string())
}

'''

if "fn render_work_packet(" not in main:
    marker = "/// Renders or applies generated patch artifacts under E19 approval gates.\n"
    if marker not in main:
        raise SystemExit("Could not find render_work_packet insertion point")
    main = main.replace(marker, render_work_packet + marker, 1)

help_replacements = [
    (
        '        "  patch --yes                             Apply generated patch artifacts after review",\n',
        '        "  patch --yes                             Apply generated patch artifacts after review",\n'
        '        "  work-packet --dry-run                   Preview work-packet workflow plan",\n'
        '        "  work-packet --dry-run --format=json     Preview work-packet workflow as JSON",\n'
        '        "  work-packet --yes                       Write generated workflow evidence",\n',
    ),
    (
        '        "  monad patch --yes",\n',
        '        "  monad patch --yes",\n'
        '        "  monad work-packet --dry-run",\n'
        '        "  monad work-packet --dry-run --format=json",\n'
        '        "  monad work-packet --yes",\n',
    ),
    (
        '        "  patch applies generated local evidence only; user-owned source mutation remains blocked.",\n',
        '        "  patch applies generated local evidence only; user-owned source mutation remains blocked.",\n'
        '        "  work-packet writes generated workflow evidence only and never closes issues automatically.",\n',
    ),
]

for before, after in help_replacements:
    if after not in main:
        if before not in main:
            raise SystemExit(f"Could not find help insertion point: {before}")
        main = main.replace(before, after, 1)

work_packet_tests = r'''

    #[test]
    fn work_packet_dry_run_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "work-packet", "--dry-run"])
                .expect("work-packet dry-run should parse"),
            CliCommand::WorkPacket {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "work-packet", "--dry-run", "--format=json"])
                .expect("work-packet dry-run json should parse"),
            CliCommand::WorkPacket {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Json,
            }
        );
    }

    #[test]
    fn work_packet_yes_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "work-packet", "--yes"])
                .expect("work-packet yes should parse"),
            CliCommand::WorkPacket {
                dry_run: false,
                yes: true,
                output_format: OutputFormat::Text,
            }
        );
    }

    #[test]
    fn work_packet_requires_mode() {
        let error = parse_arguments(&["monad", "work-packet"])
            .expect_err("work-packet should require mode");

        assert!(error.contains("work-packet currently requires either --dry-run"));
        assert!(error.contains("--yes"));
    }

    #[test]
    fn work_packet_rejects_dry_run_and_yes_together() {
        let error = parse_arguments(&["monad", "work-packet", "--dry-run", "--yes"])
            .expect_err("work-packet should reject conflicting modes");

        assert!(error.contains("work-packet accepts either --dry-run or --yes"));
    }
'''

if "fn work_packet_dry_run_command_parses" not in main:
    idx = main.rfind("}\n")
    if idx == -1:
        raise SystemExit("Could not find end of main.rs test module")
    main = main[:idx] + work_packet_tests + main[idx:]

main_path.write_text(main)
PY

cat > docs/work-packets/README.md <<'MD'
# Work-packet execution workflow

Monad work-packet workflow support is a local-first foundation for planning, verifying, evidencing, and handing off roadmap work packets.

## Command surface

```bash
monad work-packet --dry-run
monad work-packet --dry-run --format=json
monad work-packet --yes
```

## What E21 does

E21 adds a deterministic work-packet execution model inside `monad-core`:

- work-packet lifecycle status labels;
- work-packet metadata records;
- simple Markdown metadata parsing;
- implementation-plan generation;
- verification checklist rendering;
- evidence checklist rendering;
- generated closeout and handoff records;
- smoke tests and verification scripts.

## Safety boundaries

The work-packet command does not execute implementation commands, mutate GitHub issues, close work packets remotely, contact hosted services, or rewrite user-owned source files. `monad work-packet --yes` writes generated Monad workflow evidence only, through E19 generated-write approval gates.

## Generated evidence

`monad work-packet --yes` writes:

- `.monad/reports/work-packet-plan.md`
- `.monad/reports/work-packet-plan.json`
- `.monad/work-packets/e21-closeout-handoff.md`

Existing files with different content are not silently overwritten.
MD

cat > docs/roadmap/epic-21-work-packet-execution-workflow.md <<'MD'
# E21 — Work Packet Execution Workflow Foundation

## Product area

Work Packet Execution Workflow Foundation

## Objective

Create the first local, deterministic workflow foundation for executing Monad roadmap work packets with implementation planning, verification checklists, evidence records, closeout notes, and handoff artifacts.

## Implemented work packets

- WP-E21-001 — Define work-packet execution model.
- WP-E21-002 — Add work-packet metadata parser.
- WP-E21-003 — Add work-packet implementation plan generator.
- WP-E21-004 — Add verification and evidence checklist automation.
- WP-E21-005 — Add closeout and handoff record generation.
- WP-E21-006 — Add work-packet workflow smoke tests.

## Command surface

```bash
monad work-packet --dry-run
monad work-packet --dry-run --format=json
monad work-packet --yes
```

## Safety posture

E21 remains local-first and supervised:

- no autonomous work-packet execution;
- no GitHub issue mutation or closeout automation;
- no arbitrary command execution;
- no user-owned source rewrites;
- no remote service calls;
- generated writes require explicit `--yes` and E19 approval gates.

## Evidence outputs

`monad work-packet --yes` writes generated local artifacts only:

- `.monad/reports/work-packet-plan.md`
- `.monad/reports/work-packet-plan.json`
- `.monad/work-packets/e21-closeout-handoff.md`

## Verification

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-work-packet.sh
tools/scripts/verify-e21.sh
```
MD

cat > tools/scripts/verify-work-packet.sh <<'SH_VERIFY_WORK_PACKET'
#!/usr/bin/env bash
set -euo pipefail

echo "[verify-work-packet] cargo test -p monad-core --lib work_packet"
cargo test -p monad-core --lib work_packet

echo "[verify-work-packet] monad work-packet --dry-run"
cargo run -p monad-cli -- work-packet --dry-run >/tmp/monad-work-packet-dry-run.txt
grep -q "Monad work-packet execution workflow plan" /tmp/monad-work-packet-dry-run.txt
grep -q "Verification checklist" /tmp/monad-work-packet-dry-run.txt
grep -q "No GitHub issues were modified" /tmp/monad-work-packet-dry-run.txt

echo "[verify-work-packet] monad work-packet --dry-run --format=json"
cargo run -p monad-cli -- work-packet --dry-run --format=json >/tmp/monad-work-packet-dry-run.json
grep -q '"command":"work-packet"' /tmp/monad-work-packet-dry-run.json
grep -q '"mode":"dry-run"' /tmp/monad-work-packet-dry-run.json
grep -q '"work_packet_count":6' /tmp/monad-work-packet-dry-run.json

echo "[verify-work-packet] monad work-packet --yes"
cargo run -p monad-cli -- work-packet --yes >/tmp/monad-work-packet-apply.txt
grep -q "Monad work-packet workflow evidence write result" /tmp/monad-work-packet-apply.txt

test -f .monad/reports/work-packet-plan.md
test -f .monad/reports/work-packet-plan.json
test -f .monad/work-packets/e21-closeout-handoff.md

grep -q "Monad work-packet execution workflow plan" .monad/reports/work-packet-plan.md
grep -q '"command":"work-packet"' .monad/reports/work-packet-plan.json
grep -q "E21 Work-Packet Workflow Closeout Handoff" .monad/work-packets/e21-closeout-handoff.md

echo "[verify-work-packet] ok"
SH_VERIFY_WORK_PACKET

cat > tools/scripts/verify-e21.sh <<'SH_VERIFY_E21'
#!/usr/bin/env bash
set -euo pipefail

echo "[verify-e21] cargo fmt --check"
cargo fmt --check

echo "[verify-e21] cargo test"
cargo test

echo "[verify-e21] cargo clippy --all-targets --all-features -- -D warnings"
cargo clippy --all-targets --all-features -- -D warnings

echo "[verify-e21] tools/scripts/verify-work-packet.sh"
tools/scripts/verify-work-packet.sh

echo "[verify-e21] ok"
SH_VERIFY_E21

write_executable tools/scripts/verify-work-packet.sh
write_executable tools/scripts/verify-e21.sh

# Let rustfmt normalize Rust edits made by this script.
cargo fmt

echo "E21 work-packet execution workflow foundation files written."
echo "Backups, if any, are under: ${BACKUP_ROOT}"
echo
cat <<'NEXT'
Suggested verification:
  cargo fmt --check
  cargo test
  cargo clippy --all-targets --all-features -- -D warnings
  tools/scripts/verify-work-packet.sh
  tools/scripts/verify-e21.sh
NEXT
