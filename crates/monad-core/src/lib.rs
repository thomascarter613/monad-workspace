//! Core runtime library for Monad.
//!
//! In Rust, files can be compiled as either binaries or libraries.
//! This file is the root of the `monad-core` library crate.
//!
//! Monad's architecture keeps durable product logic here, while the CLI crate
//! stays thin and delegates to this library.

pub mod checks;
pub mod context;
pub mod dependency_detection;
pub mod diagnostics;
pub mod error;
pub mod manifest;
pub mod output;
pub mod repo_contract;
pub mod repository_context_pack;
pub mod repository_graph;
pub mod repository_inspection;
pub mod repository_policy;
pub mod toolchain_detection;
pub mod workspace;

pub use checks::run_workspace_checks;
pub use context::{
    ContextPackArtifact, CurrentStateArtifact, CurrentStateEpicEntry, HandoffArtifact,
    HandoffWorkPacketEntry, generate_context_pack, generate_current_state, generate_handoff,
    render_context_pack_markdown, write_context_pack_artifact, write_current_state_artifact,
    write_handoff_artifact,
};
pub use dependency_detection::{
    RepositoryDependencyDetection, RepositoryDependencySignal, RepositoryDependencySignalKind,
    detect_repository_dependency_signals,
};
pub use diagnostics::{Diagnostic, DiagnosticReport, Severity};
pub use error::{MonadError, MonadResult};
pub use manifest::{
    CURRENT_MANIFEST_SCHEMA_VERSION, ManifestProject, ManifestRuntime, ManifestSchemaVersion,
    ManifestWorkspace, MonadManifest,
};
pub use output::{
    OutputFormat, RepositoryDependencySummaryEntry, RepositoryInspectionSummary,
    RepositoryInspectionSummaryEntry, RepositoryToolchainSummaryEntry, WorkspaceSummary,
    render_diagnostic_report, render_repository_inspection_summary, render_workspace_summary,
};
pub use repo_contract::{
    RepositoryContract, RequiredPath, RequiredPathKind, check_repository_contract,
};
pub use repository_context_pack::{
    RepositoryContextPack, RepositoryContextPackExportResult, RepositoryContextPackExportedFile,
    RepositoryContextPackFact, RepositoryContextPackRenderFormat, RepositoryContextPackSection,
    RepositoryContextPackSectionKind, build_repository_context_pack,
    export_repository_context_pack, export_repository_context_pack_from_workspace,
    render_repository_context_pack, repository_context_pack_from_workspace,
};
pub use repository_graph::{
    RepositoryGraph, RepositoryGraphEdge, RepositoryGraphEdgeKind, RepositoryGraphNode,
    RepositoryGraphNodeKind, RepositoryGraphRenderFormat, build_repository_graph,
    render_repository_graph,
};
pub use repository_inspection::{
    RepositoryBoundedTraversal, RepositoryEntry, RepositoryEntryCategory, RepositoryEntryKind,
    RepositoryEntryRole, RepositoryEntryTraversalPolicy, RepositoryInspection,
    RepositoryTraversalDecision, RepositoryTraversalEntry, RepositoryTraversalGuardrails,
    RepositoryTraversalMode, RepositoryTraversalPlan, RepositoryTraversalPlanEntry,
    build_traversal_plan, inspect_workspace, traverse_workspace_bounded,
};
pub use repository_policy::{RepositoryPolicyReport, evaluate_repository_intelligence_policy};
pub use toolchain_detection::{
    RepositoryToolchainDetection, RepositoryToolchainKind, RepositoryToolchainSignal,
    RepositoryToolchainSignalKind, detect_repository_toolchains,
};
pub use workspace::{WorkspaceContext, discover_workspace_root, is_workspace_root};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeIdentity {
    pub product_name: &'static str,
    pub runtime_crate: &'static str,
    pub execution_model: &'static str,
}

impl RuntimeIdentity {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            product_name: "Monad",
            runtime_crate: "monad-core",
            execution_model: "local-first",
        }
    }

    #[must_use]
    pub fn banner(self) -> String {
        format!(
            "{} runtime foundation ready (crate: {}, model: {})",
            self.product_name, self.runtime_crate, self.execution_model
        )
    }

    #[must_use]
    pub fn startup_diagnostic(self) -> Diagnostic {
        Diagnostic::info("MONAD0001", self.banner())
    }

    #[must_use]
    pub fn default_manifest(self) -> MonadManifest {
        MonadManifest::new(
            ManifestSchemaVersion::current(),
            ManifestProject::new(
                self.product_name.to_lowercase(),
                self.product_name,
                "AI-native, repo-native, local-first Software Foundry OS for understanding, verifying, and safely evolving software repositories.",
            ),
            ManifestWorkspace::default(),
            ManifestRuntime::new("monad-core", "monad-cli", self.execution_model),
        )
    }
}

impl Default for RuntimeIdentity {
    fn default() -> Self {
        Self::new()
    }
}

#[must_use]
pub fn runtime_identity() -> RuntimeIdentity {
    RuntimeIdentity::new()
}

pub fn checked_runtime_identity() -> MonadResult<RuntimeIdentity> {
    Ok(runtime_identity())
}

pub fn load_manifest_from_workspace(context: &WorkspaceContext) -> MonadResult<MonadManifest> {
    MonadManifest::load_from_workspace(context)
}

#[must_use]
pub fn workspace_summary_from_manifest(
    context: &WorkspaceContext,
    manifest: &MonadManifest,
) -> WorkspaceSummary {
    WorkspaceSummary::from_manifest(context, manifest)
}

pub fn repository_inspection_summary_from_workspace(
    context: &WorkspaceContext,
) -> MonadResult<RepositoryInspectionSummary> {
    let inspection = inspect_workspace(context)?;
    let bounded_traversal = traverse_workspace_bounded(&inspection)?;
    let graph = build_repository_graph(&bounded_traversal);
    let toolchains = detect_repository_toolchains(&bounded_traversal);
    let dependencies = detect_repository_dependency_signals(&bounded_traversal);

    Ok(
        RepositoryInspectionSummary::from_inspection_bounded_traversal_graph_toolchains_and_dependencies(
            &inspection,
            &bounded_traversal,
            &graph,
            &toolchains,
            &dependencies,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_identity_names_monad() {
        let identity = runtime_identity();

        assert_eq!(identity.product_name, "Monad");
        assert_eq!(identity.runtime_crate, "monad-core");
        assert_eq!(identity.execution_model, "local-first");
    }

    #[test]
    fn runtime_banner_is_human_readable() {
        let banner = runtime_identity().banner();

        assert!(banner.contains("Monad"));
        assert!(banner.contains("monad-core"));
        assert!(banner.contains("local-first"));
    }

    #[test]
    fn runtime_identity_can_produce_startup_diagnostic() {
        let diagnostic = runtime_identity().startup_diagnostic();

        assert_eq!(diagnostic.severity, Severity::Info);
        assert_eq!(diagnostic.code, "MONAD0001");
        assert!(diagnostic.message.contains("Monad"));
        assert!(diagnostic.render().contains("[INFO] MONAD0001"));
    }

    #[test]
    fn checked_runtime_identity_uses_monad_result() {
        let identity = checked_runtime_identity().expect("runtime identity should be available");

        assert_eq!(identity.product_name, "Monad");
    }

    #[test]
    fn workspace_context_is_exported_from_core_root() {
        let context = WorkspaceContext::new(".").expect("workspace context should be created");

        assert_eq!(context.root(), std::path::Path::new("."));
    }

    #[test]
    fn runtime_identity_can_build_default_manifest() {
        let manifest = runtime_identity().default_manifest();

        assert_eq!(
            manifest.schema_version.as_u16(),
            CURRENT_MANIFEST_SCHEMA_VERSION
        );
        assert_eq!(manifest.project.name, "monad");
        assert_eq!(manifest.runtime.core_crate, "monad-core");
        assert_eq!(manifest.runtime.cli_crate, "monad-cli");
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn workspace_checks_are_exported_from_core_root() {
        let context = WorkspaceContext::new(".").expect("workspace context should be created");
        let report = run_workspace_checks(&context);

        assert!(!report.is_empty());
    }

    #[test]
    fn repository_contract_is_exported_from_core_root() {
        let contract = RepositoryContract::initial_monad();

        assert!(!contract.required_paths().is_empty());
    }

    #[test]
    fn output_format_is_exported_from_core_root() {
        assert_eq!(OutputFormat::parse("text"), Ok(OutputFormat::Text));
    }

    #[test]
    fn repository_inspection_types_are_exported_from_core_root() {
        assert_eq!(RepositoryEntryKind::File.as_str(), "file");
        assert_eq!(
            RepositoryEntryRole::MonadManifest.as_str(),
            "monad_manifest"
        );
        assert_eq!(
            RepositoryEntryTraversalPolicy::SkipGeneratedOrExternal.as_str(),
            "skip_generated_or_external"
        );
    }

    #[test]
    fn repository_graph_types_are_exported_from_core_root() {
        assert_eq!(
            RepositoryGraphNodeKind::WorkspaceRoot.as_str(),
            "workspace_root"
        );
        assert_eq!(RepositoryGraphEdgeKind::Contains.as_str(), "contains");
    }

    #[test]
    fn repository_graph_render_format_is_exported_from_core_root() {
        assert_eq!(RepositoryGraphRenderFormat::Text.as_str(), "text");
        assert_eq!(RepositoryGraphRenderFormat::Json.as_str(), "json");
        assert_eq!(RepositoryGraphRenderFormat::Mermaid.as_str(), "mermaid");
        assert_eq!(RepositoryGraphRenderFormat::Dot.as_str(), "dot");
    }

    #[test]
    fn repository_toolchain_detection_types_are_exported_from_core_root() {
        assert_eq!(RepositoryToolchainKind::Rust.as_str(), "rust");
        assert_eq!(
            RepositoryToolchainSignalKind::SourceFile.as_str(),
            "source_file"
        );

        let detection = RepositoryToolchainDetection::from_signals(Vec::new());

        assert_eq!(detection.detected_toolchain_count(), 0);
        assert_eq!(detection.signal_count(), 0);
    }

    #[test]
    fn repository_dependency_detection_types_are_exported_from_core_root() {
        assert_eq!(
            RepositoryDependencySignalKind::Manifest.as_str(),
            "manifest"
        );
        assert_eq!(
            RepositoryDependencySignalKind::Lockfile.as_str(),
            "lockfile"
        );

        let detection = RepositoryDependencyDetection::from_signals(Vec::new());

        assert_eq!(detection.detected_toolchain_count(), 0);
        assert_eq!(detection.signal_count(), 0);
    }

    #[test]
    fn repository_entry_category_is_exported_from_core_root() {
        assert_eq!(RepositoryEntryCategory::Source.as_str(), "source");
        assert_eq!(
            RepositoryEntryRole::SourceRoot.category(),
            RepositoryEntryCategory::Source
        );
    }

    #[test]
    fn traversal_planning_types_are_exported_from_core_root() {
        assert_eq!(
            RepositoryTraversalMode::FutureRecursiveLimited.as_str(),
            "future_recursive_limited"
        );
        assert_eq!(
            RepositoryTraversalMode::BoundedRecursive.as_str(),
            "bounded_recursive"
        );
        assert_eq!(
            RepositoryTraversalDecision::SkipByDefault.as_str(),
            "skip_by_default"
        );

        let guardrails = RepositoryTraversalGuardrails::default();

        assert_eq!(guardrails.max_depth(), 3);
        assert!(!guardrails.follow_symlinks());
        assert!(guardrails.respect_ignore_files());
    }

    #[test]
    fn repository_inspection_summary_type_is_exported_from_core_root() {
        let inspection = RepositoryInspection::new(".", Vec::new());
        let summary = RepositoryInspectionSummary::from_inspection(&inspection);

        assert_eq!(summary.root, ".");
        assert_eq!(summary.entry_count, 0);
        assert_eq!(summary.known_entry_count, 0);
        assert_eq!(summary.unknown_entry_count, 0);
        assert_eq!(summary.future_traversal_mode, "future_recursive_limited");
        assert_eq!(summary.bounded_traversal_mode, "not_run");
        assert_eq!(summary.graph_node_count, 0);
        assert_eq!(summary.graph_edge_count, 0);
        assert_eq!(summary.toolchain_count, 0);
        assert_eq!(summary.toolchain_signal_count, 0);
        assert_eq!(summary.dependency_toolchain_count, 0);
        assert_eq!(summary.dependency_signal_count, 0);
    }
}
