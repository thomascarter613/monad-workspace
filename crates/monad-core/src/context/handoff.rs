//! Handoff artifact generator for Monad.
//!
//! A handoff artifact allows a future human or AI assistant to resume work
//! without reconstructing project state from memory or scattered conversation
//! history.
//!
//! The generator reads repository state from:
//!
//! - `monad.toml` for project identity;
//! - `work/epics/` for completed and active epics;
//! - `work/packets/` for active work packets;
//! - `crates/monad-core/src/lib.rs` for runtime module exports.
//!
//! The output is a deterministic Markdown file written to
//! `.monad/context/latest-handoff.md`.

use std::fs;
use std::path::Path;

use crate::context::current_state::{CurrentStateArtifact, generate_current_state};
use crate::{MonadError, MonadResult, WorkspaceContext};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// A parsed work packet entry from the `work/packets/` directory.
///
/// Each work packet file has YAML frontmatter with `title`, `status`,
/// `epic`, and `work_packet` fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffWorkPacketEntry {
    /// The work packet identifier, e.g. `"WP-E2-001"`.
    pub id: String,

    /// The work packet title, e.g. `"Establish Repository Inspection Foundation"`.
    pub title: String,

    /// The work packet status, e.g. `"complete"` or `"in-progress"`.
    pub status: String,

    /// The parent epic identifier, e.g. `"E2"`.
    pub epic: String,
}

/// A generated handoff artifact.
///
/// This struct holds all the data the generator collected from the repository.
/// It can be rendered to Markdown deterministically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffArtifact {
    /// The current-state snapshot used as the foundation for this handoff.
    pub current_state: CurrentStateArtifact,

    /// All work packets found in `work/packets/`, sorted by identifier.
    pub work_packets: Vec<HandoffWorkPacketEntry>,

    /// Source files that were inspected to produce this artifact.
    pub source_files: Vec<String>,
}

// ---------------------------------------------------------------------------
// Generation
// ---------------------------------------------------------------------------

/// Generates a handoff artifact from repository state.
///
/// This function reads the workspace to determine the project identity,
/// epic statuses, work packet statuses, and runtime capabilities. It
/// returns a structured artifact that can be rendered to Markdown.
pub fn generate_handoff(context: &WorkspaceContext) -> MonadResult<HandoffArtifact> {
    let mut source_files: Vec<String> = Vec::new();

    // 1. Generate the current-state snapshot as foundation.
    let current_state = generate_current_state(context)?;
    source_files.extend(current_state.source_files.clone());

    // 2. Read work packets from work/packets/.
    let packets_dir = context.work_dir().join("packets");
    let work_packets = read_work_packets(&packets_dir)?;
    source_files.push("work/packets/".to_string());

    source_files.sort();
    source_files.dedup();

    Ok(HandoffArtifact {
        current_state,
        work_packets,
        source_files,
    })
}

/// Writes a handoff artifact to `.monad/context/latest-handoff.md`.
///
/// Creates the output directory if it does not exist.
pub fn write_handoff_artifact(
    context: &WorkspaceContext,
    artifact: &HandoffArtifact,
) -> MonadResult<()> {
    let output_dir = context.context_dir();

    fs::create_dir_all(&output_dir).map_err(|error| {
        MonadError::internal(format!(
            "failed to create context directory {}: {error}",
            output_dir.display()
        ))
    })?;

    let output_path = output_dir.join("latest-handoff.md");
    let rendered = render_handoff_markdown(artifact);

    fs::write(&output_path, &rendered).map_err(|error| {
        MonadError::internal(format!(
            "failed to write handoff artifact to {}: {error}",
            output_path.display()
        ))
    })?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/// Renders a handoff artifact to deterministic Markdown.
///
/// The output follows the Handoff Standard defined in
/// `docs/08-context/HANDOFF-STANDARD.md`.
#[must_use]
pub fn render_handoff_markdown(artifact: &HandoffArtifact) -> String {
    let cs = &artifact.current_state;
    let mut lines: Vec<String> = Vec::new();

    // Frontmatter.
    render_frontmatter(&mut lines, artifact);

    // # Latest Handoff
    lines.push("# Latest Handoff".to_string());
    lines.push(String::new());

    // ## Project
    render_project_section(&mut lines, cs);

    // ## Current Status
    render_current_status_section(&mut lines, cs, artifact);

    // ## Active Epic
    render_active_epic_section(&mut lines, cs);

    // ## Active Work Packet
    render_active_work_packet_section(&mut lines, artifact);

    // ## Recently Completed
    render_recently_completed_section(&mut lines, cs, artifact);

    // ## Current Files of Interest
    render_files_of_interest_section(&mut lines, cs);

    // ## Verification Status
    render_verification_section(&mut lines);

    // ## Known Blockers
    render_blockers_section(&mut lines);

    // ## Next Recommended Action
    render_next_action_section(&mut lines, artifact);

    // ## Instructions for Next Assistant
    render_instructions_section(&mut lines);

    lines.join("\n")
}

// ---------------------------------------------------------------------------
// Rendering helpers
// ---------------------------------------------------------------------------

/// Renders the YAML frontmatter block.
fn render_frontmatter(lines: &mut Vec<String>, artifact: &HandoffArtifact) {
    let cs = &artifact.current_state;

    lines.extend([
        "---".to_string(),
        "title: \"Latest Handoff\"".to_string(),
        "document_type: \"ai-handoff\"".to_string(),
        "artifact_type: \"handoff\"".to_string(),
        "status: \"current\"".to_string(),
        "generated: true".to_string(),
        "reviewed: false".to_string(),
    ]);

    if let Some(active) = cs.active_epic() {
        lines.push(format!("epic: \"{}\"", active.id));
    }

    if let Some(active_wp) = artifact.active_work_packet() {
        lines.push(format!("work_packet: \"{}\"", active_wp.id));
    }

    lines.push("source_files:".to_string());
    for source in &artifact.source_files {
        lines.push(format!("  - \"{source}\""));
    }

    lines.push("---".to_string());
    lines.push(String::new());
}

/// Renders the Project section.
fn render_project_section(lines: &mut Vec<String>, cs: &CurrentStateArtifact) {
    lines.push("## Project".to_string());
    lines.push(String::new());
    lines.push(format!("{} is {}", cs.project_name, cs.project_description));
    lines.push(String::new());
}

/// Renders the Current Status section.
fn render_current_status_section(
    lines: &mut Vec<String>,
    cs: &CurrentStateArtifact,
    artifact: &HandoffArtifact,
) {
    lines.push("## Current Status".to_string());
    lines.push(String::new());

    let completed_count = cs.completed_epics().len();
    let total_epics = cs.epics.len();
    let total_packets = artifact.work_packets.len();
    let completed_packets = artifact.completed_work_packets().len();

    lines.push(format!(
        "{completed_count} of {total_epics} epics completed. \
         {completed_packets} of {total_packets} work packets completed."
    ));

    if let Some(active) = cs.active_epic() {
        lines.push(format!("Active epic: {} — {}.", active.id, active.title));
    }

    if let Some(active_wp) = artifact.active_work_packet() {
        lines.push(format!(
            "Active work packet: {} — {}.",
            active_wp.id, active_wp.title
        ));
    }

    lines.push(String::new());
}

/// Renders the Active Epic section.
fn render_active_epic_section(lines: &mut Vec<String>, cs: &CurrentStateArtifact) {
    lines.push("## Active Epic".to_string());
    lines.push(String::new());

    if let Some(active) = cs.active_epic() {
        lines.push(format!("{} — {}", active.id, active.title));
    } else {
        lines.push("None.".to_string());
    }

    lines.push(String::new());
}

/// Renders the Active Work Packet section.
fn render_active_work_packet_section(lines: &mut Vec<String>, artifact: &HandoffArtifact) {
    lines.push("## Active Work Packet".to_string());
    lines.push(String::new());

    if let Some(active_wp) = artifact.active_work_packet() {
        lines.push(format!("{} — {}", active_wp.id, active_wp.title));
    } else {
        lines.push("None.".to_string());
    }

    lines.push(String::new());
}

/// Renders the Recently Completed section.
fn render_recently_completed_section(
    lines: &mut Vec<String>,
    cs: &CurrentStateArtifact,
    artifact: &HandoffArtifact,
) {
    lines.push("## Recently Completed".to_string());
    lines.push(String::new());

    let completed_epics = cs.completed_epics();
    let completed_packets = artifact.completed_work_packets();

    if completed_epics.is_empty() && completed_packets.is_empty() {
        lines.push("None.".to_string());
    } else {
        for epic in &completed_epics {
            lines.push(format!("- {} — {} (epic complete)", epic.id, epic.title));
        }
        for wp in &completed_packets {
            lines.push(format!("- {} — {} (work packet complete)", wp.id, wp.title));
        }
    }

    lines.push(String::new());
}

/// Renders the Current Files of Interest section.
fn render_files_of_interest_section(lines: &mut Vec<String>, cs: &CurrentStateArtifact) {
    lines.push("## Current Files of Interest".to_string());
    lines.push(String::new());

    let mut files = vec![
        "monad.toml".to_string(),
        "crates/monad-core/src/lib.rs".to_string(),
    ];

    if let Some(active) = cs.active_epic() {
        files.push(format!(
            "work/epics/{}",
            epic_filename(&active.id, &active.title)
        ));
    }

    for module in &cs.runtime_modules {
        files.push(format!("crates/monad-core/src/{module}.rs"));
    }

    for file in &files {
        lines.push(format!("- `{file}`"));
    }

    lines.push(String::new());
}

/// Renders the Verification Status section.
fn render_verification_section(lines: &mut Vec<String>) {
    lines.push("## Verification Status".to_string());
    lines.push(String::new());
    lines.push("Run:".to_string());
    lines.push(String::new());
    lines.push("```bash".to_string());
    lines.push("cargo fmt --check".to_string());
    lines.push("cargo test".to_string());
    lines.push("cargo clippy --all-targets --all-features -- -D warnings".to_string());
    lines.push("```".to_string());
    lines.push(String::new());
}

/// Renders the Known Blockers section.
fn render_blockers_section(lines: &mut Vec<String>) {
    lines.push("## Known Blockers".to_string());
    lines.push(String::new());
    lines.push("No known blockers.".to_string());
    lines.push(String::new());
}

/// Renders the Next Recommended Action section.
fn render_next_action_section(lines: &mut Vec<String>, artifact: &HandoffArtifact) {
    lines.push("## Next Recommended Action".to_string());
    lines.push(String::new());

    if let Some(active_wp) = artifact.active_work_packet() {
        lines.push(format!("Continue {} — {}.", active_wp.id, active_wp.title));
    } else if let Some(next_wp) = artifact.next_pending_work_packet() {
        lines.push(format!("Begin {} — {}.", next_wp.id, next_wp.title));
    } else if let Some(active_epic) = artifact.current_state.active_epic() {
        lines.push(format!(
            "Continue epic {} — {}.",
            active_epic.id, active_epic.title
        ));
    } else {
        lines.push("No active work identified.".to_string());
    }

    lines.push(String::new());
}

/// Renders the Instructions for Next Assistant section.
fn render_instructions_section(lines: &mut Vec<String>) {
    lines.push("## Instructions for Next Assistant".to_string());
    lines.push(String::new());
    lines.push("- Use repo files as source of truth.".to_string());
    lines.push("- Prefer forward progress over perfection.".to_string());
    lines.push("- Verify before committing.".to_string());
    lines.push("- Use conventional commits.".to_string());
    lines.push("- Keep work packets atomic.".to_string());
    lines.push(String::new());
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

impl HandoffArtifact {
    /// Returns the first in-progress work packet for the active epic.
    #[must_use]
    pub fn active_work_packet(&self) -> Option<&HandoffWorkPacketEntry> {
        let active_epic = self.current_state.active_epic()?;
        self.work_packets
            .iter()
            .find(|wp| wp.epic == active_epic.id && wp.status == "in-progress")
    }

    /// Returns completed work packets.
    #[must_use]
    pub fn completed_work_packets(&self) -> Vec<&HandoffWorkPacketEntry> {
        self.work_packets
            .iter()
            .filter(|wp| wp.status == "complete")
            .collect()
    }

    /// Returns the next pending (not started) work packet.
    #[must_use]
    pub fn next_pending_work_packet(&self) -> Option<&HandoffWorkPacketEntry> {
        self.work_packets
            .iter()
            .find(|wp| wp.status != "complete" && wp.status != "in-progress")
    }
}

/// Converts an epic ID and title to a likely filename.
///
/// Example: `("E2", "Repository Intelligence Foundation")` becomes
/// `"E2-repository-intelligence-foundation.md"`.
fn epic_filename(id: &str, title: &str) -> String {
    let slug: String = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();

    // Collapse consecutive hyphens and strip leading/trailing hyphens.
    let slug = collapse_hyphens(&slug);

    format!("{id}-{slug}.md")
}

/// Collapses consecutive hyphens and strips leading/trailing hyphens.
fn collapse_hyphens(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_hyphen = false;

    for c in s.chars() {
        if c == '-' {
            if !prev_hyphen && !result.is_empty() {
                result.push('-');
            }
            prev_hyphen = true;
        } else {
            result.push(c);
            prev_hyphen = false;
        }
    }

    // Strip trailing hyphen.
    if result.ends_with('-') {
        result.pop();
    }

    result
}

// ---------------------------------------------------------------------------
// Work packet reading
// ---------------------------------------------------------------------------

/// Reads all work packets from the `work/packets/` directory tree.
///
/// Work packets are organized in subdirectories by epic
/// (e.g. `work/packets/E2/WP-E2-001-*.md`).
fn read_work_packets(packets_dir: &Path) -> MonadResult<Vec<HandoffWorkPacketEntry>> {
    if !packets_dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries: Vec<HandoffWorkPacketEntry> = Vec::new();

    // Walk subdirectories (one per epic).
    let top_entries = fs::read_dir(packets_dir).map_err(|error| {
        MonadError::internal(format!(
            "failed to read work packets directory {}: {error}",
            packets_dir.display()
        ))
    })?;

    for dir_entry in top_entries {
        let dir_entry = dir_entry.map_err(|error| {
            MonadError::internal(format!("failed to read directory entry: {error}"))
        })?;

        let path = dir_entry.path();

        if path.is_dir() {
            let sub_entries = fs::read_dir(&path).map_err(|error| {
                MonadError::internal(format!(
                    "failed to read work packet subdirectory {}: {error}",
                    path.display()
                ))
            })?;

            for file_entry in sub_entries {
                let file_entry = file_entry.map_err(|error| {
                    MonadError::internal(format!("failed to read directory entry: {error}"))
                })?;

                let file_path = file_entry.path();

                let is_wp_md = file_path.extension().is_some_and(|ext| ext == "md")
                    && file_path
                        .file_name()
                        .is_some_and(|name| name.to_string_lossy().starts_with("WP-"));

                if !is_wp_md {
                    continue;
                }

                if let Some(entry) = read_work_packet_file(&file_path)? {
                    entries.push(entry);
                }
            }
        }
    }

    // Sort by work packet ID using natural ordering.
    entries.sort_by(|a, b| natural_wp_sort(&a.id, &b.id));

    Ok(entries)
}

/// Reads a single work packet file and extracts frontmatter fields.
fn read_work_packet_file(path: &Path) -> MonadResult<Option<HandoffWorkPacketEntry>> {
    let content = fs::read_to_string(path).map_err(|error| {
        MonadError::internal(format!(
            "failed to read work packet file {}: {error}",
            path.display()
        ))
    })?;

    Ok(parse_work_packet_frontmatter(&content))
}

/// Parses YAML frontmatter from a work packet file.
///
/// Extracts `title`, `status`, `epic`, and `work_packet` fields.
fn parse_work_packet_frontmatter(content: &str) -> Option<HandoffWorkPacketEntry> {
    let mut in_frontmatter = false;
    let mut title: Option<String> = None;
    let mut status: Option<String> = None;
    let mut epic: Option<String> = None;
    let mut work_packet: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == "---" {
            if in_frontmatter {
                break;
            }
            in_frontmatter = true;
            continue;
        }

        if !in_frontmatter {
            continue;
        }

        if let Some(value) = extract_frontmatter_field(trimmed, "title:") {
            title = Some(value);
        } else if let Some(value) = extract_frontmatter_field(trimmed, "status:") {
            status = Some(value);
        } else if let Some(value) = extract_frontmatter_field(trimmed, "epic:") {
            epic = Some(value);
        } else if let Some(value) = extract_frontmatter_field(trimmed, "work_packet:") {
            work_packet = Some(value);
        }
    }

    let id = work_packet?;
    let mut title_value = title?;
    let status_value = status.unwrap_or_else(|| "unknown".to_string());
    let epic_value = epic.unwrap_or_else(|| "unknown".to_string());

    // Strip the "WP-E2-001 — " prefix from the title if present.
    title_value = strip_wp_prefix(&title_value, &id);

    Some(HandoffWorkPacketEntry {
        id,
        title: title_value,
        status: status_value,
        epic: epic_value,
    })
}

/// Extracts a value from a YAML frontmatter line.
///
/// Handles both quoted and unquoted values.
fn extract_frontmatter_field(line: &str, key: &str) -> Option<String> {
    let rest = line.strip_prefix(key)?.trim();

    if rest.is_empty() {
        return None;
    }

    // Strip surrounding quotes.
    let value = rest
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .unwrap_or(rest);

    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

/// Strips the work packet prefix from a title.
///
/// For example, `"WP-E2-001 — Establish Repository Inspection Foundation"`
/// becomes `"Establish Repository Inspection Foundation"`.
fn strip_wp_prefix(title: &str, id: &str) -> String {
    // Try "WP-E2-001 — title" (em dash).
    if let Some(rest) = title.strip_prefix(id) {
        if let Some(after_dash) = rest.strip_prefix(" — ") {
            return after_dash.trim().to_string();
        }
        if let Some(after_dash) = rest.strip_prefix(" - ") {
            return after_dash.trim().to_string();
        }
    }

    title.to_string()
}

/// Natural sort for work packet IDs.
///
/// Sorts by epic prefix first, then by numeric suffix.
/// Example: `WP-E2-001` < `WP-E2-002` < `WP-E2-010`.
fn natural_wp_sort(a: &str, b: &str) -> std::cmp::Ordering {
    // Extract numeric suffix after last hyphen.
    let a_parts: Vec<&str> = a.split('-').collect();
    let b_parts: Vec<&str> = b.split('-').collect();

    // Compare part by part.
    for (ap, bp) in a_parts.iter().zip(b_parts.iter()) {
        // Try numeric comparison.
        match (ap.parse::<u32>(), bp.parse::<u32>()) {
            (Ok(an), Ok(bn)) => {
                let cmp = an.cmp(&bn);
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
            _ => {
                let cmp = ap.cmp(bp);
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
        }
    }

    a_parts.len().cmp(&b_parts.len())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::current_state::CurrentStateEpicEntry;

    #[test]
    fn extract_frontmatter_field_handles_quoted_values() {
        let result = extract_frontmatter_field("title: \"My Title\"", "title:");
        assert_eq!(result, Some("My Title".to_string()));
    }

    #[test]
    fn extract_frontmatter_field_handles_unquoted_values() {
        let result = extract_frontmatter_field("status: in-progress", "status:");
        assert_eq!(result, Some("in-progress".to_string()));
    }

    #[test]
    fn extract_frontmatter_field_returns_none_for_wrong_key() {
        let result = extract_frontmatter_field("title: \"My Title\"", "status:");
        assert_eq!(result, None);
    }

    #[test]
    fn extract_frontmatter_field_returns_none_for_empty_value() {
        let result = extract_frontmatter_field("title:", "title:");
        assert_eq!(result, None);
    }

    #[test]
    fn parse_work_packet_frontmatter_extracts_fields() {
        let content = r#"---
title: "WP-E2-001 — Establish Repository Inspection Foundation"
document_type: "work-packet"
status: "in-progress"
epic: "E2"
work_packet: "WP-E2-001"
---

# WP-E2-001
"#;
        let entry = parse_work_packet_frontmatter(content).expect("should parse");
        assert_eq!(entry.id, "WP-E2-001");
        assert_eq!(entry.title, "Establish Repository Inspection Foundation");
        assert_eq!(entry.status, "in-progress");
        assert_eq!(entry.epic, "E2");
    }

    #[test]
    fn parse_work_packet_frontmatter_handles_complete_status() {
        let content = r#"---
title: "WP-E0-001 — Repository Foundation"
status: "complete"
epic: "E0"
work_packet: "WP-E0-001"
---
"#;
        let entry = parse_work_packet_frontmatter(content).expect("should parse");
        assert_eq!(entry.id, "WP-E0-001");
        assert_eq!(entry.status, "complete");
    }

    #[test]
    fn parse_work_packet_frontmatter_returns_none_without_id() {
        let content = r#"---
title: "Some work"
status: "in-progress"
---
"#;
        assert!(parse_work_packet_frontmatter(content).is_none());
    }

    #[test]
    fn strip_wp_prefix_removes_em_dash() {
        let result = strip_wp_prefix("WP-E2-001 — My Title", "WP-E2-001");
        assert_eq!(result, "My Title");
    }

    #[test]
    fn strip_wp_prefix_removes_hyphen() {
        let result = strip_wp_prefix("WP-E2-001 - My Title", "WP-E2-001");
        assert_eq!(result, "My Title");
    }

    #[test]
    fn strip_wp_prefix_preserves_title_without_prefix() {
        let result = strip_wp_prefix("My Title", "WP-E2-001");
        assert_eq!(result, "My Title");
    }

    #[test]
    fn natural_wp_sort_orders_correctly() {
        let mut ids = vec![
            "WP-E2-010".to_string(),
            "WP-E2-001".to_string(),
            "WP-E1-003".to_string(),
            "WP-E2-002".to_string(),
        ];
        ids.sort_by(|a, b| natural_wp_sort(a, b));
        assert_eq!(
            ids,
            vec!["WP-E1-003", "WP-E2-001", "WP-E2-002", "WP-E2-010"]
        );
    }

    #[test]
    fn epic_filename_generates_slug() {
        let result = epic_filename("E2", "Repository Intelligence Foundation");
        assert_eq!(result, "E2-repository-intelligence-foundation.md");
    }

    #[test]
    fn collapse_hyphens_collapses_multiple() {
        assert_eq!(collapse_hyphens("foo---bar"), "foo-bar");
        assert_eq!(collapse_hyphens("--foo--"), "foo");
        assert_eq!(collapse_hyphens("hello"), "hello");
    }

    #[test]
    fn handoff_artifact_finds_active_work_packet() {
        let artifact = HandoffArtifact {
            current_state: CurrentStateArtifact {
                project_name: "Monad".to_string(),
                project_description: "Test".to_string(),
                epics: vec![CurrentStateEpicEntry {
                    id: "E2".to_string(),
                    title: "Repository Intelligence".to_string(),
                    status: "in-progress".to_string(),
                }],
                runtime_modules: vec![],
                source_files: vec![],
            },
            work_packets: vec![
                HandoffWorkPacketEntry {
                    id: "WP-E2-001".to_string(),
                    title: "First".to_string(),
                    status: "complete".to_string(),
                    epic: "E2".to_string(),
                },
                HandoffWorkPacketEntry {
                    id: "WP-E2-002".to_string(),
                    title: "Second".to_string(),
                    status: "in-progress".to_string(),
                    epic: "E2".to_string(),
                },
            ],
            source_files: vec![],
        };

        let active = artifact.active_work_packet().expect("should find active");
        assert_eq!(active.id, "WP-E2-002");
    }

    #[test]
    fn handoff_artifact_returns_none_when_no_active_work_packet() {
        let artifact = HandoffArtifact {
            current_state: CurrentStateArtifact {
                project_name: "Monad".to_string(),
                project_description: "Test".to_string(),
                epics: vec![],
                runtime_modules: vec![],
                source_files: vec![],
            },
            work_packets: vec![],
            source_files: vec![],
        };

        assert!(artifact.active_work_packet().is_none());
    }

    #[test]
    fn render_handoff_includes_required_sections() {
        let artifact = HandoffArtifact {
            current_state: CurrentStateArtifact {
                project_name: "Monad".to_string(),
                project_description: "AI-native Software Foundry OS.".to_string(),
                epics: vec![
                    CurrentStateEpicEntry {
                        id: "E0".to_string(),
                        title: "Project Foundation".to_string(),
                        status: "complete".to_string(),
                    },
                    CurrentStateEpicEntry {
                        id: "E1".to_string(),
                        title: "Runtime Foundation".to_string(),
                        status: "complete".to_string(),
                    },
                    CurrentStateEpicEntry {
                        id: "E2".to_string(),
                        title: "Repository Intelligence".to_string(),
                        status: "in-progress".to_string(),
                    },
                ],
                runtime_modules: vec!["workspace".to_string(), "checks".to_string()],
                source_files: vec!["monad.toml".to_string()],
            },
            work_packets: vec![
                HandoffWorkPacketEntry {
                    id: "WP-E2-001".to_string(),
                    title: "Inspection Foundation".to_string(),
                    status: "complete".to_string(),
                    epic: "E2".to_string(),
                },
                HandoffWorkPacketEntry {
                    id: "WP-E2-002".to_string(),
                    title: "Inspect Command".to_string(),
                    status: "in-progress".to_string(),
                    epic: "E2".to_string(),
                },
            ],
            source_files: vec!["monad.toml".to_string(), "work/packets/".to_string()],
        };

        let rendered = render_handoff_markdown(&artifact);

        // Check frontmatter.
        assert!(rendered.contains("title: \"Latest Handoff\""));
        assert!(rendered.contains("document_type: \"ai-handoff\""));
        assert!(rendered.contains("artifact_type: \"handoff\""));
        assert!(rendered.contains("generated: true"));
        assert!(rendered.contains("epic: \"E2\""));
        assert!(rendered.contains("work_packet: \"WP-E2-002\""));

        // Check required sections.
        assert!(rendered.contains("# Latest Handoff"));
        assert!(rendered.contains("## Project"));
        assert!(rendered.contains("## Current Status"));
        assert!(rendered.contains("## Active Epic"));
        assert!(rendered.contains("## Active Work Packet"));
        assert!(rendered.contains("## Recently Completed"));
        assert!(rendered.contains("## Current Files of Interest"));
        assert!(rendered.contains("## Verification Status"));
        assert!(rendered.contains("## Known Blockers"));
        assert!(rendered.contains("## Next Recommended Action"));
        assert!(rendered.contains("## Instructions for Next Assistant"));

        // Check content.
        assert!(rendered.contains("Monad is AI-native Software Foundry OS."));
        assert!(rendered.contains("E2 — Repository Intelligence"));
        assert!(rendered.contains("WP-E2-002 — Inspect Command"));
        assert!(rendered.contains("E0 — Project Foundation (epic complete)"));
        assert!(rendered.contains("WP-E2-001 — Inspection Foundation (work packet complete)"));
        assert!(rendered.contains("Continue WP-E2-002 — Inspect Command."));
    }

    #[test]
    fn render_handoff_handles_no_active_work() {
        let artifact = HandoffArtifact {
            current_state: CurrentStateArtifact {
                project_name: "Monad".to_string(),
                project_description: "Test.".to_string(),
                epics: vec![],
                runtime_modules: vec![],
                source_files: vec![],
            },
            work_packets: vec![],
            source_files: vec![],
        };

        let rendered = render_handoff_markdown(&artifact);

        assert!(rendered.contains("## Active Epic"));
        assert!(rendered.contains("None."));
        assert!(rendered.contains("## Active Work Packet"));
        assert!(rendered.contains("No active work identified."));
    }

    #[test]
    fn render_handoff_is_deterministic() {
        let artifact = HandoffArtifact {
            current_state: CurrentStateArtifact {
                project_name: "Monad".to_string(),
                project_description: "Test.".to_string(),
                epics: vec![CurrentStateEpicEntry {
                    id: "E0".to_string(),
                    title: "Foundation".to_string(),
                    status: "complete".to_string(),
                }],
                runtime_modules: vec!["workspace".to_string()],
                source_files: vec!["monad.toml".to_string()],
            },
            work_packets: vec![],
            source_files: vec!["monad.toml".to_string()],
        };

        let first = render_handoff_markdown(&artifact);
        let second = render_handoff_markdown(&artifact);

        assert_eq!(first, second, "handoff rendering must be deterministic");
    }

    #[test]
    fn render_handoff_next_action_suggests_pending_when_none_active() {
        let artifact = HandoffArtifact {
            current_state: CurrentStateArtifact {
                project_name: "Monad".to_string(),
                project_description: "Test.".to_string(),
                epics: vec![CurrentStateEpicEntry {
                    id: "E2".to_string(),
                    title: "Repo Intelligence".to_string(),
                    status: "in-progress".to_string(),
                }],
                runtime_modules: vec![],
                source_files: vec![],
            },
            work_packets: vec![
                HandoffWorkPacketEntry {
                    id: "WP-E2-001".to_string(),
                    title: "First".to_string(),
                    status: "complete".to_string(),
                    epic: "E2".to_string(),
                },
                HandoffWorkPacketEntry {
                    id: "WP-E2-002".to_string(),
                    title: "Second".to_string(),
                    status: "draft".to_string(),
                    epic: "E2".to_string(),
                },
            ],
            source_files: vec![],
        };

        let rendered = render_handoff_markdown(&artifact);
        assert!(rendered.contains("Begin WP-E2-002 — Second."));
    }

    #[test]
    fn generate_handoff_from_workspace() {
        let workspace_root = {
            let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
            manifest_dir
                .ancestors()
                .find(|p| p.join("monad.toml").exists())
                .expect("should find workspace root")
                .to_path_buf()
        };

        let context =
            WorkspaceContext::discover_from(&workspace_root).expect("should discover workspace");
        let artifact = generate_handoff(&context).expect("should generate handoff");

        // The artifact should contain project info.
        assert_eq!(artifact.current_state.project_name, "Monad");

        // Should have epics.
        assert!(!artifact.current_state.epics.is_empty());

        // Should have source files.
        assert!(artifact.source_files.contains(&"monad.toml".to_string()));
        assert!(artifact.source_files.contains(&"work/packets/".to_string()));

        // The rendered output should be non-empty and contain key sections.
        let rendered = render_handoff_markdown(&artifact);
        assert!(rendered.contains("# Latest Handoff"));
        assert!(rendered.contains("## Project"));
        assert!(rendered.contains("Monad"));
    }
}
