//! Repository context pack foundation for Monad.
//!
//! WP-E2-013 introduces an AI-readable repository context pack model.
//!
//! The context pack is intentionally built in `monad-core` rather than the CLI.
//! The CLI can later expose it through a command such as:
//!
//! - `monad context`
//! - `monad context --format=json`
//! - `monad context --format=markdown`
//!
//! This slice does not add a new CLI command yet.
//!
//! A repository context pack aggregates the repository intelligence produced by
//! earlier E2 slices:
//!
//! - shallow inspection;
//! - bounded traversal;
//! - graph model;
//! - toolchain detection;
//! - dependency signal detection;
//! - advisory policy diagnostics.

use serde_json::json;

use crate::{
    MonadError, MonadResult, RepositoryBoundedTraversal, RepositoryDependencyDetection,
    RepositoryGraph, RepositoryInspection, RepositoryPolicyReport, RepositoryToolchainDetection,
};

/// Current context-pack schema version.
///
/// This value is intentionally separate from `monad.toml` schema version.
/// The context pack is an AI-facing/read-model artifact and can evolve on a
/// different cadence from the repository manifest.
pub const CURRENT_REPOSITORY_CONTEXT_PACK_SCHEMA_VERSION: u16 = 1;

/// Supported repository context pack render formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryContextPackRenderFormat {
    /// Markdown designed for human and LLM reading.
    Markdown,

    /// Machine-readable JSON.
    Json,
}

impl RepositoryContextPackRenderFormat {
    /// Returns a stable format label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Markdown => "markdown",
            Self::Json => "json",
        }
    }

    /// Parses a context pack render format.
    pub fn parse(value: &str) -> MonadResult<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "markdown" | "md" | "text" => Ok(Self::Markdown),
            "json" => Ok(Self::Json),
            other => Err(MonadError::invalid_input(format!(
                "unsupported repository context pack render format: {other}"
            ))),
        }
    }
}

impl Default for RepositoryContextPackRenderFormat {
    fn default() -> Self {
        Self::Markdown
    }
}

/// Stable section kinds in a repository context pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryContextPackSectionKind {
    /// Repository-level overview.
    Overview,

    /// Bounded traversal summary.
    Traversal,

    /// Repository graph summary.
    Graph,

    /// Toolchain detection summary.
    Toolchains,

    /// Dependency signal summary.
    Dependencies,

    /// Advisory policy diagnostics.
    Policy,

    /// Top-level repository entries.
    TopLevelEntries,
}

impl RepositoryContextPackSectionKind {
    /// Returns a stable section label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Traversal => "traversal",
            Self::Graph => "graph",
            Self::Toolchains => "toolchains",
            Self::Dependencies => "dependencies",
            Self::Policy => "policy",
            Self::TopLevelEntries => "top_level_entries",
        }
    }
}

/// One structured context-pack fact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryContextPackFact {
    key: String,
    value: String,
}

impl RepositoryContextPackFact {
    /// Creates a context-pack fact.
    #[must_use]
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Returns the fact key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the fact value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// One context-pack section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryContextPackSection {
    kind: RepositoryContextPackSectionKind,
    title: String,
    facts: Vec<RepositoryContextPackFact>,
    notes: Vec<String>,
}

impl RepositoryContextPackSection {
    /// Creates a context-pack section.
    #[must_use]
    pub fn new(
        kind: RepositoryContextPackSectionKind,
        title: impl Into<String>,
        facts: Vec<RepositoryContextPackFact>,
        notes: Vec<String>,
    ) -> Self {
        Self {
            kind,
            title: title.into(),
            facts,
            notes,
        }
    }

    /// Returns the section kind.
    #[must_use]
    pub const fn kind(&self) -> RepositoryContextPackSectionKind {
        self.kind
    }

    /// Returns the section title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns section facts.
    #[must_use]
    pub fn facts(&self) -> &[RepositoryContextPackFact] {
        &self.facts
    }

    /// Returns section notes.
    #[must_use]
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    /// Returns a fact value by key when present.
    #[must_use]
    pub fn fact_value(&self, key: &str) -> Option<&str> {
        self.facts
            .iter()
            .find(|fact| fact.key() == key)
            .map(RepositoryContextPackFact::value)
    }
}

/// AI-readable repository context pack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryContextPack {
    schema_version: u16,
    root: String,
    sections: Vec<RepositoryContextPackSection>,
}

impl RepositoryContextPack {
    /// Creates a repository context pack.
    #[must_use]
    pub fn new(
        schema_version: u16,
        root: impl Into<String>,
        sections: Vec<RepositoryContextPackSection>,
    ) -> Self {
        Self {
            schema_version,
            root: root.into(),
            sections,
        }
    }

    /// Returns the context-pack schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the repository root string captured in the pack.
    #[must_use]
    pub fn root(&self) -> &str {
        &self.root
    }

    /// Returns all context-pack sections.
    #[must_use]
    pub fn sections(&self) -> &[RepositoryContextPackSection] {
        &self.sections
    }

    /// Returns the section count.
    #[must_use]
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }

    /// Returns the total fact count.
    #[must_use]
    pub fn fact_count(&self) -> usize {
        self.sections
            .iter()
            .map(|section| section.facts().len())
            .sum()
    }

    /// Finds a section by kind.
    #[must_use]
    pub fn section(
        &self,
        kind: RepositoryContextPackSectionKind,
    ) -> Option<&RepositoryContextPackSection> {
        self.sections.iter().find(|section| section.kind() == kind)
    }

    /// Finds a fact value by section kind and key.
    #[must_use]
    pub fn fact_value(
        &self,
        section_kind: RepositoryContextPackSectionKind,
        key: &str,
    ) -> Option<&str> {
        self.section(section_kind)
            .and_then(|section| section.fact_value(key))
    }

    /// Returns true if the policy section says warnings are present.
    #[must_use]
    pub fn has_policy_warnings(&self) -> bool {
        self.fact_value(RepositoryContextPackSectionKind::Policy, "has_warnings") == Some("true")
    }
}

/// Builds an AI-readable repository context pack from existing repository intelligence.
#[must_use]
pub fn build_repository_context_pack(
    inspection: &RepositoryInspection,
    bounded_traversal: &RepositoryBoundedTraversal,
    graph: &RepositoryGraph,
    toolchains: &RepositoryToolchainDetection,
    dependencies: &RepositoryDependencyDetection,
    policy: &RepositoryPolicyReport,
) -> RepositoryContextPack {
    let sections = vec![
        build_overview_section(inspection),
        build_traversal_section(bounded_traversal),
        build_graph_section(graph),
        build_toolchain_section(toolchains),
        build_dependency_section(dependencies),
        build_policy_section(policy),
        build_top_level_entries_section(inspection),
    ];

    RepositoryContextPack::new(
        CURRENT_REPOSITORY_CONTEXT_PACK_SCHEMA_VERSION,
        inspection.root().display().to_string(),
        sections,
    )
}

/// Renders a repository context pack.
#[must_use]
pub fn render_repository_context_pack(
    pack: &RepositoryContextPack,
    format: RepositoryContextPackRenderFormat,
) -> String {
    match format {
        RepositoryContextPackRenderFormat::Markdown => {
            render_repository_context_pack_markdown(pack)
        }
        RepositoryContextPackRenderFormat::Json => render_repository_context_pack_json(pack),
    }
}

fn build_overview_section(inspection: &RepositoryInspection) -> RepositoryContextPackSection {
    RepositoryContextPackSection::new(
        RepositoryContextPackSectionKind::Overview,
        "Repository Overview",
        vec![
            RepositoryContextPackFact::new("root", inspection.root().display().to_string()),
            RepositoryContextPackFact::new(
                "top_level_entry_count",
                inspection.entry_count().to_string(),
            ),
            RepositoryContextPackFact::new(
                "top_level_file_count",
                inspection.file_count().to_string(),
            ),
            RepositoryContextPackFact::new(
                "top_level_directory_count",
                inspection.directory_count().to_string(),
            ),
        ],
        vec![
            "This section summarizes the shallow top-level repository inspection.".to_string(),
            "Counts are intentionally based on the top-level inspection, not full traversal."
                .to_string(),
        ],
    )
}

fn build_traversal_section(
    bounded_traversal: &RepositoryBoundedTraversal,
) -> RepositoryContextPackSection {
    let guardrails = bounded_traversal.guardrails();

    RepositoryContextPackSection::new(
        RepositoryContextPackSectionKind::Traversal,
        "Bounded Traversal",
        vec![
            RepositoryContextPackFact::new("mode", bounded_traversal.mode().as_str()),
            RepositoryContextPackFact::new("entry_count", bounded_traversal.entry_count().to_string()),
            RepositoryContextPackFact::new(
                "max_observed_depth",
                bounded_traversal.max_observed_depth().to_string(),
            ),
            RepositoryContextPackFact::new("max_allowed_depth", guardrails.max_depth().to_string()),
            RepositoryContextPackFact::new("follow_symlinks", guardrails.follow_symlinks().to_string()),
            RepositoryContextPackFact::new(
                "include_generated_or_external",
                guardrails.include_generated_or_external().to_string(),
            ),
            RepositoryContextPackFact::new(
                "respect_ignore_files",
                guardrails.respect_ignore_files().to_string(),
            ),
            RepositoryContextPackFact::new(
                "deterministic_ordering",
                guardrails.deterministic_ordering().to_string(),
            ),
            RepositoryContextPackFact::new("candidate_count", bounded_traversal.candidate_count().to_string()),
            RepositoryContextPackFact::new("shallow_only_count", bounded_traversal.shallow_only_count().to_string()),
            RepositoryContextPackFact::new("skip_count", bounded_traversal.skip_count().to_string()),
        ],
        vec![
            "Bounded traversal is conservative by default.".to_string(),
            "Generated/external paths are recorded but not descended into unless future policy explicitly allows it.".to_string(),
        ],
    )
}

fn build_graph_section(graph: &RepositoryGraph) -> RepositoryContextPackSection {
    let mut notes = Vec::new();

    for (category, count) in graph.category_counts() {
        notes.push(format!("graph_category:{category}={count}"));
    }

    for (decision, count) in graph.traversal_decision_counts() {
        notes.push(format!("graph_traversal_decision:{decision}={count}"));
    }

    RepositoryContextPackSection::new(
        RepositoryContextPackSectionKind::Graph,
        "Repository Graph",
        vec![
            RepositoryContextPackFact::new("node_count", graph.node_count().to_string()),
            RepositoryContextPackFact::new("edge_count", graph.edge_count().to_string()),
            RepositoryContextPackFact::new("max_depth", graph.max_depth().to_string()),
        ],
        notes,
    )
}

fn build_toolchain_section(
    toolchains: &RepositoryToolchainDetection,
) -> RepositoryContextPackSection {
    let mut notes = Vec::new();

    for (toolchain, count) in toolchains.toolchain_counts() {
        notes.push(format!("toolchain:{toolchain}={count}"));
    }

    for (signal_kind, count) in toolchains.signal_kind_counts() {
        notes.push(format!("signal_kind:{signal_kind}={count}"));
    }

    RepositoryContextPackSection::new(
        RepositoryContextPackSectionKind::Toolchains,
        "Toolchains",
        vec![
            RepositoryContextPackFact::new(
                "detected_toolchain_count",
                toolchains.detected_toolchain_count().to_string(),
            ),
            RepositoryContextPackFact::new("signal_count", toolchains.signal_count().to_string()),
        ],
        notes,
    )
}

fn build_dependency_section(
    dependencies: &RepositoryDependencyDetection,
) -> RepositoryContextPackSection {
    let mut notes = Vec::new();

    for (toolchain, count) in dependencies.toolchain_counts() {
        notes.push(format!("dependency_toolchain:{toolchain}={count}"));
    }

    for (signal_kind, count) in dependencies.signal_kind_counts() {
        notes.push(format!("dependency_signal_kind:{signal_kind}={count}"));
    }

    RepositoryContextPackSection::new(
        RepositoryContextPackSectionKind::Dependencies,
        "Dependency Signals",
        vec![
            RepositoryContextPackFact::new(
                "detected_toolchain_count",
                dependencies.detected_toolchain_count().to_string(),
            ),
            RepositoryContextPackFact::new("signal_count", dependencies.signal_count().to_string()),
            RepositoryContextPackFact::new(
                "manifest_count",
                dependencies.manifest_signal_count().to_string(),
            ),
            RepositoryContextPackFact::new(
                "lockfile_count",
                dependencies.lockfile_signal_count().to_string(),
            ),
            RepositoryContextPackFact::new(
                "package_manager_config_count",
                dependencies
                    .package_manager_config_signal_count()
                    .to_string(),
            ),
            RepositoryContextPackFact::new(
                "build_file_count",
                dependencies.build_file_signal_count().to_string(),
            ),
        ],
        notes,
    )
}

fn build_policy_section(policy: &RepositoryPolicyReport) -> RepositoryContextPackSection {
    let notes = policy
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            format!(
                "{} [{}] {}",
                diagnostic.code(),
                diagnostic.severity().as_str(),
                diagnostic.message()
            )
        })
        .collect::<Vec<_>>();

    RepositoryContextPackSection::new(
        RepositoryContextPackSectionKind::Policy,
        "Repository Intelligence Policy",
        vec![
            RepositoryContextPackFact::new(
                "diagnostic_count",
                policy.diagnostic_count().to_string(),
            ),
            RepositoryContextPackFact::new("info_count", policy.info_count().to_string()),
            RepositoryContextPackFact::new("advisory_count", policy.advisory_count().to_string()),
            RepositoryContextPackFact::new("warning_count", policy.warning_count().to_string()),
            RepositoryContextPackFact::new("has_warnings", policy.has_warnings().to_string()),
        ],
        notes,
    )
}

fn build_top_level_entries_section(
    inspection: &RepositoryInspection,
) -> RepositoryContextPackSection {
    let notes = inspection
        .entries()
        .iter()
        .map(|entry| {
            format!(
                "{} kind={} category={} role={} traversal={}",
                entry.relative_path().display(),
                entry.kind().as_str(),
                entry.category().as_str(),
                entry.role().as_str(),
                entry.traversal_policy().as_str()
            )
        })
        .collect::<Vec<_>>();

    RepositoryContextPackSection::new(
        RepositoryContextPackSectionKind::TopLevelEntries,
        "Top-Level Entries",
        vec![RepositoryContextPackFact::new(
            "entry_count",
            inspection.entry_count().to_string(),
        )],
        notes,
    )
}

fn render_repository_context_pack_markdown(pack: &RepositoryContextPack) -> String {
    let mut lines = vec![
        "# Monad Repository Context Pack".to_string(),
        String::new(),
        format!("- schema_version: {}", pack.schema_version()),
        format!("- root: {}", pack.root()),
        format!("- sections: {}", pack.section_count()),
        format!("- facts: {}", pack.fact_count()),
    ];

    for section in pack.sections() {
        lines.push(String::new());
        lines.push(format!("## {}", section.title()));
        lines.push(String::new());
        lines.push(format!("- kind: {}", section.kind().as_str()));

        for fact in section.facts() {
            lines.push(format!("- {}: {}", fact.key(), fact.value()));
        }

        if !section.notes().is_empty() {
            lines.push(String::new());
            lines.push("Notes:".to_string());

            for note in section.notes() {
                lines.push(format!("- {note}"));
            }
        }
    }

    lines.join("\n")
}

fn render_repository_context_pack_json(pack: &RepositoryContextPack) -> String {
    let sections = pack
        .sections()
        .iter()
        .map(|section| {
            let facts = section
                .facts()
                .iter()
                .map(|fact| {
                    json!({
                        "key": fact.key(),
                        "value": fact.value(),
                    })
                })
                .collect::<Vec<_>>();

            json!({
                "kind": section.kind().as_str(),
                "title": section.title(),
                "facts": facts,
                "notes": section.notes(),
            })
        })
        .collect::<Vec<_>>();

    serde_json::to_string_pretty(&json!({
        "kind": "repository_context_pack",
        "schema_version": pack.schema_version(),
        "root": pack.root(),
        "section_count": pack.section_count(),
        "fact_count": pack.fact_count(),
        "has_policy_warnings": pack.has_policy_warnings(),
        "sections": sections,
    }))
    .expect("serializing repository context pack JSON should not fail")
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{
        WorkspaceContext, build_repository_graph, detect_repository_dependency_signals,
        detect_repository_toolchains, evaluate_repository_intelligence_policy, inspect_workspace,
        traverse_workspace_bounded,
    };

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "monad-context-pack-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_context_pack_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);

        fs::create_dir_all(root.join("crates/monad-core/src"))
            .expect("Rust source directory should be created");
        fs::create_dir_all(root.join("apps/web/src"))
            .expect("web source directory should be created");
        fs::create_dir_all(root.join("target/debug")).expect("target should be created");

        fs::write(root.join("README.md"), "# Monad\n").expect("README should be written");
        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");
        fs::write(root.join("Cargo.lock"), "# lock\n").expect("Cargo.lock should be written");
        fs::write(root.join("package.json"), "{}\n").expect("package.json should be written");
        fs::write(root.join("apps/web/src/main.ts"), "export {}\n")
            .expect("TypeScript source should be written");
        fs::write(root.join("target/debug/cache.bin"), "cache\n").expect("cache should be written");

        root
    }

    fn build_pack_from_workspace(root: &Path) -> RepositoryContextPack {
        let context = WorkspaceContext::new(root).expect("context should be created");
        let inspection = inspect_workspace(&context).expect("inspection should run");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should run");
        let graph = build_repository_graph(&traversal);
        let toolchains = detect_repository_toolchains(&traversal);
        let dependencies = detect_repository_dependency_signals(&traversal);
        let policy =
            evaluate_repository_intelligence_policy(&inspection, &traversal, &dependencies);

        build_repository_context_pack(
            &inspection,
            &traversal,
            &graph,
            &toolchains,
            &dependencies,
            &policy,
        )
    }

    #[test]
    fn context_pack_render_format_parses_supported_formats() {
        assert_eq!(
            RepositoryContextPackRenderFormat::parse("markdown"),
            Ok(RepositoryContextPackRenderFormat::Markdown)
        );
        assert_eq!(
            RepositoryContextPackRenderFormat::parse("md"),
            Ok(RepositoryContextPackRenderFormat::Markdown)
        );
        assert_eq!(
            RepositoryContextPackRenderFormat::parse("text"),
            Ok(RepositoryContextPackRenderFormat::Markdown)
        );
        assert_eq!(
            RepositoryContextPackRenderFormat::parse("json"),
            Ok(RepositoryContextPackRenderFormat::Json)
        );
    }

    #[test]
    fn context_pack_render_format_rejects_unsupported_formats() {
        let error = RepositoryContextPackRenderFormat::parse("xml")
            .expect_err("xml is not supported for context packs");

        assert_eq!(error.code(), "MONAD2001");
        assert!(
            error
                .message()
                .contains("unsupported repository context pack render format")
        );
    }

    #[test]
    fn context_pack_contains_expected_sections() {
        let root = create_context_pack_workspace("sections");
        let pack = build_pack_from_workspace(&root);

        assert_eq!(
            pack.schema_version(),
            CURRENT_REPOSITORY_CONTEXT_PACK_SCHEMA_VERSION
        );
        assert_eq!(pack.section_count(), 7);
        assert!(pack.fact_count() > 0);
        assert!(
            pack.section(RepositoryContextPackSectionKind::Overview)
                .is_some()
        );
        assert!(
            pack.section(RepositoryContextPackSectionKind::Traversal)
                .is_some()
        );
        assert!(
            pack.section(RepositoryContextPackSectionKind::Graph)
                .is_some()
        );
        assert!(
            pack.section(RepositoryContextPackSectionKind::Toolchains)
                .is_some()
        );
        assert!(
            pack.section(RepositoryContextPackSectionKind::Dependencies)
                .is_some()
        );
        assert!(
            pack.section(RepositoryContextPackSectionKind::Policy)
                .is_some()
        );
        assert!(
            pack.section(RepositoryContextPackSectionKind::TopLevelEntries)
                .is_some()
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn context_pack_exposes_facts_by_section_and_key() {
        let root = create_context_pack_workspace("facts");
        let pack = build_pack_from_workspace(&root);

        assert_eq!(
            pack.fact_value(
                RepositoryContextPackSectionKind::Traversal,
                "max_allowed_depth"
            ),
            Some("3")
        );
        assert_eq!(
            pack.fact_value(RepositoryContextPackSectionKind::Graph, "node_count")
                .is_some(),
            true
        );
        assert_eq!(
            pack.fact_value(
                RepositoryContextPackSectionKind::Dependencies,
                "manifest_count"
            )
            .is_some(),
            true
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn context_pack_reports_policy_warnings() {
        let root = create_context_pack_workspace("policy-warning");
        let pack = build_pack_from_workspace(&root);

        assert!(pack.has_policy_warnings());
        assert_eq!(
            pack.fact_value(RepositoryContextPackSectionKind::Policy, "has_warnings"),
            Some("true")
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn context_pack_renders_as_markdown() {
        let root = create_context_pack_workspace("markdown");
        let pack = build_pack_from_workspace(&root);

        let rendered =
            render_repository_context_pack(&pack, RepositoryContextPackRenderFormat::Markdown);

        assert!(rendered.starts_with("# Monad Repository Context Pack"));
        assert!(rendered.contains("## Repository Overview"));
        assert!(rendered.contains("## Bounded Traversal"));
        assert!(rendered.contains("## Repository Intelligence Policy"));
        assert!(rendered.contains("- schema_version: 1"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn context_pack_renders_as_json() {
        let root = create_context_pack_workspace("json");
        let pack = build_pack_from_workspace(&root);

        let rendered =
            render_repository_context_pack(&pack, RepositoryContextPackRenderFormat::Json);

        assert!(rendered.contains(r#""kind": "repository_context_pack""#));
        assert!(rendered.contains(r#""schema_version": 1"#));
        assert!(rendered.contains(r#""sections""#));
        assert!(rendered.contains(r#""has_policy_warnings": true"#));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn section_kind_labels_are_stable() {
        assert_eq!(
            RepositoryContextPackSectionKind::Overview.as_str(),
            "overview"
        );
        assert_eq!(
            RepositoryContextPackSectionKind::Traversal.as_str(),
            "traversal"
        );
        assert_eq!(RepositoryContextPackSectionKind::Graph.as_str(), "graph");
        assert_eq!(
            RepositoryContextPackSectionKind::Toolchains.as_str(),
            "toolchains"
        );
        assert_eq!(
            RepositoryContextPackSectionKind::Dependencies.as_str(),
            "dependencies"
        );
        assert_eq!(RepositoryContextPackSectionKind::Policy.as_str(), "policy");
        assert_eq!(
            RepositoryContextPackSectionKind::TopLevelEntries.as_str(),
            "top_level_entries"
        );
    }
}
