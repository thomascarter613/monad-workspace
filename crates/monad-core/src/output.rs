//! Output formatting primitives for Monad.
//!
//! The CLI should stay thin. It may decide when to print, but reusable
//! formatting rules should live in `monad-core` so future commands produce
//! consistent output.
//!
//! This module supports:
//!
//! - `text` for human-readable output;
//! - `json` for machine-readable output;
//! - reusable summaries for workspace, diagnostic, and repository inspection output.

use std::collections::BTreeMap;

use serde_json::json;

use crate::repository_inspection::{
    RepositoryEntryCategory, RepositoryEntryKind, RepositoryEntryRole,
    RepositoryEntryTraversalPolicy, RepositoryInspection, build_traversal_plan,
};
use crate::{DiagnosticReport, MonadError, MonadManifest, MonadResult, Severity, WorkspaceContext};

/// Output formats supported by the runtime.
///
/// `Text` remains the default. `Json` gives scripts, CI, and future tools a
/// stable machine-readable representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human-readable plain text.
    Text,

    /// Machine-readable JSON.
    Json,
}

impl OutputFormat {
    /// Returns the stable format name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Json => "json",
        }
    }

    /// Parses an output format name.
    pub fn parse(value: &str) -> MonadResult<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            other => Err(MonadError::invalid_input(format!(
                "unsupported output format: {other}"
            ))),
        }
    }
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Text
    }
}

/// Renderable workspace summary for `monad info`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceSummary {
    /// Workspace root path rendered as text.
    pub root: String,

    /// Human-readable project display name.
    pub project_display_name: String,

    /// Stable project name.
    pub project_name: String,

    /// Manifest schema version rendered as text.
    pub schema_version: String,

    /// Durable core runtime crate.
    pub core_crate: String,

    /// Thin CLI crate.
    pub cli_crate: String,

    /// Runtime execution model.
    pub execution_model: String,
}

impl WorkspaceSummary {
    /// Builds a workspace summary from a context and loaded manifest.
    #[must_use]
    pub fn from_manifest(context: &WorkspaceContext, manifest: &MonadManifest) -> Self {
        Self {
            root: context.root().display().to_string(),
            project_display_name: manifest.project.display_name.clone(),
            project_name: manifest.project.name.clone(),
            schema_version: manifest.schema_version.to_string(),
            core_crate: manifest.runtime.core_crate.clone(),
            cli_crate: manifest.runtime.cli_crate.clone(),
            execution_model: manifest.runtime.execution_model.clone(),
        }
    }
}

/// One renderable repository entry for `monad inspect`.
///
/// This type intentionally stores strings instead of filesystem objects so
/// rendering can stay deterministic and serialization-friendly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryInspectionSummaryEntry {
    /// Path relative to the workspace root.
    pub path: String,

    /// Filesystem kind as a stable label.
    pub kind: String,

    /// Broad category as a stable label.
    pub category: String,

    /// First-pass Monad role classification as a stable label.
    pub role: String,

    /// Default future traversal policy as a stable label.
    pub traversal_policy: String,

    /// Planned future traversal decision as a stable label.
    pub traversal_decision: String,

    /// Reason for the planned future traversal decision.
    pub traversal_reason: String,
}

/// Renderable repository inspection summary for `monad inspect`.
///
/// This is deliberately a summary object rather than raw CLI formatting. The
/// CLI can ask for this structure and render it as text or JSON without owning
/// durable repository-intelligence behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryInspectionSummary {
    /// Inspected workspace root.
    pub root: String,

    /// Total top-level entries inspected.
    pub entry_count: usize,

    /// Number of top-level files.
    pub file_count: usize,

    /// Number of top-level directories.
    pub directory_count: usize,

    /// Number of top-level symbolic links.
    pub symlink_count: usize,

    /// Number of other top-level filesystem entries.
    pub other_count: usize,

    /// Entries that Monad classified into a known, specific category.
    pub known_entry_count: usize,

    /// Entries that remain unknown or generic.
    pub unknown_entry_count: usize,

    /// Entries marked as generated/external.
    pub generated_or_external_count: usize,

    /// Entries safe for future traversal.
    pub safe_for_future_traversal_count: usize,

    /// Entries that should only be inspected shallowly by default.
    pub inspect_shallow_only_count: usize,

    /// Entries that should be skipped during future deep traversal by default.
    pub skip_generated_or_external_count: usize,

    /// Planned traversal mode.
    pub future_traversal_mode: String,

    /// Planned maximum traversal depth.
    pub future_traversal_max_depth: usize,

    /// Whether future traversal should follow symbolic links.
    pub future_traversal_follow_symlinks: bool,

    /// Whether future traversal should include generated/external directories.
    pub future_traversal_include_generated_or_external: bool,

    /// Whether future traversal should respect ignore files.
    pub future_traversal_respect_ignore_files: bool,

    /// Whether future traversal output must be deterministic.
    pub future_traversal_deterministic_ordering: bool,

    /// Count of future traversal candidates.
    pub future_traversal_candidate_count: usize,

    /// Count of shallow-only planned traversal entries.
    pub future_traversal_shallow_only_count: usize,

    /// Count of planned skip entries.
    pub future_traversal_skip_count: usize,

    /// Category counts keyed by stable category label.
    pub category_counts: BTreeMap<String, usize>,

    /// Role counts keyed by stable role label.
    pub role_counts: BTreeMap<String, usize>,

    /// Traversal-policy counts keyed by stable traversal-policy label.
    pub traversal_policy_counts: BTreeMap<String, usize>,

    /// Renderable inspected entries.
    pub entries: Vec<RepositoryInspectionSummaryEntry>,
}

impl RepositoryInspectionSummary {
    /// Builds a renderable repository inspection summary from the domain model.
    #[must_use]
    pub fn from_inspection(inspection: &RepositoryInspection) -> Self {
        let traversal_plan = build_traversal_plan(inspection);
        let guardrails = traversal_plan.guardrails();

        let mut category_counts = BTreeMap::new();
        let mut role_counts = BTreeMap::new();
        let mut traversal_policy_counts = BTreeMap::new();

        let entries = inspection
            .entries()
            .iter()
            .zip(traversal_plan.entries().iter())
            .map(|(entry, plan_entry)| {
                let category = entry.category().as_str().to_string();
                let role = entry.role().as_str().to_string();
                let traversal_policy = entry.traversal_policy().as_str().to_string();

                *category_counts.entry(category.clone()).or_insert(0) += 1;
                *role_counts.entry(role.clone()).or_insert(0) += 1;
                *traversal_policy_counts
                    .entry(traversal_policy.clone())
                    .or_insert(0) += 1;

                RepositoryInspectionSummaryEntry {
                    path: entry.relative_path().display().to_string(),
                    kind: entry.kind().as_str().to_string(),
                    category,
                    role,
                    traversal_policy,
                    traversal_decision: plan_entry.decision().as_str().to_string(),
                    traversal_reason: plan_entry.reason().to_string(),
                }
            })
            .collect();

        let unknown_entry_count =
            count_entries_by_category(inspection, RepositoryEntryCategory::Other)
                + count_entries_by_category(inspection, RepositoryEntryCategory::Hidden);

        Self {
            root: inspection.root().display().to_string(),
            entry_count: inspection.entry_count(),
            file_count: count_entries_by_kind(inspection, RepositoryEntryKind::File),
            directory_count: count_entries_by_kind(inspection, RepositoryEntryKind::Directory),
            symlink_count: count_entries_by_kind(inspection, RepositoryEntryKind::Symlink),
            other_count: count_entries_by_kind(inspection, RepositoryEntryKind::Other),
            known_entry_count: inspection.entry_count().saturating_sub(unknown_entry_count),
            unknown_entry_count,
            generated_or_external_count: count_entries_by_category(
                inspection,
                RepositoryEntryCategory::GeneratedOrExternal,
            ),
            safe_for_future_traversal_count: count_entries_by_traversal_policy(
                inspection,
                RepositoryEntryTraversalPolicy::SafeForFutureTraversal,
            ),
            inspect_shallow_only_count: count_entries_by_traversal_policy(
                inspection,
                RepositoryEntryTraversalPolicy::InspectShallowOnly,
            ),
            skip_generated_or_external_count: count_entries_by_traversal_policy(
                inspection,
                RepositoryEntryTraversalPolicy::SkipGeneratedOrExternal,
            ),
            future_traversal_mode: traversal_plan.mode().as_str().to_string(),
            future_traversal_max_depth: guardrails.max_depth(),
            future_traversal_follow_symlinks: guardrails.follow_symlinks(),
            future_traversal_include_generated_or_external: guardrails
                .include_generated_or_external(),
            future_traversal_respect_ignore_files: guardrails.respect_ignore_files(),
            future_traversal_deterministic_ordering: guardrails.deterministic_ordering(),
            future_traversal_candidate_count: traversal_plan.candidate_for_future_traversal_count(),
            future_traversal_shallow_only_count: traversal_plan.inspect_shallow_only_count(),
            future_traversal_skip_count: traversal_plan.skip_by_default_count(),
            category_counts,
            role_counts,
            traversal_policy_counts,
            entries,
        }
    }
}

/// Counts entries with a specific filesystem kind.
fn count_entries_by_kind(inspection: &RepositoryInspection, kind: RepositoryEntryKind) -> usize {
    inspection
        .entries()
        .iter()
        .filter(|entry| entry.kind() == kind)
        .count()
}

/// Counts entries with a specific broad category.
fn count_entries_by_category(
    inspection: &RepositoryInspection,
    category: RepositoryEntryCategory,
) -> usize {
    inspection
        .entries()
        .iter()
        .filter(|entry| entry.category() == category)
        .count()
}

/// Counts entries with a specific traversal policy.
fn count_entries_by_traversal_policy(
    inspection: &RepositoryInspection,
    traversal_policy: RepositoryEntryTraversalPolicy,
) -> usize {
    inspection
        .entries()
        .iter()
        .filter(|entry| entry.traversal_policy() == traversal_policy)
        .count()
}

/// Converts a diagnostic severity into a stable JSON string.
fn severity_name(severity: Severity) -> &'static str {
    match severity {
        Severity::Info => "info",
        Severity::Warning => "warning",
        Severity::Error => "error",
    }
}

/// Renders a diagnostic report.
#[must_use]
pub fn render_diagnostic_report(report: &DiagnosticReport, format: OutputFormat) -> String {
    match format {
        OutputFormat::Text => report.render_lines().join("\n"),
        OutputFormat::Json => {
            let diagnostics = report
                .diagnostics()
                .iter()
                .map(|diagnostic| {
                    json!({
                        "severity": severity_name(diagnostic.severity),
                        "code": diagnostic.code,
                        "message": diagnostic.message,
                        "rendered": diagnostic.render(),
                    })
                })
                .collect::<Vec<_>>();

            serde_json::to_string_pretty(&json!({
                "format": OutputFormat::Json.as_str(),
                "kind": "diagnostic_report",
                "has_errors": report.has_errors(),
                "diagnostics": diagnostics,
            }))
            .expect("serializing diagnostic report JSON should not fail")
        }
    }
}

/// Renders workspace summary information.
#[must_use]
pub fn render_workspace_summary(summary: &WorkspaceSummary, format: OutputFormat) -> String {
    match format {
        OutputFormat::Text => format!(
            "Monad workspace\n  root: {}\n  project: {} ({})\n  schema_version: {}\n  core_crate: {}\n  cli_crate: {}\n  execution_model: {}",
            summary.root,
            summary.project_display_name,
            summary.project_name,
            summary.schema_version,
            summary.core_crate,
            summary.cli_crate,
            summary.execution_model,
        ),
        OutputFormat::Json => serde_json::to_string_pretty(&json!({
            "format": OutputFormat::Json.as_str(),
            "kind": "workspace_summary",
            "workspace": {
                "root": &summary.root,
            },
            "project": {
                "display_name": &summary.project_display_name,
                "name": &summary.project_name,
            },
            "manifest": {
                "schema_version": &summary.schema_version,
            },
            "runtime": {
                "core_crate": &summary.core_crate,
                "cli_crate": &summary.cli_crate,
                "execution_model": &summary.execution_model,
            },
        }))
        .expect("serializing workspace summary JSON should not fail"),
    }
}

/// Renders repository inspection summary information.
#[must_use]
pub fn render_repository_inspection_summary(
    summary: &RepositoryInspectionSummary,
    format: OutputFormat,
) -> String {
    match format {
        OutputFormat::Text => {
            let mut lines = vec![
                "Monad repository inspection".to_string(),
                format!("  root: {}", summary.root),
                format!("  entries: {}", summary.entry_count),
                format!("  files: {}", summary.file_count),
                format!("  directories: {}", summary.directory_count),
                format!("  symlinks: {}", summary.symlink_count),
                format!("  other: {}", summary.other_count),
                "  metrics:".to_string(),
                format!("    known_entries: {}", summary.known_entry_count),
                format!("    unknown_entries: {}", summary.unknown_entry_count),
                format!(
                    "    generated_or_external_entries: {}",
                    summary.generated_or_external_count
                ),
                format!(
                    "    safe_for_future_traversal: {}",
                    summary.safe_for_future_traversal_count
                ),
                format!(
                    "    inspect_shallow_only: {}",
                    summary.inspect_shallow_only_count
                ),
                format!(
                    "    skip_generated_or_external: {}",
                    summary.skip_generated_or_external_count
                ),
                "  future_traversal_guardrails:".to_string(),
                format!("    mode: {}", summary.future_traversal_mode),
                format!("    max_depth: {}", summary.future_traversal_max_depth),
                format!(
                    "    follow_symlinks: {}",
                    summary.future_traversal_follow_symlinks
                ),
                format!(
                    "    include_generated_or_external: {}",
                    summary.future_traversal_include_generated_or_external
                ),
                format!(
                    "    respect_ignore_files: {}",
                    summary.future_traversal_respect_ignore_files
                ),
                format!(
                    "    deterministic_ordering: {}",
                    summary.future_traversal_deterministic_ordering
                ),
                format!(
                    "    candidate_entries: {}",
                    summary.future_traversal_candidate_count
                ),
                format!(
                    "    shallow_only_entries: {}",
                    summary.future_traversal_shallow_only_count
                ),
                format!("    skip_entries: {}", summary.future_traversal_skip_count),
                "  categories:".to_string(),
            ];

            for (category, count) in &summary.category_counts {
                lines.push(format!("    {category}: {count}"));
            }

            lines.push("  roles:".to_string());

            for (role, count) in &summary.role_counts {
                lines.push(format!("    {role}: {count}"));
            }

            lines.push("  traversal_policies:".to_string());

            for (policy, count) in &summary.traversal_policy_counts {
                lines.push(format!("    {policy}: {count}"));
            }

            lines.push("  top_level_entries:".to_string());

            for entry in &summary.entries {
                lines.push(format!(
                    "    - {} [{} category={} role={} traversal={} decision={} reason={}]",
                    entry.path,
                    entry.kind,
                    entry.category,
                    entry.role,
                    entry.traversal_policy,
                    entry.traversal_decision,
                    entry.traversal_reason
                ));
            }

            lines.join("\n")
        }
        OutputFormat::Json => {
            let entries = summary
                .entries
                .iter()
                .map(|entry| {
                    json!({
                        "path": &entry.path,
                        "kind": &entry.kind,
                        "category": &entry.category,
                        "role": &entry.role,
                        "traversal_policy": &entry.traversal_policy,
                        "traversal_decision": &entry.traversal_decision,
                        "traversal_reason": &entry.traversal_reason,
                    })
                })
                .collect::<Vec<_>>();

            serde_json::to_string_pretty(&json!({
                "format": OutputFormat::Json.as_str(),
                "kind": "repository_inspection_summary",
                "repository": {
                    "root": &summary.root,
                    "entry_count": summary.entry_count,
                    "file_count": summary.file_count,
                    "directory_count": summary.directory_count,
                    "symlink_count": summary.symlink_count,
                    "other_count": summary.other_count,
                    "metrics": {
                        "known_entry_count": summary.known_entry_count,
                        "unknown_entry_count": summary.unknown_entry_count,
                        "generated_or_external_count": summary.generated_or_external_count,
                        "safe_for_future_traversal_count": summary.safe_for_future_traversal_count,
                        "inspect_shallow_only_count": summary.inspect_shallow_only_count,
                        "skip_generated_or_external_count": summary.skip_generated_or_external_count,
                    },
                    "future_traversal": {
                        "mode": &summary.future_traversal_mode,
                        "guardrails": {
                            "max_depth": summary.future_traversal_max_depth,
                            "follow_symlinks": summary.future_traversal_follow_symlinks,
                            "include_generated_or_external": summary.future_traversal_include_generated_or_external,
                            "respect_ignore_files": summary.future_traversal_respect_ignore_files,
                            "deterministic_ordering": summary.future_traversal_deterministic_ordering,
                        },
                        "candidate_entry_count": summary.future_traversal_candidate_count,
                        "shallow_only_entry_count": summary.future_traversal_shallow_only_count,
                        "skip_entry_count": summary.future_traversal_skip_count,
                    },
                    "category_counts": &summary.category_counts,
                    "role_counts": &summary.role_counts,
                    "traversal_policy_counts": &summary.traversal_policy_counts,
                    "entries": entries,
                },
            }))
            .expect("serializing repository inspection summary JSON should not fail")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{
        Diagnostic, ManifestProject, ManifestRuntime, ManifestSchemaVersion, ManifestWorkspace,
    };
    use crate::{WorkspaceContext, inspect_workspace};

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "monad-output-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_inspection_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);

        fs::create_dir_all(root.join("docs")).expect("docs directory should be created");
        fs::create_dir_all(root.join("work")).expect("work directory should be created");
        fs::create_dir_all(root.join(".monad")).expect(".monad directory should be created");
        fs::create_dir_all(root.join("crates")).expect("crates directory should be created");
        fs::create_dir_all(root.join("tools")).expect("tools directory should be created");
        fs::create_dir_all(root.join("target")).expect("target directory should be created");

        fs::write(root.join("README.md"), "# Test\n").expect("README should be written");
        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");
        fs::write(root.join("monad.toml"), "schema_version = 1\n")
            .expect("monad.toml should be written");

        root
    }

    #[test]
    fn output_format_parses_text_and_json() {
        assert_eq!(OutputFormat::parse("text"), Ok(OutputFormat::Text));
        assert_eq!(OutputFormat::parse("TEXT"), Ok(OutputFormat::Text));
        assert_eq!(OutputFormat::parse("json"), Ok(OutputFormat::Json));
        assert_eq!(OutputFormat::parse("JSON"), Ok(OutputFormat::Json));
    }

    #[test]
    fn unsupported_output_format_returns_error() {
        let error = OutputFormat::parse("xml").expect_err("xml is not supported yet");

        assert_eq!(error.code(), "MONAD2001");
        assert!(error.message().contains("unsupported output format"));
    }

    #[test]
    fn diagnostic_report_renders_as_text_lines() {
        let mut report = DiagnosticReport::new();

        report.push(Diagnostic::info("MONAD0001", "runtime ready"));
        report.push(Diagnostic::warning("MONAD1000", "review later"));

        let rendered = render_diagnostic_report(&report, OutputFormat::Text);

        assert!(rendered.contains("[INFO] MONAD0001: runtime ready"));
        assert!(rendered.contains("[WARNING] MONAD1000: review later"));
    }

    #[test]
    fn diagnostic_report_renders_as_json() {
        let mut report = DiagnosticReport::new();

        report.push(Diagnostic::info("MONAD0001", "runtime ready"));

        let rendered = render_diagnostic_report(&report, OutputFormat::Json);

        assert!(rendered.contains(r#""format": "json""#));
        assert!(rendered.contains(r#""kind": "diagnostic_report""#));
        assert!(rendered.contains(r#""has_errors": false"#));
        assert!(rendered.contains(r#""severity": "info""#));
        assert!(rendered.contains(r#""code": "MONAD0001""#));
        assert!(rendered.contains(r#""message": "runtime ready""#));
    }

    #[test]
    fn workspace_summary_renders_like_info_command() {
        let context = WorkspaceContext::new("/tmp/monad").expect("context should be created");
        let manifest = MonadManifest::new(
            ManifestSchemaVersion::current(),
            ManifestProject::new("monad", "Monad", "test"),
            ManifestWorkspace::default(),
            ManifestRuntime::new("monad-core", "monad-cli", "local-first"),
        );

        let summary = WorkspaceSummary::from_manifest(&context, &manifest);
        let rendered = render_workspace_summary(&summary, OutputFormat::Text);

        assert!(rendered.contains("Monad workspace"));
        assert!(rendered.contains("root: /tmp/monad"));
        assert!(rendered.contains("project: Monad (monad)"));
        assert!(rendered.contains("schema_version: 1"));
        assert!(rendered.contains("core_crate: monad-core"));
        assert!(rendered.contains("cli_crate: monad-cli"));
        assert!(rendered.contains("execution_model: local-first"));
    }

    #[test]
    fn workspace_summary_renders_as_json() {
        let context = WorkspaceContext::new("/tmp/monad").expect("context should be created");
        let manifest = MonadManifest::new(
            ManifestSchemaVersion::current(),
            ManifestProject::new("monad", "Monad", "test"),
            ManifestWorkspace::default(),
            ManifestRuntime::new("monad-core", "monad-cli", "local-first"),
        );

        let summary = WorkspaceSummary::from_manifest(&context, &manifest);
        let rendered = render_workspace_summary(&summary, OutputFormat::Json);

        assert!(rendered.contains(r#""format": "json""#));
        assert!(rendered.contains(r#""kind": "workspace_summary""#));
        assert!(rendered.contains(r#""root": "/tmp/monad""#));
        assert!(rendered.contains(r#""display_name": "Monad""#));
        assert!(rendered.contains(r#""name": "monad""#));
        assert!(rendered.contains(r#""core_crate": "monad-core""#));
        assert!(rendered.contains(r#""cli_crate": "monad-cli""#));
        assert!(rendered.contains(r#""execution_model": "local-first""#));
    }

    #[test]
    fn repository_inspection_summary_renders_as_text() {
        let root = create_inspection_workspace("inspection-text");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let summary = RepositoryInspectionSummary::from_inspection(&inspection);

        let rendered = render_repository_inspection_summary(&summary, OutputFormat::Text);

        assert!(rendered.contains("Monad repository inspection"));
        assert!(rendered.contains("metrics:"));
        assert!(rendered.contains("future_traversal_guardrails:"));
        assert!(rendered.contains("max_depth: 3"));
        assert!(rendered.contains("categories:"));
        assert!(rendered.contains("monad.toml"));
        assert!(rendered.contains("category=monad_control"));
        assert!(rendered.contains("role=monad_manifest"));
        assert!(rendered.contains("decision=inspect_shallow_only"));
        assert!(rendered.contains("skip_generated_or_external"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn repository_inspection_summary_renders_as_json() {
        let root = create_inspection_workspace("inspection-json");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let summary = RepositoryInspectionSummary::from_inspection(&inspection);

        let rendered = render_repository_inspection_summary(&summary, OutputFormat::Json);

        assert!(rendered.contains(r#""format": "json""#));
        assert!(rendered.contains(r#""kind": "repository_inspection_summary""#));
        assert!(rendered.contains(r#""metrics""#));
        assert!(rendered.contains(r#""future_traversal""#));
        assert!(rendered.contains(r#""max_depth": 3"#));
        assert!(rendered.contains(r#""follow_symlinks": false"#));
        assert!(rendered.contains(r#""respect_ignore_files": true"#));
        assert!(rendered.contains(r#""category_counts""#));
        assert!(rendered.contains(r#""entry_count""#));
        assert!(rendered.contains(r#""path": "monad.toml""#));
        assert!(rendered.contains(r#""category": "monad_control""#));
        assert!(rendered.contains(r#""role": "monad_manifest""#));
        assert!(rendered.contains(r#""traversal_decision""#));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn repository_inspection_summary_counts_entry_kinds() {
        let root = create_inspection_workspace("inspection-counts");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let summary = RepositoryInspectionSummary::from_inspection(&inspection);

        assert!(summary.entry_count >= 7);
        assert!(summary.file_count >= 3);
        assert!(summary.directory_count >= 4);
        assert_eq!(summary.symlink_count, 0);
        assert_eq!(summary.other_count, 0);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn repository_inspection_summary_counts_categories() {
        let root = create_inspection_workspace("inspection-category-counts");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let summary = RepositoryInspectionSummary::from_inspection(&inspection);

        assert!(summary.known_entry_count > 0);
        assert!(summary.generated_or_external_count >= 1);
        assert!(summary.safe_for_future_traversal_count >= 4);
        assert!(summary.inspect_shallow_only_count >= 3);
        assert!(summary.skip_generated_or_external_count >= 1);

        assert!(
            summary
                .category_counts
                .contains_key(RepositoryEntryCategory::MonadControl.as_str())
        );
        assert!(
            summary
                .category_counts
                .contains_key(RepositoryEntryCategory::Source.as_str())
        );
        assert!(
            summary
                .category_counts
                .contains_key(RepositoryEntryCategory::Tooling.as_str())
        );
        assert!(
            summary
                .category_counts
                .contains_key(RepositoryEntryCategory::GeneratedOrExternal.as_str())
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn repository_inspection_summary_includes_future_traversal_plan_counts() {
        let root = create_inspection_workspace("future-traversal-counts");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let summary = RepositoryInspectionSummary::from_inspection(&inspection);

        assert_eq!(summary.future_traversal_mode, "future_recursive_limited");
        assert_eq!(summary.future_traversal_max_depth, 3);
        assert!(!summary.future_traversal_follow_symlinks);
        assert!(!summary.future_traversal_include_generated_or_external);
        assert!(summary.future_traversal_respect_ignore_files);
        assert!(summary.future_traversal_deterministic_ordering);
        assert!(summary.future_traversal_candidate_count > 0);
        assert!(summary.future_traversal_shallow_only_count > 0);
        assert!(summary.future_traversal_skip_count > 0);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn repository_role_enum_is_still_available_for_future_output_work() {
        assert_eq!(
            RepositoryEntryRole::MonadManifest.as_str(),
            "monad_manifest"
        );
        assert_eq!(
            RepositoryEntryTraversalPolicy::SafeForFutureTraversal.as_str(),
            "safe_for_future_traversal"
        );
    }
}
