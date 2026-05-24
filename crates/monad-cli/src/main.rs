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
    MonadError, MonadResult, RuntimeIdentity, WorkspaceContext, load_manifest_from_workspace,
    runtime_identity,
};

/// Supported CLI commands for this early runtime foundation.
///
/// This enum stays intentionally small. Later slices can replace this manual
/// parser with a richer command framework if the project chooses one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CliCommand {
    /// Print the original startup banner.
    Banner,

    /// Print usage information.
    Help,

    /// Discover the workspace and print project/runtime information.
    Info,
}

/// Formats the startup message shown by the CLI.
///
/// Keeping this as a small function makes it easy to test without spawning a
/// separate process.
fn startup_message(identity: RuntimeIdentity) -> String {
    identity.banner()
}

/// Parses command-line arguments into a supported command.
///
/// `args[0]` is conventionally the executable name. The first real command is
/// therefore `args[1]`.
fn parse_command(args: &[String]) -> MonadResult<CliCommand> {
    match args.get(1).map(String::as_str) {
        None => Ok(CliCommand::Banner),
        Some("help" | "-h" | "--help") => Ok(CliCommand::Help),
        Some("info") => Ok(CliCommand::Info),
        Some(command) => Err(MonadError::invalid_input(format!(
            "unknown command: {command}"
        ))),
    }
}

/// Renders CLI help text.
fn render_help() -> String {
    [
        "Usage: monad [COMMAND]",
        "",
        "Commands:",
        "  info      Show Monad workspace and manifest information",
        "  help      Show this help text",
        "",
        "With no command, Monad prints the runtime foundation banner.",
    ]
    .join("\n")
}

/// Discovers a workspace from `start`, loads `monad.toml`, and renders summary
/// information.
///
/// This is the first CLI behavior that composes earlier runtime primitives:
///
/// - `WorkspaceContext` from WP-E1-004;
/// - `MonadManifest` loading from WP-E1-006;
/// - `MonadResult<T>` and `MonadError` from WP-E1-003.
fn render_workspace_info(start: impl AsRef<Path>) -> MonadResult<String> {
    let context = WorkspaceContext::discover_from(start)?;
    let manifest = load_manifest_from_workspace(&context)?;

    Ok(format!(
        "Monad workspace\n  root: {}\n  project: {} ({})\n  schema_version: {}\n  core_crate: {}\n  cli_crate: {}\n  execution_model: {}",
        context.root().display(),
        manifest.project.display_name,
        manifest.project.name,
        manifest.schema_version,
        manifest.runtime.core_crate,
        manifest.runtime.cli_crate,
        manifest.runtime.execution_model,
    ))
}

/// Runs the CLI and returns the output that should be printed.
///
/// Returning a string instead of printing directly keeps command behavior easy
/// to test.
fn run_with_args(args: &[String], current_dir: impl AsRef<Path>) -> MonadResult<String> {
    match parse_command(args)? {
        CliCommand::Banner => Ok(startup_message(runtime_identity())),
        CliCommand::Help => Ok(render_help()),
        CliCommand::Info => render_workspace_info(current_dir),
    }
}

/// Program entrypoint.
///
/// Rust binaries start in `main`. Returning `ExitCode` lets us clearly signal
/// success or failure to the operating system.
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
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
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
            "monad-cli-info-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_test_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);

        fs::create_dir_all(root.join("crates")).expect("crates directory should be created");
        fs::create_dir_all(root.join(".monad")).expect(".monad directory should be created");
        fs::create_dir_all(root.join("work")).expect("work directory should be created");
        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");
        fs::write(root.join("monad.toml"), VALID_MANIFEST_TOML)
            .expect("monad.toml should be written");

        root
    }

    #[test]
    fn no_command_prints_runtime_banner() {
        let output = run_with_args(&args(&["monad"]), ".").expect("banner should render");

        assert!(output.contains("Monad"));
        assert!(output.contains("monad-core"));
        assert!(output.contains("local-first"));
    }

    #[test]
    fn help_command_prints_usage() {
        let output = run_with_args(&args(&["monad", "help"]), ".").expect("help should render");

        assert!(output.contains("Usage: monad [COMMAND]"));
        assert!(output.contains("info"));
    }

    #[test]
    fn unknown_command_returns_invalid_input_error() {
        let error = run_with_args(&args(&["monad", "unknown"]), ".")
            .expect_err("unknown command should fail");

        assert_eq!(error.code(), "MONAD2001");
        assert!(error.message().contains("unknown command"));
    }

    #[test]
    fn info_command_loads_workspace_manifest() {
        let root = create_test_workspace("info");

        let output =
            run_with_args(&args(&["monad", "info"]), &root).expect("info command should succeed");

        assert!(output.contains("Monad workspace"));
        assert!(output.contains("project: Monad (monad)"));
        assert!(output.contains("schema_version: 1"));
        assert!(output.contains("core_crate: monad-core"));
        assert!(output.contains("cli_crate: monad-cli"));
        assert!(output.contains("execution_model: local-first"));

        fs::remove_dir_all(root).ok();
    }
}
