//! Bootstrap prompt generator for Monad.
//!
//! A bootstrap prompt is a ready-to-use instruction block for starting a new
//! AI session. It tells the assistant what the project is, where to read
//! first, what rules to follow, and how to continue work from repo-resident
//! context.
//!
//! The generator reads repository state from:
//!
//! - `monad.toml` for project identity;
//! - `work/epics/` for active epics;
//! - `work/packets/` for active work packets;
//! - `docs/06-adrs/` for accepted decisions;
//! - `.monad/context/current-state.md` for current state;
//! - `.monad/context/latest-handoff.md` for latest handoff;
//! - `.monad/context/latest-context-pack.md` for context pack.
//!
//! The output is a deterministic Markdown file written to
//! `docs/ai/BOOTSTRAP-PROMPT.md`.

use std::fs;

use crate::context::current_state::{CurrentStateArtifact, generate_current_state};
use crate::context::handoff::{HandoffArtifact, generate_handoff};
use crate::{MonadError, MonadResult, WorkspaceContext};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// A generated bootstrap prompt artifact.
///
/// This struct holds all the data the generator collected from the repository.
/// It can be rendered to Markdown deterministically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPromptArtifact {
    /// The project name from `monad.toml`.
    pub project_name: String,

    /// A one-line project description.
    pub project_description: String,

    /// Ordered list of files the assistant should read first.
    pub reading_order: Vec<String>,

    /// The current-state snapshot used to derive active work context.
    pub current_state: CurrentStateArtifact,

    /// The handoff snapshot used to derive work packet context.
    pub handoff: HandoffArtifact,

    /// Workflow rules the assistant should follow.
    pub workflow_rules: Vec<String>,

    /// Source-of-truth rule text.
    pub source_of_truth_rule: String,

    /// Source files inspected to produce this artifact.
    pub source_files: Vec<String>,
}

// ---------------------------------------------------------------------------
// Generation
// ---------------------------------------------------------------------------

/// Generates a bootstrap prompt artifact from repository state.
///
/// This function reads the workspace to determine the project identity,
/// current state, active work, and reading order. It returns a structured
/// artifact that can be rendered to Markdown.
pub fn generate_bootstrap_prompt(
    context: &WorkspaceContext,
) -> MonadResult<BootstrapPromptArtifact> {
    let manifest_path = context.root().join("monad.toml");
    let manifest_content =
        fs::read_to_string(&manifest_path).map_err(|error| MonadError::NotFound {
            resource: format!("monad.toml at {}: {error}", manifest_path.display()),
        })?;

    let project_name = extract_project_name(&manifest_content);
    let project_description = extract_project_description(context);

    let current_state = generate_current_state(context)?;
    let handoff = generate_handoff(context)?;

    let reading_order = build_reading_order(context);
    let workflow_rules = build_workflow_rules();
    let source_of_truth_rule = build_source_of_truth_rule();
    let source_files = build_source_files(context);

    Ok(BootstrapPromptArtifact {
        project_name,
        project_description,
        reading_order,
        current_state,
        handoff,
        workflow_rules,
        source_of_truth_rule,
        source_files,
    })
}

/// Renders a bootstrap prompt artifact to deterministic Markdown.
pub fn render_bootstrap_prompt_markdown(artifact: &BootstrapPromptArtifact) -> String {
    let mut sections = vec![
        render_frontmatter(artifact),
        render_header(artifact),
        render_project_identity(artifact),
        render_source_of_truth(artifact),
        render_reading_order(artifact),
        render_current_work(artifact),
        render_workflow_rules(artifact),
        render_response_expectations(),
        render_continuation_protocol(),
    ];

    // Trim trailing whitespace from each section.
    for section in &mut sections {
        *section = section.trim_end().to_string();
    }

    let mut output = sections.join("\n\n");
    output.push('\n');
    output
}

/// Writes a bootstrap prompt artifact to `docs/ai/BOOTSTRAP-PROMPT.md`.
pub fn write_bootstrap_prompt_artifact(
    context: &WorkspaceContext,
    artifact: &BootstrapPromptArtifact,
) -> MonadResult<()> {
    let ai_dir = context.root().join("docs/ai");
    fs::create_dir_all(&ai_dir).map_err(|error| MonadError::Internal {
        message: format!("failed to create docs/ai: {error}"),
    })?;

    let output_path = ai_dir.join("BOOTSTRAP-PROMPT.md");
    let markdown = render_bootstrap_prompt_markdown(artifact);

    fs::write(&output_path, markdown).map_err(|error| MonadError::Internal {
        message: format!(
            "failed to write bootstrap prompt to {}: {error}",
            output_path.display()
        ),
    })?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Extracts the project name from `monad.toml` content.
///
/// Prefers `display_name` over `name` to match the existing current-state
/// generator behavior.
fn extract_project_name(content: &str) -> String {
    let table: toml::Table = match content.parse() {
        Ok(t) => t,
        Err(_) => return "Unknown".to_string(),
    };

    let project = match table.get("project").and_then(|v| v.as_table()) {
        Some(p) => p,
        None => return "Unknown".to_string(),
    };

    project
        .get("display_name")
        .and_then(|v| v.as_str())
        .or_else(|| project.get("name").and_then(|v| v.as_str()))
        .unwrap_or("Unknown")
        .to_string()
}

/// Extracts a project description from README or product charter.
fn extract_project_description(context: &WorkspaceContext) -> String {
    // Try README first line after the header.
    let readme_path = context.root().join("README.md");
    if let Ok(content) = fs::read_to_string(&readme_path)
        && let Some(desc) = extract_first_description_line(&content)
    {
        return desc;
    }

    // Fall back to product charter.
    let charter_path = context
        .root()
        .join("docs/01-project/01-charter/PRODUCT-CHARTER.md");
    if let Ok(content) = fs::read_to_string(&charter_path)
        && let Some(desc) = extract_first_description_line(&content)
    {
        return desc;
    }

    "A software project managed with Monad.".to_string()
}

/// Extracts the first non-empty, non-header, non-frontmatter line.
fn extract_first_description_line(content: &str) -> Option<String> {
    let mut in_frontmatter = false;
    let mut frontmatter_ended = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Handle YAML frontmatter.
        if trimmed == "---" && !frontmatter_ended {
            in_frontmatter = !in_frontmatter;
            if !in_frontmatter {
                frontmatter_ended = true;
            }
            continue;
        }

        if in_frontmatter {
            continue;
        }

        // Skip empty lines and headers.
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Skip badge lines, links-only lines.
        if trimmed.starts_with('[') || trimmed.starts_with('!') || trimmed.starts_with('<') {
            continue;
        }

        return Some(trimmed.to_string());
    }

    None
}

/// Builds the ordered reading list for a new session.
fn build_reading_order(context: &WorkspaceContext) -> Vec<String> {
    let candidates = [
        ".monad/context/current-state.md",
        ".monad/context/latest-handoff.md",
        ".monad/context/latest-context-pack.md",
        "docs/01-project/01-charter/PRODUCT-CHARTER.md",
        "docs/05-architecture/SYSTEM-OVERVIEW.md",
        "docs/07-workflow/OPERATING-MODEL.md",
        "docs/08-context/CONTEXT-BRIDGE.md",
        "README.md",
    ];

    candidates
        .iter()
        .filter(|path| context.root().join(path).exists())
        .map(|path| (*path).to_string())
        .collect()
}

/// Builds the standard workflow rules.
fn build_workflow_rules() -> Vec<String> {
    [
        "Read current-state and handoff before starting work.",
        "Follow the active work packet scope; do not expand beyond it.",
        "Use conventional commits (feat, fix, docs, refactor, test, chore).",
        "Run verification before committing: cargo fmt, cargo test, cargo clippy.",
        "Update context artifacts after completing a work packet.",
        "Do not introduce new dependencies without an ADR or explicit approval.",
        "Do not modify files outside the work packet scope without justification.",
        "Prefer small, reviewable changes over large rewrites.",
        "Treat docs/ and work/ files as canonical; do not contradict them.",
        "If uncertain, ask rather than assume.",
    ]
    .iter()
    .map(|s| (*s).to_string())
    .collect()
}

/// Builds the source-of-truth rule.
fn build_source_of_truth_rule() -> String {
    "The repository is the canonical source of truth. \
     Use repo files — not chat history, not memory, not assumptions — \
     as the authoritative reference for project state, decisions, and work."
        .to_string()
}

/// Builds the list of source files inspected.
fn build_source_files(context: &WorkspaceContext) -> Vec<String> {
    let candidates = [
        "monad.toml",
        "README.md",
        "docs/01-project/01-charter/PRODUCT-CHARTER.md",
        "docs/05-architecture/SYSTEM-OVERVIEW.md",
        "docs/07-workflow/OPERATING-MODEL.md",
        "docs/08-context/CONTEXT-BRIDGE.md",
        ".monad/context/current-state.md",
        ".monad/context/latest-handoff.md",
        ".monad/context/latest-context-pack.md",
        "work/epics/",
        "work/packets/",
    ];

    candidates
        .iter()
        .filter(|path| context.root().join(path).exists())
        .map(|path| (*path).to_string())
        .collect()
}

// ---------------------------------------------------------------------------
// Markdown rendering helpers
// ---------------------------------------------------------------------------

fn render_frontmatter(artifact: &BootstrapPromptArtifact) -> String {
    let source_files_yaml: String = artifact
        .source_files
        .iter()
        .map(|f| format!("  - \"{}\"", f))
        .collect::<Vec<_>>()
        .join("\n");

    [
        "---",
        "title: \"Bootstrap Prompt\"",
        "document_type: \"bootstrap-prompt\"",
        "artifact_type: \"bootstrap-prompt\"",
        "status: \"current\"",
        "generated: true",
        "reviewed: false",
        &format!("project: \"{}\"", artifact.project_name),
        "source: \"repository\"",
        "source_files:",
        &source_files_yaml,
        "---",
    ]
    .join("\n")
}

fn render_header(artifact: &BootstrapPromptArtifact) -> String {
    format!("# Bootstrap Prompt — {}", artifact.project_name)
}

fn render_project_identity(artifact: &BootstrapPromptArtifact) -> String {
    let mut lines = vec![
        "## Project Identity".to_string(),
        String::new(),
        format!("**Project:** {}", artifact.project_name),
        format!("**Description:** {}", artifact.project_description),
    ];

    let total_epics = artifact.current_state.epics.len();
    let completed_epics = artifact.current_state.completed_epics().len();
    lines.push(format!(
        "**Progress:** {completed_epics} of {total_epics} epics completed."
    ));

    lines.join("\n")
}

fn render_source_of_truth(artifact: &BootstrapPromptArtifact) -> String {
    [
        "## Source of Truth",
        "",
        &artifact.source_of_truth_rule,
        "",
        "Do not rely on prior conversation context. Read the files listed below.",
    ]
    .join("\n")
}

fn render_reading_order(artifact: &BootstrapPromptArtifact) -> String {
    let mut lines = vec![
        "## Required Reading Order".to_string(),
        String::new(),
        "Read these files in order before beginning any work:".to_string(),
        String::new(),
    ];

    for (i, path) in artifact.reading_order.iter().enumerate() {
        lines.push(format!("{}. `{}`", i + 1, path));
    }

    lines.join("\n")
}

fn render_current_work(artifact: &BootstrapPromptArtifact) -> String {
    let mut lines = vec!["## Current Work".to_string(), String::new()];

    if let Some(active_epic) = artifact.current_state.active_epic() {
        lines.push(format!(
            "**Active Epic:** {} — {}",
            active_epic.id, active_epic.title
        ));
    } else {
        lines.push("**Active Epic:** None".to_string());
    }

    if let Some(active_wp) = artifact.handoff.active_work_packet() {
        lines.push(format!(
            "**Active Work Packet:** {} — {}",
            active_wp.id, active_wp.title
        ));
        lines.push(String::new());
        lines.push(
            "Read the work packet file for scope, tasks, deliverables, and verification commands."
                .to_string(),
        );
    } else {
        lines.push("**Active Work Packet:** None".to_string());
    }

    lines.join("\n")
}

fn render_workflow_rules(artifact: &BootstrapPromptArtifact) -> String {
    let mut lines = vec!["## Workflow Rules".to_string(), String::new()];

    for rule in &artifact.workflow_rules {
        lines.push(format!("- {rule}"));
    }

    lines.join("\n")
}

fn render_response_expectations() -> String {
    [
        "## Response Expectations",
        "",
        "- Be concise and precise.",
        "- Reference file paths when discussing code or docs.",
        "- Propose changes as diffs or file edits, not vague descriptions.",
        "- Verify your changes compile and pass tests before presenting them.",
        "- If something is ambiguous, state the ambiguity and ask for clarification.",
        "- Do not invent requirements that are not in the work packet.",
    ]
    .join("\n")
}

fn render_continuation_protocol() -> String {
    [
        "## Continuation Protocol",
        "",
        "When resuming work in a new session:",
        "",
        "1. Read the bootstrap prompt (this file).",
        "2. Read `.monad/context/current-state.md` for project status.",
        "3. Read `.monad/context/latest-handoff.md` for session continuity.",
        "4. Read the active work packet file for task details.",
        "5. Run `cargo test` and `cargo clippy` to confirm the baseline is clean.",
        "6. Continue from where the handoff left off.",
        "",
        "Do not start over. Do not re-derive decisions that are already accepted.",
        "Do not contradict docs/ or ADRs without proposing a new ADR.",
    ]
    .join("\n")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- extract_project_name -------------------------------------------------

    #[test]
    fn extract_project_name_from_toml() {
        let content = "[project]\nname = \"Monad\"\nversion = \"0.1.0\"";
        assert_eq!(extract_project_name(content), "Monad");
    }

    #[test]
    fn extract_project_name_prefers_display_name() {
        let content = "[project]\nname = \"monad\"\ndisplay_name = \"Monad\"";
        assert_eq!(extract_project_name(content), "Monad");
    }

    #[test]
    fn extract_project_name_missing_falls_back() {
        let content = "[project]\nversion = \"0.1.0\"";
        assert_eq!(extract_project_name(content), "Unknown");
    }

    #[test]
    fn extract_project_name_no_project_section_falls_back() {
        let content = "version = \"0.1.0\"";
        assert_eq!(extract_project_name(content), "Unknown");
    }

    // -- extract_first_description_line ---------------------------------------

    #[test]
    fn extract_description_skips_frontmatter_and_headers() {
        let content = "---\ntitle: test\n---\n\n# My Project\n\nThis is the description.\n";
        assert_eq!(
            extract_first_description_line(content),
            Some("This is the description.".to_string())
        );
    }

    #[test]
    fn extract_description_skips_badges() {
        let content = "# Title\n\n[![badge](url)](link)\n\nActual description.";
        assert_eq!(
            extract_first_description_line(content),
            Some("Actual description.".to_string())
        );
    }

    #[test]
    fn extract_description_returns_none_for_empty() {
        let content = "# Title\n\n";
        assert_eq!(extract_first_description_line(content), None);
    }

    // -- build_workflow_rules -------------------------------------------------

    #[test]
    fn workflow_rules_are_non_empty() {
        let rules = build_workflow_rules();
        assert!(!rules.is_empty());
        assert!(rules.len() >= 5);
    }

    #[test]
    fn workflow_rules_mention_conventional_commits() {
        let rules = build_workflow_rules();
        assert!(rules.iter().any(|r| r.contains("conventional commits")));
    }

    // -- build_source_of_truth_rule -------------------------------------------

    #[test]
    fn source_of_truth_mentions_repository() {
        let rule = build_source_of_truth_rule();
        assert!(rule.contains("repository"));
        assert!(rule.contains("source of truth"));
    }

    // -- render_bootstrap_prompt_markdown --------------------------------------

    fn sample_artifact() -> BootstrapPromptArtifact {
        use crate::context::current_state::{CurrentStateArtifact, CurrentStateEpicEntry};
        use crate::context::handoff::{HandoffArtifact, HandoffWorkPacketEntry};

        let current_state = CurrentStateArtifact {
            project_name: "TestProject".to_string(),
            project_description: "A test project for unit testing.".to_string(),
            epics: vec![
                CurrentStateEpicEntry {
                    id: "E1".to_string(),
                    title: "Foundation".to_string(),
                    status: "complete".to_string(),
                },
                CurrentStateEpicEntry {
                    id: "E2".to_string(),
                    title: "Intelligence".to_string(),
                    status: "in-progress".to_string(),
                },
            ],
            runtime_modules: vec!["core".to_string()],
            source_files: vec!["monad.toml".to_string()],
        };

        let handoff = HandoffArtifact {
            current_state: current_state.clone(),
            work_packets: vec![HandoffWorkPacketEntry {
                id: "WP-E2-001".to_string(),
                title: "Add detection".to_string(),
                status: "in-progress".to_string(),
                epic: "E2".to_string(),
            }],
            source_files: vec!["monad.toml".to_string()],
        };

        BootstrapPromptArtifact {
            project_name: "TestProject".to_string(),
            project_description: "A test project for unit testing.".to_string(),
            reading_order: vec![
                ".monad/context/current-state.md".to_string(),
                "README.md".to_string(),
            ],
            current_state,
            handoff,
            workflow_rules: build_workflow_rules(),
            source_of_truth_rule: build_source_of_truth_rule(),
            source_files: vec!["monad.toml".to_string(), "README.md".to_string()],
        }
    }

    #[test]
    fn render_includes_project_identity() {
        let artifact = sample_artifact();
        let markdown = render_bootstrap_prompt_markdown(&artifact);
        assert!(markdown.contains("## Project Identity"));
        assert!(markdown.contains("TestProject"));
    }

    #[test]
    fn render_includes_source_of_truth() {
        let artifact = sample_artifact();
        let markdown = render_bootstrap_prompt_markdown(&artifact);
        assert!(markdown.contains("## Source of Truth"));
        assert!(markdown.contains("source of truth"));
    }

    #[test]
    fn render_includes_reading_order() {
        let artifact = sample_artifact();
        let markdown = render_bootstrap_prompt_markdown(&artifact);
        assert!(markdown.contains("## Required Reading Order"));
        assert!(markdown.contains("1. `.monad/context/current-state.md`"));
        assert!(markdown.contains("2. `README.md`"));
    }

    #[test]
    fn render_includes_current_work() {
        let artifact = sample_artifact();
        let markdown = render_bootstrap_prompt_markdown(&artifact);
        assert!(markdown.contains("## Current Work"));
        assert!(markdown.contains("E2 — Intelligence"));
        assert!(markdown.contains("WP-E2-001 — Add detection"));
    }

    #[test]
    fn render_includes_workflow_rules() {
        let artifact = sample_artifact();
        let markdown = render_bootstrap_prompt_markdown(&artifact);
        assert!(markdown.contains("## Workflow Rules"));
        assert!(markdown.contains("conventional commits"));
    }

    #[test]
    fn render_includes_response_expectations() {
        let artifact = sample_artifact();
        let markdown = render_bootstrap_prompt_markdown(&artifact);
        assert!(markdown.contains("## Response Expectations"));
    }

    #[test]
    fn render_includes_continuation_protocol() {
        let artifact = sample_artifact();
        let markdown = render_bootstrap_prompt_markdown(&artifact);
        assert!(markdown.contains("## Continuation Protocol"));
        assert!(markdown.contains("Do not start over"));
    }

    #[test]
    fn render_includes_frontmatter() {
        let artifact = sample_artifact();
        let markdown = render_bootstrap_prompt_markdown(&artifact);
        assert!(markdown.starts_with("---\n"));
        assert!(markdown.contains("artifact_type: \"bootstrap-prompt\""));
        assert!(markdown.contains("generated: true"));
    }

    #[test]
    fn render_is_deterministic() {
        let artifact = sample_artifact();
        let first = render_bootstrap_prompt_markdown(&artifact);
        let second = render_bootstrap_prompt_markdown(&artifact);
        assert_eq!(first, second);
    }

    #[test]
    fn render_epic_progress_is_included() {
        let artifact = sample_artifact();
        let markdown = render_bootstrap_prompt_markdown(&artifact);
        assert!(markdown.contains("1 of 2 epics completed"));
    }

    #[test]
    fn render_with_no_active_work_packet() {
        let mut artifact = sample_artifact();
        artifact.handoff.work_packets.clear();
        let markdown = render_bootstrap_prompt_markdown(&artifact);
        assert!(markdown.contains("**Active Work Packet:** None"));
    }

    #[test]
    fn render_with_no_active_epic() {
        let mut artifact = sample_artifact();
        for epic in &mut artifact.current_state.epics {
            epic.status = "complete".to_string();
        }
        // Also update the handoff's current_state.
        artifact.handoff.current_state = artifact.current_state.clone();
        let markdown = render_bootstrap_prompt_markdown(&artifact);
        assert!(markdown.contains("**Active Epic:** None"));
    }

    // -- Integration test -----------------------------------------------------

    #[test]
    fn generate_bootstrap_prompt_from_workspace_produces_artifact() {
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
        let result = generate_bootstrap_prompt(&context);

        assert!(
            result.is_ok(),
            "generate_bootstrap_prompt failed: {result:?}"
        );

        let artifact = result.expect("bootstrap prompt should generate");
        assert_eq!(artifact.project_name, "Monad");
        assert!(!artifact.reading_order.is_empty());
        assert!(!artifact.workflow_rules.is_empty());
        assert!(!artifact.source_files.is_empty());

        // Verify the Markdown renders.
        let markdown = render_bootstrap_prompt_markdown(&artifact);
        assert!(markdown.contains("# Bootstrap Prompt"));
        assert!(markdown.contains("## Project Identity"));
        assert!(markdown.contains("## Source of Truth"));
        assert!(markdown.contains("## Required Reading Order"));
        assert!(markdown.contains("## Continuation Protocol"));
    }
}
