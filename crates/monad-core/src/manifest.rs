//! Manifest model types for Monad.
//!
//! Monad uses `monad.toml` as a repo-native intent file.
//!
//! This module defines the first in-memory model for that file. It does not
//! parse TOML yet. Parsing will be a later slice after the stable data shape is
//! in place.

use std::fmt;

use crate::{Diagnostic, DiagnosticReport, MonadError, MonadResult};

/// Current supported manifest schema version.
///
/// A schema version lets Monad evolve `monad.toml` over time without guessing
/// how to interpret old or future manifest files.
pub const CURRENT_MANIFEST_SCHEMA_VERSION: u16 = 1;

/// Manifest schema version wrapper.
///
/// This is a tiny type around `u16`. Wrapping the number gives us a named domain
/// concept instead of passing loose integers through the runtime.
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    ///
    /// The first version returns one structured error when any diagnostic has
    /// error severity. Later slices can return richer multi-diagnostic failures.
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
    fn unsupported_schema_version_fails_validation() {
        let manifest = MonadManifest::new(
            ManifestSchemaVersion::new(999),
            ManifestProject::new("monad", "Monad", "test"),
            ManifestWorkspace::default(),
            ManifestRuntime::new("monad-core", "monad-cli", "local-first"),
        );

        let diagnostics = manifest.diagnostics();

        assert!(diagnostics.has_errors());
        assert!(
            diagnostics
                .render_lines()
                .iter()
                .any(|line| line.contains("MONAD3000"))
        );
        assert!(manifest.validate().is_err());
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
}
