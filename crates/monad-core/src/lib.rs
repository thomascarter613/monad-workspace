//! Core runtime library for Monad.
//!
//! In Rust, files can be compiled as either binaries or libraries.
//! This file is the root of the `monad-core` library crate.
//!
//! Monad's architecture keeps durable product logic here, while the CLI crate
//! stays thin and delegates to this library.

pub mod checks;
pub mod diagnostics;
pub mod error;
pub mod manifest;
pub mod output;
pub mod repo_contract;
pub mod repository_inspection;
pub mod workspace;

pub use checks::run_workspace_checks;
pub use diagnostics::{Diagnostic, DiagnosticReport, Severity};
pub use error::{MonadError, MonadResult};
pub use manifest::{
    CURRENT_MANIFEST_SCHEMA_VERSION, ManifestProject, ManifestRuntime, ManifestSchemaVersion,
    ManifestWorkspace, MonadManifest,
};
pub use output::{
    OutputFormat, WorkspaceSummary, render_diagnostic_report, render_workspace_summary,
};
pub use repo_contract::{
    RepositoryContract, RequiredPath, RequiredPathKind, check_repository_contract,
};
pub use repository_inspection::{
    RepositoryEntry, RepositoryEntryKind, RepositoryEntryRole, RepositoryEntryTraversalPolicy,
    RepositoryInspection, inspect_workspace,
};
pub use workspace::{WorkspaceContext, discover_workspace_root, is_workspace_root};

/// Describes the currently compiled Monad runtime identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeIdentity {
    /// The public product name.
    pub product_name: &'static str,

    /// The crate responsible for durable runtime logic.
    pub runtime_crate: &'static str,

    /// The operating model Monad is currently enforcing.
    pub execution_model: &'static str,
}

impl RuntimeIdentity {
    /// Creates the canonical runtime identity for this early workspace slice.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            product_name: "Monad",
            runtime_crate: "monad-core",
            execution_model: "local-first",
        }
    }

    /// Builds a human-readable startup banner.
    #[must_use]
    pub fn banner(self) -> String {
        format!(
            "{} runtime foundation ready (crate: {}, model: {})",
            self.product_name, self.runtime_crate, self.execution_model
        )
    }

    /// Builds a structured startup diagnostic.
    #[must_use]
    pub fn startup_diagnostic(self) -> Diagnostic {
        Diagnostic::info("MONAD0001", self.banner())
    }

    /// Builds the default manifest corresponding to this runtime identity.
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

/// Returns Monad's canonical runtime identity.
#[must_use]
pub fn runtime_identity() -> RuntimeIdentity {
    RuntimeIdentity::new()
}

/// Returns Monad's runtime identity through the shared result type.
pub fn checked_runtime_identity() -> MonadResult<RuntimeIdentity> {
    Ok(runtime_identity())
}

/// Loads `monad.toml` from a workspace context.
pub fn load_manifest_from_workspace(context: &WorkspaceContext) -> MonadResult<MonadManifest> {
    MonadManifest::load_from_workspace(context)
}

/// Builds a renderable workspace summary from a context and loaded manifest.
#[must_use]
pub fn workspace_summary_from_manifest(
    context: &WorkspaceContext,
    manifest: &MonadManifest,
) -> WorkspaceSummary {
    WorkspaceSummary::from_manifest(context, manifest)
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
}
