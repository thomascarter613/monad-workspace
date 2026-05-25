//! Repository toolchain detection for Monad.
//!
//! WP-E2-010 introduces conservative file-pattern-based toolchain detection.
//!
//! This module does not invoke external tools. It only reads the bounded
//! traversal model and detects signals such as:
//!
//! - Cargo.toml;
//! - package.json;
//! - tsconfig.json;
//! - pyproject.toml;
//! - go.mod;
//! - pom.xml;
//! - composer.json;
//! - Gemfile;
//! - source file extensions.
//!
//! This gives Monad a safe foundation for later dependency intelligence,
//! toolchain checks, and policy-aware repository analysis.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::{RepositoryBoundedTraversal, RepositoryEntryKind};

/// Programming language or ecosystem detected in a repository.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RepositoryToolchainKind {
    /// Rust / Cargo.
    Rust,

    /// JavaScript / Node ecosystem.
    JavaScript,

    /// TypeScript ecosystem.
    TypeScript,

    /// Python ecosystem.
    Python,

    /// Go ecosystem.
    Go,

    /// Java / JVM ecosystem.
    Java,

    /// PHP / Composer ecosystem.
    Php,

    /// Ruby / Bundler ecosystem.
    Ruby,
}

impl RepositoryToolchainKind {
    /// Returns a stable toolchain label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::JavaScript => "javascript",
            Self::TypeScript => "typescript",
            Self::Python => "python",
            Self::Go => "go",
            Self::Java => "java",
            Self::Php => "php",
            Self::Ruby => "ruby",
        }
    }
}

/// The kind of signal that caused Monad to detect a toolchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryToolchainSignalKind {
    /// A manifest file such as `Cargo.toml` or `package.json`.
    Manifest,

    /// A lockfile such as `Cargo.lock` or `composer.lock`.
    Lockfile,

    /// A source file such as `main.rs` or `app.py`.
    SourceFile,

    /// A config file such as `tsconfig.json`.
    ConfigFile,

    /// A build file such as `pom.xml` or `build.gradle`.
    BuildFile,
}

impl RepositoryToolchainSignalKind {
    /// Returns a stable signal-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Manifest => "manifest",
            Self::Lockfile => "lockfile",
            Self::SourceFile => "source_file",
            Self::ConfigFile => "config_file",
            Self::BuildFile => "build_file",
        }
    }
}

/// One detected toolchain signal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryToolchainSignal {
    toolchain: RepositoryToolchainKind,
    signal_kind: RepositoryToolchainSignalKind,
    relative_path: PathBuf,
}

impl RepositoryToolchainSignal {
    /// Creates a toolchain signal.
    #[must_use]
    pub fn new(
        toolchain: RepositoryToolchainKind,
        signal_kind: RepositoryToolchainSignalKind,
        relative_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            toolchain,
            signal_kind,
            relative_path: relative_path.into(),
        }
    }

    /// Returns the detected toolchain.
    #[must_use]
    pub const fn toolchain(&self) -> RepositoryToolchainKind {
        self.toolchain
    }

    /// Returns the signal kind.
    #[must_use]
    pub const fn signal_kind(&self) -> RepositoryToolchainSignalKind {
        self.signal_kind
    }

    /// Returns the relative path that produced the signal.
    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }
}

/// Toolchain detection result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryToolchainDetection {
    signals: Vec<RepositoryToolchainSignal>,
    detected_toolchains: BTreeSet<RepositoryToolchainKind>,
}

impl RepositoryToolchainDetection {
    /// Creates a detection result from raw signals.
    #[must_use]
    pub fn from_signals(mut signals: Vec<RepositoryToolchainSignal>) -> Self {
        signals.sort_by(|left, right| {
            left.toolchain()
                .as_str()
                .cmp(right.toolchain().as_str())
                .then_with(|| left.relative_path().cmp(right.relative_path()))
                .then_with(|| {
                    left.signal_kind()
                        .as_str()
                        .cmp(right.signal_kind().as_str())
                })
        });

        let detected_toolchains = signals
            .iter()
            .map(RepositoryToolchainSignal::toolchain)
            .collect::<BTreeSet<_>>();

        Self {
            signals,
            detected_toolchains,
        }
    }

    /// Returns all detected signals.
    #[must_use]
    pub fn signals(&self) -> &[RepositoryToolchainSignal] {
        &self.signals
    }

    /// Returns the detected toolchains.
    #[must_use]
    pub fn detected_toolchains(&self) -> &BTreeSet<RepositoryToolchainKind> {
        &self.detected_toolchains
    }

    /// Returns true when a toolchain was detected.
    #[must_use]
    pub fn has_toolchain(&self, toolchain: RepositoryToolchainKind) -> bool {
        self.detected_toolchains.contains(&toolchain)
    }

    /// Returns the number of detected toolchains.
    #[must_use]
    pub fn detected_toolchain_count(&self) -> usize {
        self.detected_toolchains.len()
    }

    /// Returns the number of detected signals.
    #[must_use]
    pub fn signal_count(&self) -> usize {
        self.signals.len()
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

    /// Returns counts keyed by stable signal-kind label.
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

    /// Returns signal paths for a specific toolchain.
    #[must_use]
    pub fn signal_paths_for_toolchain(&self, toolchain: RepositoryToolchainKind) -> Vec<String> {
        self.signals
            .iter()
            .filter(|signal| signal.toolchain() == toolchain)
            .map(|signal| signal.relative_path().display().to_string())
            .collect()
    }
}

/// Detects repository toolchains from bounded traversal output.
///
/// Detection is conservative and pattern-based. It should prefer false
/// negatives over false positives until later slices add richer analysis.
#[must_use]
pub fn detect_repository_toolchains(
    traversal: &RepositoryBoundedTraversal,
) -> RepositoryToolchainDetection {
    let mut signals = Vec::new();

    for entry in traversal.entries() {
        if entry.kind() != RepositoryEntryKind::File {
            continue;
        }

        signals.extend(classify_toolchain_signals(entry.relative_path()));
    }

    RepositoryToolchainDetection::from_signals(signals)
}

/// Classifies toolchain signals from a relative path.
fn classify_toolchain_signals(path: &Path) -> Vec<RepositoryToolchainSignal> {
    let mut signals = Vec::new();

    let file_name = path
        .file_name()
        .map(|value| value.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();

    let extension = path
        .extension()
        .map(|value| value.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();

    match file_name.as_str() {
        "cargo.toml" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Rust,
            RepositoryToolchainSignalKind::Manifest,
            path,
        )),
        "cargo.lock" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Rust,
            RepositoryToolchainSignalKind::Lockfile,
            path,
        )),
        "package.json" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::JavaScript,
            RepositoryToolchainSignalKind::Manifest,
            path,
        )),
        "bun.lock" | "bun.lockb" | "package-lock.json" | "pnpm-lock.yaml" | "yarn.lock" => {
            signals.push(RepositoryToolchainSignal::new(
                RepositoryToolchainKind::JavaScript,
                RepositoryToolchainSignalKind::Lockfile,
                path,
            ));
        }
        "tsconfig.json" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::TypeScript,
            RepositoryToolchainSignalKind::ConfigFile,
            path,
        )),
        "pyproject.toml" | "setup.py" | "requirements.txt" | "pipfile" => {
            signals.push(RepositoryToolchainSignal::new(
                RepositoryToolchainKind::Python,
                RepositoryToolchainSignalKind::Manifest,
                path,
            ));
        }
        "poetry.lock" | "uv.lock" | "pipfile.lock" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Python,
            RepositoryToolchainSignalKind::Lockfile,
            path,
        )),
        "go.mod" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Go,
            RepositoryToolchainSignalKind::Manifest,
            path,
        )),
        "go.sum" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Go,
            RepositoryToolchainSignalKind::Lockfile,
            path,
        )),
        "pom.xml"
        | "build.gradle"
        | "build.gradle.kts"
        | "settings.gradle"
        | "settings.gradle.kts" => {
            signals.push(RepositoryToolchainSignal::new(
                RepositoryToolchainKind::Java,
                RepositoryToolchainSignalKind::BuildFile,
                path,
            ));
        }
        "composer.json" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Php,
            RepositoryToolchainSignalKind::Manifest,
            path,
        )),
        "composer.lock" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Php,
            RepositoryToolchainSignalKind::Lockfile,
            path,
        )),
        "gemfile" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Ruby,
            RepositoryToolchainSignalKind::Manifest,
            path,
        )),
        "gemfile.lock" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Ruby,
            RepositoryToolchainSignalKind::Lockfile,
            path,
        )),
        _ => {}
    }

    match extension.as_str() {
        "rs" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Rust,
            RepositoryToolchainSignalKind::SourceFile,
            path,
        )),
        "js" | "jsx" | "mjs" | "cjs" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::JavaScript,
            RepositoryToolchainSignalKind::SourceFile,
            path,
        )),
        "ts" | "tsx" | "mts" | "cts" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::TypeScript,
            RepositoryToolchainSignalKind::SourceFile,
            path,
        )),
        "py" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Python,
            RepositoryToolchainSignalKind::SourceFile,
            path,
        )),
        "go" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Go,
            RepositoryToolchainSignalKind::SourceFile,
            path,
        )),
        "java" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Java,
            RepositoryToolchainSignalKind::SourceFile,
            path,
        )),
        "php" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Php,
            RepositoryToolchainSignalKind::SourceFile,
            path,
        )),
        "rb" => signals.push(RepositoryToolchainSignal::new(
            RepositoryToolchainKind::Ruby,
            RepositoryToolchainSignalKind::SourceFile,
            path,
        )),
        _ => {}
    }

    signals
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::time::SystemTime;

    use crate::{WorkspaceContext, inspect_workspace, traverse_workspace_bounded};

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "monad-toolchain-detection-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_toolchain_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);

        fs::create_dir_all(root.join("crates/monad-core/src"))
            .expect("Rust source directory should be created");
        fs::create_dir_all(root.join("apps/web/src"))
            .expect("web source directory should be created");
        fs::create_dir_all(root.join("services/api"))
            .expect("Python service directory should be created");
        fs::create_dir_all(root.join("services/go-api"))
            .expect("Go service directory should be created");
        fs::create_dir_all(root.join("services/java-api/src/main/java"))
            .expect("Java service directory should be created");
        fs::create_dir_all(root.join("services/php-app"))
            .expect("PHP service directory should be created");
        fs::create_dir_all(root.join("tools/ruby"))
            .expect("Ruby tools directory should be created");

        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");
        fs::write(root.join("Cargo.lock"), "# lock\n").expect("Cargo.lock should be written");
        fs::write(
            root.join("crates/monad-core/src/lib.rs"),
            "pub fn test() {}\n",
        )
        .expect("Rust source should be written");

        fs::write(root.join("package.json"), "{}\n").expect("package.json should be written");
        fs::write(root.join("tsconfig.json"), "{}\n").expect("tsconfig.json should be written");
        fs::write(root.join("apps/web/src/main.ts"), "export {}\n")
            .expect("TypeScript source should be written");
        fs::write(root.join("apps/web/src/main.js"), "export {}\n")
            .expect("JavaScript source should be written");

        fs::write(root.join("pyproject.toml"), "[project]\n")
            .expect("pyproject.toml should be written");
        fs::write(root.join("services/api/app.py"), "print('hello')\n")
            .expect("Python source should be written");

        fs::write(root.join("go.mod"), "module example.com/test\n")
            .expect("go.mod should be written");
        fs::write(root.join("services/go-api/main.go"), "package main\n")
            .expect("Go source should be written");

        fs::write(root.join("pom.xml"), "<project></project>\n")
            .expect("pom.xml should be written");
        fs::write(
            root.join("services/java-api/src/main/java/App.java"),
            "class App {}\n",
        )
        .expect("Java source should be written");

        fs::write(root.join("composer.json"), "{}\n").expect("composer.json should be written");
        fs::write(root.join("services/php-app/index.php"), "<?php\n")
            .expect("PHP source should be written");

        fs::write(root.join("Gemfile"), "source 'https://rubygems.org'\n")
            .expect("Gemfile should be written");
        fs::write(root.join("tools/ruby/tool.rb"), "puts 'hello'\n")
            .expect("Ruby source should be written");

        root
    }

    fn detect_from_workspace(root: &Path) -> RepositoryToolchainDetection {
        let context = WorkspaceContext::new(root).expect("context should be created");
        let inspection = inspect_workspace(&context).expect("inspection should run");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should run");

        detect_repository_toolchains(&traversal)
    }

    #[test]
    fn toolchain_kind_labels_are_stable() {
        assert_eq!(RepositoryToolchainKind::Rust.as_str(), "rust");
        assert_eq!(RepositoryToolchainKind::JavaScript.as_str(), "javascript");
        assert_eq!(RepositoryToolchainKind::TypeScript.as_str(), "typescript");
        assert_eq!(RepositoryToolchainKind::Python.as_str(), "python");
        assert_eq!(RepositoryToolchainKind::Go.as_str(), "go");
        assert_eq!(RepositoryToolchainKind::Java.as_str(), "java");
        assert_eq!(RepositoryToolchainKind::Php.as_str(), "php");
        assert_eq!(RepositoryToolchainKind::Ruby.as_str(), "ruby");
    }

    #[test]
    fn signal_kind_labels_are_stable() {
        assert_eq!(RepositoryToolchainSignalKind::Manifest.as_str(), "manifest");
        assert_eq!(RepositoryToolchainSignalKind::Lockfile.as_str(), "lockfile");
        assert_eq!(
            RepositoryToolchainSignalKind::SourceFile.as_str(),
            "source_file"
        );
        assert_eq!(
            RepositoryToolchainSignalKind::ConfigFile.as_str(),
            "config_file"
        );
        assert_eq!(
            RepositoryToolchainSignalKind::BuildFile.as_str(),
            "build_file"
        );
    }

    #[test]
    fn detects_common_repository_toolchains() {
        let root = create_toolchain_workspace("common");

        let detection = detect_from_workspace(&root);

        assert!(detection.has_toolchain(RepositoryToolchainKind::Rust));
        assert!(detection.has_toolchain(RepositoryToolchainKind::JavaScript));
        assert!(detection.has_toolchain(RepositoryToolchainKind::TypeScript));
        assert!(detection.has_toolchain(RepositoryToolchainKind::Python));
        assert!(detection.has_toolchain(RepositoryToolchainKind::Go));
        assert!(detection.has_toolchain(RepositoryToolchainKind::Java));
        assert!(detection.has_toolchain(RepositoryToolchainKind::Php));
        assert!(detection.has_toolchain(RepositoryToolchainKind::Ruby));

        assert_eq!(detection.detected_toolchain_count(), 8);
        assert!(detection.signal_count() >= 8);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn toolchain_counts_are_stable_and_machine_readable() {
        let root = create_toolchain_workspace("counts");

        let detection = detect_from_workspace(&root);
        let counts = detection.toolchain_counts();

        assert!(counts.get("rust").copied().unwrap_or(0) >= 2);
        assert!(counts.get("javascript").copied().unwrap_or(0) >= 2);
        assert!(counts.get("typescript").copied().unwrap_or(0) >= 2);
        assert!(counts.get("python").copied().unwrap_or(0) >= 2);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn signal_kind_counts_are_stable_and_machine_readable() {
        let root = create_toolchain_workspace("signal-counts");

        let detection = detect_from_workspace(&root);
        let counts = detection.signal_kind_counts();

        assert!(counts.get("manifest").copied().unwrap_or(0) >= 1);
        assert!(counts.get("source_file").copied().unwrap_or(0) >= 1);
        assert!(counts.get("lockfile").copied().unwrap_or(0) >= 1);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn signal_paths_can_be_grouped_by_toolchain() {
        let root = create_toolchain_workspace("signal-paths");

        let detection = detect_from_workspace(&root);
        let rust_paths = detection.signal_paths_for_toolchain(RepositoryToolchainKind::Rust);

        assert!(rust_paths.iter().any(|path| path == "Cargo.toml"));
        assert!(
            rust_paths
                .iter()
                .any(|path| path == "crates/monad-core/src/lib.rs")
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn detection_is_empty_when_no_known_signals_exist() {
        let root = unique_temp_dir("empty");
        fs::create_dir_all(root.join("notes")).expect("notes directory should be created");
        fs::write(root.join("notes/readme.txt"), "hello\n").expect("notes file should be written");

        let detection = detect_from_workspace(&root);

        assert_eq!(detection.detected_toolchain_count(), 0);
        assert_eq!(detection.signal_count(), 0);
        assert!(detection.toolchain_counts().is_empty());

        fs::remove_dir_all(root).ok();
    }
}
