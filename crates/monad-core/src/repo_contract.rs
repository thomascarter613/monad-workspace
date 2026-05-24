//! Repository contract checks for Monad.
//!
//! A repository contract is Monad's machine-checkable expectation for what a
//! valid Monad workspace should contain.
//!
//! This first version is intentionally small. It checks the canonical paths
//! that E0 and early E1 established before later slices add richer policy,
//! manifest-driven contracts, generated-artifact checks, and architecture
//! boundary checks.

use std::path::{Path, PathBuf};

use crate::{Diagnostic, DiagnosticReport, WorkspaceContext};

/// The expected kind of a required repository path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiredPathKind {
    /// The path must be a regular file.
    File,

    /// The path must be a directory.
    Directory,
}

impl RequiredPathKind {
    /// Returns a stable human-readable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Directory => "directory",
        }
    }
}

/// One required path in a repository contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredPath {
    relative_path: PathBuf,
    kind: RequiredPathKind,
    reason: &'static str,
}

impl RequiredPath {
    /// Creates a required file path.
    #[must_use]
    pub fn file(relative_path: impl Into<PathBuf>, reason: &'static str) -> Self {
        Self {
            relative_path: relative_path.into(),
            kind: RequiredPathKind::File,
            reason,
        }
    }

    /// Creates a required directory path.
    #[must_use]
    pub fn directory(relative_path: impl Into<PathBuf>, reason: &'static str) -> Self {
        Self {
            relative_path: relative_path.into(),
            kind: RequiredPathKind::Directory,
            reason,
        }
    }

    /// Returns the required relative path.
    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// Returns the required path kind.
    #[must_use]
    pub const fn kind(&self) -> RequiredPathKind {
        self.kind
    }

    /// Returns why this path is required.
    #[must_use]
    pub const fn reason(&self) -> &'static str {
        self.reason
    }

    /// Resolves this required path against a workspace root.
    #[must_use]
    pub fn absolute_path(&self, context: &WorkspaceContext) -> PathBuf {
        context.root().join(&self.relative_path)
    }

    /// Checks this one required path and returns a diagnostic.
    #[must_use]
    pub fn check(&self, context: &WorkspaceContext) -> Diagnostic {
        let absolute_path = self.absolute_path(context);
        let exists = match self.kind {
            RequiredPathKind::File => absolute_path.is_file(),
            RequiredPathKind::Directory => absolute_path.is_dir(),
        };

        if exists {
            Diagnostic::info(
                "MONAD4500",
                format!(
                    "repository contract path satisfied: {} ({})",
                    self.relative_path.display(),
                    self.kind.as_str()
                ),
            )
        } else {
            let code = match self.kind {
                RequiredPathKind::File => "MONAD4501",
                RequiredPathKind::Directory => "MONAD4502",
            };

            Diagnostic::error(
                code,
                format!(
                    "repository contract path missing: {} ({}) — {}",
                    self.relative_path.display(),
                    self.kind.as_str(),
                    self.reason
                ),
            )
        }
    }
}

/// A set of required paths for a workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryContract {
    required_paths: Vec<RequiredPath>,
}

impl RepositoryContract {
    /// Creates a repository contract from required paths.
    #[must_use]
    pub fn new(required_paths: Vec<RequiredPath>) -> Self {
        Self { required_paths }
    }

    /// Creates Monad's initial repository contract.
    ///
    /// These paths reflect the repository contract established by E0 and early
    /// E1. Later slices can make this manifest-driven and extensible.
    #[must_use]
    pub fn initial_monad() -> Self {
        Self::new(vec![
            RequiredPath::file("monad.toml", "Monad requires a root intent manifest."),
            RequiredPath::file(
                "Cargo.toml",
                "Monad currently requires a Rust workspace manifest.",
            ),
            RequiredPath::directory("docs", "Monad requires repo-native documentation."),
            RequiredPath::directory("work", "Monad requires repo-native work records."),
            RequiredPath::directory(".monad", "Monad requires local operational context state."),
            RequiredPath::directory("crates/monad-cli", "Monad requires the thin CLI crate."),
            RequiredPath::directory(
                "crates/monad-core",
                "Monad requires the durable core runtime crate.",
            ),
        ])
    }

    /// Returns required paths as a read-only slice.
    #[must_use]
    pub fn required_paths(&self) -> &[RequiredPath] {
        &self.required_paths
    }

    /// Checks all required paths against a workspace context.
    #[must_use]
    pub fn check(&self, context: &WorkspaceContext) -> DiagnosticReport {
        let mut report = DiagnosticReport::new();

        for required_path in &self.required_paths {
            report.push(required_path.check(context));
        }

        report
    }
}

/// Runs the initial Monad repository contract check.
#[must_use]
pub fn check_repository_contract(context: &WorkspaceContext) -> DiagnosticReport {
    RepositoryContract::initial_monad().check(context)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "monad-repo-contract-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_contract_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);

        fs::create_dir_all(root.join("docs")).expect("docs directory should be created");
        fs::create_dir_all(root.join("work")).expect("work directory should be created");
        fs::create_dir_all(root.join(".monad")).expect(".monad directory should be created");
        fs::create_dir_all(root.join("crates/monad-cli"))
            .expect("monad-cli directory should be created");
        fs::create_dir_all(root.join("crates/monad-core"))
            .expect("monad-core directory should be created");

        fs::write(root.join("monad.toml"), "schema_version = 1\n")
            .expect("monad.toml should be written");
        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");

        root
    }

    #[test]
    fn initial_contract_contains_expected_paths() {
        let contract = RepositoryContract::initial_monad();
        let paths: Vec<String> = contract
            .required_paths()
            .iter()
            .map(|required_path| required_path.relative_path().display().to_string())
            .collect();

        assert!(paths.contains(&"monad.toml".to_string()));
        assert!(paths.contains(&"Cargo.toml".to_string()));
        assert!(paths.contains(&"docs".to_string()));
        assert!(paths.contains(&"work".to_string()));
        assert!(paths.contains(&".monad".to_string()));
        assert!(paths.contains(&"crates/monad-cli".to_string()));
        assert!(paths.contains(&"crates/monad-core".to_string()));
    }

    #[test]
    fn contract_passes_for_valid_workspace_shape() {
        let root = create_contract_workspace("valid");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let report = check_repository_contract(&context);
        let rendered = report.render_lines().join("\n");

        assert!(!report.has_errors());
        assert!(rendered.contains("MONAD4500"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn contract_reports_missing_directory() {
        let root = create_contract_workspace("missing-docs");
        fs::remove_dir_all(root.join("docs")).expect("docs directory should be removed");

        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let report = check_repository_contract(&context);
        let rendered = report.render_lines().join("\n");

        assert!(report.has_errors());
        assert!(rendered.contains("MONAD4502"));
        assert!(rendered.contains("docs"));

        fs::remove_dir_all(root).ok();
    }
}
