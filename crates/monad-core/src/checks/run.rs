//! Initial workspace verification runner for Monad.
//!
//! This module connects the E4 check model to executable verification behavior.
//! It intentionally starts small: the first `monad check` should be useful,
//! readable, and predictable without becoming a full CI system.

use std::path::Path;

use crate::{
    CheckDefinition, CheckId, CheckRegistry, CheckResult, CheckSeverity, CheckStatus,
    CommandResult, CommandSpec, WorkspaceContext,
};

/// Aggregated result of running Monad workspace checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckRunReport {
    registry: CheckRegistry,
    results: Vec<CheckResult>,
    command_results: Vec<CommandResult>,
}

impl CheckRunReport {
    /// Creates a report from a registry and ordered results.
    #[must_use]
    pub fn new(registry: CheckRegistry, results: Vec<CheckResult>) -> Self {
        Self {
            registry,
            results,
            command_results: Vec::new(),
        }
    }

    /// Creates a report from a registry, ordered results, and command results.
    #[must_use]
    pub fn with_command_results(
        registry: CheckRegistry,
        results: Vec<CheckResult>,
        command_results: Vec<CommandResult>,
    ) -> Self {
        Self {
            registry,
            results,
            command_results,
        }
    }

    /// Returns the check registry used for this run.
    #[must_use]
    pub const fn registry(&self) -> &CheckRegistry {
        &self.registry
    }

    /// Returns check results in deterministic execution order.
    #[must_use]
    pub fn results(&self) -> &[CheckResult] {
        &self.results
    }

    /// Returns command results captured while running checks.
    #[must_use]
    pub fn command_results(&self) -> &[CommandResult] {
        &self.command_results
    }

    /// Returns the number of checks that ran.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Returns true when one or more checks failed.
    #[must_use]
    pub fn has_failures(&self) -> bool {
        self.results().iter().any(CheckResult::is_failure)
    }

    /// Returns the number of passed checks.
    #[must_use]
    pub fn passed_count(&self) -> usize {
        self.count_by_status(CheckStatus::Passed)
    }

    /// Returns the number of failed checks.
    #[must_use]
    pub fn failed_count(&self) -> usize {
        self.count_by_status(CheckStatus::Failed)
    }

    /// Returns the number of warning checks.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.count_by_status(CheckStatus::Warning)
    }

    /// Returns the number of skipped checks.
    #[must_use]
    pub fn skipped_count(&self) -> usize {
        self.count_by_status(CheckStatus::Skipped)
    }

    fn count_by_status(&self, status: CheckStatus) -> usize {
        self.results()
            .iter()
            .filter(|result| result.status() == status)
            .count()
    }
}

/// Builds the initial workspace check registry.
///
/// The registry is separated from execution so future slices can list,
/// describe, filter, or render checks without running them.
#[must_use]
pub fn initial_workspace_check_registry() -> CheckRegistry {
    CheckRegistry::from_definitions([
        CheckDefinition::new(
            CheckId::new("MONAD-CHECK-0001"),
            "Workspace manifest exists",
            CheckSeverity::Error,
            "Checks whether the repository root contains monad.toml.",
        ),
        CheckDefinition::new(
            CheckId::new("MONAD-CHECK-0002"),
            "Cargo manifest exists",
            CheckSeverity::Error,
            "Checks whether the repository root contains Cargo.toml.",
        ),
        CheckDefinition::new(
            CheckId::new("MONAD-CHECK-0003"),
            "Cargo is available",
            CheckSeverity::Error,
            "Runs cargo --version to confirm the Rust toolchain command is available.",
        ),
    ])
}

/// Runs the initial Monad workspace checks.
#[must_use]
pub fn run_monad_workspace_checks(context: &WorkspaceContext) -> CheckRunReport {
    let registry = initial_workspace_check_registry();
    let root = context.root();

    let cargo_check = check_cargo_available(root);
    let mut command_results = Vec::new();

    if let Some(command_result) = cargo_check.command_result {
        command_results.push(command_result);
    }

    let results = vec![
        check_required_file(root, "monad.toml", "MONAD-CHECK-0001"),
        check_required_file(root, "Cargo.toml", "MONAD-CHECK-0002"),
        cargo_check.result,
    ];

    CheckRunReport::with_command_results(registry, results, command_results)
}

/// Renders a human-readable check report.
#[must_use]
pub fn render_check_run_report(report: &CheckRunReport) -> String {
    let mut lines = Vec::new();

    lines.push("Monad check report".to_string());
    lines.push(String::new());
    lines.push(format!("Checks run: {}", report.result_count()));
    lines.push(format!("Passed: {}", report.passed_count()));
    lines.push(format!("Failed: {}", report.failed_count()));
    lines.push(format!("Warnings: {}", report.warning_count()));
    lines.push(format!("Skipped: {}", report.skipped_count()));
    lines.push(String::new());

    for result in report.results() {
        lines.push(format!(
            "[{}] {}: {}",
            result.status().as_str().to_uppercase(),
            result.check_id().as_str(),
            result.message()
        ));
    }

    if report.has_failures() {
        lines.push(String::new());
        lines.push("Result: failed".to_string());
    } else {
        lines.push(String::new());
        lines.push("Result: passed".to_string());
    }

    lines.join("\n")
}

fn check_required_file(root: &Path, file_name: &str, check_id: &str) -> CheckResult {
    let path = root.join(file_name);

    if path.is_file() {
        CheckResult::passed(
            CheckId::new(check_id),
            format!("required file exists: {file_name}"),
        )
    } else {
        CheckResult::failed(
            CheckId::new(check_id),
            format!("required file is missing: {file_name}"),
        )
    }
}

struct CommandCheckOutcome {
    result: CheckResult,
    command_result: Option<CommandResult>,
}

fn check_cargo_available(root: &Path) -> CommandCheckOutcome {
    let spec = CommandSpec::new("cargo")
        .arg("--version")
        .working_directory(root);

    match spec.run() {
        Ok(command_result) if command_result.success() => {
            let message = first_non_empty_line(command_result.stdout())
                .unwrap_or_else(|| "cargo is available".to_string());

            CommandCheckOutcome {
                result: CheckResult::passed(CheckId::new("MONAD-CHECK-0003"), message),
                command_result: Some(command_result),
            }
        }
        Ok(command_result) => {
            let exit_code = command_result.exit_code();
            let stderr_summary = first_non_empty_line(command_result.stderr())
                .unwrap_or_else(|| "no stderr output".to_string());

            CommandCheckOutcome {
                result: CheckResult::failed(
                    CheckId::new("MONAD-CHECK-0003"),
                    format!(
                        "cargo --version failed with exit code {exit_code:?}: {stderr_summary}"
                    ),
                ),
                command_result: Some(command_result),
            }
        }
        Err(error) => CommandCheckOutcome {
            result: CheckResult::failed(
                CheckId::new("MONAD-CHECK-0003"),
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

    use super::*;

    fn unique_temp_root(name: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);

        std::env::temp_dir().join(format!("monad-{name}-{unique}"))
    }

    #[test]
    fn initial_registry_contains_expected_checks() {
        let registry = initial_workspace_check_registry();

        assert_eq!(registry.len(), 3);
        assert!(registry.contains(&CheckId::new("MONAD-CHECK-0001")));
        assert!(registry.contains(&CheckId::new("MONAD-CHECK-0002")));
        assert!(registry.contains(&CheckId::new("MONAD-CHECK-0003")));
    }

    #[test]
    fn report_counts_statuses() {
        let registry = initial_workspace_check_registry();
        let report = CheckRunReport::new(
            registry,
            vec![
                CheckResult::passed(CheckId::new("A"), "passed"),
                CheckResult::failed(CheckId::new("B"), "failed"),
                CheckResult::warning(CheckId::new("C"), "warning"),
                CheckResult::skipped(CheckId::new("D"), "skipped"),
            ],
        );

        assert_eq!(report.result_count(), 4);
        assert_eq!(report.passed_count(), 1);
        assert_eq!(report.failed_count(), 1);
        assert_eq!(report.warning_count(), 1);
        assert_eq!(report.skipped_count(), 1);
        assert!(report.has_failures());
    }

    #[test]
    fn report_renders_human_readable_summary() {
        let registry = initial_workspace_check_registry();
        let report = CheckRunReport::new(
            registry,
            vec![CheckResult::passed(
                CheckId::new("MONAD-CHECK-0001"),
                "required file exists: monad.toml",
            )],
        );

        let rendered = render_check_run_report(&report);

        assert!(rendered.contains("Monad check report"));
        assert!(rendered.contains("Checks run: 1"));
        assert!(rendered.contains("[PASSED] MONAD-CHECK-0001"));
        assert!(rendered.contains("Result: passed"));
    }

    #[test]
    fn required_file_check_fails_when_file_is_missing() {
        let root = unique_temp_root("missing-required-file");

        let result = check_required_file(&root, "monad.toml", "MONAD-CHECK-0001");

        assert_eq!(result.status(), CheckStatus::Failed);
        assert!(result.message().contains("missing"));
    }

    #[test]
    fn workspace_checks_report_missing_files() -> crate::MonadResult<()> {
        let root = unique_temp_root("workspace-checks-missing");
        fs::create_dir_all(&root).map_err(|error| {
            crate::MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;

        let report = run_monad_workspace_checks(&context);

        assert!(report.failed_count() >= 2);

        fs::remove_dir_all(&root).ok();

        Ok(())
    }
}
