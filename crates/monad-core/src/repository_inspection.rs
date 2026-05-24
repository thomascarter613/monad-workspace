//! Repository inspection primitives for Monad.
//!
//! E1 gave Monad a working runtime foundation.
//! E2 begins repository intelligence: the ability to inspect a workspace and
//! describe what exists there in typed, reusable Rust structures.
//!
//! This version still performs a shallow top-level inspection. It does not
//! recursively walk the whole repository yet. That is deliberate: recursive
//! inspection needs ignore rules, performance safeguards, policy controls, and
//! graph-oriented design.
//!
//! WP-E2-005 adds a future recursive traversal plan and guardrails. This gives
//! Monad a safe API shape before any deep filesystem walking is implemented.

use std::fs::{self, DirEntry, FileType};
use std::path::{Path, PathBuf};

use crate::{MonadError, MonadResult, WorkspaceContext};

/// Top-level directories that should not be deeply traversed by future
/// repository intelligence work unless explicitly requested.
///
/// This is a safeguard against accidentally walking huge dependency caches,
/// build outputs, virtual environments, coverage reports, or VCS internals.
const GENERATED_OR_EXTERNAL_TOP_LEVEL_DIRS: &[&str] = &[
    ".cache",
    ".git",
    ".mypy_cache",
    ".next",
    ".pytest_cache",
    ".ruff_cache",
    ".turbo",
    ".venv",
    "build",
    "coverage",
    "dist",
    "node_modules",
    "out",
    "target",
    "tmp",
    "vendor",
    "venv",
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

/// Broad category bucket for a repository entry.
///
/// Roles answer "what is this exact thing?". Categories answer "what broad
/// area of the repository does this thing belong to?".
///
/// Keeping both levels lets `monad inspect` show useful summary metrics without
/// losing precise role information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryEntryCategory {
    /// Monad-specific control files or state roots.
    MonadControl,

    /// Source code roots.
    Source,

    /// Documentation roots or files.
    Documentation,

    /// Work management, planning, packet, task, and deliverable records.
    WorkManagement,

    /// Runtime/toolchain files for Rust.
    RustRuntime,

    /// JavaScript/TypeScript package management files.
    JavaScriptPackageManagement,

    /// General developer tooling configuration.
    Tooling,

    /// General configuration roots.
    Configuration,

    /// Infrastructure/deployment roots or files.
    Infrastructure,

    /// API contracts, schemas, protobuf, OpenAPI, or related contract roots.
    Contracts,

    /// Database, migration, seed, or data roots.
    Data,

    /// Governance, policy, or security roots.
    Governance,

    /// Public/static/assets roots.
    Assets,

    /// Test roots.
    Tests,

    /// CI or repository automation roots.
    ContinuousIntegration,

    /// Development environment roots.
    DevelopmentEnvironment,

    /// AI/agent context configuration.
    AiContext,

    /// Legal or licensing files.
    Legal,

    /// VCS support files.
    VersionControl,

    /// Build output, dependency cache, virtual environment, VCS internals, or similar.
    GeneratedOrExternal,

    /// Hidden path without a more specific category.
    Hidden,

    /// No category detected yet.
    Other,
}

impl RepositoryEntryCategory {
    /// Returns a stable category label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MonadControl => "monad_control",
            Self::Source => "source",
            Self::Documentation => "documentation",
            Self::WorkManagement => "work_management",
            Self::RustRuntime => "rust_runtime",
            Self::JavaScriptPackageManagement => "javascript_package_management",
            Self::Tooling => "tooling",
            Self::Configuration => "configuration",
            Self::Infrastructure => "infrastructure",
            Self::Contracts => "contracts",
            Self::Data => "data",
            Self::Governance => "governance",
            Self::Assets => "assets",
            Self::Tests => "tests",
            Self::ContinuousIntegration => "continuous_integration",
            Self::DevelopmentEnvironment => "development_environment",
            Self::AiContext => "ai_context",
            Self::Legal => "legal",
            Self::VersionControl => "version_control",
            Self::GeneratedOrExternal => "generated_or_external",
            Self::Hidden => "hidden",
            Self::Other => "other",
        }
    }
}

/// Monad's first-pass interpretation of a repository entry.
///
/// These roles are intentionally conservative. They describe what Monad can
/// infer from a top-level file or directory name without reading file contents
/// or recursively traversing the repository.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryEntryRole {
    /// Root `monad.toml`.
    MonadManifest,

    /// Root `Cargo.toml`.
    RustWorkspaceManifest,

    /// Root `Cargo.lock`.
    RustLockfile,

    /// Root Rust toolchain file such as `rust-toolchain.toml`.
    RustToolchain,

    /// Root Rust lint/security config such as `clippy.toml` or `deny.toml`.
    RustQualityConfig,

    /// Root README file.
    Readme,

    /// Root license/copying file.
    License,

    /// Root `.gitignore`.
    GitIgnore,

    /// Root `.editorconfig`.
    EditorConfig,

    /// Root JavaScript/TypeScript package manifest or lockfile.
    JavaScriptPackageConfig,

    /// Root formatting, linting, hook, or general developer-tool config.
    ToolingConfig,

    /// Root Docker or deployment config file.
    InfrastructureConfig,

    /// Root AI-assistant or agent instruction file.
    AiContextConfig,

    /// `docs/`.
    DocumentationRoot,

    /// `work/`.
    WorkRoot,

    /// `.monad/`.
    MonadStateRoot,

    /// `apps/`, `crates/`, `packages/`, `services/`, or `src/`.
    SourceRoot,

    /// `tools/`, `scripts/`, or `bin/`.
    ToolingRoot,

    /// `config/` or `.config/`.
    ConfigurationRoot,

    /// `infra/`, `infrastructure/`, `deploy/`, `docker/`, `k8s/`, or `terraform/`.
    InfrastructureRoot,

    /// `contracts/`, `schemas/`, `proto/`, or `openapi/`.
    ContractRoot,

    /// `db/`, `database/`, `migrations/`, or `seeds/`.
    DataRoot,

    /// `governance/`, `policies/`, or `security/`.
    GovernanceRoot,

    /// `assets/`, `public/`, or `static/`.
    AssetRoot,

    /// `test/`, `tests/`, or `e2e/`.
    TestRoot,

    /// `.github/`.
    CiRoot,

    /// `.devcontainer/`.
    DevEnvironmentRoot,

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
            Self::RustLockfile => "rust_lockfile",
            Self::RustToolchain => "rust_toolchain",
            Self::RustQualityConfig => "rust_quality_config",
            Self::Readme => "readme",
            Self::License => "license",
            Self::GitIgnore => "gitignore",
            Self::EditorConfig => "editorconfig",
            Self::JavaScriptPackageConfig => "javascript_package_config",
            Self::ToolingConfig => "tooling_config",
            Self::InfrastructureConfig => "infrastructure_config",
            Self::AiContextConfig => "ai_context_config",
            Self::DocumentationRoot => "documentation_root",
            Self::WorkRoot => "work_root",
            Self::MonadStateRoot => "monad_state_root",
            Self::SourceRoot => "source_root",
            Self::ToolingRoot => "tooling_root",
            Self::ConfigurationRoot => "configuration_root",
            Self::InfrastructureRoot => "infrastructure_root",
            Self::ContractRoot => "contract_root",
            Self::DataRoot => "data_root",
            Self::GovernanceRoot => "governance_root",
            Self::AssetRoot => "asset_root",
            Self::TestRoot => "test_root",
            Self::CiRoot => "ci_root",
            Self::DevEnvironmentRoot => "dev_environment_root",
            Self::GeneratedOrExternal => "generated_or_external",
            Self::Hidden => "hidden",
            Self::Other => "other",
        }
    }

    /// Returns the broad category associated with this role.
    #[must_use]
    pub const fn category(self) -> RepositoryEntryCategory {
        match self {
            Self::MonadManifest | Self::MonadStateRoot => RepositoryEntryCategory::MonadControl,
            Self::SourceRoot => RepositoryEntryCategory::Source,
            Self::Readme | Self::DocumentationRoot => RepositoryEntryCategory::Documentation,
            Self::WorkRoot => RepositoryEntryCategory::WorkManagement,
            Self::RustWorkspaceManifest
            | Self::RustLockfile
            | Self::RustToolchain
            | Self::RustQualityConfig => RepositoryEntryCategory::RustRuntime,
            Self::JavaScriptPackageConfig => RepositoryEntryCategory::JavaScriptPackageManagement,
            Self::ToolingConfig | Self::ToolingRoot => RepositoryEntryCategory::Tooling,
            Self::EditorConfig | Self::ConfigurationRoot => RepositoryEntryCategory::Configuration,
            Self::InfrastructureConfig | Self::InfrastructureRoot => {
                RepositoryEntryCategory::Infrastructure
            }
            Self::ContractRoot => RepositoryEntryCategory::Contracts,
            Self::DataRoot => RepositoryEntryCategory::Data,
            Self::GovernanceRoot => RepositoryEntryCategory::Governance,
            Self::AssetRoot => RepositoryEntryCategory::Assets,
            Self::TestRoot => RepositoryEntryCategory::Tests,
            Self::CiRoot => RepositoryEntryCategory::ContinuousIntegration,
            Self::DevEnvironmentRoot => RepositoryEntryCategory::DevelopmentEnvironment,
            Self::AiContextConfig => RepositoryEntryCategory::AiContext,
            Self::License => RepositoryEntryCategory::Legal,
            Self::GitIgnore => RepositoryEntryCategory::VersionControl,
            Self::GeneratedOrExternal => RepositoryEntryCategory::GeneratedOrExternal,
            Self::Hidden => RepositoryEntryCategory::Hidden,
            Self::Other => RepositoryEntryCategory::Other,
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

/// Future traversal mode.
///
/// This is a planning type. It does not mean recursive traversal has been
/// implemented yet. It tells future code what kind of traversal the current
/// plan is preparing for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryTraversalMode {
    /// Current behavior: inspect only top-level repository entries.
    TopLevelOnly,

    /// Future behavior: bounded recursive traversal with conservative defaults.
    FutureRecursiveLimited,
}

impl RepositoryTraversalMode {
    /// Returns a stable traversal mode label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopLevelOnly => "top_level_only",
            Self::FutureRecursiveLimited => "future_recursive_limited",
        }
    }
}

/// Planned decision for a repository entry during future recursive traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryTraversalDecision {
    /// This entry is a candidate for bounded future traversal.
    CandidateForFutureTraversal,

    /// This entry should be recorded but not recursively traversed by default.
    InspectShallowOnly,

    /// This entry should be skipped by default during recursive traversal.
    SkipByDefault,
}

impl RepositoryTraversalDecision {
    /// Returns a stable traversal decision label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CandidateForFutureTraversal => "candidate_for_future_traversal",
            Self::InspectShallowOnly => "inspect_shallow_only",
            Self::SkipByDefault => "skip_by_default",
        }
    }
}

/// Conservative guardrails for future recursive traversal.
///
/// These are intentionally strict. Monad should never accidentally walk huge
/// folders, follow symlink loops, ignore ignore-file intent, or produce
/// nondeterministic output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepositoryTraversalGuardrails {
    max_depth: usize,
    follow_symlinks: bool,
    include_generated_or_external: bool,
    respect_ignore_files: bool,
    deterministic_ordering: bool,
}

impl RepositoryTraversalGuardrails {
    /// Returns the conservative default guardrails for a future bounded
    /// recursive traversal implementation.
    #[must_use]
    pub const fn conservative_future_recursive() -> Self {
        Self {
            max_depth: 3,
            follow_symlinks: false,
            include_generated_or_external: false,
            respect_ignore_files: true,
            deterministic_ordering: true,
        }
    }

    /// Maximum recursive depth future traversal should use by default.
    #[must_use]
    pub const fn max_depth(self) -> usize {
        self.max_depth
    }

    /// Whether future traversal should follow symlinks.
    #[must_use]
    pub const fn follow_symlinks(self) -> bool {
        self.follow_symlinks
    }

    /// Whether future traversal should include generated/external directories.
    #[must_use]
    pub const fn include_generated_or_external(self) -> bool {
        self.include_generated_or_external
    }

    /// Whether future traversal should respect ignore files.
    #[must_use]
    pub const fn respect_ignore_files(self) -> bool {
        self.respect_ignore_files
    }

    /// Whether future traversal output must remain deterministic.
    #[must_use]
    pub const fn deterministic_ordering(self) -> bool {
        self.deterministic_ordering
    }
}

impl Default for RepositoryTraversalGuardrails {
    fn default() -> Self {
        Self::conservative_future_recursive()
    }
}

/// One planned traversal entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryTraversalPlanEntry {
    relative_path: PathBuf,
    decision: RepositoryTraversalDecision,
    traversal_policy: RepositoryEntryTraversalPolicy,
    reason: &'static str,
}

impl RepositoryTraversalPlanEntry {
    /// Builds a traversal plan entry from an inspected repository entry.
    fn from_repository_entry(entry: &RepositoryEntry) -> Self {
        Self {
            relative_path: entry.relative_path().to_path_buf(),
            decision: traversal_decision_for_entry(entry),
            traversal_policy: entry.traversal_policy(),
            reason: traversal_reason_for_entry(entry),
        }
    }

    /// Returns the path relative to the workspace root.
    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// Returns the planned traversal decision.
    #[must_use]
    pub const fn decision(&self) -> RepositoryTraversalDecision {
        self.decision
    }

    /// Returns the original traversal policy that informed the decision.
    #[must_use]
    pub const fn traversal_policy(&self) -> RepositoryEntryTraversalPolicy {
        self.traversal_policy
    }

    /// Returns a human-readable reason for the decision.
    #[must_use]
    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

/// A plan for future recursive traversal.
///
/// This is derived from the shallow inspection result. It does not perform any
/// deep traversal. It simply records what future traversal should consider,
/// inspect shallowly, or skip by default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryTraversalPlan {
    root: PathBuf,
    mode: RepositoryTraversalMode,
    guardrails: RepositoryTraversalGuardrails,
    entries: Vec<RepositoryTraversalPlanEntry>,
}

impl RepositoryTraversalPlan {
    /// Creates a traversal plan.
    #[must_use]
    pub fn new(
        root: impl Into<PathBuf>,
        mode: RepositoryTraversalMode,
        guardrails: RepositoryTraversalGuardrails,
        entries: Vec<RepositoryTraversalPlanEntry>,
    ) -> Self {
        Self {
            root: root.into(),
            mode,
            guardrails,
            entries,
        }
    }

    /// Returns the planned traversal root.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns the traversal mode.
    #[must_use]
    pub const fn mode(&self) -> RepositoryTraversalMode {
        self.mode
    }

    /// Returns traversal guardrails.
    #[must_use]
    pub const fn guardrails(&self) -> RepositoryTraversalGuardrails {
        self.guardrails
    }

    /// Returns planned traversal entries.
    #[must_use]
    pub fn entries(&self) -> &[RepositoryTraversalPlanEntry] {
        &self.entries
    }

    /// Counts entries with a specific traversal decision.
    #[must_use]
    pub fn decision_count(&self, decision: RepositoryTraversalDecision) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.decision() == decision)
            .count()
    }

    /// Counts candidate entries for future bounded traversal.
    #[must_use]
    pub fn candidate_for_future_traversal_count(&self) -> usize {
        self.decision_count(RepositoryTraversalDecision::CandidateForFutureTraversal)
    }

    /// Counts entries that should remain shallow by default.
    #[must_use]
    pub fn inspect_shallow_only_count(&self) -> usize {
        self.decision_count(RepositoryTraversalDecision::InspectShallowOnly)
    }

    /// Counts entries that should be skipped by default.
    #[must_use]
    pub fn skip_by_default_count(&self) -> usize {
        self.decision_count(RepositoryTraversalDecision::SkipByDefault)
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

    /// Returns the entry's broad category.
    #[must_use]
    pub const fn category(&self) -> RepositoryEntryCategory {
        self.role.category()
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

    /// Returns entries with a specific broad category.
    #[must_use]
    pub fn entries_with_category(
        &self,
        category: RepositoryEntryCategory,
    ) -> Vec<&RepositoryEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.category() == category)
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

/// Builds a traversal plan from a shallow repository inspection.
///
/// This is intentionally not a recursive walk. It produces a future traversal
/// contract from the already-inspected top-level entries.
#[must_use]
pub fn build_traversal_plan(inspection: &RepositoryInspection) -> RepositoryTraversalPlan {
    let entries = inspection
        .entries()
        .iter()
        .map(RepositoryTraversalPlanEntry::from_repository_entry)
        .collect();

    RepositoryTraversalPlan::new(
        inspection.root().to_path_buf(),
        RepositoryTraversalMode::FutureRecursiveLimited,
        RepositoryTraversalGuardrails::conservative_future_recursive(),
        entries,
    )
}

/// Classifies an entry into a first-pass repository role.
///
/// This function only uses the entry name and filesystem kind. Later slices can
/// add deeper content-aware inspection without changing the CLI command.
fn classify_entry(name: &str, kind: RepositoryEntryKind) -> RepositoryEntryRole {
    let normalized = name.to_ascii_lowercase();

    match (normalized.as_str(), kind) {
        ("monad.toml", RepositoryEntryKind::File) => RepositoryEntryRole::MonadManifest,
        ("cargo.toml", RepositoryEntryKind::File) => RepositoryEntryRole::RustWorkspaceManifest,
        ("cargo.lock", RepositoryEntryKind::File) => RepositoryEntryRole::RustLockfile,
        ("rust-toolchain" | "rust-toolchain.toml", RepositoryEntryKind::File) => {
            RepositoryEntryRole::RustToolchain
        }
        ("clippy.toml" | "deny.toml", RepositoryEntryKind::File) => {
            RepositoryEntryRole::RustQualityConfig
        }
        ("readme" | "readme.md" | "readme.txt", RepositoryEntryKind::File) => {
            RepositoryEntryRole::Readme
        }
        ("license" | "license.md" | "license.txt" | "copying", RepositoryEntryKind::File) => {
            RepositoryEntryRole::License
        }
        (".gitignore", RepositoryEntryKind::File) => RepositoryEntryRole::GitIgnore,
        (".editorconfig", RepositoryEntryKind::File) => RepositoryEntryRole::EditorConfig,
        (
            "package.json" | "bun.lock" | "bun.lockb" | "pnpm-lock.yaml" | "package-lock.json"
            | "yarn.lock",
            RepositoryEntryKind::File,
        ) => RepositoryEntryRole::JavaScriptPackageConfig,
        (
            "biome.json" | "biome.jsonc" | "lefthook.yml" | "lefthook.yaml" | ".prettierrc"
            | ".prettierrc.json" | ".prettierrc.yml" | ".prettierrc.yaml" | ".eslintrc"
            | ".eslintrc.json",
            RepositoryEntryKind::File,
        ) => RepositoryEntryRole::ToolingConfig,
        (
            "dockerfile"
            | "docker-compose.yml"
            | "docker-compose.yaml"
            | "compose.yml"
            | "compose.yaml",
            RepositoryEntryKind::File,
        ) => RepositoryEntryRole::InfrastructureConfig,
        (
            "agents.md" | "agent.md" | "claude.md" | "cursor.md" | "copilot-instructions.md",
            RepositoryEntryKind::File,
        ) => RepositoryEntryRole::AiContextConfig,
        ("docs", RepositoryEntryKind::Directory) => RepositoryEntryRole::DocumentationRoot,
        ("work", RepositoryEntryKind::Directory) => RepositoryEntryRole::WorkRoot,
        (".monad", RepositoryEntryKind::Directory) => RepositoryEntryRole::MonadStateRoot,
        ("apps" | "crates" | "packages" | "services" | "src", RepositoryEntryKind::Directory) => {
            RepositoryEntryRole::SourceRoot
        }
        ("tools" | "scripts" | "bin", RepositoryEntryKind::Directory) => {
            RepositoryEntryRole::ToolingRoot
        }
        ("config" | ".config", RepositoryEntryKind::Directory) => {
            RepositoryEntryRole::ConfigurationRoot
        }
        (
            "infra" | "infrastructure" | "deploy" | "deployments" | "docker" | "k8s" | "kubernetes"
            | "terraform",
            RepositoryEntryKind::Directory,
        ) => RepositoryEntryRole::InfrastructureRoot,
        (
            "contracts" | "schemas" | "schema" | "proto" | "protobuf" | "openapi",
            RepositoryEntryKind::Directory,
        ) => RepositoryEntryRole::ContractRoot,
        ("db" | "database" | "migrations" | "seeds", RepositoryEntryKind::Directory) => {
            RepositoryEntryRole::DataRoot
        }
        ("governance" | "policies" | "security", RepositoryEntryKind::Directory) => {
            RepositoryEntryRole::GovernanceRoot
        }
        ("assets" | "public" | "static", RepositoryEntryKind::Directory) => {
            RepositoryEntryRole::AssetRoot
        }
        ("test" | "tests" | "e2e", RepositoryEntryKind::Directory) => RepositoryEntryRole::TestRoot,
        (".github", RepositoryEntryKind::Directory) => RepositoryEntryRole::CiRoot,
        (".devcontainer", RepositoryEntryKind::Directory) => {
            RepositoryEntryRole::DevEnvironmentRoot
        }
        (normalized_name, RepositoryEntryKind::Directory)
            if GENERATED_OR_EXTERNAL_TOP_LEVEL_DIRS.contains(&normalized_name) =>
        {
            RepositoryEntryRole::GeneratedOrExternal
        }
        _ if name.starts_with('.') => RepositoryEntryRole::Hidden,
        _ => RepositoryEntryRole::Other,
    }
}

/// Determines the default traversal policy for an entry.
fn traversal_policy_for(name: &str, kind: RepositoryEntryKind) -> RepositoryEntryTraversalPolicy {
    if kind != RepositoryEntryKind::Directory {
        return RepositoryEntryTraversalPolicy::InspectShallowOnly;
    }

    let normalized = name.to_ascii_lowercase();

    if GENERATED_OR_EXTERNAL_TOP_LEVEL_DIRS.contains(&normalized.as_str()) {
        return RepositoryEntryTraversalPolicy::SkipGeneratedOrExternal;
    }

    match normalized.as_str() {
        ".monad" | ".github" | ".devcontainer" | ".config" | "apps" | "assets" | "bin"
        | "config" | "contracts" | "crates" | "database" | "db" | "deploy" | "deployments"
        | "docker" | "docs" | "e2e" | "governance" | "infra" | "infrastructure" | "k8s"
        | "kubernetes" | "migrations" | "openapi" | "packages" | "policies" | "proto"
        | "protobuf" | "public" | "schema" | "schemas" | "scripts" | "security" | "seeds"
        | "services" | "src" | "static" | "terraform" | "test" | "tests" | "tools" | "work" => {
            RepositoryEntryTraversalPolicy::SafeForFutureTraversal
        }
        _ if name.starts_with('.') => RepositoryEntryTraversalPolicy::InspectShallowOnly,
        _ => RepositoryEntryTraversalPolicy::SafeForFutureTraversal,
    }
}

/// Converts an entry traversal policy into a future traversal decision.
fn traversal_decision_for_entry(entry: &RepositoryEntry) -> RepositoryTraversalDecision {
    match entry.traversal_policy() {
        RepositoryEntryTraversalPolicy::SafeForFutureTraversal => {
            RepositoryTraversalDecision::CandidateForFutureTraversal
        }
        RepositoryEntryTraversalPolicy::InspectShallowOnly => {
            RepositoryTraversalDecision::InspectShallowOnly
        }
        RepositoryEntryTraversalPolicy::SkipGeneratedOrExternal => {
            RepositoryTraversalDecision::SkipByDefault
        }
    }
}

/// Explains why a future traversal decision was selected.
fn traversal_reason_for_entry(entry: &RepositoryEntry) -> &'static str {
    match entry.traversal_policy() {
        RepositoryEntryTraversalPolicy::SafeForFutureTraversal => {
            "directory is a safe candidate for future bounded traversal"
        }
        RepositoryEntryTraversalPolicy::InspectShallowOnly => {
            if entry.name().starts_with('.') {
                "hidden path is inspected shallowly unless explicitly supported"
            } else {
                "non-directory or shallow-only path is recorded without recursive traversal"
            }
        }
        RepositoryEntryTraversalPolicy::SkipGeneratedOrExternal => {
            "generated or external path is skipped by default"
        }
    }
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
        fs::create_dir_all(root.join("tools")).expect("tools directory should be created");
        fs::create_dir_all(root.join("infra")).expect("infra directory should be created");
        fs::create_dir_all(root.join("contracts")).expect("contracts directory should be created");
        fs::create_dir_all(root.join("db")).expect("db directory should be created");
        fs::create_dir_all(root.join("governance"))
            .expect("governance directory should be created");
        fs::create_dir_all(root.join("assets")).expect("assets directory should be created");
        fs::create_dir_all(root.join("tests")).expect("tests directory should be created");
        fs::create_dir_all(root.join(".github")).expect(".github directory should be created");
        fs::create_dir_all(root.join(".devcontainer"))
            .expect(".devcontainer directory should be created");
        fs::create_dir_all(root.join("target")).expect("target directory should be created");
        fs::create_dir_all(root.join("node_modules"))
            .expect("node_modules directory should be created");
        fs::create_dir_all(root.join(".git")).expect(".git directory should be created");

        fs::write(root.join("README.md"), "# Test\n").expect("README should be written");
        fs::write(root.join("LICENSE"), "MIT\n").expect("LICENSE should be written");
        fs::write(root.join(".gitignore"), "target/\n").expect(".gitignore should be written");
        fs::write(root.join(".editorconfig"), "root = true\n")
            .expect(".editorconfig should be written");
        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");
        fs::write(root.join("Cargo.lock"), "# lock\n").expect("Cargo.lock should be written");
        fs::write(root.join("monad.toml"), "schema_version = 1\n")
            .expect("monad.toml should be written");
        fs::write(root.join("rust-toolchain.toml"), "[toolchain]\n")
            .expect("rust-toolchain.toml should be written");
        fs::write(root.join("package.json"), "{}\n").expect("package.json should be written");
        fs::write(root.join("biome.json"), "{}\n").expect("biome.json should be written");
        fs::write(root.join("docker-compose.yml"), "services: {}\n")
            .expect("docker-compose.yml should be written");
        fs::write(root.join("AGENTS.md"), "# Agents\n").expect("AGENTS.md should be written");

        root
    }

    #[test]
    fn repository_inspection_lists_top_level_entries() {
        let root = create_inspection_workspace("lists");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");

        assert_eq!(inspection.root(), root.as_path());
        assert!(inspection.entry_count() >= 20);
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

        assert!(inspection.file_count() >= 10);
        assert!(inspection.directory_count() >= 10);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn repository_inspection_classifies_core_monad_roles() {
        let root = create_inspection_workspace("core-roles");
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
    fn repository_inspection_exposes_category_queries() {
        let root = create_inspection_workspace("category-queries");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");

        assert!(
            inspection
                .entries_with_category(RepositoryEntryCategory::MonadControl)
                .len()
                >= 2
        );
        assert_eq!(
            inspection
                .entries_with_category(RepositoryEntryCategory::Source)
                .len(),
            1
        );
        assert_eq!(
            inspection
                .entries_with_category(RepositoryEntryCategory::WorkManagement)
                .len(),
            1
        );
        assert_eq!(
            inspection
                .entries_with_category(RepositoryEntryCategory::ContinuousIntegration)
                .len(),
            1
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn repository_entry_roles_map_to_stable_categories() {
        assert_eq!(
            RepositoryEntryRole::MonadManifest.category(),
            RepositoryEntryCategory::MonadControl
        );
        assert_eq!(
            RepositoryEntryRole::SourceRoot.category(),
            RepositoryEntryCategory::Source
        );
        assert_eq!(
            RepositoryEntryRole::DocumentationRoot.category(),
            RepositoryEntryCategory::Documentation
        );
        assert_eq!(
            RepositoryEntryRole::GeneratedOrExternal.category(),
            RepositoryEntryCategory::GeneratedOrExternal
        );
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

        let node_modules = inspection
            .entries()
            .iter()
            .find(|entry| entry.name() == "node_modules")
            .expect("node_modules should be inspected");

        let git = inspection
            .entries()
            .iter()
            .find(|entry| entry.name() == ".git")
            .expect(".git should be inspected");

        assert_eq!(target.role(), RepositoryEntryRole::GeneratedOrExternal);
        assert_eq!(
            target.category(),
            RepositoryEntryCategory::GeneratedOrExternal
        );
        assert_eq!(
            target.traversal_policy(),
            RepositoryEntryTraversalPolicy::SkipGeneratedOrExternal
        );

        assert_eq!(
            node_modules.role(),
            RepositoryEntryRole::GeneratedOrExternal
        );
        assert_eq!(
            node_modules.category(),
            RepositoryEntryCategory::GeneratedOrExternal
        );
        assert_eq!(
            node_modules.traversal_policy(),
            RepositoryEntryTraversalPolicy::SkipGeneratedOrExternal
        );

        assert_eq!(git.role(), RepositoryEntryRole::GeneratedOrExternal);
        assert_eq!(git.category(), RepositoryEntryCategory::GeneratedOrExternal);
        assert_eq!(
            git.traversal_policy(),
            RepositoryEntryTraversalPolicy::SkipGeneratedOrExternal
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn traversal_guardrails_are_conservative_by_default() {
        let guardrails = RepositoryTraversalGuardrails::conservative_future_recursive();

        assert_eq!(guardrails.max_depth(), 3);
        assert!(!guardrails.follow_symlinks());
        assert!(!guardrails.include_generated_or_external());
        assert!(guardrails.respect_ignore_files());
        assert!(guardrails.deterministic_ordering());
    }

    #[test]
    fn traversal_plan_is_built_from_shallow_inspection() {
        let root = create_inspection_workspace("traversal-plan");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let plan = build_traversal_plan(&inspection);

        assert_eq!(plan.root(), inspection.root());
        assert_eq!(plan.mode(), RepositoryTraversalMode::FutureRecursiveLimited);
        assert_eq!(plan.entries().len(), inspection.entries().len());
        assert!(plan.candidate_for_future_traversal_count() > 0);
        assert!(plan.inspect_shallow_only_count() > 0);
        assert!(plan.skip_by_default_count() > 0);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn traversal_plan_marks_generated_paths_as_skip_by_default() {
        let root = create_inspection_workspace("traversal-skip");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let plan = build_traversal_plan(&inspection);

        let target = plan
            .entries()
            .iter()
            .find(|entry| entry.relative_path() == std::path::Path::new("target"))
            .expect("target should have a traversal plan entry");

        assert_eq!(
            target.decision(),
            RepositoryTraversalDecision::SkipByDefault
        );
        assert_eq!(
            target.traversal_policy(),
            RepositoryEntryTraversalPolicy::SkipGeneratedOrExternal
        );
        assert_eq!(
            target.reason(),
            "generated or external path is skipped by default"
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn traversal_plan_marks_safe_roots_as_future_candidates() {
        let root = create_inspection_workspace("traversal-candidates");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let plan = build_traversal_plan(&inspection);

        for name in ["docs", "work", ".monad", "crates", "tools"] {
            let entry = plan
                .entries()
                .iter()
                .find(|entry| entry.relative_path() == std::path::Path::new(name))
                .expect("known safe root should have a traversal plan entry");

            assert_eq!(
                entry.decision(),
                RepositoryTraversalDecision::CandidateForFutureTraversal,
                "{name} should be a future traversal candidate"
            );
        }

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
