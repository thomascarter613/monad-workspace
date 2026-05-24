//! Output formatting primitives for Monad.
//!
//! The CLI should stay thin. It may decide when to print, but reusable
//! formatting rules should live in `monad-core` so future commands produce
//! consistent output.

use std::collections::BTreeMap;

use serde_json::json;

use crate::repository_graph::{RepositoryGraph, build_repository_graph};
use crate::repository_inspection::{
    RepositoryBoundedTraversal, RepositoryEntryCategory, RepositoryEntryKind,
    RepositoryEntryTraversalPolicy, RepositoryInspection, build_traversal_plan,
};
use crate::{DiagnosticReport, MonadError, MonadManifest, MonadResult, Severity, WorkspaceContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
}

impl OutputFormat {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Json => "json",
        }
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceSummary {
    pub root: String,
    pub project_display_name: String,
    pub project_name: String,
    pub schema_version: String,
    pub core_crate: String,
    pub cli_crate: String,
    pub execution_model: String,
}

impl WorkspaceSummary {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryInspectionSummaryEntry {
    pub path: String,
    pub kind: String,
    pub category: String,
    pub role: String,
    pub traversal_policy: String,
    pub traversal_decision: String,
    pub traversal_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryInspectionSummary {
    pub root: String,
    pub entry_count: usize,
    pub file_count: usize,
    pub directory_count: usize,
    pub symlink_count: usize,
    pub other_count: usize,
    pub known_entry_count: usize,
    pub unknown_entry_count: usize,
    pub generated_or_external_count: usize,
    pub safe_for_future_traversal_count: usize,
    pub inspect_shallow_only_count: usize,
    pub skip_generated_or_external_count: usize,
    pub future_traversal_mode: String,
    pub future_traversal_max_depth: usize,
    pub future_traversal_follow_symlinks: bool,
    pub future_traversal_include_generated_or_external: bool,
    pub future_traversal_respect_ignore_files: bool,
    pub future_traversal_deterministic_ordering: bool,
    pub future_traversal_candidate_count: usize,
    pub future_traversal_shallow_only_count: usize,
    pub future_traversal_skip_count: usize,
    pub bounded_traversal_mode: String,
    pub bounded_traversal_entry_count: usize,
    pub bounded_traversal_max_observed_depth: usize,
    pub bounded_traversal_candidate_count: usize,
    pub bounded_traversal_shallow_only_count: usize,
    pub bounded_traversal_skip_count: usize,
    pub bounded_traversal_generated_or_external_count: usize,
    pub graph_node_count: usize,
    pub graph_edge_count: usize,
    pub graph_max_depth: usize,
    pub graph_category_counts: BTreeMap<String, usize>,
    pub graph_traversal_decision_counts: BTreeMap<String, usize>,
    pub category_counts: BTreeMap<String, usize>,
    pub role_counts: BTreeMap<String, usize>,
    pub traversal_policy_counts: BTreeMap<String, usize>,
    pub entries: Vec<RepositoryInspectionSummaryEntry>,
}

impl RepositoryInspectionSummary {
    #[must_use]
    pub fn from_inspection(inspection: &RepositoryInspection) -> Self {
        Self::from_parts(inspection, None, None)
    }

    #[must_use]
    pub fn from_inspection_and_bounded_traversal(
        inspection: &RepositoryInspection,
        bounded_traversal: &RepositoryBoundedTraversal,
    ) -> Self {
        let graph = build_repository_graph(bounded_traversal);

        Self::from_parts(inspection, Some(bounded_traversal), Some(&graph))
    }

    #[must_use]
    pub fn from_inspection_bounded_traversal_and_graph(
        inspection: &RepositoryInspection,
        bounded_traversal: &RepositoryBoundedTraversal,
        graph: &RepositoryGraph,
    ) -> Self {
        Self::from_parts(inspection, Some(bounded_traversal), Some(graph))
    }

    fn from_parts(
        inspection: &RepositoryInspection,
        bounded_traversal: Option<&RepositoryBoundedTraversal>,
        graph: Option<&RepositoryGraph>,
    ) -> Self {
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
            bounded_traversal_mode: bounded_traversal
                .map(|traversal| traversal.mode().as_str().to_string())
                .unwrap_or_else(|| "not_run".to_string()),
            bounded_traversal_entry_count: bounded_traversal
                .map(RepositoryBoundedTraversal::entry_count)
                .unwrap_or(0),
            bounded_traversal_max_observed_depth: bounded_traversal
                .map(RepositoryBoundedTraversal::max_observed_depth)
                .unwrap_or(0),
            bounded_traversal_candidate_count: bounded_traversal
                .map(RepositoryBoundedTraversal::candidate_count)
                .unwrap_or(0),
            bounded_traversal_shallow_only_count: bounded_traversal
                .map(RepositoryBoundedTraversal::shallow_only_count)
                .unwrap_or(0),
            bounded_traversal_skip_count: bounded_traversal
                .map(RepositoryBoundedTraversal::skip_count)
                .unwrap_or(0),
            bounded_traversal_generated_or_external_count: bounded_traversal
                .map(|traversal| {
                    traversal.category_count(RepositoryEntryCategory::GeneratedOrExternal)
                })
                .unwrap_or(0),
            graph_node_count: graph.map(RepositoryGraph::node_count).unwrap_or(0),
            graph_edge_count: graph.map(RepositoryGraph::edge_count).unwrap_or(0),
            graph_max_depth: graph.map(RepositoryGraph::max_depth).unwrap_or(0),
            graph_category_counts: graph
                .map(RepositoryGraph::category_counts)
                .unwrap_or_default(),
            graph_traversal_decision_counts: graph
                .map(RepositoryGraph::traversal_decision_counts)
                .unwrap_or_default(),
            category_counts,
            role_counts,
            traversal_policy_counts,
            entries,
        }
    }
}

fn count_entries_by_kind(inspection: &RepositoryInspection, kind: RepositoryEntryKind) -> usize {
    inspection
        .entries()
        .iter()
        .filter(|entry| entry.kind() == kind)
        .count()
}

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

fn severity_name(severity: Severity) -> &'static str {
    match severity {
        Severity::Info => "info",
        Severity::Warning => "warning",
        Severity::Error => "error",
    }
}

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
                "  bounded_traversal:".to_string(),
                format!("    mode: {}", summary.bounded_traversal_mode),
                format!("    entries: {}", summary.bounded_traversal_entry_count),
                format!(
                    "    max_observed_depth: {}",
                    summary.bounded_traversal_max_observed_depth
                ),
                format!(
                    "    candidate_entries: {}",
                    summary.bounded_traversal_candidate_count
                ),
                format!(
                    "    shallow_only_entries: {}",
                    summary.bounded_traversal_shallow_only_count
                ),
                format!("    skip_entries: {}", summary.bounded_traversal_skip_count),
                format!(
                    "    generated_or_external_entries: {}",
                    summary.bounded_traversal_generated_or_external_count
                ),
                "  graph:".to_string(),
                format!("    nodes: {}", summary.graph_node_count),
                format!("    edges: {}", summary.graph_edge_count),
                format!("    max_depth: {}", summary.graph_max_depth),
                "  graph_categories:".to_string(),
            ];

            for (category, count) in &summary.graph_category_counts {
                lines.push(format!("    {category}: {count}"));
            }

            lines.push("  graph_traversal_decisions:".to_string());

            for (decision, count) in &summary.graph_traversal_decision_counts {
                lines.push(format!("    {decision}: {count}"));
            }

            lines.push("  categories:".to_string());

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
                    "bounded_traversal": {
                        "mode": &summary.bounded_traversal_mode,
                        "entry_count": summary.bounded_traversal_entry_count,
                        "max_observed_depth": summary.bounded_traversal_max_observed_depth,
                        "candidate_entry_count": summary.bounded_traversal_candidate_count,
                        "shallow_only_entry_count": summary.bounded_traversal_shallow_only_count,
                        "skip_entry_count": summary.bounded_traversal_skip_count,
                        "generated_or_external_entry_count": summary.bounded_traversal_generated_or_external_count,
                    },
                    "graph": {
                        "node_count": summary.graph_node_count,
                        "edge_count": summary.graph_edge_count,
                        "max_depth": summary.graph_max_depth,
                        "category_counts": &summary.graph_category_counts,
                        "traversal_decision_counts": &summary.graph_traversal_decision_counts,
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
    use crate::{
        WorkspaceContext, build_repository_graph, inspect_workspace, traverse_workspace_bounded,
    };

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

        fs::create_dir_all(root.join("docs/guide")).expect("docs directory should be created");
        fs::create_dir_all(root.join("work")).expect("work directory should be created");
        fs::create_dir_all(root.join(".monad")).expect(".monad directory should be created");
        fs::create_dir_all(root.join("crates/monad-core/src"))
            .expect("crates directory should be created");
        fs::create_dir_all(root.join("tools")).expect("tools directory should be created");
        fs::create_dir_all(root.join("target")).expect("target directory should be created");

        fs::write(root.join("README.md"), "# Test\n").expect("README should be written");
        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");
        fs::write(root.join("monad.toml"), "schema_version = 1\n")
            .expect("monad.toml should be written");
        fs::write(root.join("docs/guide/intro.md"), "# Intro\n").expect("intro should be written");
        fs::write(
            root.join("crates/monad-core/src/lib.rs"),
            "pub fn test() {}\n",
        )
        .expect("lib should be written");

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
    }

    #[test]
    fn repository_inspection_summary_includes_graph_metrics() {
        let root = create_inspection_workspace("graph-summary");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let bounded =
            traverse_workspace_bounded(&inspection).expect("bounded traversal should run");
        let graph = build_repository_graph(&bounded);
        let summary = RepositoryInspectionSummary::from_inspection_bounded_traversal_and_graph(
            &inspection,
            &bounded,
            &graph,
        );

        assert!(summary.graph_node_count > 0);
        assert!(summary.graph_edge_count > 0);
        assert!(summary.graph_max_depth > 0);
        assert!(summary.graph_category_counts.contains_key("source"));
        assert!(
            summary
                .graph_traversal_decision_counts
                .contains_key("candidate_for_future_traversal")
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn repository_inspection_summary_renders_graph_metrics_as_text() {
        let root = create_inspection_workspace("graph-text");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let bounded =
            traverse_workspace_bounded(&inspection).expect("bounded traversal should run");
        let graph = build_repository_graph(&bounded);
        let summary = RepositoryInspectionSummary::from_inspection_bounded_traversal_and_graph(
            &inspection,
            &bounded,
            &graph,
        );

        let rendered = render_repository_inspection_summary(&summary, OutputFormat::Text);

        assert!(rendered.contains("graph:"));
        assert!(rendered.contains("nodes:"));
        assert!(rendered.contains("edges:"));
        assert!(rendered.contains("graph_categories:"));
        assert!(rendered.contains("graph_traversal_decisions:"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn repository_inspection_summary_renders_graph_metrics_as_json() {
        let root = create_inspection_workspace("graph-json");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let bounded =
            traverse_workspace_bounded(&inspection).expect("bounded traversal should run");
        let graph = build_repository_graph(&bounded);
        let summary = RepositoryInspectionSummary::from_inspection_bounded_traversal_and_graph(
            &inspection,
            &bounded,
            &graph,
        );

        let rendered = render_repository_inspection_summary(&summary, OutputFormat::Json);

        assert!(rendered.contains(r#""graph""#));
        assert!(rendered.contains(r#""node_count""#));
        assert!(rendered.contains(r#""edge_count""#));
        assert!(rendered.contains(r#""category_counts""#));
        assert!(rendered.contains(r#""traversal_decision_counts""#));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn repository_role_enum_is_still_available_for_future_output_work() {
        assert_eq!(
            crate::repository_inspection::RepositoryEntryRole::MonadManifest.as_str(),
            "monad_manifest"
        );
        assert_eq!(
            RepositoryEntryTraversalPolicy::SafeForFutureTraversal.as_str(),
            "safe_for_future_traversal"
        );
    }
}
