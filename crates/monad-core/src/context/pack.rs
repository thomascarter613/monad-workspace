//! Context pack assembler for Monad.
//!
//! A context pack is a compact, structured bundle of project context that
//! helps a human or AI assistant understand the repository and continue work.
//!
//! The assembler gathers key files from the repository and combines them into
//! a single, deterministic Markdown document at
//! `.monad/context/latest-context-pack.md`.
//!
//! Files are included in a defined order following the Context Pack Standard
//! (`docs/08-context/CONTEXT-PACK-STANDARD.md`).

use std::fs;
use std::path::Path;

use crate::context::current_state::{CurrentStateArtifact, generate_current_state};
use crate::context::handoff::{HandoffArtifact, generate_handoff};
use crate::{MonadError, MonadResult, WorkspaceContext};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// An assembled context pack ready for rendering.
///
/// Holds the structured project context gathered from repository files
/// plus pre-generated current-state and handoff data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextPackArtifact {
    /// Project display name from `monad.toml`.
    pub project_name: String,

    /// Project description from `monad.toml`.
    pub project_description: String,

    /// Current-state data (epics, runtime modules).
    pub current_state: CurrentStateArtifact,

    /// Handoff data (work packets, active work).
    pub handoff: HandoffArtifact,

    /// Accepted ADR summaries (title lines from ADR files).
    pub accepted_decisions: Vec<String>,

    /// Key document paths included in the pack.
    pub important_documents: Vec<String>,

    /// Architecture summary extracted from the repo.
    pub architecture_summary: String,

    /// Workflow summary extracted from the repo.
    pub workflow_summary: String,

    /// Files read as input to create this artifact.
    pub source_files: Vec<String>,
}

// ---------------------------------------------------------------------------
// Default file order
// ---------------------------------------------------------------------------

/// Returns the default ordered list of files to attempt to include.
///
/// Files are attempted in this order. Missing files are skipped gracefully.
fn default_file_order() -> Vec<&'static str> {
    vec![
        "monad.toml",
        "README.md",
        "docs/01-project/01-charter/PRODUCT-CHARTER.md",
        "docs/05-architecture/SYSTEM-OVERVIEW.md",
        "docs/07-workflow/OPERATING-MODEL.md",
        "docs/08-context/CONTEXT-BRIDGE.md",
        ".monad/context/current-state.md",
        ".monad/context/latest-handoff.md",
    ]
}

/// Returns the list of ADR files in sorted order, excluding the template.
fn read_adr_summaries(root: &Path) -> MonadResult<Vec<String>> {
    let adr_dir = root.join("docs/06-adrs");

    if !adr_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut summaries: Vec<String> = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(&adr_dir)
        .map_err(|error| MonadError::internal(format!("failed to read ADR directory: {error}")))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            name.starts_with("ADR-") && name.ends_with(".md") && !name.contains("template")
        })
        .collect();

    entries.sort_by_key(|entry| entry.file_name());

    for entry in &entries {
        let path = entry.path();
        let content = fs::read_to_string(&path).unwrap_or_default();

        // Extract title from frontmatter or first heading.
        let title = extract_adr_title(&content)
            .unwrap_or_else(|| entry.file_name().to_string_lossy().to_string());

        summaries.push(title);
    }

    Ok(summaries)
}

/// Extracts the title from an ADR file.
///
/// Looks for a `title:` field in YAML frontmatter first, then falls back
/// to the first `#` heading.
fn extract_adr_title(content: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();

    // Try frontmatter title field.
    if lines.first().is_some_and(|line| line.trim() == "---") {
        for line in lines.iter().skip(1) {
            let trimmed = line.trim();
            if trimmed == "---" {
                break;
            }
            if let Some(rest) = trimmed.strip_prefix("title:") {
                let value = rest.trim().trim_matches('"').trim_matches('\'');
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }

    // Fall back to first heading.
    for line in &lines {
        let trimmed = line.trim();
        if let Some(heading) = trimmed.strip_prefix("# ") {
            return Some(heading.trim().to_string());
        }
    }

    None
}

/// Reads a short summary from a document file.
///
/// Extracts the first non-empty, non-frontmatter paragraph as a summary.
/// Returns an empty string if the file does not exist.
fn read_document_summary(root: &Path, relative_path: &str) -> String {
    let path = root.join(relative_path);

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return String::new(),
    };

    extract_first_paragraph(&content)
}

/// Extracts the first substantive paragraph from Markdown content.
///
/// Skips frontmatter, headings, and blank lines.
fn extract_first_paragraph(content: &str) -> String {
    let mut in_frontmatter = false;
    let mut after_frontmatter = false;
    let mut paragraph_lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Handle frontmatter.
        if trimmed == "---" && !after_frontmatter {
            in_frontmatter = !in_frontmatter;
            if !in_frontmatter {
                after_frontmatter = true;
            }
            continue;
        }

        if in_frontmatter {
            continue;
        }

        // Skip headings.
        if trimmed.starts_with('#') {
            // If we already have paragraph content, this heading ends it.
            if !paragraph_lines.is_empty() {
                break;
            }
            continue;
        }

        // Skip blank lines before content starts.
        if trimmed.is_empty() {
            if !paragraph_lines.is_empty() {
                break;
            }
            continue;
        }

        paragraph_lines.push(trimmed);
    }

    paragraph_lines.join(" ")
}

// ---------------------------------------------------------------------------
// Generation
// ---------------------------------------------------------------------------

/// Generates a context pack artifact from repository state.
///
/// Reads project identity, current state, handoff data, ADRs, and key
/// documents. Returns a structured artifact ready for Markdown rendering.
pub fn generate_context_pack(context: &WorkspaceContext) -> MonadResult<ContextPackArtifact> {
    let root = context.root();
    let mut source_files: Vec<String> = Vec::new();

    // 1. Generate current state (reuses existing generator).
    let current_state = generate_current_state(context)?;
    source_files.extend(current_state.source_files.clone());

    // 2. Generate handoff (reuses existing generator).
    let handoff = generate_handoff(context)?;
    for file in &handoff.source_files {
        if !source_files.contains(file) {
            source_files.push(file.clone());
        }
    }

    // 3. Read ADR summaries.
    let accepted_decisions = read_adr_summaries(root)?;
    if root.join("docs/06-adrs").is_dir() {
        let adr_source = "docs/06-adrs/".to_string();
        if !source_files.contains(&adr_source) {
            source_files.push(adr_source);
        }
    }

    // 4. Identify important documents (from default file order).
    let important_documents: Vec<String> = default_file_order()
        .iter()
        .filter(|path| root.join(path).exists())
        .map(|path| (*path).to_string())
        .collect();

    // Track which default files were found.
    for path in &important_documents {
        if !source_files.contains(path) {
            source_files.push(path.clone());
        }
    }

    // 5. Read architecture summary.
    let architecture_summary =
        read_document_summary(root, "docs/05-architecture/SYSTEM-OVERVIEW.md");
    if !architecture_summary.is_empty() {
        let arch_source = "docs/05-architecture/SYSTEM-OVERVIEW.md".to_string();
        if !source_files.contains(&arch_source) {
            source_files.push(arch_source);
        }
    }

    // 6. Read workflow summary.
    let workflow_summary = read_document_summary(root, "docs/07-workflow/OPERATING-MODEL.md");
    if !workflow_summary.is_empty() {
        let wf_source = "docs/07-workflow/OPERATING-MODEL.md".to_string();
        if !source_files.contains(&wf_source) {
            source_files.push(wf_source);
        }
    }

    source_files.sort();
    source_files.dedup();

    let project_name = current_state.project_name.clone();
    let project_description = current_state.project_description.clone();

    Ok(ContextPackArtifact {
        project_name,
        project_description,
        current_state,
        handoff,
        accepted_decisions,
        important_documents,
        architecture_summary,
        workflow_summary,
        source_files,
    })
}

// ---------------------------------------------------------------------------
// Writing
// ---------------------------------------------------------------------------

/// Writes a context pack artifact to `.monad/context/latest-context-pack.md`.
///
/// Creates the `.monad/context/` directory if it does not exist.
pub fn write_context_pack_artifact(
    context: &WorkspaceContext,
    artifact: &ContextPackArtifact,
) -> MonadResult<std::path::PathBuf> {
    let context_dir = context.root().join(".monad/context");

    fs::create_dir_all(&context_dir).map_err(|error| {
        MonadError::internal(format!(
            "failed to create .monad/context directory: {error}"
        ))
    })?;

    let output_path = context_dir.join("latest-context-pack.md");
    let markdown = render_context_pack_markdown(artifact);

    fs::write(&output_path, markdown).map_err(|error| {
        MonadError::internal(format!(
            "failed to write context pack to {}: {error}",
            output_path.display()
        ))
    })?;

    Ok(output_path)
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/// Renders a context pack artifact to deterministic Markdown.
pub fn render_context_pack_markdown(artifact: &ContextPackArtifact) -> String {
    let sections: Vec<String> = vec![
        render_frontmatter(artifact),
        render_heading(),
        render_project_identity(artifact),
        render_current_status(artifact),
        render_active_work(artifact),
        render_accepted_decisions(artifact),
        render_important_documents(artifact),
        render_architecture_summary(artifact),
        render_workflow_summary(artifact),
        render_verification_summary(),
        render_risks_and_blockers(artifact),
        render_next_recommended_action(artifact),
        render_source_files(artifact),
        render_trust_notes(),
    ];

    sections.join("\n")
}

fn render_frontmatter(artifact: &ContextPackArtifact) -> String {
    let mut lines = Vec::new();
    lines.push("---".to_string());
    lines.push("title: \"Latest Context Pack\"".to_string());
    lines.push("document_type: \"context-pack\"".to_string());
    lines.push("artifact_type: \"context-pack\"".to_string());
    lines.push("status: \"current\"".to_string());
    lines.push("generated: true".to_string());
    lines.push("reviewed: false".to_string());
    lines.push(format!("project: \"{}\"", artifact.project_name));
    lines.push("source: \"repository\"".to_string());

    // Source files list in frontmatter.
    lines.push("source_files:".to_string());
    for file in &artifact.source_files {
        lines.push(format!("  - \"{}\"", file));
    }

    lines.push("---".to_string());
    lines.push(String::new());
    lines.join("\n")
}

fn render_heading() -> String {
    "# Latest Context Pack\n".to_string()
}

fn render_project_identity(artifact: &ContextPackArtifact) -> String {
    let mut lines = Vec::new();
    lines.push("## Project Identity".to_string());
    lines.push(String::new());
    lines.push(format!(
        "{} is {}",
        artifact.project_name, artifact.project_description
    ));
    lines.push(String::new());
    lines.join("\n")
}

fn render_current_status(artifact: &ContextPackArtifact) -> String {
    let mut lines = Vec::new();
    lines.push("## Current Status".to_string());
    lines.push(String::new());

    let total_epics = artifact.current_state.epics.len();
    let completed_epics = artifact
        .current_state
        .epics
        .iter()
        .filter(|e| e.status == "complete")
        .count();

    lines.push(format!(
        "{} of {} epics completed.",
        completed_epics, total_epics
    ));

    // Find active epic.
    if let Some(active) = artifact
        .current_state
        .epics
        .iter()
        .find(|e| e.status != "complete")
    {
        lines.push(format!("Active epic: {} — {}.", active.id, active.title));
    }

    // Work packet summary from handoff.
    let total_wp = artifact.handoff.work_packets.len();
    let completed_wp = artifact.handoff.completed_work_packets().len();
    lines.push(format!(
        "{} of {} work packets completed.",
        completed_wp, total_wp
    ));

    lines.push(String::new());
    lines.join("\n")
}

fn render_active_work(artifact: &ContextPackArtifact) -> String {
    let mut lines = Vec::new();
    lines.push("## Active Work".to_string());
    lines.push(String::new());

    // Active epic.
    if let Some(active_epic) = artifact
        .current_state
        .epics
        .iter()
        .find(|e| e.status != "complete")
    {
        lines.push(format!(
            "- Active epic: {} — {}",
            active_epic.id, active_epic.title
        ));
    }

    // Active work packet from handoff.
    if let Some(active_wp) = artifact.handoff.active_work_packet() {
        lines.push(format!(
            "- Active work packet: {} — {}",
            active_wp.id, active_wp.title
        ));
    }

    // Next pending work packet.
    if let Some(next_wp) = artifact.handoff.next_pending_work_packet() {
        lines.push(format!(
            "- Next pending work packet: {} — {}",
            next_wp.id, next_wp.title
        ));
    }

    if lines.len() == 2 {
        lines.push("No active work recorded.".to_string());
    }

    lines.push(String::new());
    lines.join("\n")
}

fn render_accepted_decisions(artifact: &ContextPackArtifact) -> String {
    let mut lines = Vec::new();
    lines.push("## Accepted Decisions".to_string());
    lines.push(String::new());

    if artifact.accepted_decisions.is_empty() {
        lines.push("No accepted ADRs found.".to_string());
    } else {
        for decision in &artifact.accepted_decisions {
            lines.push(format!("- {decision}"));
        }
    }

    lines.push(String::new());
    lines.join("\n")
}

fn render_important_documents(artifact: &ContextPackArtifact) -> String {
    let mut lines = Vec::new();
    lines.push("## Important Documents".to_string());
    lines.push(String::new());

    if artifact.important_documents.is_empty() {
        lines.push("No important documents identified.".to_string());
    } else {
        for doc in &artifact.important_documents {
            lines.push(format!("- `{doc}`"));
        }
    }

    lines.push(String::new());
    lines.join("\n")
}

fn render_architecture_summary(artifact: &ContextPackArtifact) -> String {
    let mut lines = Vec::new();
    lines.push("## Architecture Summary".to_string());
    lines.push(String::new());

    if artifact.architecture_summary.is_empty() {
        lines.push(
            "No architecture summary available. See `docs/05-architecture/` for details."
                .to_string(),
        );
    } else {
        lines.push(artifact.architecture_summary.clone());
    }

    lines.push(String::new());
    lines.join("\n")
}

fn render_workflow_summary(artifact: &ContextPackArtifact) -> String {
    let mut lines = Vec::new();
    lines.push("## Workflow Summary".to_string());
    lines.push(String::new());

    if artifact.workflow_summary.is_empty() {
        lines.push(
            "No workflow summary available. See `docs/07-workflow/` for details.".to_string(),
        );
    } else {
        lines.push(artifact.workflow_summary.clone());
    }

    lines.push(String::new());
    lines.join("\n")
}

fn render_verification_summary() -> String {
    [
        "## Verification Summary",
        "",
        "Run:",
        "",
        "```bash",
        "cargo fmt --check",
        "cargo test",
        "cargo clippy --all-targets --all-features -- -D warnings",
        "```",
        "",
    ]
    .join("\n")
}

fn render_risks_and_blockers(artifact: &ContextPackArtifact) -> String {
    let mut lines = Vec::new();
    lines.push("## Risks and Blockers".to_string());
    lines.push(String::new());

    // Check for any in-progress work packets with blockers.
    // For now, report a default message.
    let _ = artifact; // Avoid unused warning; future versions may check artifact.
    lines.push("No known blockers.".to_string());

    lines.push(String::new());
    lines.join("\n")
}

fn render_next_recommended_action(artifact: &ContextPackArtifact) -> String {
    let mut lines = Vec::new();
    lines.push("## Next Recommended Action".to_string());
    lines.push(String::new());

    if let Some(active_wp) = artifact.handoff.active_work_packet() {
        lines.push(format!("Continue {} — {}.", active_wp.id, active_wp.title));
    } else if let Some(next_wp) = artifact.handoff.next_pending_work_packet() {
        lines.push(format!("Begin {} — {}.", next_wp.id, next_wp.title));
    } else {
        lines.push("All known work packets complete. Review epics for next steps.".to_string());
    }

    lines.push(String::new());
    lines.join("\n")
}

fn render_source_files(artifact: &ContextPackArtifact) -> String {
    let mut lines = Vec::new();
    lines.push("## Source Files Used".to_string());
    lines.push(String::new());

    if artifact.source_files.is_empty() {
        lines.push("No source files recorded.".to_string());
    } else {
        for file in &artifact.source_files {
            lines.push(format!("- `{file}`"));
        }
    }

    lines.push(String::new());
    lines.join("\n")
}

fn render_trust_notes() -> String {
    [
        "## Trust Notes",
        "",
        "- This context pack is *generated*, not human-authored.",
        "- It has *not* been reviewed.",
        "- All data comes from repository files.",
        "- Verify critical facts before acting on them.",
        "",
    ]
    .join("\n")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::current_state::CurrentStateEpicEntry;
    use crate::context::handoff::HandoffWorkPacketEntry;

    fn sample_current_state() -> CurrentStateArtifact {
        CurrentStateArtifact {
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
                    status: "in-progress".to_string(),
                },
            ],
            runtime_modules: vec!["context".to_string(), "workspace".to_string()],
            source_files: vec!["monad.toml".to_string()],
        }
    }

    fn sample_handoff() -> HandoffArtifact {
        HandoffArtifact {
            current_state: sample_current_state(),
            work_packets: vec![
                HandoffWorkPacketEntry {
                    id: "WP-E1-001".to_string(),
                    title: "Establish runtime".to_string(),
                    status: "complete".to_string(),
                    epic: "E1".to_string(),
                },
                HandoffWorkPacketEntry {
                    id: "WP-E1-002".to_string(),
                    title: "Add error handling".to_string(),
                    status: "in-progress".to_string(),
                    epic: "E1".to_string(),
                },
                HandoffWorkPacketEntry {
                    id: "WP-E1-003".to_string(),
                    title: "Add workspace context".to_string(),
                    status: "pending".to_string(),
                    epic: "E1".to_string(),
                },
            ],
            source_files: vec!["monad.toml".to_string(), "work/packets/".to_string()],
        }
    }

    fn sample_artifact() -> ContextPackArtifact {
        ContextPackArtifact {
            project_name: "Monad".to_string(),
            project_description: "AI-native Software Foundry OS.".to_string(),
            current_state: sample_current_state(),
            handoff: sample_handoff(),
            accepted_decisions: vec![
                "ADR-0001: Use Rust for Core Runtime".to_string(),
                "ADR-0002: Use Monad as Unified Product Name".to_string(),
            ],
            important_documents: vec!["monad.toml".to_string(), "README.md".to_string()],
            architecture_summary: "Monad starts with monad-cli and monad-core.".to_string(),
            workflow_summary: "Monad uses epics and work packets.".to_string(),
            source_files: vec![
                "monad.toml".to_string(),
                "README.md".to_string(),
                "docs/06-adrs/".to_string(),
            ],
        }
    }

    #[test]
    fn render_context_pack_includes_project_identity() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("## Project Identity"));
        assert!(markdown.contains("Monad is AI-native Software Foundry OS."));
    }

    #[test]
    fn render_context_pack_includes_current_status() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("## Current Status"));
        assert!(markdown.contains("1 of 2 epics completed."));
        assert!(markdown.contains("Active epic: E1"));
    }

    #[test]
    fn render_context_pack_includes_active_work() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("## Active Work"));
        assert!(markdown.contains("Active work packet: WP-E1-002"));
    }

    #[test]
    fn render_context_pack_includes_accepted_decisions() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("## Accepted Decisions"));
        assert!(markdown.contains("ADR-0001: Use Rust for Core Runtime"));
        assert!(markdown.contains("ADR-0002: Use Monad as Unified Product Name"));
    }

    #[test]
    fn render_context_pack_includes_important_documents() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("## Important Documents"));
        assert!(markdown.contains("`monad.toml`"));
        assert!(markdown.contains("`README.md`"));
    }

    #[test]
    fn render_context_pack_includes_architecture_summary() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("## Architecture Summary"));
        assert!(markdown.contains("monad-cli and monad-core"));
    }

    #[test]
    fn render_context_pack_includes_workflow_summary() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("## Workflow Summary"));
        assert!(markdown.contains("epics and work packets"));
    }

    #[test]
    fn render_context_pack_includes_verification_summary() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("## Verification Summary"));
        assert!(markdown.contains("cargo fmt --check"));
        assert!(markdown.contains("cargo test"));
    }

    #[test]
    fn render_context_pack_includes_risks_and_blockers() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("## Risks and Blockers"));
        assert!(markdown.contains("No known blockers."));
    }

    #[test]
    fn render_context_pack_includes_next_recommended_action() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("## Next Recommended Action"));
        assert!(markdown.contains("Continue WP-E1-002"));
    }

    #[test]
    fn render_context_pack_includes_source_files() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("## Source Files Used"));
        assert!(markdown.contains("`monad.toml`"));
    }

    #[test]
    fn render_context_pack_includes_trust_notes() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("## Trust Notes"));
        assert!(markdown.contains("generated"));
        assert!(markdown.contains("*not* been reviewed"));
    }

    #[test]
    fn render_context_pack_includes_frontmatter() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.starts_with("---\n"));
        assert!(markdown.contains("artifact_type: \"context-pack\""));
        assert!(markdown.contains("generated: true"));
        assert!(markdown.contains("reviewed: false"));
    }

    #[test]
    fn render_context_pack_is_deterministic() {
        let artifact = sample_artifact();
        let first = render_context_pack_markdown(&artifact);
        let second = render_context_pack_markdown(&artifact);

        assert_eq!(first, second);
    }

    #[test]
    fn render_context_pack_next_action_when_all_complete() {
        let mut artifact = sample_artifact();
        for wp in &mut artifact.handoff.work_packets {
            wp.status = "complete".to_string();
        }
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("All known work packets complete."));
    }

    #[test]
    fn render_context_pack_next_action_when_none_active() {
        let mut artifact = sample_artifact();
        artifact.handoff.work_packets = vec![HandoffWorkPacketEntry {
            id: "WP-E1-001".to_string(),
            title: "First packet".to_string(),
            status: "pending".to_string(),
            epic: "E1".to_string(),
        }];
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("Begin WP-E1-001"));
    }

    #[test]
    fn render_context_pack_empty_decisions() {
        let mut artifact = sample_artifact();
        artifact.accepted_decisions = Vec::new();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("No accepted ADRs found."));
    }

    #[test]
    fn render_context_pack_empty_architecture_summary() {
        let mut artifact = sample_artifact();
        artifact.architecture_summary = String::new();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("No architecture summary available."));
    }

    #[test]
    fn render_context_pack_empty_workflow_summary() {
        let mut artifact = sample_artifact();
        artifact.workflow_summary = String::new();
        let markdown = render_context_pack_markdown(&artifact);

        assert!(markdown.contains("No workflow summary available."));
    }

    #[test]
    fn extract_adr_title_from_frontmatter() {
        let content =
            "---\ntitle: \"Use Rust for Core Runtime\"\nstatus: accepted\n---\n\n# ADR-0001\n";
        assert_eq!(
            extract_adr_title(content),
            Some("Use Rust for Core Runtime".to_string())
        );
    }

    #[test]
    fn extract_adr_title_from_heading() {
        let content = "# ADR-0001: Use Rust for Core Runtime\n\nSome body text.\n";
        assert_eq!(
            extract_adr_title(content),
            Some("ADR-0001: Use Rust for Core Runtime".to_string())
        );
    }

    #[test]
    fn extract_adr_title_returns_none_for_empty() {
        assert_eq!(extract_adr_title(""), None);
    }

    #[test]
    fn extract_first_paragraph_skips_frontmatter() {
        let content = "---\ntitle: \"Test\"\n---\n\n# Heading\n\nFirst paragraph here.\n\nSecond paragraph.\n";
        assert_eq!(extract_first_paragraph(content), "First paragraph here.");
    }

    #[test]
    fn extract_first_paragraph_handles_no_frontmatter() {
        let content = "# Heading\n\nParagraph text.\n";
        assert_eq!(extract_first_paragraph(content), "Paragraph text.");
    }

    #[test]
    fn extract_first_paragraph_joins_multi_line() {
        let content = "# H\n\nLine one.\nLine two.\n\nAnother paragraph.\n";
        assert_eq!(extract_first_paragraph(content), "Line one. Line two.");
    }

    #[test]
    fn extract_first_paragraph_returns_empty_for_empty_content() {
        assert_eq!(extract_first_paragraph(""), "");
    }

    #[test]
    fn default_file_order_is_stable() {
        let first = default_file_order();
        let second = default_file_order();

        assert_eq!(first, second);
        assert!(!first.is_empty());
        assert_eq!(first[0], "monad.toml");
    }

    #[test]
    fn context_pack_all_required_sections_present() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        let required_sections = [
            "## Project Identity",
            "## Current Status",
            "## Active Work",
            "## Accepted Decisions",
            "## Important Documents",
            "## Architecture Summary",
            "## Workflow Summary",
            "## Verification Summary",
            "## Risks and Blockers",
            "## Next Recommended Action",
            "## Source Files Used",
            "## Trust Notes",
        ];

        for section in &required_sections {
            assert!(
                markdown.contains(section),
                "Missing required section: {section}"
            );
        }
    }

    #[test]
    fn context_pack_work_packet_counts_are_correct() {
        let artifact = sample_artifact();
        let markdown = render_context_pack_markdown(&artifact);

        // 1 complete out of 3 total.
        assert!(markdown.contains("1 of 3 work packets completed."));
    }

    #[test]
    fn generate_context_pack_from_workspace_produces_artifact() {
        // Navigate from the crate manifest directory up to the workspace root.
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = {
            manifest_dir
                .ancestors()
                .find(|p| p.join("monad.toml").exists())
                .expect("should find workspace root")
                .to_path_buf()
        };

        let context =
            WorkspaceContext::discover_from(&workspace_root).expect("should discover workspace");
        let result = generate_context_pack(&context);

        assert!(result.is_ok(), "generate_context_pack failed: {result:?}");

        let artifact = result.expect("context pack should generate");
        assert_eq!(artifact.project_name, "Monad");
        assert!(!artifact.source_files.is_empty());

        // Verify the Markdown renders.
        let markdown = render_context_pack_markdown(&artifact);
        assert!(markdown.contains("# Latest Context Pack"));
        assert!(markdown.contains("## Project Identity"));
    }
}
