//! Thin command-line interface for Monad.
//!
//! The CLI should parse user intent, delegate durable behavior to
//! `monad-core`, and print the result.
//!
//! Durable repository intelligence, inspection, graph construction, graph
//! rendering, context-pack generation, and context-pack export belong in
//! `monad-core`.

use std::env;
use std::process::ExitCode;

use monad_core::{
    OutputFormat, RepositoryContextPackExportResult, RepositoryContextPackRenderFormat,
    RepositoryGraphRenderFormat, WorkspaceContext, build_repository_graph,
    checked_runtime_identity, 
    export_repository_context_pack_from_workspace, inspect_workspace, load_manifest_from_workspace,
    render_diagnostic_report, render_repository_context_pack, render_repository_graph,
    render_repository_inspection_summary, render_workspace_summary,
    repository_context_pack_from_workspace, repository_inspection_summary_from_workspace,
    run_workspace_checks, traverse_workspace_bounded, workspace_summary_from_manifest,
};

/// Parsed CLI command.
///
/// This enum intentionally stays small. It represents command intent only.
/// The implementation still delegates product behavior to `monad-core`.
#[derive(Debug, Clone, PartialEq, Eq)]
enum CliCommand {
    /// Print help text.
    Help,

    /// Print runtime identity.
    Version,

    /// Print workspace summary.
    Info {
        /// Requested output format.
        output_format: OutputFormat,
    },

    /// Run workspace checks.
    Check {
        /// Requested output format.
        output_format: OutputFormat,
    },

    /// Inspect repository structure.
    Inspect {
        /// Requested output format.
        output_format: OutputFormat,
    },

    /// Render repository graph.
    Graph {
        /// Requested graph render format.
        graph_format: RepositoryGraphRenderFormat,
    },

    /// Render or export AI-readable repository context pack.
    Context {
        /// Requested context-pack render format.
        context_format: RepositoryContextPackRenderFormat,

        /// Whether to write generated context-pack files.
        write: bool,
    },
}

impl CliCommand {
    /// Parses CLI arguments into a command.
    fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut requested_format: Option<String> = None;
        let mut command: Option<String> = None;
        let mut write = false;

        for argument in args.into_iter().skip(1) {
            if argument == "--help" || argument == "-h" {
                return Ok(Self::Help);
            }

            if argument == "--version" || argument == "-V" {
                return Ok(Self::Version);
            }

            if argument == "--write" {
                write = true;
                continue;
            }

            if let Some(value) = argument.strip_prefix("--format=") {
                requested_format = Some(value.to_string());
                continue;
            }

            if argument == "--format" {
                return Err("expected a value after --format, such as --format=json".to_string());
            }

            if argument.starts_with('-') {
                return Err(format!("unsupported argument: {argument}"));
            }

            if command.is_some() {
                return Err(format!("unexpected extra command argument: {argument}"));
            }

            command = Some(argument);
        }

        match command.as_deref() {
            None => {
                reject_write_for_non_context(write)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::Info { output_format })
            }
            Some("help") => {
                reject_write_for_non_context(write)?;
                Ok(Self::Help)
            }
            Some("version") => {
                reject_write_for_non_context(write)?;
                Ok(Self::Version)
            }
            Some("info") => {
                reject_write_for_non_context(write)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::Info { output_format })
            }
            Some("check") => {
                reject_write_for_non_context(write)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::Check { output_format })
            }
            Some("inspect") => {
                reject_write_for_non_context(write)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::Inspect { output_format })
            }
            Some("graph") => {
                reject_write_for_non_context(write)?;
                let graph_format = parse_graph_format_or_default(requested_format.as_deref())?;
                Ok(Self::Graph { graph_format })
            }
            Some("context") => {
                let context_format = parse_context_format_or_default(requested_format.as_deref())?;
                Ok(Self::Context {
                    context_format,
                    write,
                })
            }
            Some(other) => Err(format!("unknown command: {other}")),
        }
    }
}

/// CLI entrypoint.
fn main() -> ExitCode {
    match run(env::args()) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

/// Runs a parsed command and returns printable output.
fn run(args: impl IntoIterator<Item = String>) -> Result<String, String> {
    let command = CliCommand::parse(args)?;

    match command {
        CliCommand::Help => Ok(help_text()),
        CliCommand::Version => render_version(),
        CliCommand::Info { output_format } => render_info(output_format),
        CliCommand::Check { output_format } => render_check(output_format),
        CliCommand::Inspect { output_format } => render_inspect(output_format),
        CliCommand::Graph { graph_format } => render_graph(graph_format),
        CliCommand::Context {
            context_format,
            write,
        } => render_context(context_format, write),
    }
}

/// Rejects `--write` when the selected command does not support writing.
fn reject_write_for_non_context(write: bool) -> Result<(), String> {
    if write {
        Err("--write is only supported for the context command".to_string())
    } else {
        Ok(())
    }
}

/// Parses the normal command output format.
///
/// Normal commands currently support only text and JSON.
fn parse_output_format_or_default(value: Option<&str>) -> Result<OutputFormat, String> {
    match value {
        Some(value) => OutputFormat::parse(value).map_err(|error| error.to_string()),
        None => Ok(OutputFormat::Text),
    }
}

/// Parses the graph command render format.
///
/// Graph output supports text, JSON, Mermaid, and DOT.
fn parse_graph_format_or_default(
    value: Option<&str>,
) -> Result<RepositoryGraphRenderFormat, String> {
    match value {
        Some(value) => RepositoryGraphRenderFormat::parse(value).map_err(|error| error.to_string()),
        None => Ok(RepositoryGraphRenderFormat::Text),
    }
}

/// Parses the context command render format.
///
/// Context output supports Markdown and JSON. The aliases `text` and `md`
/// intentionally map to Markdown because the context pack is designed to be
/// human-readable and LLM-readable by default.
fn parse_context_format_or_default(
    value: Option<&str>,
) -> Result<RepositoryContextPackRenderFormat, String> {
    match value {
        Some(value) => {
            RepositoryContextPackRenderFormat::parse(value).map_err(|error| error.to_string())
        }
        None => Ok(RepositoryContextPackRenderFormat::Markdown),
    }
}

/// Builds help text.
///
/// Keep this text boring and stable. It is user-facing contract documentation.
fn help_text() -> String {
    [
        "Monad",
        "",
        "Usage:",
        "  monad [command] [--format=<format>] [--write]",
        "",
        "Commands:",
        "  info      Show workspace summary",
        "  check     Run workspace checks",
        "  inspect   Inspect repository structure",
        "  graph     Render repository graph",
        "  context   Render AI-readable repository context pack",
        "  version   Show runtime version",
        "  help      Show this help",
        "",
        "General formats:",
        "  text",
        "  json",
        "",
        "Graph formats:",
        "  text",
        "  json",
        "  mermaid",
        "  dot",
        "",
        "Context formats:",
        "  markdown",
        "  md",
        "  text",
        "  json",
        "",
        "Context write mode:",
        "  monad context --write",
    ]
    .join("\n")
}

/// Renders runtime identity.
fn render_version() -> Result<String, String> {
    let identity = checked_runtime_identity().map_err(|error| error.to_string())?;

    Ok(identity.banner())
}

/// Renders workspace info.
fn render_info(output_format: OutputFormat) -> Result<String, String> {
    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;
    let manifest = load_manifest_from_workspace(&context).map_err(|error| error.to_string())?;
    let summary = workspace_summary_from_manifest(&context, &manifest);

    Ok(render_workspace_summary(&summary, output_format))
}

/// Renders workspace checks.
fn render_check(output_format: OutputFormat) -> Result<String, String> {
    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;
    let report = run_workspace_checks(&context);

    Ok(render_diagnostic_report(&report, output_format))
}

/// Renders repository inspection.
fn render_inspect(output_format: OutputFormat) -> Result<String, String> {
    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;
    let summary = repository_inspection_summary_from_workspace(&context)
        .map_err(|error| error.to_string())?;

    Ok(render_repository_inspection_summary(
        &summary,
        output_format,
    ))
}

/// Renders repository graph.
fn render_graph(graph_format: RepositoryGraphRenderFormat) -> Result<String, String> {
    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;
    let inspection = inspect_workspace(&context).map_err(|error| error.to_string())?;
    let bounded_traversal =
        traverse_workspace_bounded(&inspection).map_err(|error| error.to_string())?;
    let graph = build_repository_graph(&bounded_traversal);

    Ok(render_repository_graph(&graph, graph_format))
}

/// Renders or exports AI-readable repository context pack.
fn render_context(
    context_format: RepositoryContextPackRenderFormat,
    write: bool,
) -> Result<String, String> {
    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;

    //    let context_pack = build_repository_context_pack(
    //        &inspection,
    //        &bounded_traversal,
    //        &graph,
    //        &toolchains,
    //        &dependencies,
    //        &policy,
    //    );

    if write {
        let export_result = export_repository_context_pack_from_workspace(&context)
            .map_err(|error| error.to_string())?;

        Ok(render_context_export_summary(&export_result))
    } else {
        let pack =
            repository_context_pack_from_workspace(&context).map_err(|error| error.to_string())?;
        Ok(render_repository_context_pack(&pack, context_format))
    }
}

/// Renders a concise context-pack export summary.
fn render_context_export_summary(result: &RepositoryContextPackExportResult) -> String {
    let mut lines = vec![
        "Monad repository context pack export".to_string(),
        format!("  output_dir: {}", result.output_dir().display()),
        format!("  files: {}", result.file_count()),
        format!("  bytes: {}", result.total_bytes_written()),
        "  exported_files:".to_string(),
    ];

    for file in result.files() {
        lines.push(format!(
            "    - {}: {} bytes={}",
            file.format().as_str(),
            file.path().display(),
            file.bytes_written()
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_arguments(arguments: &[&str]) -> Result<CliCommand, String> {
        CliCommand::parse(arguments.iter().map(|argument| argument.to_string()))
    }

    #[test]
    fn no_command_defaults_to_info() {
        let command = parse_arguments(&["monad"]).expect("command should parse");

        assert_eq!(
            command,
            CliCommand::Info {
                output_format: OutputFormat::Text
            }
        );
    }

    #[test]
    fn help_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "help"]).expect("help should parse"),
            CliCommand::Help
        );

        assert_eq!(
            parse_arguments(&["monad", "--help"]).expect("--help should parse"),
            CliCommand::Help
        );
    }

    #[test]
    fn version_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "version"]).expect("version should parse"),
            CliCommand::Version
        );

        assert_eq!(
            parse_arguments(&["monad", "--version"]).expect("--version should parse"),
            CliCommand::Version
        );
    }

    #[test]
    fn info_command_parses_text_and_json_formats() {
        assert_eq!(
            parse_arguments(&["monad", "info"]).expect("info should parse"),
            CliCommand::Info {
                output_format: OutputFormat::Text
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "info", "--format=json"]).expect("info json should parse"),
            CliCommand::Info {
                output_format: OutputFormat::Json
            }
        );
    }

    #[test]
    fn check_command_parses_text_and_json_formats() {
        assert_eq!(
            parse_arguments(&["monad", "check"]).expect("check should parse"),
            CliCommand::Check {
                output_format: OutputFormat::Text
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "check", "--format=json"]).expect("check json should parse"),
            CliCommand::Check {
                output_format: OutputFormat::Json
            }
        );
    }

    #[test]
    fn inspect_command_parses_text_and_json_formats() {
        assert_eq!(
            parse_arguments(&["monad", "inspect"]).expect("inspect should parse"),
            CliCommand::Inspect {
                output_format: OutputFormat::Text
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "inspect", "--format=json"])
                .expect("inspect json should parse"),
            CliCommand::Inspect {
                output_format: OutputFormat::Json
            }
        );
    }

    #[test]
    fn graph_command_parses_supported_formats() {
        assert_eq!(
            parse_arguments(&["monad", "graph"]).expect("graph should parse"),
            CliCommand::Graph {
                graph_format: RepositoryGraphRenderFormat::Text
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "graph", "--format=json"]).expect("graph json should parse"),
            CliCommand::Graph {
                graph_format: RepositoryGraphRenderFormat::Json
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "graph", "--format=mermaid"])
                .expect("graph mermaid should parse"),
            CliCommand::Graph {
                graph_format: RepositoryGraphRenderFormat::Mermaid
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "graph", "--format=dot"]).expect("graph dot should parse"),
            CliCommand::Graph {
                graph_format: RepositoryGraphRenderFormat::Dot
            }
        );
    }

    #[test]
    fn context_command_parses_supported_formats() {
        assert_eq!(
            parse_arguments(&["monad", "context"]).expect("context should parse"),
            CliCommand::Context {
                context_format: RepositoryContextPackRenderFormat::Markdown,
                write: false,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "context", "--format=markdown"])
                .expect("context markdown should parse"),
            CliCommand::Context {
                context_format: RepositoryContextPackRenderFormat::Markdown,
                write: false,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "context", "--format=md"]).expect("context md should parse"),
            CliCommand::Context {
                context_format: RepositoryContextPackRenderFormat::Markdown,
                write: false,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "context", "--format=text"])
                .expect("context text alias should parse"),
            CliCommand::Context {
                context_format: RepositoryContextPackRenderFormat::Markdown,
                write: false,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "context", "--format=json"])
                .expect("context json should parse"),
            CliCommand::Context {
                context_format: RepositoryContextPackRenderFormat::Json,
                write: false,
            }
        );
    }

    #[test]
    fn context_command_parses_write_flag() {
        assert_eq!(
            parse_arguments(&["monad", "context", "--write"]).expect("context write should parse"),
            CliCommand::Context {
                context_format: RepositoryContextPackRenderFormat::Markdown,
                write: true,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "--write", "context"])
                .expect("write before context should parse"),
            CliCommand::Context {
                context_format: RepositoryContextPackRenderFormat::Markdown,
                write: true,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "context", "--format=json", "--write"])
                .expect("context json write should parse"),
            CliCommand::Context {
                context_format: RepositoryContextPackRenderFormat::Json,
                write: true,
            }
        );
    }

    #[test]
    fn format_can_appear_before_command() {
        assert_eq!(
            parse_arguments(&["monad", "--format=json", "context"])
                .expect("format before context should parse"),
            CliCommand::Context {
                context_format: RepositoryContextPackRenderFormat::Json,
                write: false,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "--format=mermaid", "graph"])
                .expect("format before graph should parse"),
            CliCommand::Graph {
                graph_format: RepositoryGraphRenderFormat::Mermaid
            }
        );
    }

    #[test]
    fn write_flag_is_rejected_for_non_context_commands() {
        let error = parse_arguments(&["monad", "inspect", "--write"])
            .expect_err("inspect should not accept write");

        assert_eq!(error, "--write is only supported for the context command");

        let error = parse_arguments(&["monad", "--write", "graph"])
            .expect_err("graph should not accept write");

        assert_eq!(error, "--write is only supported for the context command");
    }

    #[test]
    fn non_graph_commands_reject_graph_only_formats() {
        let error = parse_arguments(&["monad", "inspect", "--format=mermaid"])
            .expect_err("inspect should reject mermaid format");

        assert!(error.contains("unsupported output format"));
    }

    #[test]
    fn graph_command_rejects_unknown_formats() {
        let error = parse_arguments(&["monad", "graph", "--format=svg"])
            .expect_err("svg should not be supported yet");

        assert!(error.contains("unsupported repository graph render format"));
    }

    #[test]
    fn context_command_rejects_unknown_formats() {
        let error = parse_arguments(&["monad", "context", "--format=xml"])
            .expect_err("xml should not be supported for context packs");

        assert!(error.contains("unsupported repository context pack render format"));
    }

    #[test]
    fn unknown_command_returns_error() {
        let error =
            parse_arguments(&["monad", "unknown"]).expect_err("unknown command should fail");

        assert_eq!(error, "unknown command: unknown");
    }

    #[test]
    fn unexpected_extra_command_argument_returns_error() {
        let error = parse_arguments(&["monad", "info", "extra"])
            .expect_err("extra command argument should fail");

        assert_eq!(error, "unexpected extra command argument: extra");
    }

    #[test]
    fn help_text_mentions_context_command_formats_and_write_mode() {
        let text = help_text();

        assert!(text.contains("context"));
        assert!(text.contains("markdown"));
        assert!(text.contains("md"));
        assert!(text.contains("monad context --write"));
    }

    #[test]
    fn help_text_mentions_graph_command_and_formats() {
        let text = help_text();

        assert!(text.contains("graph"));
        assert!(text.contains("mermaid"));
        assert!(text.contains("dot"));
    }
}
