//! CLI smoke tests for Monad.
//!
//! These tests exercise the compiled `monad` binary the way a user would call
//! it from a terminal. They intentionally stay narrow: the goal is to catch
//! command-surface regressions without building a full end-to-end test harness.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Returns the path to the compiled `monad` test binary.
///
/// Cargo sets this environment variable for integration tests when the package
/// declares a binary named `monad`.
fn monad_binary() -> &'static str {
    env!("CARGO_BIN_EXE_monad")
}

/// Returns the repository workspace root for CLI smoke tests.
///
/// Cargo runs package integration tests with the package directory as a common
/// working-directory assumption. Monad commands such as `inspect`, `check`, and
/// `evolve ... --dry-run` need to execute from the repository workspace root so
/// `WorkspaceContext::discover_from(".")` can find the Monad workspace.
fn workspace_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    manifest_dir
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .unwrap_or_else(|| manifest_dir.to_path_buf())
}

/// Runs the `monad` binary with arguments and returns process output.
fn run_monad(arguments: &[&str]) -> Result<Output, std::io::Error> {
    Command::new(monad_binary())
        .current_dir(workspace_root())
        .args(arguments)
        .output()
}

/// Converts stdout bytes into a lossy UTF-8 string for test assertions.
fn stdout_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Converts stderr bytes into a lossy UTF-8 string for test assertions.
fn stderr_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Asserts that a command succeeded and includes the expected stdout fragment.
fn assert_success_contains(arguments: &[&str], expected_stdout: &str) {
    let output_result = run_monad(arguments);

    assert!(
        output_result.is_ok(),
        "monad binary should execute for arguments {arguments:?}: {:?}",
        output_result.as_ref().err()
    );

    let output = match output_result {
        Ok(output) => output,
        Err(_) => return,
    };

    assert!(
        output.status.success(),
        "expected command to succeed: monad {arguments:?}\nstdout:\n{}\nstderr:\n{}",
        stdout_text(&output),
        stderr_text(&output)
    );

    assert!(
        stdout_text(&output).contains(expected_stdout),
        "expected stdout to contain {expected_stdout:?} for command: monad {arguments:?}\nstdout:\n{}",
        stdout_text(&output)
    );
}

/// Asserts that a command failed and includes the expected stderr fragment.
fn assert_failure_contains(arguments: &[&str], expected_stderr: &str) {
    let output_result = run_monad(arguments);

    assert!(
        output_result.is_ok(),
        "monad binary should execute for arguments {arguments:?}: {:?}",
        output_result.as_ref().err()
    );

    let output = match output_result {
        Ok(output) => output,
        Err(_) => return,
    };

    assert!(
        !output.status.success(),
        "expected command to fail: monad {arguments:?}\nstdout:\n{}\nstderr:\n{}",
        stdout_text(&output),
        stderr_text(&output)
    );

    assert!(
        stderr_text(&output).contains(expected_stderr),
        "expected stderr to contain {expected_stderr:?} for command: monad {arguments:?}\nstderr:\n{}",
        stderr_text(&output)
    );
}

#[test]
fn help_command_smoke_test() {
    assert_success_contains(&["--help"], "Usage:");
    assert_success_contains(&["help"], "Core commands:");
    assert_success_contains(&["help"], "plan \"<intent>\"");
    assert_success_contains(&["help"], "evolve verify-baseline --dry-run");
    assert_success_contains(&["help"], "evolve context-baseline --dry-run");
}

#[test]
fn version_command_smoke_test() {
    assert_success_contains(&["version"], "Monad");
    assert_success_contains(&["--version"], "Monad");
}

#[test]
fn inspect_command_smoke_test() {
    assert_success_contains(&["inspect"], "repository");
}

#[test]
fn check_command_smoke_test() {
    assert_success_contains(&["check"], "Checks run:");
}

#[test]
fn plan_command_smoke_test() {
    assert_success_contains(
        &["plan", "explain", "this", "repository"],
        "Monad supervised plan",
    );
    assert_success_contains(
        &["plan", "explain", "this", "repository"],
        "No files were created, updated, or deleted.",
    );
    assert_success_contains(
        &["plan", "explain", "this", "repository"],
        "No shell commands were run.",
    );
    assert_success_contains(
        &["plan", "explain", "this", "repository"],
        "No Git state was changed.",
    );
    assert_success_contains(
        &["plan", "explain", "this", "repository"],
        "No real model provider or external AI API was called.",
    );
}

#[test]
fn plan_missing_intent_failure_smoke_test() {
    assert_failure_contains(&["plan"], "missing plan intent");
    assert_failure_contains(&["plan"], "explain this repository");
}

#[test]
fn plan_unsupported_format_failure_smoke_test() {
    assert_failure_contains(
        &["plan", "explain", "this", "repository", "--format=json"],
        "--format is not supported for plan yet",
    );
}

#[test]
fn evolve_verify_baseline_requires_dry_run_smoke_test() {
    assert_failure_contains(
        &["evolve", "verify-baseline"],
        "evolve verify-baseline currently requires --dry-run",
    );
}

#[test]
fn evolve_context_baseline_requires_dry_run_smoke_test() {
    assert_failure_contains(
        &["evolve", "context-baseline"],
        "evolve context-baseline currently requires --dry-run",
    );
}

#[test]
fn evolve_verify_baseline_dry_run_smoke_test() {
    assert_success_contains(
        &["evolve", "verify-baseline", "--dry-run"],
        "Mode: dry-run",
    );
    assert_success_contains(
        &["evolve", "verify-baseline", "--dry-run"],
        "No files were written.",
    );
}

#[test]
fn evolve_context_baseline_dry_run_smoke_test() {
    assert_success_contains(
        &["evolve", "context-baseline", "--dry-run"],
        "Mode: dry-run",
    );
    assert_success_contains(
        &["evolve", "context-baseline", "--dry-run"],
        "No files were written.",
    );
    assert_success_contains(
        &["evolve", "context-baseline", "--dry-run"],
        "No AI summarization was performed.",
    );
}

#[test]
fn unsupported_argument_failure_smoke_test() {
    assert_failure_contains(&["inspect", "--wat"], "unsupported argument: --wat");
}

#[test]
fn unsupported_write_flag_failure_smoke_test() {
    assert_failure_contains(
        &["inspect", "--write"],
        "--write is only supported for the context command",
    );
}
