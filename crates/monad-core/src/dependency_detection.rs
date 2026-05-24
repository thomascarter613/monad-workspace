//! Dependency signal detection for Monad.
//!
//! WP-E2-011 introduces conservative dependency signal detection.
//!
//! This module does not parse dependency contents and does not invoke package
//! managers. It only identifies files that strongly indicate dependency
//! management surfaces, such as:
//!
//! - Cargo.toml / Cargo.lock;
//! - package.json / bun.lock / pnpm-lock.yaml;
//! - pyproject.toml / requirements.txt / poetry.lock;
//! - go.mod / go.sum;
//! - pom.xml / build.gradle;
//! - composer.json / composer.lock;
//! - Gemfile / Gemfile.lock.
//!
//! Later slices can parse dependency contents and attach policy checks on top
//! of this signal layer.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::{RepositoryBoundedTraversal, RepositoryEntryKind, RepositoryToolchainKind};

/// The kind of dependency-related file detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryDependencySignalKind {
    /// A dependency manifest, such as `Cargo.toml` or `package.json`.
    Manifest,

    /// A dependency lockfile, such as `Cargo.lock` or `composer.lock`.
    Lockfile,

    /// A package manager configuration file, such as `.npmrc`.
    PackageManagerConfig,

    /// A build file that also carries dependency information, such as Gradle files.
    BuildFile,
}

impl RepositoryDependencySignalKind {
    /// Returns a stable signal-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Manifest => "manifest",
            Self::Lockfile => "lockfile",
            Self::PackageManagerConfig => "package_manager_config",
            Self::BuildFile => "build_file",
        }
    }
}

/// One dependency-related signal discovered in the repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryDependencySignal {
    toolchain: RepositoryToolchainKind,
    signal_kind: RepositoryDependencySignalKind,
    relative_path: PathBuf,
}

impl RepositoryDependencySignal {
    /// Creates a dependency signal.
    #[must_use]
    pub fn new(
        toolchain: RepositoryToolchainKind,
        signal_kind: RepositoryDependencySignalKind,
        relative_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            toolchain,
            signal_kind,
            relative_path: relative_path.into(),
        }
    }

    /// Returns the toolchain associated with the signal.
    #[must_use]
    pub const fn toolchain(&self) -> RepositoryToolchainKind {
        self.toolchain
    }

    /// Returns the dependency signal kind.
    #[must_use]
    pub const fn signal_kind(&self) -> RepositoryDependencySignalKind {
        self.signal_kind
    }

    /// Returns the signal path relative to the workspace root.
    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }
}

/// Dependency signal detection result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryDependencyDetection {
    signals: Vec<RepositoryDependencySignal>,
    detected_toolchains: BTreeSet<RepositoryToolchainKind>,
}

impl RepositoryDependencyDetection {
    /// Builds a detection result from raw signals.
    #[must_use]
    pub fn from_signals(mut signals: Vec<RepositoryDependencySignal>) -> Self {
        signals.sort_by(|left, right| {
            left.toolchain()
                .as_str()
                .cmp(right.toolchain().as_str())
                .then_with(|| {
                    left.signal_kind()
                        .as_str()
                        .cmp(right.signal_kind().as_str())
                })
                .then_with(|| left.relative_path().cmp(right.relative_path()))
        });

        let detected_toolchains = signals
            .iter()
            .map(RepositoryDependencySignal::toolchain)
            .collect::<BTreeSet<_>>();

        Self {
            signals,
            detected_toolchains,
        }
    }

    /// Returns all dependency signals.
    #[must_use]
    pub fn signals(&self) -> &[RepositoryDependencySignal] {
        &self.signals
    }

    /// Returns toolchains with dependency signals.
    #[must_use]
    pub fn detected_toolchains(&self) -> &BTreeSet<RepositoryToolchainKind> {
        &self.detected_toolchains
    }

    /// Returns true when dependency signals exist for a toolchain.
    #[must_use]
    pub fn has_toolchain(&self, toolchain: RepositoryToolchainKind) -> bool {
        self.detected_toolchains.contains(&toolchain)
    }

    /// Returns the number of dependency signals.
    #[must_use]
    pub fn signal_count(&self) -> usize {
        self.signals.len()
    }

    /// Returns the number of toolchains with dependency signals.
    #[must_use]
    pub fn detected_toolchain_count(&self) -> usize {
        self.detected_toolchains.len()
    }

    /// Returns the number of manifest signals.
    #[must_use]
    pub fn manifest_signal_count(&self) -> usize {
        self.signal_count_by_kind(RepositoryDependencySignalKind::Manifest)
    }

    /// Returns the number of lockfile signals.
    #[must_use]
    pub fn lockfile_signal_count(&self) -> usize {
        self.signal_count_by_kind(RepositoryDependencySignalKind::Lockfile)
    }

    /// Returns the number of package manager config signals.
    #[must_use]
    pub fn package_manager_config_signal_count(&self) -> usize {
        self.signal_count_by_kind(RepositoryDependencySignalKind::PackageManagerConfig)
    }

    /// Returns the number of build file signals.
    #[must_use]
    pub fn build_file_signal_count(&self) -> usize {
        self.signal_count_by_kind(RepositoryDependencySignalKind::BuildFile)
    }

    /// Counts signals by signal kind.
    #[must_use]
    pub fn signal_count_by_kind(&self, signal_kind: RepositoryDependencySignalKind) -> usize {
        self.signals
            .iter()
            .filter(|signal| signal.signal_kind() == signal_kind)
            .count()
    }

    /// Counts signals by toolchain and signal kind.
    #[must_use]
    pub fn signal_count_for_toolchain_and_kind(
        &self,
        toolchain: RepositoryToolchainKind,
        signal_kind: RepositoryDependencySignalKind,
    ) -> usize {
        self.signals
            .iter()
            .filter(|signal| signal.toolchain() == toolchain)
            .filter(|signal| signal.signal_kind() == signal_kind)
            .count()
    }

    /// Returns counts keyed by stable toolchain label.
    #[must_use]
    pub fn toolchain_counts(&self) -> BTreeMap<String, usize> {
        let mut counts = BTreeMap::new();

        for signal in &self.signals {
            *counts
                .entry(signal.toolchain().as_str().to_string())
                .or_insert(0) += 1;
        }

        counts
    }

    /// Returns counts keyed by stable dependency signal kind.
    #[must_use]
    pub fn signal_kind_counts(&self) -> BTreeMap<String, usize> {
        let mut counts = BTreeMap::new();

        for signal in &self.signals {
            *counts
                .entry(signal.signal_kind().as_str().to_string())
                .or_insert(0) += 1;
        }

        counts
    }

    /// Returns dependency signal paths for a toolchain.
    #[must_use]
    pub fn signal_paths_for_toolchain(&self, toolchain: RepositoryToolchainKind) -> Vec<String> {
        self.signals
            .iter()
            .filter(|signal| signal.toolchain() == toolchain)
            .map(|signal| signal.relative_path().display().to_string())
            .collect()
    }
}

/// Detects dependency signals from bounded traversal output.
#[must_use]
pub fn detect_repository_dependency_signals(
    traversal: &RepositoryBoundedTraversal,
) -> RepositoryDependencyDetection {
    let mut signals = Vec::new();

    for entry in traversal.entries() {
        if entry.kind() != RepositoryEntryKind::File {
            continue;
        }

        signals.extend(classify_dependency_signals(entry.relative_path()));
    }

    RepositoryDependencyDetection::from_signals(signals)
}

/// Classifies dependency signals from one relative path.
fn classify_dependency_signals(path: &Path) -> Vec<RepositoryDependencySignal> {
    let file_name = path
        .file_name()
        .map(|value| value.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();

    match file_name.as_str() {
        "cargo.toml" => vec![RepositoryDependencySignal::new(
            RepositoryToolchainKind::Rust,
            RepositoryDependencySignalKind::Manifest,
            path,
        )],
        "cargo.lock" => vec![RepositoryDependencySignal::new(
            RepositoryToolchainKind::Rust,
            RepositoryDependencySignalKind::Lockfile,
            path,
        )],
        "package.json" => vec![RepositoryDependencySignal::new(
            RepositoryToolchainKind::JavaScript,
            RepositoryDependencySignalKind::Manifest,
            path,
        )],
        "bun.lock" | "bun.lockb" | "package-lock.json" | "pnpm-lock.yaml" | "yarn.lock" => {
            vec![RepositoryDependencySignal::new(
                RepositoryToolchainKind::JavaScript,
                RepositoryDependencySignalKind::Lockfile,
                path,
            )]
        }
        ".npmrc" | ".yarnrc" | ".yarnrc.yml" | ".pnpmfile.cjs" | "bunfig.toml" => {
            vec![RepositoryDependencySignal::new(
                RepositoryToolchainKind::JavaScript,
                RepositoryDependencySignalKind::PackageManagerConfig,
                path,
            )]
        }
        "pyproject.toml" | "setup.py" | "requirements.txt" | "pipfile" => {
            vec![RepositoryDependencySignal::new(
                RepositoryToolchainKind::Python,
                RepositoryDependencySignalKind::Manifest,
                path,
            )]
        }
        "poetry.lock" | "uv.lock" | "pipfile.lock" => vec![RepositoryDependencySignal::new(
            RepositoryToolchainKind::Python,
            RepositoryDependencySignalKind::Lockfile,
            path,
        )],
        "go.mod" => vec![RepositoryDependencySignal::new(
            RepositoryToolchainKind::Go,
            RepositoryDependencySignalKind::Manifest,
            path,
        )],
        "go.sum" => vec![RepositoryDependencySignal::new(
            RepositoryToolchainKind::Go,
            RepositoryDependencySignalKind::Lockfile,
            path,
        )],
        "pom.xml" => vec![RepositoryDependencySignal::new(
            RepositoryToolchainKind::Java,
            RepositoryDependencySignalKind::Manifest,
            path,
        )],
        "build.gradle" | "build.gradle.kts" | "settings.gradle" | "settings.gradle.kts" => {
            vec![RepositoryDependencySignal::new(
                RepositoryToolchainKind::Java,
                RepositoryDependencySignalKind::BuildFile,
                path,
            )]
        }
        "composer.json" => vec![RepositoryDependencySignal::new(
            RepositoryToolchainKind::Php,
            RepositoryDependencySignalKind::Manifest,
            path,
        )],
        "composer.lock" => vec![RepositoryDependencySignal::new(
            RepositoryToolchainKind::Php,
            RepositoryDependencySignalKind::Lockfile,
            path,
        )],
        "gemfile" => vec![RepositoryDependencySignal::new(
            RepositoryToolchainKind::Ruby,
            RepositoryDependencySignalKind::Manifest,
            path,
        )],
        "gemfile.lock" => vec![RepositoryDependencySignal::new(
            RepositoryToolchainKind::Ruby,
            RepositoryDependencySignalKind::Lockfile,
            path,
        )],
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{WorkspaceContext, inspect_workspace, traverse_workspace_bounded};

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "monad-dependency-detection-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_dependency_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);

        fs::create_dir_all(root.join("apps/web")).expect("web directory should be created");
        fs::create_dir_all(root.join("services/python"))
            .expect("python directory should be created");
        fs::create_dir_all(root.join("services/go")).expect("go directory should be created");
        fs::create_dir_all(root.join("services/java")).expect("java directory should be created");
        fs::create_dir_all(root.join("services/php")).expect("php directory should be created");
        fs::create_dir_all(root.join("tools/ruby")).expect("ruby directory should be created");

        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");
        fs::write(root.join("Cargo.lock"), "# lock\n").expect("Cargo.lock should be written");

        fs::write(root.join("apps/web/package.json"), "{}\n")
            .expect("package.json should be written");
        fs::write(root.join("apps/web/bun.lock"), "# lock\n").expect("bun.lock should be written");
        fs::write(root.join("apps/web/.npmrc"), "engine-strict=true\n")
            .expect(".npmrc should be written");

        fs::write(root.join("services/python/pyproject.toml"), "[project]\n")
            .expect("pyproject.toml should be written");
        fs::write(root.join("services/python/poetry.lock"), "# lock\n")
            .expect("poetry.lock should be written");

        fs::write(root.join("services/go/go.mod"), "module example.com/test\n")
            .expect("go.mod should be written");
        fs::write(root.join("services/go/go.sum"), "# sum\n").expect("go.sum should be written");

        fs::write(root.join("services/java/pom.xml"), "<project></project>\n")
            .expect("pom.xml should be written");
        fs::write(root.join("services/java/build.gradle"), "plugins {}\n")
            .expect("build.gradle should be written");

        fs::write(root.join("services/php/composer.json"), "{}\n")
            .expect("composer.json should be written");
        fs::write(root.join("services/php/composer.lock"), "{}\n")
            .expect("composer.lock should be written");

        fs::write(
            root.join("tools/ruby/Gemfile"),
            "source 'https://rubygems.org'\n",
        )
        .expect("Gemfile should be written");
        fs::write(root.join("tools/ruby/Gemfile.lock"), "# lock\n")
            .expect("Gemfile.lock should be written");

        root
    }

    fn detect_from_workspace(root: &Path) -> RepositoryDependencyDetection {
        let context = WorkspaceContext::new(root).expect("context should be created");
        let inspection = inspect_workspace(&context).expect("inspection should run");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should run");

        detect_repository_dependency_signals(&traversal)
    }

    #[test]
    fn dependency_signal_kind_labels_are_stable() {
        assert_eq!(
            RepositoryDependencySignalKind::Manifest.as_str(),
            "manifest"
        );
        assert_eq!(
            RepositoryDependencySignalKind::Lockfile.as_str(),
            "lockfile"
        );
        assert_eq!(
            RepositoryDependencySignalKind::PackageManagerConfig.as_str(),
            "package_manager_config"
        );
        assert_eq!(
            RepositoryDependencySignalKind::BuildFile.as_str(),
            "build_file"
        );
    }

    #[test]
    fn detects_dependency_signals_for_common_toolchains() {
        let root = create_dependency_workspace("common");

        let detection = detect_from_workspace(&root);

        assert!(detection.has_toolchain(RepositoryToolchainKind::Rust));
        assert!(detection.has_toolchain(RepositoryToolchainKind::JavaScript));
        assert!(detection.has_toolchain(RepositoryToolchainKind::Python));
        assert!(detection.has_toolchain(RepositoryToolchainKind::Go));
        assert!(detection.has_toolchain(RepositoryToolchainKind::Java));
        assert!(detection.has_toolchain(RepositoryToolchainKind::Php));
        assert!(detection.has_toolchain(RepositoryToolchainKind::Ruby));

        assert_eq!(detection.detected_toolchain_count(), 7);
        assert!(detection.signal_count() >= 13);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn dependency_signal_kind_counts_are_stable() {
        let root = create_dependency_workspace("kind-counts");

        let detection = detect_from_workspace(&root);
        let counts = detection.signal_kind_counts();

        assert!(counts.get("manifest").copied().unwrap_or(0) >= 7);
        assert!(counts.get("lockfile").copied().unwrap_or(0) >= 6);
        assert_eq!(
            counts.get("package_manager_config").copied().unwrap_or(0),
            1
        );
        assert_eq!(counts.get("build_file").copied().unwrap_or(0), 1);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn dependency_toolchain_counts_are_stable() {
        let root = create_dependency_workspace("toolchain-counts");

        let detection = detect_from_workspace(&root);
        let counts = detection.toolchain_counts();

        assert_eq!(counts.get("rust").copied().unwrap_or(0), 2);
        assert_eq!(counts.get("javascript").copied().unwrap_or(0), 3);
        assert_eq!(counts.get("python").copied().unwrap_or(0), 2);
        assert_eq!(counts.get("go").copied().unwrap_or(0), 2);
        assert_eq!(counts.get("java").copied().unwrap_or(0), 2);
        assert_eq!(counts.get("php").copied().unwrap_or(0), 2);
        assert_eq!(counts.get("ruby").copied().unwrap_or(0), 2);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn dependency_paths_can_be_grouped_by_toolchain() {
        let root = create_dependency_workspace("paths");

        let detection = detect_from_workspace(&root);
        let javascript_paths =
            detection.signal_paths_for_toolchain(RepositoryToolchainKind::JavaScript);

        assert!(
            javascript_paths
                .iter()
                .any(|path| path == "apps/web/package.json")
        );
        assert!(
            javascript_paths
                .iter()
                .any(|path| path == "apps/web/bun.lock")
        );
        assert!(
            javascript_paths
                .iter()
                .any(|path| path == "apps/web/.npmrc")
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn dependency_detection_is_empty_without_known_signals() {
        let root = unique_temp_dir("empty");
        fs::create_dir_all(root.join("notes")).expect("notes directory should be created");
        fs::write(root.join("notes/readme.txt"), "hello\n").expect("notes file should be written");

        let detection = detect_from_workspace(&root);

        assert_eq!(detection.detected_toolchain_count(), 0);
        assert_eq!(detection.signal_count(), 0);
        assert!(detection.toolchain_counts().is_empty());
        assert!(detection.signal_kind_counts().is_empty());

        fs::remove_dir_all(root).ok();
    }
}
