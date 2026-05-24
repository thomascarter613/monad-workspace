//! Repository inspection primitives for Monad.
//!
//! E1 gave Monad a working runtime foundation.
//! E2 begins repository intelligence: the ability to inspect a workspace and
//! describe what exists there in typed, reusable Rust structures.
//!
//! This first version intentionally performs a shallow top-level inspection.
//! It does not recursively walk the whole repository yet. That is deliberate:
//! recursive inspection needs ignore rules, performance safeguards, and later
//! graph-oriented design.

use std::fs::{self, DirEntry, FileType};
use std::path::{Path, PathBuf};

use crate::{MonadError, MonadResult, WorkspaceContext};

/// Top-level directories that should not be deeply traversed by future
/// repository intelligence work unless explicitly requested.
///
/// This is a safeguard against accidentally walking huge dependency caches,
/// build outputs, or VCS internals.
const GENERATED_OR_EXTERNAL_TOP_LEVEL_DIRS: &[&str] = &[
    ".git",
    ".next",
    "build",
    "coverage",
    "dist",
    "node_modules",
    "target",
];

/// The broad filesystem kind of a repository entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryEntryKind {
    /// A regular file.
    File,

    /// A directory.
    Directory,

    /// A symbolic link.
    Symlink,

    /// Any other filesystem entry type.
    Other,
}

impl RepositoryEntryKind {
    /// Converts standard-library file metadata into Monad's domain type.
    #[must_use]
    pub fn from_file_type(file_type: FileType) -> Self {
        if file_type.is_file() {
            Self::File
        } else if file_type.is_dir() {
            Self::Directory
        } else if file_type.is_symlink() {
            Self::Symlink
        } else {
            Self::Other
        }
    }

    /// Returns a stable label for human-readable and future machine output.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Directory => "directory",
            Self::Symlink => "symlink",
            Self::Other => "other",
        }
    }
}

/// Monad's first-pass interpretation of a repository entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryEntryRole {
    /// Root `monad.toml`.
    MonadManifest,

    /// Root `Cargo.toml`.
    RustWorkspaceManifest,

    /// Root README file.
    Readme,

    /// `docs/`.
    DocumentationRoot,

    /// `work/`.
    WorkRoot,

    /// `.monad/`.
    MonadStateRoot,

    /// `crates/`, `src/`, `apps/`, `packages/`, or `services/`.
    SourceRoot,

    /// Build output, dependency cache, VCS internals, or similar.
    GeneratedOrExternal,

    /// Other hidden path.
    Hidden,

    /// No special role detected yet.
    Other,
}

impl RepositoryEntryRole {
    /// Returns a stable role label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MonadManifest => "monad_manifest",
            Self::RustWorkspaceManifest => "rust_workspace_manifest",
            Self::Readme => "readme",
            Self::DocumentationRoot => "documentation_root",
            Self::WorkRoot => "work_root",
            Self::MonadStateRoot => "monad_state_root",
            Self::SourceRoot => "source_root",
            Self::GeneratedOrExternal => "generated_or_external",
            Self::Hidden => "hidden",
            Self::Other => "other",
        }
    }
}

/// Whether future deeper inspection should descend into an entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryEntryTraversalPolicy {
    /// This entry is safe for a future recursive inspector to enter.
    SafeForFutureTraversal,

    /// This entry should be described but not deeply inspected by default.
    InspectShallowOnly,

    /// This entry should be skipped by default during deep traversal.
    SkipGeneratedOrExternal,
}

impl RepositoryEntryTraversalPolicy {
    /// Returns a stable policy label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeForFutureTraversal => "safe_for_future_traversal",
            Self::InspectShallowOnly => "inspect_shallow_only",
            Self::SkipGeneratedOrExternal => "skip_generated_or_external",
        }
    }
}

/// One top-level repository entry discovered during inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryEntry {
    name: String,
    relative_path: PathBuf,
    kind: RepositoryEntryKind,
    role: RepositoryEntryRole,
    traversal_policy: RepositoryEntryTraversalPolicy,
}

impl RepositoryEntry {
    /// Builds a repository entry from a filesystem directory entry.
    fn from_dir_entry(root: &Path, entry: DirEntry) -> MonadResult<Self> {
        let path = entry.path();

        let file_type = entry.file_type().map_err(|error| {
            MonadError::internal(format!(
                "failed to inspect filesystem entry {}: {error}",
                path.display()
            ))
        })?;

        let kind = RepositoryEntryKind::from_file_type(file_type);

        let name = entry.file_name().to_string_lossy().to_string();

        let relative_path = path
            .strip_prefix(root)
            .unwrap_or(path.as_path())
            .to_path_buf();

        let role = classify_entry(&name, kind);
        let traversal_policy = traversal_policy_for(&name, kind);

        Ok(Self {
            name,
            relative_path,
            kind,
            role,
            traversal_policy,
        })
    }

    /// Returns the entry's file or directory name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the path relative to the workspace root.
    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// Returns the filesystem kind.
    #[must_use]
    pub const fn kind(&self) -> RepositoryEntryKind {
        self.kind
    }

    /// Returns Monad's first-pass role classification.
    #[must_use]
    pub const fn role(&self) -> RepositoryEntryRole {
        self.role
    }

    /// Returns the default future traversal policy.
    #[must_use]
    pub const fn traversal_policy(&self) -> RepositoryEntryTraversalPolicy {
        self.traversal_policy
    }
}

/// A shallow inspection result for a Monad workspace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryInspection {
    root: PathBuf,
    entries: Vec<RepositoryEntry>,
}

impl RepositoryInspection {
    /// Creates an inspection result.
    #[must_use]
    pub fn new(root: impl Into<PathBuf>, entries: Vec<RepositoryEntry>) -> Self {
        Self {
            root: root.into(),
            entries,
        }
    }

    /// Returns the inspected workspace root.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns all inspected entries.
    #[must_use]
    pub fn entries(&self) -> &[RepositoryEntry] {
        &self.entries
    }

    /// Returns the number of inspected entries.
    #[must_use]
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Returns the number of top-level files.
    #[must_use]
    pub fn file_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.kind() == RepositoryEntryKind::File)
            .count()
    }

    /// Returns the number of top-level directories.
    #[must_use]
    pub fn directory_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.kind() == RepositoryEntryKind::Directory)
            .count()
    }

    /// Returns true when an inspected relative path exists.
    #[must_use]
    pub fn has_relative_path(&self, relative_path: impl AsRef<Path>) -> bool {
        let relative_path = relative_path.as_ref();

        self.entries
            .iter()
            .any(|entry| entry.relative_path() == relative_path)
    }

    /// Returns entries with a specific role.
    #[must_use]
    pub fn entries_with_role(&self, role: RepositoryEntryRole) -> Vec<&RepositoryEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.role() == role)
            .collect()
    }
}

/// Performs a shallow inspection of the workspace root.
///
/// This is intentionally non-recursive. The output is deterministic because
/// entries are sorted by relative path.
pub fn inspect_workspace(context: &WorkspaceContext) -> MonadResult<RepositoryInspection> {
    let mut entries = Vec::new();

    let directory_entries = fs::read_dir(context.root()).map_err(|error| {
        MonadError::internal(format!(
            "failed to read workspace root {}: {error}",
            context.root().display()
        ))
    })?;

    for entry_result in directory_entries {
        let entry = entry_result.map_err(|error| {
            MonadError::internal(format!(
                "failed to read an entry from workspace root {}: {error}",
                context.root().display()
            ))
        })?;

        entries.push(RepositoryEntry::from_dir_entry(context.root(), entry)?);
    }

    entries.sort_by(|left, right| left.relative_path().cmp(right.relative_path()));

    Ok(RepositoryInspection::new(
        context.root().to_path_buf(),
        entries,
    ))
}

/// Classifies an entry into a first-pass repository role.
fn classify_entry(name: &str, kind: RepositoryEntryKind) -> RepositoryEntryRole {
    match (name, kind) {
        ("monad.toml", RepositoryEntryKind::File) => RepositoryEntryRole::MonadManifest,
        ("Cargo.toml", RepositoryEntryKind::File) => RepositoryEntryRole::RustWorkspaceManifest,
        ("README.md", RepositoryEntryKind::File) => RepositoryEntryRole::Readme,
        ("docs", RepositoryEntryKind::Directory) => RepositoryEntryRole::DocumentationRoot,
        ("work", RepositoryEntryKind::Directory) => RepositoryEntryRole::WorkRoot,
        (".monad", RepositoryEntryKind::Directory) => RepositoryEntryRole::MonadStateRoot,
        ("apps" | "crates" | "packages" | "services" | "src", RepositoryEntryKind::Directory) => {
            RepositoryEntryRole::SourceRoot
        }
        (name, RepositoryEntryKind::Directory)
            if GENERATED_OR_EXTERNAL_TOP_LEVEL_DIRS.contains(&name) =>
        {
            RepositoryEntryRole::GeneratedOrExternal
        }
        (name, _) if name.starts_with('.') => RepositoryEntryRole::Hidden,
        _ => RepositoryEntryRole::Other,
    }
}

/// Determines the default traversal policy for an entry.
fn traversal_policy_for(name: &str, kind: RepositoryEntryKind) -> RepositoryEntryTraversalPolicy {
    if kind != RepositoryEntryKind::Directory {
        return RepositoryEntryTraversalPolicy::InspectShallowOnly;
    }

    if GENERATED_OR_EXTERNAL_TOP_LEVEL_DIRS.contains(&name) {
        return RepositoryEntryTraversalPolicy::SkipGeneratedOrExternal;
    }

    if name.starts_with('.') && name != ".monad" {
        return RepositoryEntryTraversalPolicy::InspectShallowOnly;
    }

    RepositoryEntryTraversalPolicy::SafeForFutureTraversal
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
            "monad-repository-inspection-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_inspection_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);

        fs::create_dir_all(root.join("docs")).expect("docs directory should be created");
        fs::create_dir_all(root.join("work")).expect("work directory should be created");
        fs::create_dir_all(root.join(".monad")).expect(".monad directory should be created");
        fs::create_dir_all(root.join("crates")).expect("crates directory should be created");
        fs::create_dir_all(root.join("target")).expect("target directory should be created");
        fs::create_dir_all(root.join(".git")).expect(".git directory should be created");

        fs::write(root.join("README.md"), "# Test\n").expect("README should be written");
        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");
        fs::write(root.join("monad.toml"), "schema_version = 1\n")
            .expect("monad.toml should be written");

        root
    }

    #[test]
    fn repository_inspection_lists_top_level_entries() {
        let root = create_inspection_workspace("lists");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");

        assert_eq!(inspection.root(), root.as_path());
        assert!(inspection.entry_count() >= 8);
        assert!(inspection.has_relative_path("monad.toml"));
        assert!(inspection.has_relative_path("Cargo.toml"));
        assert!(inspection.has_relative_path("docs"));
        assert!(inspection.has_relative_path("work"));
        assert!(inspection.has_relative_path(".monad"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn repository_inspection_counts_files_and_directories() {
        let root = create_inspection_workspace("counts");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");

        assert!(inspection.file_count() >= 3);
        assert!(inspection.directory_count() >= 5);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn repository_inspection_classifies_known_roles() {
        let root = create_inspection_workspace("roles");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");

        assert_eq!(
            inspection
                .entries_with_role(RepositoryEntryRole::MonadManifest)
                .len(),
            1
        );
        assert_eq!(
            inspection
                .entries_with_role(RepositoryEntryRole::RustWorkspaceManifest)
                .len(),
            1
        );
        assert_eq!(
            inspection
                .entries_with_role(RepositoryEntryRole::DocumentationRoot)
                .len(),
            1
        );
        assert_eq!(
            inspection
                .entries_with_role(RepositoryEntryRole::WorkRoot)
                .len(),
            1
        );
        assert_eq!(
            inspection
                .entries_with_role(RepositoryEntryRole::MonadStateRoot)
                .len(),
            1
        );
        assert_eq!(
            inspection
                .entries_with_role(RepositoryEntryRole::SourceRoot)
                .len(),
            1
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn generated_or_external_directories_are_marked_for_skip() {
        let root = create_inspection_workspace("generated");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");

        let target = inspection
            .entries()
            .iter()
            .find(|entry| entry.name() == "target")
            .expect("target should be inspected");

        assert_eq!(target.role(), RepositoryEntryRole::GeneratedOrExternal);
        assert_eq!(
            target.traversal_policy(),
            RepositoryEntryTraversalPolicy::SkipGeneratedOrExternal
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn inspection_entries_are_sorted_by_relative_path() {
        let root = create_inspection_workspace("sorted");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");

        let paths: Vec<String> = inspection
            .entries()
            .iter()
            .map(|entry| entry.relative_path().display().to_string())
            .collect();

        let mut sorted = paths.clone();
        sorted.sort();

        assert_eq!(paths, sorted);

        fs::remove_dir_all(root).ok();
    }
}
