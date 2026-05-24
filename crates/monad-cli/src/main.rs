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

/// Parses command-line arguments into a supported command.
fn parse_command(args: &[String]) -> MonadResult<CliCommand> {
    match args.get(1).map(String::as_str) {
        None => Ok(CliCommand::Banner),
        Some("help" | "-h" | "--help") => Ok(CliCommand::Help),
        Some("info") => Ok(CliCommand::Info),
        Some("check") => Ok(CliCommand::Check),
        Some(command) => Err(MonadError::invalid_input(format!(
            "unknown command: {command}"
        ))),
    }
}

/// Renders CLI help text.
///
/// Help remains in the CLI because it describes CLI command names and usage.
fn render_help() -> String {
    [
        "Usage: monad [COMMAND]",
        "",
        "Commands:",
        "  info      Show Monad workspace and manifest information",
        "  check     Run Monad workspace checks",
        "  help      Show this help text",
        "",
        "With no command, Monad prints the runtime foundation banner.",
    ]
    .join("\n")
}

/// Discovers a workspace from `start`, loads `monad.toml`, and renders summary
/// information through `monad-core` output formatting.
fn render_workspace_info(start: impl AsRef<Path>) -> MonadResult<String> {
    let context = WorkspaceContext::discover_from(start)?;
    let manifest = load_manifest_from_workspace(&context)?;
    let summary = workspace_summary_from_manifest(&context, &manifest);

    Ok(render_workspace_summary(&summary, OutputFormat::Text))
}

/// Discovers a workspace from `start`, runs workspace checks, and returns a CLI
/// outcome.
fn run_workspace_check(start: impl AsRef<Path>) -> MonadResult<CliOutcome> {
    let context = WorkspaceContext::discover_from(start)?;
    let report = run_workspace_checks(&context);
    let output = render_diagnostic_report(&report, OutputFormat::Text);

    if report.has_errors() {
        Ok(CliOutcome::failure(output))
    } else {
        Ok(CliOutcome::success(output))
    }
}

/// Runs the CLI and returns output plus success/failure state.
fn run_with_args(args: &[String], current_dir: impl AsRef<Path>) -> MonadResult<CliOutcome> {
    match parse_command(args)? {
        CliCommand::Banner => Ok(CliOutcome::success(startup_message(runtime_identity()))),
        CliCommand::Help => Ok(CliOutcome::success(render_help())),
        CliCommand::Info => render_workspace_info(current_dir).map(CliOutcome::success),
        CliCommand::Check => run_workspace_check(current_dir),
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
            "monad-cli-output-{test_name}-{}-{unique}",
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
    fn help_command_prints_usage() {
        let outcome = run_with_args(&args(&["monad", "help"]), ".").expect("help should render");

        assert!(outcome.success);
        assert!(outcome.output.contains("Usage: monad [COMMAND]"));
        assert!(outcome.output.contains("info"));
        assert!(outcome.output.contains("check"));
    }

    #[test]
    fn unknown_command_returns_invalid_input_error() {
        let error = run_with_args(&args(&["monad", "unknown"]), ".")
            .expect_err("unknown command should fail");

        assert_eq!(error.code(), "MONAD2001");
        assert!(error.message().contains("unknown command"));
    }

    #[test]
    fn info_command_uses_core_output_formatting() {
        let root = create_test_workspace("info");

        let outcome =
            run_with_args(&args(&["monad", "info"]), &root).expect("info command should succeed");

        assert!(outcome.success);
        assert!(outcome.output.contains("Monad workspace"));
        assert!(outcome.output.contains("project: Monad (monad)"));
        assert!(outcome.output.contains("schema_version: 1"));
        assert!(outcome.output.contains("core_crate: monad-core"));
        assert!(outcome.output.contains("cli_crate: monad-cli"));
        assert!(outcome.output.contains("execution_model: local-first"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn check_command_uses_core_diagnostic_output_formatting() {
        let root = create_test_workspace("check");

        let outcome =
            run_with_args(&args(&["monad", "check"]), &root).expect("check command should run");

        assert!(outcome.success);
        assert!(outcome.output.contains("[INFO] MONAD4000"));
        assert!(outcome.output.contains("[INFO] MONAD4001"));
        assert!(outcome.output.contains("[INFO] MONAD4002"));
        assert!(outcome.output.contains("[INFO] MONAD4003"));
        assert!(outcome.output.contains("[INFO] MONAD4004"));
        assert!(outcome.output.contains("[INFO] MONAD4500"));

        fs::remove_dir_all(root).ok();
    }
}
