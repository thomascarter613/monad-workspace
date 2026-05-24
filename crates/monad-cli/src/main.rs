//! Command-line entrypoint for Monad.
//!
//! This crate should stay thin. Its job is to parse command-line concerns,
//! call `monad-core`, and present results to the user.
//!
//! Durable product behavior belongs in `monad-core`.

use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use monad_core::{
    MonadError, MonadResult, OutputFormat, RuntimeIdentity, WorkspaceContext,
    load_manifest_from_workspace, render_diagnostic_report, render_workspace_summary,
    run_workspace_checks, runtime_identity, workspace_summary_from_manifest,
};

/// Supported CLI commands for this early runtime foundation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CliCommand {
    /// Print the original startup banner.
    Banner,

    /// Print usage information.
    Help,

    /// Discover the workspace and print project/runtime information.
    Info,

    /// Run the initial workspace checks and print diagnostics.
    Check,
}

/// Parsed command-line invocation.
///
/// This separates command selection from output-format selection. Today only
/// `text` is supported, but this structure gives us a clean place to add JSON,
/// NDJSON, Markdown, or other formats later.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CliInvocation {
    command: CliCommand,
    output_format: OutputFormat,
}

impl Default for CliInvocation {
    fn default() -> Self {
        Self {
            command: CliCommand::Banner,
            output_format: OutputFormat::Text,
        }
    }
}

/// Represents the result of running a CLI command.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CliOutcome {
    output: String,
    success: bool,
}

impl CliOutcome {
    fn success(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            success: true,
        }
    }

    fn failure(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            success: false,
        }
    }

    fn exit_code(&self) -> ExitCode {
        if self.success {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }
}

/// Formats the startup message shown by the CLI.
fn startup_message(identity: RuntimeIdentity) -> String {
    identity.banner()
}

/// Returns a stable command name for error messages.
fn command_name(command: CliCommand) -> &'static str {
    match command {
        CliCommand::Banner => "banner",
        CliCommand::Help => "help",
        CliCommand::Info => "info",
        CliCommand::Check => "check",
    }
}

/// Sets the command on an invocation.
///
/// Only one command is allowed. Options such as `--format text` may appear
/// before or after the command, but command words cannot be repeated.
fn set_command(invocation: &mut CliInvocation, command: CliCommand) -> MonadResult<()> {
    if invocation.command != CliCommand::Banner {
        return Err(MonadError::invalid_input(format!(
            "multiple commands provided: {} and {}",
            command_name(invocation.command),
            command_name(command)
        )));
    }

    invocation.command = command;
    Ok(())
}

/// Parses command-line arguments into a command invocation.
///
/// Supported forms:
///
/// - `monad`
/// - `monad help`
/// - `monad info`
/// - `monad check`
/// - `monad info --format text`
/// - `monad --format text info`
/// - `monad check --format=text`
fn parse_invocation(args: &[String]) -> MonadResult<CliInvocation> {
    let mut invocation = CliInvocation::default();
    let mut index = 1;

    while index < args.len() {
        let argument = &args[index];

        match argument.as_str() {
            "help" | "-h" | "--help" => {
                set_command(&mut invocation, CliCommand::Help)?;
            }
            "info" => {
                set_command(&mut invocation, CliCommand::Info)?;
            }
            "check" => {
                set_command(&mut invocation, CliCommand::Check)?;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    MonadError::invalid_input("--format requires a value, such as text")
                })?;

                invocation.output_format = OutputFormat::parse(value)?;
                index += 1;
            }
            value if value.starts_with("--format=") => {
                let format_value = value
                    .strip_prefix("--format=")
                    .expect("prefix was checked above");

                if format_value.trim().is_empty() {
                    return Err(MonadError::invalid_input(
                        "--format requires a value, such as text",
                    ));
                }

                invocation.output_format = OutputFormat::parse(format_value)?;
            }
            value if value.starts_with('-') => {
                return Err(MonadError::invalid_input(format!(
                    "unknown option: {value}"
                )));
            }
            value => {
                return Err(MonadError::invalid_input(format!(
                    "unknown command: {value}"
                )));
            }
        }

        index += 1;
    }

    Ok(invocation)
}

/// Renders CLI help text.
///
/// Help remains in the CLI because it describes CLI command names and usage.
fn render_help() -> String {
    [
        "Usage: monad [COMMAND] [OPTIONS]",
        "",
        "Commands:",
        "  info      Show Monad workspace and manifest information",
        "  check     Run Monad workspace checks",
        "  help      Show this help text",
        "",
        "Options:",
        "  --format text     Render output as human-readable text",
        "  --format json     Render output as machine-readable JSON",
        "",
        "With no command, Monad prints the runtime foundation banner.",
    ]
    .join("\n")
}

/// Discovers a workspace from `start`, loads `monad.toml`, and renders summary
/// information through `monad-core` output formatting.
fn render_workspace_info(start: impl AsRef<Path>, format: OutputFormat) -> MonadResult<String> {
    let context = WorkspaceContext::discover_from(start)?;
    let manifest = load_manifest_from_workspace(&context)?;
    let summary = workspace_summary_from_manifest(&context, &manifest);

    Ok(render_workspace_summary(&summary, format))
}

/// Discovers a workspace from `start`, runs workspace checks, and returns a CLI
/// outcome.
fn run_workspace_check(
    start: impl AsRef<Path>,
    output_format: OutputFormat,
) -> MonadResult<CliOutcome> {
    let context = WorkspaceContext::discover_from(start)?;
    let report = run_workspace_checks(&context);
    let output = render_diagnostic_report(&report, output_format);

    if report.has_errors() {
        Ok(CliOutcome::failure(output))
    } else {
        Ok(CliOutcome::success(output))
    }
}

/// Runs the CLI and returns output plus success/failure state.
fn run_with_args(args: &[String], current_dir: impl AsRef<Path>) -> MonadResult<CliOutcome> {
    let invocation = parse_invocation(args)?;

    match invocation.command {
        CliCommand::Banner => Ok(CliOutcome::success(startup_message(runtime_identity()))),
        CliCommand::Help => Ok(CliOutcome::success(render_help())),
        CliCommand::Info => {
            render_workspace_info(current_dir, invocation.output_format).map(CliOutcome::success)
        }
        CliCommand::Check => run_workspace_check(current_dir, invocation.output_format),
    }
}

/// Program entrypoint.
fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(error) => {
            let error = MonadError::internal(format!("failed to read current directory: {error}"));
            eprintln!("{}", error.to_diagnostic().render());
            return ExitCode::FAILURE;
        }
    };

    match run_with_args(&args, current_dir) {
        Ok(outcome) => {
            if outcome.success {
                println!("{}", outcome.output);
            } else {
                eprintln!("{}", outcome.output);
            }

            outcome.exit_code()
        }
        Err(error) => {
            eprintln!("{}", error.to_diagnostic().render());
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

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

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        env::temp_dir().join(format!(
            "monad-cli-format-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_test_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);

        fs::create_dir_all(root.join("docs")).expect("docs directory should be created");
        fs::create_dir_all(root.join("work")).expect("work directory should be created");
        fs::create_dir_all(root.join(".monad")).expect(".monad directory should be created");
        fs::create_dir_all(root.join("crates/monad-cli"))
            .expect("monad-cli directory should be created");
        fs::create_dir_all(root.join("crates/monad-core"))
            .expect("monad-core directory should be created");

        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");
        fs::write(root.join("monad.toml"), VALID_MANIFEST_TOML)
            .expect("monad.toml should be written");

        root
    }

    #[test]
    fn no_command_prints_runtime_banner() {
        let outcome = run_with_args(&args(&["monad"]), ".").expect("banner should render");

        assert!(outcome.success);
        assert!(outcome.output.contains("Monad"));
        assert!(outcome.output.contains("monad-core"));
        assert!(outcome.output.contains("local-first"));
    }

    #[test]
    fn help_command_prints_usage_and_format_option() {
        let outcome = run_with_args(&args(&["monad", "help"]), ".").expect("help should render");

        assert!(outcome.success);
        assert!(outcome.output.contains("Usage: monad [COMMAND] [OPTIONS]"));
        assert!(outcome.output.contains("info"));
        assert!(outcome.output.contains("check"));
        assert!(outcome.output.contains("--format text"));
    }

    #[test]
    fn unknown_command_returns_invalid_input_error() {
        let error = run_with_args(&args(&["monad", "unknown"]), ".")
            .expect_err("unknown command should fail");

        assert_eq!(error.code(), "MONAD2001");
        assert!(error.message().contains("unknown command"));
    }

    #[test]
    fn unknown_option_returns_invalid_input_error() {
        let error = run_with_args(&args(&["monad", "--verbose"]), ".")
            .expect_err("unknown option should fail");

        assert_eq!(error.code(), "MONAD2001");
        assert!(error.message().contains("unknown option"));
    }

    #[test]
    fn missing_format_value_returns_invalid_input_error() {
        let error = run_with_args(&args(&["monad", "info", "--format"]), ".")
            .expect_err("missing format should fail");

        assert_eq!(error.code(), "MONAD2001");
        assert!(error.message().contains("--format requires a value"));
    }

    #[test]
    fn json_format_is_supported_for_info_command() {
        let root = create_test_workspace("info-json");

        let outcome = run_with_args(&args(&["monad", "info", "--format", "json"]), &root)
            .expect("json format should now be supported");

        assert!(outcome.success);
        assert!(outcome.output.contains(r#""format": "json""#));
        assert!(outcome.output.contains(r#""kind": "workspace_summary""#));
        assert!(outcome.output.contains(r#""display_name": "Monad""#));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn multiple_commands_return_invalid_input_error() {
        let error = run_with_args(&args(&["monad", "info", "check"]), ".")
            .expect_err("multiple commands should fail");

        assert_eq!(error.code(), "MONAD2001");
        assert!(error.message().contains("multiple commands provided"));
    }

    #[test]
    fn info_command_accepts_format_option_after_command() {
        let root = create_test_workspace("info-format-after");

        let outcome = run_with_args(&args(&["monad", "info", "--format", "text"]), &root)
            .expect("info command should succeed");

        assert!(outcome.success);
        assert!(outcome.output.contains("Monad workspace"));
        assert!(outcome.output.contains("project: Monad (monad)"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn info_command_accepts_format_option_before_command() {
        let root = create_test_workspace("info-format-before");

        let outcome = run_with_args(&args(&["monad", "--format", "text", "info"]), &root)
            .expect("info command should succeed");

        assert!(outcome.success);
        assert!(outcome.output.contains("Monad workspace"));
        assert!(outcome.output.contains("project: Monad (monad)"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn check_command_accepts_format_equals_option() {
        let root = create_test_workspace("check-format-equals");

        let outcome = run_with_args(&args(&["monad", "check", "--format=text"]), &root)
            .expect("check command should run");

        assert!(outcome.success);
        assert!(outcome.output.contains("[INFO] MONAD4000"));
        assert!(outcome.output.contains("[INFO] MONAD4500"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn check_command_accepts_json_format() {
        let root = create_test_workspace("check-json");

        let outcome = run_with_args(&args(&["monad", "check", "--format=json"]), &root)
            .expect("check command should run with JSON output");

        assert!(outcome.success);
        assert!(outcome.output.contains(r#""format": "json""#));
        assert!(outcome.output.contains(r#""kind": "diagnostic_report""#));
        assert!(outcome.output.contains(r#""has_errors": false"#));
        assert!(outcome.output.contains(r#""code": "MONAD4000""#));
        assert!(outcome.output.contains(r#""code": "MONAD4500""#));

        fs::remove_dir_all(root).ok();
    }
}
