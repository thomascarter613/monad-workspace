//! Workspace check primitives for Monad.
//!
//! This module composes runtime primitives:
//!
//! - diagnostics;
//! - core errors;
//! - workspace context;
//! - manifest loading;
//! - repository contract checks;
//! - repository inspection.

use crate::repository_inspection::inspect_workspace;
use crate::{Diagnostic, DiagnosticReport, MonadManifest, RepositoryContract, WorkspaceContext};

/// Runs the initial workspace checks for a Monad workspace.
///
/// This function deliberately returns a `DiagnosticReport` rather than failing
/// at the first problem. A check command should report as much useful
/// information as it safely can in one pass.
#[must_use]
pub fn run_workspace_checks(context: &WorkspaceContext) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();

    report.push(Diagnostic::info(
        "MONAD4000",
        format!("workspace root detected: {}", context.root().display()),
    ));

    check_manifest_path(context, &mut report);
    check_cargo_manifest_path(context, &mut report);
    check_manifest_loading(context, &mut report);
    check_repository_contract(context, &mut report);
    check_repository_inspection(context, &mut report);

    report
}

/// Checks whether the root `monad.toml` file exists.
fn check_manifest_path(context: &WorkspaceContext, report: &mut DiagnosticReport) {
    let path = context.monad_manifest_path();

    if path.is_file() {
        report.push(Diagnostic::info(
            "MONAD4001",
            format!("Monad manifest found: {}", path.display()),
        ));
    } else {
        report.push(Diagnostic::error(
            "MONAD4401",
            format!("Monad manifest missing: {}", path.display()),
        ));
    }
}

/// Checks whether the root `Cargo.toml` file exists.
///
/// This is currently informational for Monad's own repository because E1 is a
/// Rust runtime foundation. Later Monad target repositories may not all be Rust
/// projects, so this should not become a universal hard failure yet.
fn check_cargo_manifest_path(context: &WorkspaceContext, report: &mut DiagnosticReport) {
    let path = context.cargo_manifest_path();

    if path.is_file() {
        report.push(Diagnostic::info(
            "MONAD4002",
            format!("Rust workspace manifest found: {}", path.display()),
        ));
    } else {
        report.push(Diagnostic::warning(
            "MONAD4102",
            format!("Rust workspace manifest not found: {}", path.display()),
        ));
    }
}

/// Loads and validates `monad.toml`.
fn check_manifest_loading(context: &WorkspaceContext, report: &mut DiagnosticReport) {
    match MonadManifest::load_from_workspace(context) {
        Ok(manifest) => {
            report.push(Diagnostic::info(
                "MONAD4003",
                format!(
                    "Monad manifest loaded: {} ({})",
                    manifest.project.display_name, manifest.project.name
                ),
            ));

            report.push(Diagnostic::info(
                "MONAD4004",
                format!(
                    "runtime: core_crate={}, cli_crate={}, execution_model={}",
                    manifest.runtime.core_crate,
                    manifest.runtime.cli_crate,
                    manifest.runtime.execution_model
                ),
            ));

            for diagnostic in manifest.diagnostics().diagnostics() {
                report.push(diagnostic.clone());
            }
        }
        Err(error) => {
            report.push(error.to_diagnostic());
        }
    }
}

/// Runs the initial repository contract check and appends its diagnostics.
fn check_repository_contract(context: &WorkspaceContext, report: &mut DiagnosticReport) {
    for diagnostic in RepositoryContract::initial_monad()
        .check(context)
        .diagnostics()
    {
        report.push(diagnostic.clone());
    }
}

/// Runs the first shallow repository inspection and appends a summary
/// diagnostic.
fn check_repository_inspection(context: &WorkspaceContext, report: &mut DiagnosticReport) {
    match inspect_workspace(context) {
        Ok(inspection) => {
            report.push(Diagnostic::info(
                "MONAD4600",
                format!(
                    "repository inspection completed: {} top-level entries ({} files, {} directories)",
                    inspection.entry_count(),
                    inspection.file_count(),
                    inspection.directory_count()
                ),
            ));
        }
        Err(error) => {
            report.push(error.to_diagnostic());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

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

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "monad-checks-{test_name}-{}-{unique}",
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
    fn workspace_checks_pass_for_valid_workspace() {
        let root = create_test_workspace("valid");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let report = run_workspace_checks(&context);
        let rendered = report.render_lines().join("\n");

        assert!(!report.has_errors());
        assert!(rendered.contains("MONAD4000"));
        assert!(rendered.contains("MONAD4001"));
        assert!(rendered.contains("MONAD4002"));
        assert!(rendered.contains("MONAD4003"));
        assert!(rendered.contains("MONAD4004"));
        assert!(rendered.contains("MONAD4500"));
        assert!(rendered.contains("MONAD4600"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn workspace_checks_report_missing_manifest() {
        let root = unique_temp_dir("missing-manifest");

        fs::create_dir_all(root.join("docs")).expect("docs directory should be created");
        fs::create_dir_all(root.join("work")).expect("work directory should be created");
        fs::create_dir_all(root.join(".monad")).expect(".monad directory should be created");
        fs::create_dir_all(root.join("crates/monad-cli"))
            .expect("monad-cli directory should be created");
        fs::create_dir_all(root.join("crates/monad-core"))
            .expect("monad-core directory should be created");

        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");

        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let report = run_workspace_checks(&context);
        let rendered = report.render_lines().join("\n");

        assert!(report.has_errors());
        assert!(rendered.contains("MONAD4401"));
        assert!(rendered.contains("MONAD4501"));
        assert!(rendered.contains("MONAD4600"));

        fs::remove_dir_all(root).ok();
    }
}
