//! Output formatting primitives for Monad.
//!
//! The CLI should stay thin. It may decide when to print, but reusable
//! formatting rules should live in `monad-core` so future commands produce
//! consistent output.
//!
//! This first output module supports text output only. JSON and other machine
//! formats can be added later without changing every command.

use crate::{DiagnosticReport, MonadError, MonadManifest, MonadResult, WorkspaceContext};

/// Output formats supported by the runtime.
///
/// The first format is plain text because that is what the current CLI prints.
/// Later slices can add JSON, NDJSON, Markdown, or other formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human-readable plain text.
    Text,
}

impl OutputFormat {
    /// Returns the stable format name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
        }
    }

    /// Parses an output format name.
    ///
    /// This gives future CLI argument parsing one stable runtime function to
    /// call instead of scattering string comparisons across the CLI.
    pub fn parse(value: &str) -> MonadResult<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "text" => Ok(Self::Text),
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
///
/// This type keeps the information to print separate from how it is printed.
/// The CLI can ask `monad-core` to build this summary and then render it.
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

/// Renders a diagnostic report.
#[must_use]
pub fn render_diagnostic_report(report: &DiagnosticReport, format: OutputFormat) -> String {
    match format {
        OutputFormat::Text => report.render_lines().join("\n"),
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        Diagnostic, ManifestProject, ManifestRuntime, ManifestSchemaVersion, ManifestWorkspace,
    };

    #[test]
    fn output_format_parses_text() {
        assert_eq!(OutputFormat::parse("text"), Ok(OutputFormat::Text));
        assert_eq!(OutputFormat::parse("TEXT"), Ok(OutputFormat::Text));
    }

    #[test]
    fn unsupported_output_format_returns_error() {
        let error = OutputFormat::parse("json").expect_err("json is not supported yet");

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
        assert!(rendered.contains("schema_version: 1"));
        assert!(rendered.contains("core_crate: monad-core"));
        assert!(rendered.contains("cli_crate: monad-cli"));
        assert!(rendered.contains("execution_model: local-first"));
    }
}
