//! Output formatting primitives for Monad.
//!
//! The CLI should stay thin. It may decide when to print, but reusable
//! formatting rules should live in `monad-core` so future commands produce
//! consistent output.
//!
//! This module now supports:
//!
//! - `text` for human-readable output;
//! - `json` for machine-readable output.

use serde_json::json;

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
                "root": summary.root,
            },
            "project": {
                "display_name": summary.project_display_name,
                "name": summary.project_name,
            },
            "manifest": {
                "schema_version": summary.schema_version,
            },
            "runtime": {
                "core_crate": summary.core_crate,
                "cli_crate": summary.cli_crate,
                "execution_model": summary.execution_model,
            },
        }))
        .expect("serializing workspace summary JSON should not fail"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        Diagnostic, ManifestProject, ManifestRuntime, ManifestSchemaVersion, ManifestWorkspace,
    };

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
}
