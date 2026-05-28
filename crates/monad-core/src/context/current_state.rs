//! Current-state artifact generator for Monad.
//!
//! A current-state artifact is the simplest context artifact. It answers one
//! question: "Where are we right now?"
//!
//! The generator reads repository state from:
//!
//! - `monad.toml` for project identity;
//! - `work/epics/` for completed and active epics;
//! - `crates/monad-core/src/lib.rs` for runtime module exports.
//!
//! The output is a deterministic Markdown file written to
//! `.monad/context/current-state.md`.

use std::fs;
use std::path::Path;

use crate::{MonadError, MonadResult, WorkspaceContext};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// A parsed epic entry from the `work/epics/` directory.
///
/// Each epic file has YAML frontmatter with `title`, `epic`, and `status`
/// fields. This struct captures the fields the current-state generator needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentStateEpicEntry {
    /// The epic identifier, e.g. `"E0"`, `"E1"`.
    pub id: String,

    /// The epic title, e.g. `"Project Foundation"`.
    pub title: String,

    /// The epic status, e.g. `"complete"` or `"in-progress"`.
    pub status: String,
}

/// A generated current-state artifact.
///
/// This struct holds all the data the generator collected from the repository.
/// It can be rendered to Markdown deterministically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentStateArtifact {
    /// Project display name from `monad.toml`.
    pub project_name: String,

    /// Project description from `monad.toml`.
    pub project_description: String,

    /// All epics found in `work/epics/`, sorted by identifier.
    pub epics: Vec<CurrentStateEpicEntry>,

    /// Public module names exported from `monad-core/src/lib.rs`.
    pub runtime_modules: Vec<String>,

    /// Source files that were inspected to produce this artifact.
    pub source_files: Vec<String>,
}

// ---------------------------------------------------------------------------
// Generation
// ---------------------------------------------------------------------------

/// Generates a current-state artifact from repository state.
///
/// This function reads the workspace to determine the project identity,
/// epic statuses, and runtime capabilities. It returns a structured
/// artifact that can be rendered to Markdown.
pub fn generate_current_state(context: &WorkspaceContext) -> MonadResult<CurrentStateArtifact> {
    let mut source_files: Vec<String> = Vec::new();

    // 1. Read project identity from monad.toml.
    let manifest_path = context.monad_manifest_path();
    let (project_name, project_description) = read_project_identity(&manifest_path)?;
    source_files.push("monad.toml".to_string());

    // 2. Read epics from work/epics/.
    let epics_dir = context.work_dir().join("epics");
    let epics = read_epics(&epics_dir)?;
    source_files.push("work/epics/".to_string());

    // 3. Read runtime modules from monad-core/src/lib.rs.
    let lib_path = context.root().join("crates/monad-core/src/lib.rs");
    let runtime_modules = read_runtime_modules(&lib_path)?;
    source_files.push("crates/monad-core/src/lib.rs".to_string());

    source_files.sort();

    Ok(CurrentStateArtifact {
        project_name,
        project_description,
        epics,
        runtime_modules,
        source_files,
    })
}

/// Writes a current-state artifact to `.monad/context/current-state.md`.
///
/// Creates the output directory if it does not exist.
pub fn write_current_state_artifact(
    context: &WorkspaceContext,
    artifact: &CurrentStateArtifact,
) -> MonadResult<()> {
    let output_dir = context.context_dir();

    fs::create_dir_all(&output_dir).map_err(|error| {
        MonadError::internal(format!(
            "failed to create context directory {}: {error}",
            output_dir.display()
        ))
    })?;

    let output_path = output_dir.join("current-state.md");
    let rendered = render_current_state_markdown(artifact);

    fs::write(&output_path, &rendered).map_err(|error| {
        MonadError::internal(format!(
            "failed to write current-state artifact to {}: {error}",
            output_path.display()
        ))
    })?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/// Renders a current-state artifact to deterministic Markdown.
///
/// The output follows the Current State Standard defined in
/// `docs/08-context/CURRENT-STATE-STANDARD.md`.
#[must_use]
pub fn render_current_state_markdown(artifact: &CurrentStateArtifact) -> String {
    // Frontmatter.
    let mut lines: Vec<String> = vec![
        "---".to_string(),
        "title: \"Current State\"".to_string(),
        "document_type: \"context-artifact\"".to_string(),
        "artifact_type: \"current-state\"".to_string(),
        "status: \"current\"".to_string(),
        "generated: true".to_string(),
        "reviewed: false".to_string(),
    ];

    // Epic and work packet from the active epic.
    if let Some(active) = artifact.active_epic() {
        lines.push(format!("epic: \"{}\"", active.id));
    }

    // Source files.
    lines.push("source_files:".to_string());
    for source in &artifact.source_files {
        lines.push(format!("  - \"{source}\""));
    }

    lines.push("---".to_string());
    lines.push(String::new());

    // # Current State
    lines.push("# Current State".to_string());
    lines.push(String::new());

    // ## Project
    lines.push("## Project".to_string());
    lines.push(String::new());
    lines.push(format!(
        "{} is {}",
        artifact.project_name, artifact.project_description
    ));
    lines.push(String::new());

    // ## Completed Epics
    let completed: Vec<&CurrentStateEpicEntry> = artifact
        .epics
        .iter()
        .filter(|e| e.status == "complete")
        .collect();

    lines.push("## Completed Epics".to_string());
    lines.push(String::new());

    if completed.is_empty() {
        lines.push("None.".to_string());
    } else {
        for epic in &completed {
            lines.push(format!("{} — {} is complete.", epic.id, epic.title));
        }
    }
    lines.push(String::new());

    // ## Current Epic
    lines.push("## Current Epic".to_string());
    lines.push(String::new());

    if let Some(active) = artifact.active_epic() {
        lines.push(format!("{} — {}", active.id, active.title));
    } else {
        lines.push("None.".to_string());
    }
    lines.push(String::new());

    // ## Active Focus
    lines.push("## Active Focus".to_string());
    lines.push(String::new());

    if let Some(active) = artifact.active_epic() {
        lines.push(format!(
            "The current focus is {} — {}.",
            active.id, active.title
        ));
    } else {
        lines.push("No active epic.".to_string());
    }
    lines.push(String::new());

    // ## Runtime Capabilities
    lines.push("## Runtime Capabilities".to_string());
    lines.push(String::new());

    if artifact.runtime_modules.is_empty() {
        lines.push("None detected.".to_string());
    } else {
        lines.push("Public modules in `monad-core`:".to_string());
        lines.push(String::new());

        for module in &artifact.runtime_modules {
            lines.push(format!("- `{module}`"));
        }
    }
    lines.push(String::new());

    // ## Verification
    lines.push("## Verification".to_string());
    lines.push(String::new());
    lines.push("Run:".to_string());
    lines.push(String::new());
    lines.push("```bash".to_string());
    lines.push("cargo fmt --check".to_string());
    lines.push("cargo test".to_string());
    lines.push("cargo clippy --all-targets --all-features -- -D warnings".to_string());
    lines.push("```".to_string());
    lines.push(String::new());

    lines.join("\n")
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

impl CurrentStateArtifact {
    /// Returns the first in-progress epic, if any.
    #[must_use]
    pub fn active_epic(&self) -> Option<&CurrentStateEpicEntry> {
        self.epics.iter().find(|e| e.status == "in-progress")
    }

    /// Returns completed epics.
    #[must_use]
    pub fn completed_epics(&self) -> Vec<&CurrentStateEpicEntry> {
        self.epics
            .iter()
            .filter(|e| e.status == "complete")
            .collect()
    }
}

/// Reads project identity from `monad.toml`.
///
/// Returns `(display_name, description)`.
fn read_project_identity(manifest_path: &Path) -> MonadResult<(String, String)> {
    let content = fs::read_to_string(manifest_path).map_err(|error| {
        MonadError::not_found(format!(
            "monad.toml at {}: {error}",
            manifest_path.display()
        ))
    })?;

    let table: toml::Table = content
        .parse()
        .map_err(|error| MonadError::internal(format!("failed to parse monad.toml: {error}")))?;

    let project = table
        .get("project")
        .and_then(|v| v.as_table())
        .ok_or_else(|| MonadError::not_found("[project] section in monad.toml"))?;

    let display_name = project
        .get("display_name")
        .and_then(|v| v.as_str())
        .or_else(|| project.get("name").and_then(|v| v.as_str()))
        .unwrap_or("Unknown")
        .to_string();

    let description = project
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("No description available.")
        .to_string();

    Ok((display_name, description))
}

/// Reads epic entries from the `work/epics/` directory.
///
/// Each `.md` file in the directory is parsed for YAML frontmatter fields:
/// `epic`, `title`, and `status`.
///
/// Results are sorted by epic identifier for deterministic output.
fn read_epics(epics_dir: &Path) -> MonadResult<Vec<CurrentStateEpicEntry>> {
    if !epics_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut entries: Vec<CurrentStateEpicEntry> = Vec::new();

    let dir_entries = fs::read_dir(epics_dir).map_err(|error| {
        MonadError::internal(format!(
            "failed to read epics directory {}: {error}",
            epics_dir.display()
        ))
    })?;

    for entry in dir_entries {
        let entry = entry.map_err(|error| {
            MonadError::internal(format!("failed to read directory entry: {error}"))
        })?;

        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let content = fs::read_to_string(&path).map_err(|error| {
            MonadError::internal(format!(
                "failed to read epic file {}: {error}",
                path.display()
            ))
        })?;

        if let Some(epic_entry) = parse_epic_frontmatter(&content) {
            entries.push(epic_entry);
        }
    }

    // Sort by epic identifier for deterministic output.
    entries.sort_by(|a, b| natural_epic_sort(&a.id, &b.id));

    Ok(entries)
}

/// Parses YAML frontmatter from an epic file to extract `epic`, `title`,
/// and `status` fields.
///
/// This is a simple line-based parser that avoids pulling in a full YAML
/// library. Frontmatter is expected between `---` delimiters.
fn parse_epic_frontmatter(content: &str) -> Option<CurrentStateEpicEntry> {
    let mut in_frontmatter = false;
    let mut epic_id: Option<String> = None;
    let mut title: Option<String> = None;
    let mut status: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == "---" {
            if in_frontmatter {
                // End of frontmatter.
                break;
            }
            in_frontmatter = true;
            continue;
        }

        if !in_frontmatter {
            continue;
        }

        if let Some(value) = extract_frontmatter_value(trimmed, "epic:") {
            epic_id = Some(value);
        } else if let Some(value) = extract_frontmatter_value(trimmed, "title:") {
            title = Some(value);
        } else if let Some(value) = extract_frontmatter_value(trimmed, "status:") {
            status = Some(value);
        }
    }

    let epic_id = epic_id?;
    let raw_title = title?;
    let status = status.unwrap_or_else(|| "unknown".to_string());

    // The title field often includes the epic ID prefix like
    // "E0 — Project Foundation". Strip the prefix to get just the title.
    let clean_title = strip_epic_prefix(&raw_title, &epic_id);

    Some(CurrentStateEpicEntry {
        id: epic_id,
        title: clean_title,
        status,
    })
}

/// Extracts a value from a YAML frontmatter line.
///
/// Given a line like `epic: "E0"` and prefix `"epic:"`, returns `"E0"`.
/// Handles both quoted and unquoted values.
fn extract_frontmatter_value(line: &str, key: &str) -> Option<String> {
    if !line.starts_with(key) {
        return None;
    }

    let value = line[key.len()..].trim();

    // Remove surrounding quotes if present.
    let cleaned = if (value.starts_with('"') && value.ends_with('"'))
        || (value.starts_with('\'') && value.ends_with('\''))
    {
        &value[1..value.len() - 1]
    } else {
        value
    };

    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned.to_string())
    }
}

/// Strips the epic ID prefix from a title.
///
/// For example, `"E0 — Project Foundation"` becomes `"Project Foundation"`.
fn strip_epic_prefix(title: &str, epic_id: &str) -> String {
    // Try common separators: " — ", " - ", ": ".
    let prefixes = [
        format!("{epic_id} — "),
        format!("{epic_id} - "),
        format!("{epic_id}: "),
    ];

    for prefix in &prefixes {
        if let Some(rest) = title.strip_prefix(prefix.as_str()) {
            return rest.to_string();
        }
    }

    title.to_string()
}

/// Natural sort for epic identifiers like "E0", "E1", "E10".
///
/// Splits the identifier into a prefix letter and numeric suffix, then sorts
/// alphabetically by prefix and numerically by suffix.
fn natural_epic_sort(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |s: &str| -> (String, u32) {
        let prefix: String = s.chars().take_while(|c| !c.is_ascii_digit()).collect();
        let number: u32 = s
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap_or(0);
        (prefix, number)
    };

    let (pa, na) = parse(a);
    let (pb, nb) = parse(b);

    pa.cmp(&pb).then(na.cmp(&nb))
}

/// Reads public module names from `monad-core/src/lib.rs`.
///
/// Looks for lines matching `pub mod <name>;` and collects the module names.
/// Results are sorted for deterministic output.
fn read_runtime_modules(lib_path: &Path) -> MonadResult<Vec<String>> {
    if !lib_path.is_file() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(lib_path).map_err(|error| {
        MonadError::internal(format!(
            "failed to read lib.rs at {}: {error}",
            lib_path.display()
        ))
    })?;

    let mut modules: Vec<String> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Match lines like: pub mod workspace;
        if let Some(name) = trimmed
            .strip_prefix("pub mod ")
            .and_then(|rest| rest.strip_suffix(';'))
        {
            modules.push(name.trim().to_string());
        }
    }

    modules.sort();

    Ok(modules)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_frontmatter_value_handles_quoted_values() {
        assert_eq!(
            extract_frontmatter_value("epic: \"E0\"", "epic:"),
            Some("E0".to_string())
        );
    }

    #[test]
    fn extract_frontmatter_value_handles_unquoted_values() {
        assert_eq!(
            extract_frontmatter_value("status: complete", "status:"),
            Some("complete".to_string())
        );
    }

    #[test]
    fn extract_frontmatter_value_returns_none_for_wrong_key() {
        assert_eq!(extract_frontmatter_value("title: \"Foo\"", "epic:"), None);
    }

    #[test]
    fn extract_frontmatter_value_returns_none_for_empty_value() {
        assert_eq!(extract_frontmatter_value("epic: ", "epic:"), None);
    }

    #[test]
    fn parse_epic_frontmatter_extracts_fields() {
        let content = "\
---
title: \"E0 — Project Foundation\"
document_type: \"epic\"
status: \"complete\"
epic: \"E0\"
---

# E0 — Project Foundation
";

        let entry = parse_epic_frontmatter(content).expect("should parse frontmatter");

        assert_eq!(entry.id, "E0");
        assert_eq!(entry.title, "Project Foundation");
        assert_eq!(entry.status, "complete");
    }

    #[test]
    fn parse_epic_frontmatter_handles_in_progress() {
        let content = "\
---
title: \"E2 — Repository Intelligence Foundation\"
status: \"in-progress\"
epic: \"E2\"
---
";

        let entry = parse_epic_frontmatter(content).expect("should parse frontmatter");

        assert_eq!(entry.id, "E2");
        assert_eq!(entry.title, "Repository Intelligence Foundation");
        assert_eq!(entry.status, "in-progress");
    }

    #[test]
    fn parse_epic_frontmatter_returns_none_without_epic_id() {
        let content = "\
---
title: \"Some Title\"
status: \"complete\"
---
";

        assert!(parse_epic_frontmatter(content).is_none());
    }

    #[test]
    fn strip_epic_prefix_removes_em_dash_separator() {
        assert_eq!(
            strip_epic_prefix("E0 — Project Foundation", "E0"),
            "Project Foundation"
        );
    }

    #[test]
    fn strip_epic_prefix_removes_hyphen_separator() {
        assert_eq!(
            strip_epic_prefix("E0 - Project Foundation", "E0"),
            "Project Foundation"
        );
    }

    #[test]
    fn strip_epic_prefix_preserves_title_without_prefix() {
        assert_eq!(
            strip_epic_prefix("Project Foundation", "E0"),
            "Project Foundation"
        );
    }

    #[test]
    fn natural_epic_sort_orders_correctly() {
        use std::cmp::Ordering;

        assert_eq!(natural_epic_sort("E0", "E1"), Ordering::Less);
        assert_eq!(natural_epic_sort("E1", "E0"), Ordering::Greater);
        assert_eq!(natural_epic_sort("E2", "E10"), Ordering::Less);
        assert_eq!(natural_epic_sort("E1", "E1"), Ordering::Equal);
    }

    #[test]
    fn read_runtime_modules_from_content() {
        // Simulate reading lib.rs content by testing the line parsing directly.
        let content = "\
pub mod checks;
pub mod diagnostics;
pub mod error;
pub mod workspace;
";

        let mut modules = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("pub mod ")
                && let Some(name) = rest.strip_suffix(';')
            {
                modules.push(name.trim().to_string());
            }
        }

        modules.sort();

        assert_eq!(modules, vec!["checks", "diagnostics", "error", "workspace"]);
    }

    #[test]
    fn current_state_artifact_finds_active_epic() {
        let artifact = CurrentStateArtifact {
            project_name: "Monad".to_string(),
            project_description: "test".to_string(),
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
            runtime_modules: vec!["workspace".to_string()],
            source_files: vec!["monad.toml".to_string()],
        };

        let active = artifact.active_epic().expect("should have active epic");

        assert_eq!(active.id, "E1");
    }

    #[test]
    fn current_state_artifact_returns_none_when_no_active_epic() {
        let artifact = CurrentStateArtifact {
            project_name: "Monad".to_string(),
            project_description: "test".to_string(),
            epics: vec![CurrentStateEpicEntry {
                id: "E0".to_string(),
                title: "Project Foundation".to_string(),
                status: "complete".to_string(),
            }],
            runtime_modules: Vec::new(),
            source_files: Vec::new(),
        };

        assert!(artifact.active_epic().is_none());
    }

    #[test]
    fn render_current_state_includes_required_sections() {
        let artifact = CurrentStateArtifact {
            project_name: "Monad".to_string(),
            project_description: "an AI-native Software Foundry OS.".to_string(),
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
            runtime_modules: vec!["diagnostics".to_string(), "workspace".to_string()],
            source_files: vec![
                "crates/monad-core/src/lib.rs".to_string(),
                "monad.toml".to_string(),
                "work/epics/".to_string(),
            ],
        };

        let rendered = render_current_state_markdown(&artifact);

        // Frontmatter fields.
        assert!(rendered.contains("generated: true"));
        assert!(rendered.contains("artifact_type: \"current-state\""));
        assert!(rendered.contains("epic: \"E1\""));

        // Required sections.
        assert!(rendered.contains("# Current State"));
        assert!(rendered.contains("## Project"));
        assert!(rendered.contains("## Completed Epics"));
        assert!(rendered.contains("## Current Epic"));
        assert!(rendered.contains("## Active Focus"));
        assert!(rendered.contains("## Runtime Capabilities"));
        assert!(rendered.contains("## Verification"));

        // Content.
        assert!(rendered.contains("Monad is an AI-native Software Foundry OS."));
        assert!(rendered.contains("E0 — Project Foundation is complete."));
        assert!(rendered.contains("E1 — Runtime Foundation"));
        assert!(rendered.contains("- `diagnostics`"));
        assert!(rendered.contains("- `workspace`"));
    }

    #[test]
    fn render_current_state_handles_no_epics() {
        let artifact = CurrentStateArtifact {
            project_name: "Monad".to_string(),
            project_description: "test project.".to_string(),
            epics: Vec::new(),
            runtime_modules: Vec::new(),
            source_files: Vec::new(),
        };

        let rendered = render_current_state_markdown(&artifact);

        assert!(rendered.contains("None."));
        assert!(rendered.contains("No active epic."));
        assert!(rendered.contains("None detected."));
    }

    #[test]
    fn render_current_state_is_deterministic() {
        let artifact = CurrentStateArtifact {
            project_name: "Monad".to_string(),
            project_description: "test project.".to_string(),
            epics: vec![CurrentStateEpicEntry {
                id: "E0".to_string(),
                title: "Foundation".to_string(),
                status: "complete".to_string(),
            }],
            runtime_modules: vec!["workspace".to_string()],
            source_files: vec!["monad.toml".to_string()],
        };

        let first = render_current_state_markdown(&artifact);
        let second = render_current_state_markdown(&artifact);

        assert_eq!(first, second, "rendering should be deterministic");
    }

    #[test]
    fn generate_current_state_from_workspace() {
        // Use CARGO_MANIFEST_DIR to locate the crate directory, then walk
        // up to the workspace root. This works regardless of the working
        // directory that `cargo test` uses.
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let context = WorkspaceContext::discover_from(manifest_dir)
            .expect("workspace root should be discoverable");
        let artifact = generate_current_state(&context).expect("generation should succeed");

        assert_eq!(artifact.project_name, "Monad");
        assert!(!artifact.project_description.is_empty());
        assert!(!artifact.epics.is_empty());
        assert!(!artifact.runtime_modules.is_empty());

        // Should find at least E0 as complete.
        let completed = artifact.completed_epics();
        assert!(
            !completed.is_empty(),
            "should have at least one completed epic"
        );

        // Runtime modules should include known modules.
        assert!(
            artifact.runtime_modules.contains(&"workspace".to_string()),
            "should detect workspace module"
        );
        assert!(
            artifact.runtime_modules.contains(&"error".to_string()),
            "should detect error module"
        );
    }

    #[test]
    fn write_and_read_current_state_artifact() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        let temp_dir = std::env::temp_dir().join(format!(
            "monad-current-state-write-{}-{unique}",
            std::process::id()
        ));

        // Set up a minimal workspace structure.
        let monad_dir = temp_dir.join(".monad");
        let context_dir = monad_dir.join("context");

        fs::create_dir_all(&context_dir).expect("context directory should be created");
        fs::write(temp_dir.join("monad.toml"), "[project]\nname = \"test\"\ndisplay_name = \"Test\"\ndescription = \"A test project.\"\n[workspace]\nroot_markers = [\"monad.toml\"]\n[runtime]\ncore_crate = \"test\"\ncli_crate = \"test\"\nexecution_model = \"local\"\n")
            .expect("monad.toml should be written");

        let context =
            WorkspaceContext::new(&temp_dir).expect("workspace context should be created");

        let artifact = CurrentStateArtifact {
            project_name: "Test".to_string(),
            project_description: "A test project.".to_string(),
            epics: Vec::new(),
            runtime_modules: Vec::new(),
            source_files: vec!["monad.toml".to_string()],
        };

        write_current_state_artifact(&context, &artifact).expect("write should succeed");

        let output_path = context_dir.join("current-state.md");
        assert!(output_path.exists(), "current-state.md should exist");

        let content = fs::read_to_string(&output_path).expect("should read back written file");
        assert!(content.contains("# Current State"));
        assert!(content.contains("generated: true"));

        fs::remove_dir_all(temp_dir).ok();
    }
}
