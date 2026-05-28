//! Adapter-specific verification checks.
//!
//! Adapter checks connect repository intelligence to check selection. They
//! should remain conservative: select checks only when tooling is clearly
//! detected, skip unsupported checks explicitly, and avoid destructive commands.

pub mod javascript;
pub mod rust;

use crate::{
    CheckDefinition, CheckResult, CommandResult, RepositoryToolchainDetection, WorkspaceContext,
};

/// Adapter-specific check output.
///
/// This groups newly selected check definitions, check results, and command
/// results so the main check runner can merge adapter checks into the normal
/// report.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AdapterCheckRun {
    definitions: Vec<CheckDefinition>,
    results: Vec<CheckResult>,
    command_results: Vec<CommandResult>,
}

impl AdapterCheckRun {
    /// Creates an empty adapter check run.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an adapter check run from its parts.
    #[must_use]
    pub fn from_parts(
        definitions: Vec<CheckDefinition>,
        results: Vec<CheckResult>,
        command_results: Vec<CommandResult>,
    ) -> Self {
        Self {
            definitions,
            results,
            command_results,
        }
    }

    /// Appends another adapter check run.
    pub fn append(&mut self, mut other: Self) {
        self.definitions.append(&mut other.definitions);
        self.results.append(&mut other.results);
        self.command_results.append(&mut other.command_results);
    }

    /// Returns selected check definitions.
    #[must_use]
    pub fn definitions(&self) -> &[CheckDefinition] {
        &self.definitions
    }

    /// Returns adapter check results.
    #[must_use]
    pub fn results(&self) -> &[CheckResult] {
        &self.results
    }

    /// Returns adapter command results.
    #[must_use]
    pub fn command_results(&self) -> &[CommandResult] {
        &self.command_results
    }

    /// Consumes this run into owned parts.
    #[must_use]
    pub fn into_parts(self) -> (Vec<CheckDefinition>, Vec<CheckResult>, Vec<CommandResult>) {
        (self.definitions, self.results, self.command_results)
    }
}

/// Selects adapter checks from detected repository tooling.
#[must_use]
pub fn select_adapter_checks(
    context: &WorkspaceContext,
    detection: &RepositoryToolchainDetection,
) -> AdapterCheckRun {
    let mut run = AdapterCheckRun::new();

    run.append(rust::select_rust_checks(context, detection));
    run.append(javascript::select_javascript_checks(context, detection));

    run
}

#[cfg(test)]
mod tests {
    use crate::{
        RepositoryToolchainDetection, RepositoryToolchainKind, RepositoryToolchainSignal,
        RepositoryToolchainSignalKind, WorkspaceContext,
    };

    use super::*;

    #[test]
    fn selected_adapter_checks_include_rust_when_rust_detected() {
        let detection =
            RepositoryToolchainDetection::from_signals(vec![RepositoryToolchainSignal::new(
                RepositoryToolchainKind::Rust,
                RepositoryToolchainSignalKind::Manifest,
                "Cargo.toml",
            )]);

        let context = WorkspaceContext::new(".").expect("workspace context should be created");
        let run = select_adapter_checks(&context, &detection);

        assert!(
            run.definitions()
                .iter()
                .any(|definition| { definition.id().as_str() == "MONAD-CHECK-RUST-0001" })
        );
    }

    #[test]
    fn selected_adapter_checks_include_javascript_when_javascript_detected() {
        let detection =
            RepositoryToolchainDetection::from_signals(vec![RepositoryToolchainSignal::new(
                RepositoryToolchainKind::JavaScript,
                RepositoryToolchainSignalKind::Manifest,
                "package.json",
            )]);

        let context = WorkspaceContext::new(".").expect("workspace context should be created");
        let run = select_adapter_checks(&context, &detection);

        assert!(
            run.definitions()
                .iter()
                .any(|definition| { definition.id().as_str() == "MONAD-CHECK-JS-0001" })
        );
    }
}
