//! Repository context pack foundation for Monad.
//!
//! WP-E2-013 introduced an AI-readable repository context pack model.
//! WP-E2-015 adds deterministic file export support.
//!
//! The context pack is intentionally built and exported in `monad-core` rather
//! than the CLI. The CLI can later expose this through a thin command flag such
//! as:
//!
//! - `monad context --write`
//! - `monad context --write --format=json`
//!
//! This slice does not add a CLI write flag yet.
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

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::json;

use crate::{
    WorkspaceContext, build_repository_graph, detect_repository_dependency_signals,
    detect_repository_toolchains, evaluate_repository_intelligence_policy, inspect_workspace,
    traverse_workspace_bounded,
};

use crate::{
    MonadError, MonadResult,
    dependency_detection::RepositoryDependencyDetection,
    repository_graph::RepositoryGraph,
    repository_inspection::{RepositoryBoundedTraversal, RepositoryInspection},
    repository_policy::RepositoryPolicyReport,
    toolchain_detection::RepositoryToolchainDetection,
};

/// Current context-pack schema version.
///
/// This value is intentionally separate from `monad.toml` schema version.
/// The context pack is an AI-facing/read-model artifact and can evolve on a
/// different cadence from the repository manifest.
pub const CURRENT_REPOSITORY_CONTEXT_PACK_SCHEMA_VERSION: u16 = 1;

/// Default Markdown context-pack filename.
pub const REPOSITORY_CONTEXT_PACK_MARKDOWN_FILENAME: &str = "repository-context-pack.md";

/// Default JSON context-pack filename.
pub const REPOSITORY_CONTEXT_PACK_JSON_FILENAME: &str = "repository-context-pack.json";

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

    /// Returns the default file extension for this render format.
    #[must_use]
    pub const fn file_extension(self) -> &'static str {
        match self {
            Self::Markdown => "md",
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

/// One exported context-pack file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryContextPackExportedFile {
    format: RepositoryContextPackRenderFormat,
    path: PathBuf,
    bytes_written: usize,
}

impl RepositoryContextPackExportedFile {
    /// Creates an exported file record.
    #[must_use]
    pub fn new(
        format: RepositoryContextPackRenderFormat,
        path: impl Into<PathBuf>,
        bytes_written: usize,
    ) -> Self {
        Self {
            format,
            path: path.into(),
            bytes_written,
        }
    }

    /// Returns the exported file format.
    #[must_use]
    pub const fn format(&self) -> RepositoryContextPackRenderFormat {
        self.format
    }

    /// Returns the exported file path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the number of bytes written.
    #[must_use]
    pub const fn bytes_written(&self) -> usize {
        self.bytes_written
    }
}

/// Result of exporting a repository context pack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryContextPackExportResult {
    output_dir: PathBuf,
    files: Vec<RepositoryContextPackExportedFile>,
}

impl RepositoryContextPackExportResult {
    /// Creates an export result.
    #[must_use]
    pub fn new(
        output_dir: impl Into<PathBuf>,
        files: Vec<RepositoryContextPackExportedFile>,
    ) -> Self {
        Self {
            output_dir: output_dir.into(),
            files,
        }
    }

    /// Returns the output directory.
    #[must_use]
    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }

    /// Returns exported files.
    #[must_use]
    pub fn files(&self) -> &[RepositoryContextPackExportedFile] {
        &self.files
    }

    /// Returns the number of exported files.
    #[must_use]
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Returns total bytes written.
    #[must_use]
    pub fn total_bytes_written(&self) -> usize {
        self.files
            .iter()
            .map(RepositoryContextPackExportedFile::bytes_written)
            .sum()
    }

    /// Returns true when a file for the requested format was exported.
    #[must_use]
    pub fn has_format(&self, format: RepositoryContextPackRenderFormat) -> bool {
        self.files.iter().any(|file| file.format() == format)
    }
}

/// Returns the default generated context-pack export directory for a repository root.
#[must_use]
pub fn default_repository_context_pack_export_dir(repository_root: &Path) -> PathBuf {
    repository_root
        .join(".monad")
        .join("context")
        .join("generated")
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

/// Builds an AI-readable repository context pack directly from a workspace context.
///
/// This helper is the high-level construction path used by CLI commands. It gathers
/// the repository intelligence inputs first, then assembles the final context pack.
pub fn repository_context_pack_from_workspace(
    context: &WorkspaceContext,
) -> MonadResult<RepositoryContextPack> {
    let inspection = inspect_workspace(context)?;

    let bounded_traversal = traverse_workspace_bounded(&inspection)?;

    let graph = build_repository_graph(&bounded_traversal);

    let toolchains = detect_repository_toolchains(&bounded_traversal);

    let dependencies = detect_repository_dependency_signals(&bounded_traversal);

    let policy =
        evaluate_repository_intelligence_policy(&inspection, &bounded_traversal, &dependencies);

    Ok(build_repository_context_pack(
        &inspection,
        &bounded_traversal,
        &graph,
        &toolchains,
        &dependencies,
        &policy,
    ))
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

/// Exports a repository context pack as deterministic Markdown and JSON files.
///
/// The caller supplies the output directory. For the default repository-local
/// location, use [`default_repository_context_pack_export_dir`].
pub fn export_repository_context_pack(
    pack: &RepositoryContextPack,
    output_dir: impl AsRef<Path>,
) -> MonadResult<RepositoryContextPackExportResult> {
    let output_dir = output_dir.as_ref();

    fs::create_dir_all(output_dir).map_err(|error| {
        MonadError::invalid_input(format!(
            "failed to create context pack export directory {}: {error}",
            output_dir.display()
        ))
    })?;

    let markdown_path = output_dir.join(REPOSITORY_CONTEXT_PACK_MARKDOWN_FILENAME);
    let json_path = output_dir.join(REPOSITORY_CONTEXT_PACK_JSON_FILENAME);

    let markdown =
        render_repository_context_pack(pack, RepositoryContextPackRenderFormat::Markdown);
    let json = render_repository_context_pack(pack, RepositoryContextPackRenderFormat::Json);

    write_context_pack_file(&markdown_path, &markdown)?;
    write_context_pack_file(&json_path, &json)?;

    Ok(RepositoryContextPackExportResult::new(
        output_dir.to_path_buf(),
        vec![
            RepositoryContextPackExportedFile::new(
                RepositoryContextPackRenderFormat::Markdown,
                markdown_path,
                markdown.len(),
            ),
            RepositoryContextPackExportedFile::new(
                RepositoryContextPackRenderFormat::Json,
                json_path,
                json.len(),
            ),
        ],
    ))
}

/// Builds and exports an AI-readable repository context pack from a workspace context.
pub fn export_repository_context_pack_from_workspace(
    context: &WorkspaceContext,
) -> MonadResult<RepositoryContextPackExportResult> {
    let inspection = inspect_workspace(context)?;

    let bounded_traversal = traverse_workspace_bounded(&inspection)?;

    let graph = build_repository_graph(&bounded_traversal);

    let toolchains = detect_repository_toolchains(&bounded_traversal);

    let dependencies = detect_repository_dependency_signals(&bounded_traversal);

    let policy =
        evaluate_repository_intelligence_policy(&inspection, &bounded_traversal, &dependencies);

    let context_pack = build_repository_context_pack(
        &inspection,
        &bounded_traversal,
        &graph,
        &toolchains,
        &dependencies,
        &policy,
    );

    let output_dir = default_repository_context_pack_export_dir(inspection.root());

    export_repository_context_pack(&context_pack, output_dir)
}

fn write_context_pack_file(path: &Path, contents: &str) -> MonadResult<()> {
    fs::write(path, contents).map_err(|error| {
        MonadError::invalid_input(format!(
            "failed to write context pack file {}: {error}",
            path.display()
        ))
    })
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
            RepositoryContextPackFact::new(
                "entry_count",
                bounded_traversal.entry_count().to_string(),
            ),
            RepositoryContextPackFact::new(
                "max_observed_depth",
                bounded_traversal.max_observed_depth().to_string(),
            ),
            RepositoryContextPackFact::new("max_allowed_depth", guardrails.max_depth().to_string()),
            RepositoryContextPackFact::new(
                "follow_symlinks",
                guardrails.follow_symlinks().to_string(),
            ),
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
            RepositoryContextPackFact::new(
                "candidate_count",
                bounded_traversal.candidate_count().to_string(),
            ),
            RepositoryContextPackFact::new(
                "shallow_only_count",
                bounded_traversal.shallow_only_count().to_string(),
            ),
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
    use std::time::SystemTime;

    use crate::{
        dependency_detection::detect_repository_dependency_signals,
        repository_graph::build_repository_graph, repository_inspection::inspect_workspace,
        repository_policy::evaluate_repository_intelligence_policy,
        toolchain_detection::detect_repository_toolchains, traverse_workspace_bounded,
        workspace::WorkspaceContext,
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
        assert!(
            pack.fact_value(RepositoryContextPackSectionKind::Graph, "node_count")
                .is_some()
        );
        assert!(
            pack.fact_value(
                RepositoryContextPackSectionKind::Dependencies,
                "manifest_count"
            )
            .is_some()
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
    fn context_pack_default_export_dir_is_repository_local_and_deterministic() {
        let root = PathBuf::from("/tmp/monad");

        let output_dir = default_repository_context_pack_export_dir(&root);

        assert_eq!(
            output_dir,
            PathBuf::from("/tmp/monad/.monad/context/generated")
        );
    }

    #[test]
    fn context_pack_exports_markdown_and_json_files() {
        let root = create_context_pack_workspace("export");
        let pack = build_pack_from_workspace(&root);
        let output_dir = default_repository_context_pack_export_dir(&root);

        let result =
            export_repository_context_pack(&pack, &output_dir).expect("context pack should export");

        let markdown_path = output_dir.join(REPOSITORY_CONTEXT_PACK_MARKDOWN_FILENAME);
        let json_path = output_dir.join(REPOSITORY_CONTEXT_PACK_JSON_FILENAME);

        assert_eq!(result.file_count(), 2);
        assert!(result.total_bytes_written() > 0);
        assert!(result.has_format(RepositoryContextPackRenderFormat::Markdown));
        assert!(result.has_format(RepositoryContextPackRenderFormat::Json));
        assert!(markdown_path.exists());
        assert!(json_path.exists());

        let markdown =
            fs::read_to_string(markdown_path).expect("markdown export should be readable");
        let json = fs::read_to_string(json_path).expect("json export should be readable");

        assert!(markdown.starts_with("# Monad Repository Context Pack"));
        assert!(json.contains(r#""kind": "repository_context_pack""#));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn exported_file_records_capture_format_path_and_bytes() {
        let file = RepositoryContextPackExportedFile::new(
            RepositoryContextPackRenderFormat::Json,
            "repository-context-pack.json",
            42,
        );

        assert_eq!(file.format(), RepositoryContextPackRenderFormat::Json);
        assert_eq!(file.path(), Path::new("repository-context-pack.json"));
        assert_eq!(file.bytes_written(), 42);
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
