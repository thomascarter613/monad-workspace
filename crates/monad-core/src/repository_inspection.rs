//! Repository inspection primitives for Monad.
//!
//! E1 gave Monad a working runtime foundation.
//! E2 begins repository intelligence: the ability to inspect a workspace and
//! describe what exists there in typed, reusable Rust structures.
//!
//! WP-E2-006 implements the first bounded recursive traversal foundation.
//! The traversal remains conservative by default:
//!
//! - bounded depth;
//! - no symlink following;
//! - generated/external paths skipped;
//! - root `.gitignore` exact/simple directory patterns respected;
//! - deterministic output ordering.

use std::fs::{self, DirEntry, FileType};
use std::path::{Path, PathBuf};

use crate::{MonadError, MonadResult, WorkspaceContext};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryEntryKind {
    File,
    Directory,
    Symlink,
    Other,
}

impl RepositoryEntryKind {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryEntryCategory {
    MonadControl,
    Source,
    Documentation,
    WorkManagement,
    RustRuntime,
    JavaScriptPackageManagement,
    Tooling,
    Configuration,
    Infrastructure,
    Contracts,
    Data,
    Governance,
    Assets,
    Tests,
    ContinuousIntegration,
    DevelopmentEnvironment,
    AiContext,
    Legal,
    VersionControl,
    GeneratedOrExternal,
    Hidden,
    Other,
}

impl RepositoryEntryCategory {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryEntryRole {
    MonadManifest,
    RustWorkspaceManifest,
    RustLockfile,
    RustToolchain,
    RustQualityConfig,
    Readme,
    License,
    GitIgnore,
    EditorConfig,
    JavaScriptPackageConfig,
    ToolingConfig,
    InfrastructureConfig,
    AiContextConfig,
    DocumentationRoot,
    WorkRoot,
    MonadStateRoot,
    SourceRoot,
    ToolingRoot,
    ConfigurationRoot,
    InfrastructureRoot,
    ContractRoot,
    DataRoot,
    GovernanceRoot,
    AssetRoot,
    TestRoot,
    CiRoot,
    DevEnvironmentRoot,
    GeneratedOrExternal,
    Hidden,
    Other,
}

impl RepositoryEntryRole {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryEntryTraversalPolicy {
    SafeForFutureTraversal,
    InspectShallowOnly,
    SkipGeneratedOrExternal,
}

impl RepositoryEntryTraversalPolicy {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeForFutureTraversal => "safe_for_future_traversal",
            Self::InspectShallowOnly => "inspect_shallow_only",
            Self::SkipGeneratedOrExternal => "skip_generated_or_external",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryTraversalMode {
    TopLevelOnly,
    FutureRecursiveLimited,
    BoundedRecursive,
}

impl RepositoryTraversalMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopLevelOnly => "top_level_only",
            Self::FutureRecursiveLimited => "future_recursive_limited",
            Self::BoundedRecursive => "bounded_recursive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryTraversalDecision {
    CandidateForFutureTraversal,
    InspectShallowOnly,
    SkipByDefault,
}

impl RepositoryTraversalDecision {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CandidateForFutureTraversal => "candidate_for_future_traversal",
            Self::InspectShallowOnly => "inspect_shallow_only",
            Self::SkipByDefault => "skip_by_default",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepositoryTraversalGuardrails {
    max_depth: usize,
    follow_symlinks: bool,
    include_generated_or_external: bool,
    respect_ignore_files: bool,
    deterministic_ordering: bool,
}

impl RepositoryTraversalGuardrails {
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

    #[must_use]
    pub const fn max_depth(self) -> usize {
        self.max_depth
    }

    #[must_use]
    pub const fn follow_symlinks(self) -> bool {
        self.follow_symlinks
    }

    #[must_use]
    pub const fn include_generated_or_external(self) -> bool {
        self.include_generated_or_external
    }

    #[must_use]
    pub const fn respect_ignore_files(self) -> bool {
        self.respect_ignore_files
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryTraversalPlanEntry {
    relative_path: PathBuf,
    decision: RepositoryTraversalDecision,
    traversal_policy: RepositoryEntryTraversalPolicy,
    reason: &'static str,
}

impl RepositoryTraversalPlanEntry {
    fn from_repository_entry(entry: &RepositoryEntry) -> Self {
        Self {
            relative_path: entry.relative_path().to_path_buf(),
            decision: traversal_decision_for_policy(entry.traversal_policy()),
            traversal_policy: entry.traversal_policy(),
            reason: traversal_reason_for_policy(entry.traversal_policy(), entry.name()),
        }
    }

    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    #[must_use]
    pub const fn decision(&self) -> RepositoryTraversalDecision {
        self.decision
    }

    #[must_use]
    pub const fn traversal_policy(&self) -> RepositoryEntryTraversalPolicy {
        self.traversal_policy
    }

    #[must_use]
    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryTraversalPlan {
    root: PathBuf,
    mode: RepositoryTraversalMode,
    guardrails: RepositoryTraversalGuardrails,
    entries: Vec<RepositoryTraversalPlanEntry>,
}

impl RepositoryTraversalPlan {
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

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[must_use]
    pub const fn mode(&self) -> RepositoryTraversalMode {
        self.mode
    }

    #[must_use]
    pub const fn guardrails(&self) -> RepositoryTraversalGuardrails {
        self.guardrails
    }

    #[must_use]
    pub fn entries(&self) -> &[RepositoryTraversalPlanEntry] {
        &self.entries
    }

    #[must_use]
    pub fn decision_count(&self, decision: RepositoryTraversalDecision) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.decision() == decision)
            .count()
    }

    #[must_use]
    pub fn candidate_for_future_traversal_count(&self) -> usize {
        self.decision_count(RepositoryTraversalDecision::CandidateForFutureTraversal)
    }

    #[must_use]
    pub fn inspect_shallow_only_count(&self) -> usize {
        self.decision_count(RepositoryTraversalDecision::InspectShallowOnly)
    }

    #[must_use]
    pub fn skip_by_default_count(&self) -> usize {
        self.decision_count(RepositoryTraversalDecision::SkipByDefault)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryTraversalEntry {
    relative_path: PathBuf,
    depth: usize,
    kind: RepositoryEntryKind,
    role: RepositoryEntryRole,
    category: RepositoryEntryCategory,
    traversal_policy: RepositoryEntryTraversalPolicy,
    decision: RepositoryTraversalDecision,
    reason: &'static str,
}

impl RepositoryTraversalEntry {
    fn from_repository_entry(
        entry: &RepositoryEntry,
        depth: usize,
        ignore_rules: &RepositoryIgnoreRules,
    ) -> Self {
        let ignored = ignore_rules.is_ignored(entry.relative_path(), entry.kind());
        let decision =
            traversal_decision_for_entry(entry.kind(), entry.traversal_policy(), ignored);
        let reason = traversal_reason_for_entry(
            entry.kind(),
            entry.traversal_policy(),
            ignored,
            entry.name(),
        );

        Self {
            relative_path: entry.relative_path().to_path_buf(),
            depth,
            kind: entry.kind(),
            role: entry.role(),
            category: entry.category(),
            traversal_policy: entry.traversal_policy(),
            decision,
            reason,
        }
    }

    fn from_path(
        root: &Path,
        path: &Path,
        depth: usize,
        ignore_rules: &RepositoryIgnoreRules,
    ) -> MonadResult<Self> {
        let metadata = fs::symlink_metadata(path).map_err(|error| {
            MonadError::internal(format!(
                "failed to inspect filesystem entry {}: {error}",
                path.display()
            ))
        })?;

        let kind = RepositoryEntryKind::from_file_type(metadata.file_type());
        let name = path
            .file_name()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_default();

        let relative_path = path.strip_prefix(root).unwrap_or(path).to_path_buf();

        let role = classify_entry(&name, kind);
        let traversal_policy = traversal_policy_for(&name, kind);
        let ignored = ignore_rules.is_ignored(&relative_path, kind);
        let decision = traversal_decision_for_entry(kind, traversal_policy, ignored);
        let reason = traversal_reason_for_entry(kind, traversal_policy, ignored, &name);

        Ok(Self {
            relative_path,
            depth,
            kind,
            role,
            category: role.category(),
            traversal_policy,
            decision,
            reason,
        })
    }

    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    #[must_use]
    pub const fn kind(&self) -> RepositoryEntryKind {
        self.kind
    }

    #[must_use]
    pub const fn role(&self) -> RepositoryEntryRole {
        self.role
    }

    #[must_use]
    pub const fn category(&self) -> RepositoryEntryCategory {
        self.category
    }

    #[must_use]
    pub const fn traversal_policy(&self) -> RepositoryEntryTraversalPolicy {
        self.traversal_policy
    }

    #[must_use]
    pub const fn decision(&self) -> RepositoryTraversalDecision {
        self.decision
    }

    #[must_use]
    pub const fn reason(&self) -> &'static str {
        self.reason
    }

    fn should_descend(&self, guardrails: RepositoryTraversalGuardrails) -> bool {
        self.kind == RepositoryEntryKind::Directory
            && self.decision == RepositoryTraversalDecision::CandidateForFutureTraversal
            && self.depth < guardrails.max_depth()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryBoundedTraversal {
    root: PathBuf,
    mode: RepositoryTraversalMode,
    guardrails: RepositoryTraversalGuardrails,
    entries: Vec<RepositoryTraversalEntry>,
}

impl RepositoryBoundedTraversal {
    #[must_use]
    pub fn new(
        root: impl Into<PathBuf>,
        mode: RepositoryTraversalMode,
        guardrails: RepositoryTraversalGuardrails,
        entries: Vec<RepositoryTraversalEntry>,
    ) -> Self {
        Self {
            root: root.into(),
            mode,
            guardrails,
            entries,
        }
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[must_use]
    pub const fn mode(&self) -> RepositoryTraversalMode {
        self.mode
    }

    #[must_use]
    pub const fn guardrails(&self) -> RepositoryTraversalGuardrails {
        self.guardrails
    }

    #[must_use]
    pub fn entries(&self) -> &[RepositoryTraversalEntry] {
        &self.entries
    }

    #[must_use]
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn max_observed_depth(&self) -> usize {
        self.entries
            .iter()
            .map(RepositoryTraversalEntry::depth)
            .max()
            .unwrap_or(0)
    }

    #[must_use]
    pub fn contains_relative_path(&self, relative_path: impl AsRef<Path>) -> bool {
        let relative_path = relative_path.as_ref();

        self.entries
            .iter()
            .any(|entry| entry.relative_path() == relative_path)
    }

    #[must_use]
    pub fn decision_count(&self, decision: RepositoryTraversalDecision) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.decision() == decision)
            .count()
    }

    #[must_use]
    pub fn category_count(&self, category: RepositoryEntryCategory) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.category() == category)
            .count()
    }

    #[must_use]
    pub fn candidate_count(&self) -> usize {
        self.decision_count(RepositoryTraversalDecision::CandidateForFutureTraversal)
    }

    #[must_use]
    pub fn shallow_only_count(&self) -> usize {
        self.decision_count(RepositoryTraversalDecision::InspectShallowOnly)
    }

    #[must_use]
    pub fn skip_count(&self) -> usize {
        self.decision_count(RepositoryTraversalDecision::SkipByDefault)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryEntry {
    name: String,
    relative_path: PathBuf,
    kind: RepositoryEntryKind,
    role: RepositoryEntryRole,
    traversal_policy: RepositoryEntryTraversalPolicy,
}

impl RepositoryEntry {
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

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    #[must_use]
    pub const fn kind(&self) -> RepositoryEntryKind {
        self.kind
    }

    #[must_use]
    pub const fn role(&self) -> RepositoryEntryRole {
        self.role
    }

    #[must_use]
    pub const fn category(&self) -> RepositoryEntryCategory {
        self.role.category()
    }

    #[must_use]
    pub const fn traversal_policy(&self) -> RepositoryEntryTraversalPolicy {
        self.traversal_policy
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryInspection {
    root: PathBuf,
    entries: Vec<RepositoryEntry>,
}

impl RepositoryInspection {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>, entries: Vec<RepositoryEntry>) -> Self {
        Self {
            root: root.into(),
            entries,
        }
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[must_use]
    pub fn entries(&self) -> &[RepositoryEntry] {
        &self.entries
    }

    #[must_use]
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn file_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.kind() == RepositoryEntryKind::File)
            .count()
    }

    #[must_use]
    pub fn directory_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.kind() == RepositoryEntryKind::Directory)
            .count()
    }

    #[must_use]
    pub fn has_relative_path(&self, relative_path: impl AsRef<Path>) -> bool {
        let relative_path = relative_path.as_ref();

        self.entries
            .iter()
            .any(|entry| entry.relative_path() == relative_path)
    }

    #[must_use]
    pub fn entries_with_role(&self, role: RepositoryEntryRole) -> Vec<&RepositoryEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.role() == role)
            .collect()
    }

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

pub fn traverse_workspace_bounded(
    inspection: &RepositoryInspection,
) -> MonadResult<RepositoryBoundedTraversal> {
    let guardrails = RepositoryTraversalGuardrails::conservative_future_recursive();
    let ignore_rules = if guardrails.respect_ignore_files() {
        RepositoryIgnoreRules::load_from_root(inspection.root())?
    } else {
        RepositoryIgnoreRules::empty()
    };

    let mut traversal_entries = Vec::new();

    for entry in inspection.entries() {
        let traversal_entry =
            RepositoryTraversalEntry::from_repository_entry(entry, 0, &ignore_rules);
        let should_descend = traversal_entry.should_descend(guardrails);
        let relative_path = traversal_entry.relative_path().to_path_buf();

        traversal_entries.push(traversal_entry);

        if should_descend {
            traverse_directory_bounded(
                inspection.root(),
                &relative_path,
                1,
                guardrails,
                &ignore_rules,
                &mut traversal_entries,
            )?;
        }
    }

    Ok(RepositoryBoundedTraversal::new(
        inspection.root().to_path_buf(),
        RepositoryTraversalMode::BoundedRecursive,
        guardrails,
        traversal_entries,
    ))
}

fn traverse_directory_bounded(
    root: &Path,
    relative_directory: &Path,
    depth: usize,
    guardrails: RepositoryTraversalGuardrails,
    ignore_rules: &RepositoryIgnoreRules,
    traversal_entries: &mut Vec<RepositoryTraversalEntry>,
) -> MonadResult<()> {
    if depth > guardrails.max_depth() {
        return Ok(());
    }

    let absolute_directory = root.join(relative_directory);

    let directory_entries = fs::read_dir(&absolute_directory).map_err(|error| {
        MonadError::internal(format!(
            "failed to traverse directory {}: {error}",
            absolute_directory.display()
        ))
    })?;

    let mut child_paths = Vec::new();

    for entry_result in directory_entries {
        let entry = entry_result.map_err(|error| {
            MonadError::internal(format!(
                "failed to read an entry from directory {}: {error}",
                absolute_directory.display()
            ))
        })?;

        child_paths.push(entry.path());
    }

    child_paths.sort();

    for child_path in child_paths {
        let traversal_entry =
            RepositoryTraversalEntry::from_path(root, &child_path, depth, ignore_rules)?;
        let should_descend = traversal_entry.should_descend(guardrails);
        let child_relative_path = traversal_entry.relative_path().to_path_buf();

        traversal_entries.push(traversal_entry);

        if should_descend {
            traverse_directory_bounded(
                root,
                &child_relative_path,
                depth + 1,
                guardrails,
                ignore_rules,
                traversal_entries,
            )?;
        }
    }

    Ok(())
}

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

fn traversal_decision_for_policy(
    traversal_policy: RepositoryEntryTraversalPolicy,
) -> RepositoryTraversalDecision {
    match traversal_policy {
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

fn traversal_decision_for_entry(
    kind: RepositoryEntryKind,
    traversal_policy: RepositoryEntryTraversalPolicy,
    ignored: bool,
) -> RepositoryTraversalDecision {
    if ignored {
        return RepositoryTraversalDecision::SkipByDefault;
    }

    if kind == RepositoryEntryKind::Symlink {
        return RepositoryTraversalDecision::InspectShallowOnly;
    }

    traversal_decision_for_policy(traversal_policy)
}

fn traversal_reason_for_policy(
    traversal_policy: RepositoryEntryTraversalPolicy,
    name: &str,
) -> &'static str {
    match traversal_policy {
        RepositoryEntryTraversalPolicy::SafeForFutureTraversal => {
            "directory is a safe candidate for future bounded traversal"
        }
        RepositoryEntryTraversalPolicy::InspectShallowOnly => {
            if name.starts_with('.') {
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

fn traversal_reason_for_entry(
    kind: RepositoryEntryKind,
    traversal_policy: RepositoryEntryTraversalPolicy,
    ignored: bool,
    name: &str,
) -> &'static str {
    if ignored {
        return "path is ignored by repository ignore rules";
    }

    if kind == RepositoryEntryKind::Symlink {
        return "symlink is not followed by default";
    }

    traversal_reason_for_policy(traversal_policy, name)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RepositoryIgnorePattern {
    pattern: String,
    directory_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RepositoryIgnoreRules {
    patterns: Vec<RepositoryIgnorePattern>,
}

impl RepositoryIgnoreRules {
    fn empty() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    fn load_from_root(root: &Path) -> MonadResult<Self> {
        let gitignore = root.join(".gitignore");

        if !gitignore.exists() {
            return Ok(Self::empty());
        }

        let text = fs::read_to_string(&gitignore).map_err(|error| {
            MonadError::internal(format!(
                "failed to read ignore file {}: {error}",
                gitignore.display()
            ))
        })?;

        Ok(Self::from_gitignore_text(&text))
    }

    fn from_gitignore_text(text: &str) -> Self {
        let patterns = text
            .lines()
            .filter_map(parse_ignore_pattern)
            .collect::<Vec<_>>();

        Self { patterns }
    }

    fn is_ignored(&self, relative_path: &Path, kind: RepositoryEntryKind) -> bool {
        self.patterns
            .iter()
            .any(|pattern| pattern.matches(relative_path, kind))
    }
}

impl RepositoryIgnorePattern {
    fn matches(&self, relative_path: &Path, kind: RepositoryEntryKind) -> bool {
        if self.directory_only && kind != RepositoryEntryKind::Directory {
            return false;
        }

        let normalized_path = normalize_relative_path(relative_path);

        if self.pattern.contains('/') {
            return normalized_path == self.pattern
                || (self.directory_only
                    && normalized_path.starts_with(&format!("{}/", self.pattern)));
        }

        let file_name = relative_path
            .file_name()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_default();

        file_name == self.pattern
            || (self.directory_only
                && normalized_path
                    .split('/')
                    .any(|component| component == self.pattern))
    }
}

fn parse_ignore_pattern(line: &str) -> Option<RepositoryIgnorePattern> {
    let trimmed = line.trim();

    if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('!') {
        return None;
    }

    let directory_only = trimmed.ends_with('/');

    let pattern = trimmed
        .trim_start_matches("./")
        .trim_start_matches('/')
        .trim_end_matches('/')
        .trim()
        .to_string();

    if pattern.is_empty() {
        return None;
    }

    Some(RepositoryIgnorePattern {
        pattern,
        directory_only,
    })
}

fn normalize_relative_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join("/")
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

        fs::create_dir_all(root.join("docs/guide/a/b/c")).expect("nested docs should be created");
        fs::create_dir_all(root.join("docs/ignored-cache"))
            .expect("ignored docs cache should be created");
        fs::create_dir_all(root.join("work")).expect("work directory should be created");
        fs::create_dir_all(root.join(".monad")).expect(".monad directory should be created");
        fs::create_dir_all(root.join("crates/monad-core/src"))
            .expect("crate source directory should be created");
        fs::create_dir_all(root.join("tools/scripts")).expect("tools should be created");
        fs::create_dir_all(root.join("target/debug")).expect("target should be created");
        fs::create_dir_all(root.join("node_modules/pkg")).expect("node_modules should be created");
        fs::create_dir_all(root.join(".git/objects")).expect(".git should be created");

        fs::write(root.join("README.md"), "# Test\n").expect("README should be written");
        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");
        fs::write(root.join("Cargo.lock"), "# lock\n").expect("Cargo.lock should be written");
        fs::write(root.join("monad.toml"), "schema_version = 1\n")
            .expect("monad.toml should be written");
        fs::write(root.join(".gitignore"), "ignored-cache/\n")
            .expect(".gitignore should be written");
        fs::write(root.join("docs/guide/intro.md"), "# Intro\n").expect("intro should be written");
        fs::write(root.join("docs/guide/a/b/c/deep.md"), "# Deep\n")
            .expect("deep file should be written");
        fs::write(root.join("docs/ignored-cache/secret.md"), "# Secret\n")
            .expect("ignored file should be written");
        fs::write(
            root.join("crates/monad-core/src/lib.rs"),
            "pub fn test() {}\n",
        )
        .expect("lib.rs should be written");
        fs::write(root.join("target/debug/cache.bin"), "cache\n")
            .expect("target cache should be written");
        fs::write(root.join("node_modules/pkg/index.js"), "export {}\n")
            .expect("node module file should be written");

        root
    }

    #[test]
    fn repository_inspection_lists_top_level_entries() {
        let root = create_inspection_workspace("lists");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");

        assert_eq!(inspection.root(), root.as_path());
        assert!(inspection.has_relative_path("monad.toml"));
        assert!(inspection.has_relative_path("Cargo.toml"));
        assert!(inspection.has_relative_path("docs"));
        assert!(inspection.has_relative_path("work"));
        assert!(inspection.has_relative_path(".monad"));

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
    fn bounded_traversal_walks_safe_directories() {
        let root = create_inspection_workspace("bounded-safe");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should succeed");

        assert_eq!(traversal.mode(), RepositoryTraversalMode::BoundedRecursive);
        assert!(traversal.entry_count() > inspection.entry_count());
        assert!(traversal.contains_relative_path("docs/guide"));
        assert!(traversal.contains_relative_path("docs/guide/intro.md"));
        assert!(traversal.contains_relative_path("crates/monad-core/src"));
        assert!(traversal.contains_relative_path("crates/monad-core/src/lib.rs"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn bounded_traversal_respects_max_depth() {
        let root = create_inspection_workspace("bounded-depth");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should succeed");

        assert_eq!(traversal.guardrails().max_depth(), 3);
        assert!(traversal.max_observed_depth() <= 3);
        assert!(traversal.contains_relative_path("docs/guide/a/b"));
        assert!(!traversal.contains_relative_path("docs/guide/a/b/c/deep.md"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn bounded_traversal_skips_generated_or_external_directories() {
        let root = create_inspection_workspace("bounded-skip-generated");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should succeed");

        assert!(traversal.contains_relative_path("target"));
        assert!(traversal.contains_relative_path("node_modules"));
        assert!(traversal.contains_relative_path(".git"));
        assert!(!traversal.contains_relative_path("target/debug/cache.bin"));
        assert!(!traversal.contains_relative_path("node_modules/pkg/index.js"));
        assert!(!traversal.contains_relative_path(".git/objects"));

        assert!(traversal.skip_count() >= 3);
        assert!(traversal.category_count(RepositoryEntryCategory::GeneratedOrExternal) >= 3);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn bounded_traversal_respects_simple_root_gitignore_patterns() {
        let root = create_inspection_workspace("bounded-ignore");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should succeed");

        assert!(traversal.contains_relative_path("docs/ignored-cache"));
        assert!(!traversal.contains_relative_path("docs/ignored-cache/secret.md"));

        let ignored_entry = traversal
            .entries()
            .iter()
            .find(|entry| entry.relative_path() == Path::new("docs/ignored-cache"))
            .expect("ignored directory should be present as a skipped entry");

        assert_eq!(
            ignored_entry.decision(),
            RepositoryTraversalDecision::SkipByDefault
        );
        assert_eq!(
            ignored_entry.reason(),
            "path is ignored by repository ignore rules"
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn bounded_traversal_output_is_deterministic() {
        let root = create_inspection_workspace("bounded-deterministic");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");

        let inspection = inspect_workspace(&context).expect("workspace should inspect");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should succeed");

        let paths: Vec<String> = traversal
            .entries()
            .iter()
            .map(|entry| entry.relative_path().display().to_string())
            .collect();

        let mut sorted_by_parent_order = paths.clone();
        sorted_by_parent_order.sort();

        assert!(paths.contains(&"Cargo.toml".to_string()));
        assert!(sorted_by_parent_order.contains(&"Cargo.toml".to_string()));
        assert!(traversal.guardrails().deterministic_ordering());

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
}
