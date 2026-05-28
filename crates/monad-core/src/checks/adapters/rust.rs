//! Rust/Cargo adapter checks.

use crate::{
    AdapterCheckRun, CheckDefinition, CheckId, CheckResult, CheckSeverity, CommandResult,
    CommandSpec, RepositoryToolchainDetection, RepositoryToolchainKind, WorkspaceContext,
};

/// Selects Rust/Cargo checks when Rust tooling is detected.
#[must_use]
pub fn select_rust_checks(
    context: &WorkspaceContext,
    detection: &RepositoryToolchainDetection,
) -> AdapterCheckRun {
    let definitions = rust_check_definitions();

    if !detection.has_toolchain(RepositoryToolchainKind::Rust) {
        return AdapterCheckRun::from_parts(
            definitions,
            vec![
                CheckResult::skipped(
                    CheckId::new("MONAD-CHECK-RUST-0001"),
                    "Rust/Cargo checks skipped because no Rust toolchain signals were detected.",
                ),
                CheckResult::skipped(
                    CheckId::new("MONAD-CHECK-RUST-0002"),
                    "Cargo manifest check skipped because no Rust toolchain signals were detected.",
                ),
            ],
            Vec::new(),
        );
    }

    let manifest_result = if context.root().join("Cargo.toml").is_file() {
        CheckResult::passed(
            CheckId::new("MONAD-CHECK-RUST-0002"),
            "Cargo manifest exists: Cargo.toml",
        )
    } else {
        CheckResult::failed(
            CheckId::new("MONAD-CHECK-RUST-0002"),
            "Rust was detected but Cargo.toml was not found at the workspace root.",
        )
    };

    let cargo_version = run_cargo_version_check(context);

    AdapterCheckRun::from_parts(
        definitions,
        vec![cargo_version.result, manifest_result],
        cargo_version.command_result.into_iter().collect(),
    )
}

fn rust_check_definitions() -> Vec<CheckDefinition> {
    vec![
        CheckDefinition::new(
            CheckId::new("MONAD-CHECK-RUST-0001"),
            "Cargo command is available",
            CheckSeverity::Error,
            "Runs cargo --version for repositories with Rust toolchain signals.",
        ),
        CheckDefinition::new(
            CheckId::new("MONAD-CHECK-RUST-0002"),
            "Cargo manifest exists",
            CheckSeverity::Error,
            "Checks whether a detected Rust repository has a root Cargo.toml.",
        ),
    ]
}

struct CommandBackedCheck {
    result: CheckResult,
    command_result: Option<CommandResult>,
}

fn run_cargo_version_check(context: &WorkspaceContext) -> CommandBackedCheck {
    let spec = CommandSpec::new("cargo")
        .arg("--version")
        .working_directory(context.root());

    match spec.run() {
        Ok(command_result) if command_result.success() => {
            let message = first_non_empty_line(command_result.stdout())
                .unwrap_or_else(|| "cargo is available".to_string());

            CommandBackedCheck {
                result: CheckResult::passed(CheckId::new("MONAD-CHECK-RUST-0001"), message),
                command_result: Some(command_result),
            }
        }
        Ok(command_result) => {
            let exit_code = command_result.exit_code();
            let stderr = first_non_empty_line(command_result.stderr())
                .unwrap_or_else(|| "no stderr output".to_string());

            CommandBackedCheck {
                result: CheckResult::failed(
                    CheckId::new("MONAD-CHECK-RUST-0001"),
                    format!("cargo --version failed with exit code {exit_code:?}: {stderr}"),
                ),
                command_result: Some(command_result),
            }
        }
        Err(error) => CommandBackedCheck {
            result: CheckResult::failed(
                CheckId::new("MONAD-CHECK-RUST-0001"),
                format!("failed to run cargo --version: {}", error.message()),
            ),
            command_result: None,
        },
    }
}

fn first_non_empty_line(text: &str) -> Option<String> {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToOwned::to_owned)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{RepositoryToolchainSignal, RepositoryToolchainSignalKind};

    use super::*;

    fn unique_temp_root(name: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);

        std::env::temp_dir().join(format!("monad-rust-adapter-{name}-{unique}"))
    }

    fn rust_detection() -> RepositoryToolchainDetection {
        RepositoryToolchainDetection::from_signals(vec![RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Rust,
            RepositoryToolchainSignalKind::Manifest,
            "Cargo.toml",
        )])
    }

    #[test]
    fn rust_checks_are_skipped_when_rust_is_not_detected() {
        let context = WorkspaceContext::new(".").expect("workspace context should be created");
        let detection = RepositoryToolchainDetection::from_signals(Vec::new());

        let run = select_rust_checks(&context, &detection);

        assert_eq!(run.results().len(), 2);
        assert!(
            run.results()
                .iter()
                .all(|result| { result.status() == crate::CheckStatus::Skipped })
        );
    }

    #[test]
    fn rust_manifest_check_fails_when_root_manifest_is_missing() {
        let root = unique_temp_root("missing-manifest");
        fs::create_dir_all(&root).expect("test root should be created");

        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let run = select_rust_checks(&context, &rust_detection());

        assert!(run.results().iter().any(|result| {
            result.check_id().as_str() == "MONAD-CHECK-RUST-0002"
                && result.status() == crate::CheckStatus::Failed
        }));

        fs::remove_dir_all(&root).ok();
    }
}
