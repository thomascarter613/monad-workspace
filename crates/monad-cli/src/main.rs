//! Thin command-line interface for Monad.
//!
//! The CLI should parse user intent, delegate durable behavior to
//! `monad-core`, and print the result.
//!
//! Durable repository intelligence, inspection, graph construction, graph
//! rendering, context-pack generation, context-pack export, and context
//! artifact generation belong in `monad-core`.

use monad_core::{
    BootstrapPromptArtifact, ContextPackArtifact, CurrentStateArtifact, HandoffArtifact,
    OutputFormat, RepositoryContextPackExportResult, RepositoryContextPackRenderFormat,
    RepositoryGraphRenderFormat, WorkspaceContext, build_repository_graph,
    checked_runtime_identity, export_repository_context_pack_from_workspace,
    generate_bootstrap_prompt, generate_context_pack, generate_current_state, generate_handoff,
    inspect_workspace, load_manifest_from_workspace, render_check_run_report,
    render_check_run_report_json, render_context_verify_summary, render_repository_context_pack,
    render_repository_graph, render_repository_inspection_summary, render_verify_baseline_dry_run,
    render_workspace_summary, repository_context_pack_from_workspace,
    repository_inspection_summary_from_workspace, run_monad_workspace_checks,
    traverse_workspace_bounded, verify_context, workspace_summary_from_manifest,
    write_bootstrap_prompt_artifact, write_check_evidence_packet, write_context_pack_artifact,
    write_current_state_artifact, write_handoff_artifact,
};
use std::env;
use std::process::ExitCode;

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

    /// Generate a context artifact and write it to the repository.
    ContextGenerate {
        /// Which context artifact to generate.
        artifact: ContextArtifactKind,
    },

    /// Assemble and write a project-level context pack.
    ContextPack,

    /// Verify context files exist and meet structural expectations.
    ContextVerify,

    /// Plan verification baseline evolution.
    EvolveVerifyBaseline {
        /// Whether to run in dry-run mode.
        dry_run: bool,
    },
}

/// Supported context artifact kinds for `context generate`.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ContextArtifactKind {
    /// Generate `.monad/context/current-state.md`.
    CurrentState,

    /// Generate `.monad/context/latest-handoff.md`.
    Handoff,

    /// Generate `docs/ai/BOOTSTRAP-PROMPT.md`.
    Bootstrap,
}

impl CliCommand {
    /// Parses CLI arguments into a command.
    ///
    /// Supports both single-word commands like `monad info` and multi-word
    /// subcommands like `monad context generate current-state`.
    fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut requested_format: Option<String> = None;
        let mut positional: Vec<String> = Vec::new();
        let mut write = false;
        let mut dry_run = false;

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

            if argument == "--dry-run" {
                dry_run = true;
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

            positional.push(argument);
        }

        // Convert positional args to string slices for pattern matching.
        let parts: Vec<&str> = positional.iter().map(|s| s.as_str()).collect();

        match parts.as_slice() {
            [] => {
                reject_write_for_non_context(write)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::Info { output_format })
            }
            ["help"] => {
                reject_write_for_non_context(write)?;
                Ok(Self::Help)
            }
            ["version"] => {
                reject_write_for_non_context(write)?;
                Ok(Self::Version)
            }
            ["info"] => {
                reject_write_for_non_context(write)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::Info { output_format })
            }
            ["check"] => {
                reject_write_for_non_context(write)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::Check { output_format })
            }
            ["inspect"] => {
                reject_write_for_non_context(write)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::Inspect { output_format })
            }
            ["graph"] => {
                reject_write_for_non_context(write)?;
                let graph_format = parse_graph_format_or_default(requested_format.as_deref())?;
                Ok(Self::Graph { graph_format })
            }
            ["context"] => {
                let context_format = parse_context_format_or_default(requested_format.as_deref())?;
                Ok(Self::Context {
                    context_format,
                    write,
                })
            }
            ["context", "pack"] => {
                reject_write_for_non_context(write)?;
                Ok(Self::ContextPack)
            }
            ["context", "verify"] => {
                reject_write_for_non_context(write)?;
                Ok(Self::ContextVerify)
            }
            ["context", "generate", "current-state"] => {
                reject_write_for_non_context(write)?;
                Ok(Self::ContextGenerate {
                    artifact: ContextArtifactKind::CurrentState,
                })
            }
            ["context", "generate", "handoff"] => {
                reject_write_for_non_context(write)?;
                Ok(Self::ContextGenerate {
                    artifact: ContextArtifactKind::Handoff,
                })
            }
            ["context", "generate", "bootstrap"] => {
                reject_write_for_non_context(write)?;
                Ok(Self::ContextGenerate {
                    artifact: ContextArtifactKind::Bootstrap,
                })
            }
            ["context", "generate"] => {
                Err("missing artifact kind: try 'context generate current-state', 'context generate handoff', or 'context generate bootstrap'".to_string())
            }
            ["context", "generate", other] => Err(format!("unknown context artifact: {other}")),
            ["context", other, ..] => Err(format!("unknown context subcommand: {other}")),
            ["evolve", "verify-baseline"] => {
                reject_write_for_non_context(write)?;
                require_dry_run_for_evolve(dry_run)?;
                reject_format_for_evolve(requested_format.as_deref())?;
                Ok(Self::EvolveVerifyBaseline { dry_run })
            }
            ["evolve", "verify-baseline", other] => {
                reject_write_for_non_context(write)?;
                Err(format!("unknown evolve verify-baseline argument: {other}"))
            }
            ["evolve"] => Err("missing evolve subcommand: try 'evolve verify-baseline --dry-run'".to_string()),
            ["evolve", other, ..] => Err(format!("unknown evolve subcommand: {other}")),
            [single] => {
                reject_write_for_non_context(write)?;
                Err(format!("unknown command: {single}"))
            }
            [first, ..] => {
                reject_write_for_non_context(write)?;
                Err(format!("unknown command: {first}"))
            }
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
        CliCommand::ContextGenerate { artifact } => render_context_generate(artifact),
        CliCommand::ContextPack => render_context_pack(),
        CliCommand::ContextVerify => render_context_verify(),
        CliCommand::EvolveVerifyBaseline { dry_run } => render_evolve_verify_baseline(dry_run),
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

/// Requires dry-run mode for early evolution commands.
///
/// WP-E5-004 intentionally does not add apply/write behavior.
fn require_dry_run_for_evolve(dry_run: bool) -> Result<(), String> {
    if dry_run {
        Ok(())
    } else {
        Err("evolve verify-baseline currently requires --dry-run".to_string())
    }
}

/// Rejects output-format flags for early evolution commands.
fn reject_format_for_evolve(requested_format: Option<&str>) -> Result<(), String> {
    if requested_format.is_some() {
        Err("--format is not supported for evolve verify-baseline yet".to_string())
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
        "  info                              Show workspace summary",
        "  check                             Run workspace checks",
        "  inspect                           Inspect repository structure",
        "  graph                             Render repository graph",
        "  context                           Render AI-readable repository context pack",
        "  context generate current-state    Generate current-state artifact",
        "  context generate handoff          Generate latest handoff artifact",
        "  context generate bootstrap        Generate bootstrap prompt for AI sessions",
        "  context pack                      Assemble project-level context pack",
        "  context verify                    Verify context files exist and are well-formed",
        "  version                           Show runtime version",
        "  help                              Show this help",
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
        "",
        "Context generation:",
        "  monad context generate current-state",
        "  monad context generate handoff",
        "  monad context generate bootstrap",
        "  monad context pack",
        "",
        "Context verification:",
        "  monad context verify",
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
    let report = run_monad_workspace_checks(&context);

    match output_format {
        OutputFormat::Text => {
            let evidence_path = write_check_evidence_packet(&context, &report)
                .map_err(|error| error.to_string())?;
            Ok(format!(
                "{}\n\nEvidence report written: {}",
                render_check_run_report(&report),
                evidence_path.display()
            ))
        }
        OutputFormat::Json => Ok(render_check_run_report_json(&report)),
    }
}

/// Renders verification baseline evolution dry-run output.
fn render_evolve_verify_baseline(dry_run: bool) -> Result<String, String> {
    if !dry_run {
        return Err("evolve verify-baseline currently requires --dry-run".to_string());
    }

    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;

    render_verify_baseline_dry_run(&context).map_err(|error| error.to_string())
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

/// Generates a context artifact and writes it to the repository.
fn render_context_generate(artifact: ContextArtifactKind) -> Result<String, String> {
    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;

    match artifact {
        ContextArtifactKind::CurrentState => {
            let current_state =
                generate_current_state(&context).map_err(|error| error.to_string())?;

            write_current_state_artifact(&context, &current_state)
                .map_err(|error| error.to_string())?;

            Ok(render_current_state_summary(&context, &current_state))
        }
        ContextArtifactKind::Handoff => {
            let handoff = generate_handoff(&context).map_err(|error| error.to_string())?;

            write_handoff_artifact(&context, &handoff).map_err(|error| error.to_string())?;

            Ok(render_handoff_summary(&context, &handoff))
        }
        ContextArtifactKind::Bootstrap => {
            let bootstrap =
                generate_bootstrap_prompt(&context).map_err(|error| error.to_string())?;

            write_bootstrap_prompt_artifact(&context, &bootstrap)
                .map_err(|error| error.to_string())?;

            Ok(render_bootstrap_summary(&context, &bootstrap))
        }
    }
}

/// Renders a concise current-state generation summary.
fn render_current_state_summary(
    context: &WorkspaceContext,
    artifact: &CurrentStateArtifact,
) -> String {
    let output_path = context.context_dir().join("current-state.md");

    let mut lines = vec![
        "Monad current-state artifact generated".to_string(),
        format!("  output: {}", output_path.display()),
        format!("  project: {}", artifact.project_name),
        format!("  epics: {}", artifact.epics.len()),
        format!("  runtime_modules: {}", artifact.runtime_modules.len()),
    ];

    if let Some(active) = artifact.active_epic() {
        lines.push(format!("  active_epic: {} — {}", active.id, active.title));
    }

    let completed_count = artifact.completed_epics().len();
    lines.push(format!("  completed_epics: {completed_count}"));

    lines.join("\n")
}

/// Renders a concise handoff generation summary.
fn render_handoff_summary(context: &WorkspaceContext, artifact: &HandoffArtifact) -> String {
    let output_path = context.context_dir().join("latest-handoff.md");

    let mut lines = vec![
        "Monad handoff artifact generated".to_string(),
        format!("  output: {}", output_path.display()),
        format!("  project: {}", artifact.current_state.project_name),
        format!("  epics: {}", artifact.current_state.epics.len()),
        format!("  work_packets: {}", artifact.work_packets.len()),
    ];

    if let Some(active) = artifact.current_state.active_epic() {
        lines.push(format!("  active_epic: {} — {}", active.id, active.title));
    }

    if let Some(active_wp) = artifact.active_work_packet() {
        lines.push(format!(
            "  active_work_packet: {} — {}",
            active_wp.id, active_wp.title
        ));
    }

    let completed_packets = artifact.completed_work_packets().len();
    lines.push(format!("  completed_work_packets: {completed_packets}"));

    lines.join("\n")
}

/// Renders a concise bootstrap prompt generation summary.
fn render_bootstrap_summary(
    context: &WorkspaceContext,
    artifact: &BootstrapPromptArtifact,
) -> String {
    let output_path = context.root().join("docs/ai/BOOTSTRAP-PROMPT.md");

    let mut lines = vec![
        "Monad bootstrap prompt generated".to_string(),
        format!("  output: {}", output_path.display()),
        format!("  project: {}", artifact.project_name),
        format!("  reading_order: {} files", artifact.reading_order.len()),
        format!("  workflow_rules: {}", artifact.workflow_rules.len()),
        format!("  source_files: {}", artifact.source_files.len()),
    ];

    if let Some(active_epic) = artifact.current_state.active_epic() {
        lines.push(format!(
            "  active_epic: {} — {}",
            active_epic.id, active_epic.title
        ));
    }

    if let Some(active_wp) = artifact.handoff.active_work_packet() {
        lines.push(format!(
            "  active_work_packet: {} — {}",
            active_wp.id, active_wp.title
        ));
    }

    lines.join("\n")
}

/// Assembles and writes a project-level context pack.
fn render_context_pack() -> Result<String, String> {
    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;

    let artifact = generate_context_pack(&context).map_err(|error| error.to_string())?;

    write_context_pack_artifact(&context, &artifact).map_err(|error| error.to_string())?;

    Ok(render_context_pack_summary(&context, &artifact))
}

/// Renders a concise context pack assembly summary.
fn render_context_pack_summary(
    context: &WorkspaceContext,
    artifact: &ContextPackArtifact,
) -> String {
    let output_path = context.context_dir().join("latest-context-pack.md");

    let mut lines = vec![
        "Monad context pack assembled".to_string(),
        format!("  output: {}", output_path.display()),
        format!("  project: {}", artifact.project_name),
        format!("  epics: {}", artifact.current_state.epics.len()),
        format!("  work_packets: {}", artifact.handoff.work_packets.len()),
        format!("  decisions: {}", artifact.accepted_decisions.len()),
        format!("  documents: {}", artifact.important_documents.len()),
        format!("  source_files: {}", artifact.source_files.len()),
    ];

    if let Some(active_wp) = artifact.handoff.active_work_packet() {
        lines.push(format!(
            "  active_work_packet: {} — {}",
            active_wp.id, active_wp.title
        ));
    }

    lines.join("\n")
}

/// Verifies context files and renders the result.
fn render_context_verify() -> Result<String, String> {
    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;
    let report = verify_context(&context);
    let summary = render_context_verify_summary(&report);

    if report.has_errors() {
        Err(summary)
    } else {
        Ok(summary)
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
    fn context_generate_current_state_parses() {
        assert_eq!(
            parse_arguments(&["monad", "context", "generate", "current-state"])
                .expect("context generate current-state should parse"),
            CliCommand::ContextGenerate {
                artifact: ContextArtifactKind::CurrentState
            }
        );
    }

    #[test]
    fn context_generate_handoff_parses() {
        assert_eq!(
            parse_arguments(&["monad", "context", "generate", "handoff"])
                .expect("context generate handoff should parse"),
            CliCommand::ContextGenerate {
                artifact: ContextArtifactKind::Handoff
            }
        );
    }

    #[test]
    fn context_pack_parses() {
        assert_eq!(
            parse_arguments(&["monad", "context", "pack"]).expect("context pack should parse"),
            CliCommand::ContextPack
        );
    }

    #[test]
    fn context_generate_without_artifact_returns_error() {
        let error = parse_arguments(&["monad", "context", "generate"])
            .expect_err("context generate without artifact should fail");

        assert!(error.contains("missing artifact kind"));
        assert!(error.contains("current-state"));
    }

    #[test]
    fn context_generate_unknown_artifact_returns_error() {
        let error = parse_arguments(&["monad", "context", "generate", "foobar"])
            .expect_err("unknown artifact should fail");

        assert!(error.contains("unknown context artifact"));
        assert!(error.contains("foobar"));
    }

    #[test]
    fn context_unknown_subcommand_returns_error() {
        let error = parse_arguments(&["monad", "context", "foobar"])
            .expect_err("unknown subcommand should fail");

        assert!(error.contains("unknown context subcommand"));
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

    #[test]
    fn help_text_mentions_context_generate_current_state() {
        let text = help_text();

        assert!(text.contains("context generate current-state"));
    }

    #[test]
    fn help_text_mentions_context_generate_handoff() {
        let text = help_text();

        assert!(text.contains("context generate handoff"));
    }

    #[test]
    fn help_text_mentions_context_pack() {
        let text = help_text();

        assert!(text.contains("context pack"));
    }

    #[test]
    fn context_generate_bootstrap_parses() {
        assert_eq!(
            parse_arguments(&["monad", "context", "generate", "bootstrap"])
                .expect("context generate bootstrap should parse"),
            CliCommand::ContextGenerate {
                artifact: ContextArtifactKind::Bootstrap,
            }
        );
    }

    #[test]
    fn help_text_mentions_context_generate_bootstrap() {
        let text = help_text();

        assert!(text.contains("context generate bootstrap"));
    }

    #[test]
    fn context_verify_parses() {
        assert_eq!(
            parse_arguments(&["monad", "context", "verify"]).expect("context verify should parse"),
            CliCommand::ContextVerify
        );
    }

    #[test]
    fn help_text_mentions_context_verify() {
        let text = help_text();

        assert!(text.contains("context verify"));
    }
}
