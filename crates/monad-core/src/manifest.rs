//! Manifest model and loading support for Monad.
//!
//! Monad uses `monad.toml` as a repo-native intent file.
//!
//! WP-E1-005 introduced the in-memory model.
//! WP-E1-006 adds actual TOML parsing and file loading.

use std::fmt;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::workspace::WorkspaceContext;
use crate::{Diagnostic, DiagnosticReport, MonadError, MonadResult};

/// Current supported manifest schema version.
pub const CURRENT_MANIFEST_SCHEMA_VERSION: u16 = 1;

/// Manifest schema version wrapper.
///
/// This newtype makes schema versions explicit instead of passing loose numbers
/// through the runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestSchemaVersion(u16);

impl ManifestSchemaVersion {
    /// Creates a schema version value.
    #[must_use]
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Returns the current supported schema version.
    #[must_use]
    pub const fn current() -> Self {
        Self(CURRENT_MANIFEST_SCHEMA_VERSION)
    }

    /// Returns the raw numeric version.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0
    }

    /// Returns true when this schema version is supported by this runtime.
    #[must_use]
    pub const fn is_supported(self) -> bool {
        self.0 == CURRENT_MANIFEST_SCHEMA_VERSION
    }
}

impl Default for ManifestSchemaVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl fmt::Display for ManifestSchemaVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Project identity from `monad.toml`.
///
/// `Deserialize` lets Serde build this struct from TOML data.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestProject {
    /// Stable project name used by tools.
    pub name: String,

    /// Human-readable display name.
    pub display_name: String,

    /// Human-readable project description.
    pub description: String,
}

impl ManifestProject {
    /// Creates project manifest metadata.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        display_name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            display_name: display_name.into(),
            description: description.into(),
        }
    }

    /// Returns a diagnostic report describing validation findings.
    #[must_use]
    pub fn diagnostics(&self) -> DiagnosticReport {
        let mut report = DiagnosticReport::new();

        if self.name.trim().is_empty() {
            report.push(Diagnostic::error(
                "MONAD3001",
                "manifest project.name must not be empty",
            ));
        }

        if self.display_name.trim().is_empty() {
            report.push(Diagnostic::warning(
                "MONAD3002",
                "manifest project.display_name should not be empty",
            ));
        }

        if self.description.trim().is_empty() {
            report.push(Diagnostic::warning(
                "MONAD3003",
                "manifest project.description should not be empty",
            ));
        }

        report
    }
}

/// Workspace section from `monad.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestWorkspace {
    /// Files or directories that help identify the workspace root.
    pub root_markers: Vec<String>,
}

impl ManifestWorkspace {
    /// Creates workspace manifest metadata.
    #[must_use]
    pub fn new(root_markers: impl Into<Vec<String>>) -> Self {
        Self {
            root_markers: root_markers.into(),
        }
    }

    /// Creates the default root markers Monad expects at repository root.
    #[must_use]
    pub fn default_root_markers() -> Vec<String> {
        vec![
            "monad.toml".to_string(),
            "Cargo.toml".to_string(),
            ".monad".to_string(),
            "work".to_string(),
        ]
    }

    /// Returns diagnostics for the workspace section.
    #[must_use]
    pub fn diagnostics(&self) -> DiagnosticReport {
        let mut report = DiagnosticReport::new();

        if self.root_markers.is_empty() {
            report.push(Diagnostic::error(
                "MONAD3010",
                "manifest workspace.root_markers must not be empty",
            ));
        }

        if !self
            .root_markers
            .iter()
            .any(|marker| marker == "monad.toml")
        {
            report.push(Diagnostic::warning(
                "MONAD3011",
                "manifest workspace.root_markers should include monad.toml",
            ));
        }

        report
    }
}

impl Default for ManifestWorkspace {
    fn default() -> Self {
        Self::new(Self::default_root_markers())
    }
}

/// Runtime section from `monad.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestRuntime {
    /// Durable core runtime crate.
    pub core_crate: String,

    /// Thin CLI crate.
    pub cli_crate: String,

    /// Runtime execution model.
    pub execution_model: String,
}

impl ManifestRuntime {
    /// Creates runtime manifest metadata.
    #[must_use]
    pub fn new(
        core_crate: impl Into<String>,
        cli_crate: impl Into<String>,
        execution_model: impl Into<String>,
    ) -> Self {
        Self {
            core_crate: core_crate.into(),
            cli_crate: cli_crate.into(),
            execution_model: execution_model.into(),
        }
    }

    /// Returns diagnostics for the runtime section.
    #[must_use]
    pub fn diagnostics(&self) -> DiagnosticReport {
        let mut report = DiagnosticReport::new();

        if self.core_crate.trim().is_empty() {
            report.push(Diagnostic::error(
                "MONAD3020",
                "manifest runtime.core_crate must not be empty",
            ));
        }

        if self.cli_crate.trim().is_empty() {
            report.push(Diagnostic::error(
                "MONAD3021",
                "manifest runtime.cli_crate must not be empty",
            ));
        }

        if self.execution_model.trim().is_empty() {
            report.push(Diagnostic::error(
                "MONAD3022",
                "manifest runtime.execution_model must not be empty",
            ));
        }

        report
    }
}

/// Raw TOML-deserializable manifest shape.
///
/// This type is private because callers should work with `MonadManifest`, which
/// wraps the schema version in `ManifestSchemaVersion`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMonadManifest {
    schema_version: u16,
    project: ManifestProject,
    workspace: ManifestWorkspace,
    runtime: ManifestRuntime,
}

impl RawMonadManifest {
    fn into_manifest(self) -> MonadManifest {
        MonadManifest::new(
            ManifestSchemaVersion::new(self.schema_version),
            self.project,
            self.workspace,
            self.runtime,
        )
    }
}

/// In-memory model of `monad.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonadManifest {
    /// Manifest schema version.
    pub schema_version: ManifestSchemaVersion,

    /// Project identity.
    pub project: ManifestProject,

    /// Workspace metadata.
    pub workspace: ManifestWorkspace,

    /// Runtime metadata.
    pub runtime: ManifestRuntime,
}

impl MonadManifest {
    /// Creates a manifest model.
    #[must_use]
    pub fn new(
        schema_version: ManifestSchemaVersion,
        project: ManifestProject,
        workspace: ManifestWorkspace,
        runtime: ManifestRuntime,
    ) -> Self {
        Self {
            schema_version,
            project,
            workspace,
            runtime,
        }
    }

    /// Creates Monad's default manifest model for this repository.
    #[must_use]
    pub fn default_for_monad() -> Self {
        Self::new(
            ManifestSchemaVersion::current(),
            ManifestProject::new(
                "monad",
                "Monad",
                "AI-native, repo-native, local-first Software Foundry OS for understanding, verifying, and safely evolving software repositories.",
            ),
            ManifestWorkspace::default(),
            ManifestRuntime::new("monad-core", "monad-cli", "local-first"),
        )
    }

    /// Parses and validates a manifest from TOML text.
    pub fn from_toml_str(input: &str) -> MonadResult<Self> {
        let raw: RawMonadManifest = toml::from_str(input).map_err(|error| {
            MonadError::invalid_input(format!("failed to parse monad.toml: {error}"))
        })?;

        let manifest = raw.into_manifest();
        manifest.validate()?;

        Ok(manifest)
    }

    /// Loads, parses, and validates a manifest from a file path.
    pub fn load_from_path(path: impl AsRef<Path>) -> MonadResult<Self> {
        let path = path.as_ref();

        let text = fs::read_to_string(path).map_err(|error| {
            MonadError::not_found(format!("monad manifest at {} ({error})", path.display()))
        })?;

        Self::from_toml_str(&text)
    }

    /// Loads a manifest using a workspace context.
    pub fn load_from_workspace(context: &WorkspaceContext) -> MonadResult<Self> {
        Self::load_from_path(context.monad_manifest_path())
    }

    /// Returns all diagnostics for this manifest.
    #[must_use]
    pub fn diagnostics(&self) -> DiagnosticReport {
        let mut report = DiagnosticReport::new();

        if !self.schema_version.is_supported() {
            report.push(Diagnostic::error(
                "MONAD3000",
                format!(
                    "unsupported manifest schema_version {}; expected {}",
                    self.schema_version, CURRENT_MANIFEST_SCHEMA_VERSION
                ),
            ));
        }

        extend_report(&mut report, self.project.diagnostics());
        extend_report(&mut report, self.workspace.diagnostics());
        extend_report(&mut report, self.runtime.diagnostics());

        report
    }

    /// Validates this manifest.
    pub fn validate(&self) -> MonadResult<()> {
        let diagnostics = self.diagnostics();

        if diagnostics.has_errors() {
            return Err(MonadError::verification_failed(
                "manifest validation failed",
            ));
        }

        Ok(())
    }
}

/// Appends all diagnostics from `source` into `target`.
fn extend_report(target: &mut DiagnosticReport, source: DiagnosticReport) {
    for diagnostic in source.diagnostics() {
        target.push(diagnostic.clone());
    }
}

impl Default for MonadManifest {
    fn default() -> Self {
        Self::default_for_monad()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    const VALID_MANIFEST_TOML: &str = r#"
schema_version = 1

[project]
name = "monad"
display_name = "Monad"
description = "AI-native, repo-native, local-first Software Foundry OS."

[workspace]
root_markers = ["monad.toml", "Cargo.toml", ".monad", "work"]

[runtime]
core_crate = "monad-core"
cli_crate = "monad-cli"
execution_model = "local-first"
"#;

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "monad-manifest-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    #[test]
    fn current_schema_version_is_supported() {
        let version = ManifestSchemaVersion::current();

        assert_eq!(version.as_u16(), 1);
        assert!(version.is_supported());
        assert_eq!(version.to_string(), "1");
    }

    #[test]
    fn future_schema_version_is_not_supported() {
        let version = ManifestSchemaVersion::new(999);

        assert!(!version.is_supported());
    }

    #[test]
    fn default_manifest_matches_monad_runtime_shape() {
        let manifest = MonadManifest::default_for_monad();

        assert_eq!(manifest.schema_version.as_u16(), 1);
        assert_eq!(manifest.project.name, "monad");
        assert_eq!(manifest.project.display_name, "Monad");
        assert_eq!(manifest.runtime.core_crate, "monad-core");
        assert_eq!(manifest.runtime.cli_crate, "monad-cli");
        assert_eq!(manifest.runtime.execution_model, "local-first");
        assert!(
            manifest
                .workspace
                .root_markers
                .contains(&"monad.toml".to_string())
        );
    }

    #[test]
    fn valid_default_manifest_has_no_error_diagnostics() {
        let manifest = MonadManifest::default_for_monad();
        let diagnostics = manifest.diagnostics();

        assert!(!diagnostics.has_errors());
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn manifest_parses_from_toml_string() {
        let manifest =
            MonadManifest::from_toml_str(VALID_MANIFEST_TOML).expect("manifest should parse");

        assert_eq!(manifest.schema_version.as_u16(), 1);
        assert_eq!(manifest.project.name, "monad");
        assert_eq!(manifest.runtime.core_crate, "monad-core");
        assert_eq!(manifest.runtime.cli_crate, "monad-cli");
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn invalid_toml_returns_invalid_input_error() {
        let error = MonadManifest::from_toml_str("not valid toml =").expect_err("TOML should fail");

        assert_eq!(error.code(), "MONAD2001");
        assert!(error.message().contains("failed to parse monad.toml"));
    }

    #[test]
    fn unsupported_schema_version_fails_validation() {
        let manifest_text =
            VALID_MANIFEST_TOML.replace("schema_version = 1", "schema_version = 999");

        let error = MonadManifest::from_toml_str(&manifest_text)
            .expect_err("unsupported schema should fail validation");

        assert_eq!(error.code(), "MONAD2003");
    }

    #[test]
    fn empty_project_name_fails_validation() {
        let manifest = MonadManifest::new(
            ManifestSchemaVersion::current(),
            ManifestProject::new("", "Monad", "test"),
            ManifestWorkspace::default(),
            ManifestRuntime::new("monad-core", "monad-cli", "local-first"),
        );

        assert!(manifest.diagnostics().has_errors());
        assert!(manifest.validate().is_err());
    }

    #[test]
    fn manifest_loads_from_path() {
        let root = unique_temp_dir("load-from-path");
        fs::create_dir_all(&root).expect("test directory should be created");

        let manifest_path = root.join("monad.toml");
        fs::write(&manifest_path, VALID_MANIFEST_TOML).expect("manifest should be written");

        let manifest =
            MonadManifest::load_from_path(&manifest_path).expect("manifest should load from path");

        assert_eq!(manifest.project.name, "monad");

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn manifest_loads_from_workspace_context() {
        let root = unique_temp_dir("load-from-workspace");
        fs::create_dir_all(&root).expect("test directory should be created");

        fs::write(root.join("monad.toml"), VALID_MANIFEST_TOML)
            .expect("manifest should be written");

        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let manifest = MonadManifest::load_from_workspace(&context)
            .expect("manifest should load from workspace context");

        assert_eq!(manifest.project.display_name, "Monad");

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn missing_manifest_path_returns_not_found_error() {
        let root = unique_temp_dir("missing-path");
        let error = MonadManifest::load_from_path(root.join("monad.toml"))
            .expect_err("missing manifest should fail");

        assert_eq!(error.code(), "MONAD2002");
    }
}
