//! Workspace context types for Monad.
//!
//! A workspace is the repository or project directory Monad is operating on.
//! Future commands will need to know paths such as:
//!
//! - the workspace root;
//! - `docs/`;
//! - `work/`;
//! - `.monad/`;
//! - `monad.toml`;
//! - `Cargo.toml`.
//!
//! This module gives those commands one shared path model instead of scattering
//! path-building logic across the CLI.

use std::path::{Path, PathBuf};

use crate::{MonadError, MonadResult};

/// Runtime view of the repository Monad is operating on.
///
/// `PathBuf` is Rust's owned, growable filesystem path type.
/// We store the root as a `PathBuf` because a workspace context owns its root
/// path and can derive other paths from it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceContext {
    root: PathBuf,
}

impl WorkspaceContext {
    /// Creates a workspace context from an explicit root path.
    ///
    /// This does not require the path to exist yet. That keeps the type useful
    /// for planning, scaffolding, tests, and future file-operation previews.
    pub fn new(root: impl AsRef<Path>) -> MonadResult<Self> {
        let root = root.as_ref();

        if root.as_os_str().is_empty() {
            return Err(MonadError::invalid_input(
                "workspace root path must not be empty",
            ));
        }

        Ok(Self {
            root: root.to_path_buf(),
        })
    }

    /// Discovers a workspace root by walking upward from a starting path.
    ///
    /// This is intentionally conservative for the first implementation:
    ///
    /// - a directory containing `monad.toml` is a workspace root;
    /// - a directory containing both `Cargo.toml` and `crates/` is a workspace root;
    /// - a directory containing both `.monad/` and `work/` is a workspace root.
    pub fn discover_from(start: impl AsRef<Path>) -> MonadResult<Self> {
        let root = discover_workspace_root(start)?;
        Self::new(root)
    }

    /// Returns the workspace root path.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns the expected Monad manifest path.
    #[must_use]
    pub fn monad_manifest_path(&self) -> PathBuf {
        self.root.join("monad.toml")
    }

    /// Returns the Rust workspace manifest path.
    #[must_use]
    pub fn cargo_manifest_path(&self) -> PathBuf {
        self.root.join("Cargo.toml")
    }

    /// Returns the documentation directory path.
    #[must_use]
    pub fn docs_dir(&self) -> PathBuf {
        self.root.join("docs")
    }

    /// Returns the work-record directory path.
    #[must_use]
    pub fn work_dir(&self) -> PathBuf {
        self.root.join("work")
    }

    /// Returns the Monad operational-state directory path.
    #[must_use]
    pub fn monad_dir(&self) -> PathBuf {
        self.root.join(".monad")
    }

    /// Returns the Monad context directory path.
    #[must_use]
    pub fn context_dir(&self) -> PathBuf {
        self.monad_dir().join("context")
    }
}

/// Discovers the workspace root by walking upward from `start`.
pub fn discover_workspace_root(start: impl AsRef<Path>) -> MonadResult<PathBuf> {
    let start = start.as_ref();

    if start.as_os_str().is_empty() {
        return Err(MonadError::invalid_input(
            "workspace discovery start path must not be empty",
        ));
    }

    let mut current = if start.is_file() {
        start
            .parent()
            .ok_or_else(|| MonadError::invalid_input("start file has no parent directory"))?
            .to_path_buf()
    } else {
        start.to_path_buf()
    };

    loop {
        if is_workspace_root(&current) {
            return Ok(current);
        }

        if !current.pop() {
            break;
        }
    }

    Err(MonadError::not_found(format!(
        "Monad workspace root from {}",
        start.display()
    )))
}

/// Returns true if a directory looks like a Monad workspace root.
#[must_use]
pub fn is_workspace_root(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();

    path.join("monad.toml").is_file()
        || (path.join("Cargo.toml").is_file() && path.join("crates").is_dir())
        || (path.join(".monad").is_dir() && path.join("work").is_dir())
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
            "monad-workspace-context-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    #[test]
    fn workspace_context_builds_standard_paths() {
        let context = WorkspaceContext::new("/example/monad").expect("path should be valid");

        assert_eq!(context.root(), Path::new("/example/monad"));
        assert_eq!(
            context.monad_manifest_path(),
            PathBuf::from("/example/monad/monad.toml")
        );
        assert_eq!(
            context.cargo_manifest_path(),
            PathBuf::from("/example/monad/Cargo.toml")
        );
        assert_eq!(context.docs_dir(), PathBuf::from("/example/monad/docs"));
        assert_eq!(context.work_dir(), PathBuf::from("/example/monad/work"));
        assert_eq!(context.monad_dir(), PathBuf::from("/example/monad/.monad"));
        assert_eq!(
            context.context_dir(),
            PathBuf::from("/example/monad/.monad/context")
        );
    }

    #[test]
    fn workspace_discovery_finds_root_from_nested_directory() {
        let root = unique_temp_dir("nested");
        let nested = root.join("crates/monad-core/src");

        fs::create_dir_all(&nested).expect("test directories should be created");
        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");
        fs::create_dir_all(root.join("crates")).expect("crates directory should be created");

        let discovered = discover_workspace_root(&nested).expect("workspace root should be found");

        assert_eq!(discovered, root);

        fs::remove_dir_all(discovered).ok();
    }

    #[test]
    fn workspace_discovery_reports_not_found() {
        let root = unique_temp_dir("missing");
        let nested = root.join("some/deep/path");

        fs::create_dir_all(&nested).expect("test directories should be created");

        let error = discover_workspace_root(&nested).expect_err("workspace root should be missing");

        assert_eq!(error.code(), "MONAD2002");

        fs::remove_dir_all(root).ok();
    }
}
