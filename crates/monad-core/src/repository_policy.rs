//! Repository intelligence policy checks for Monad.
//!
//! WP-E2-012 introduces advisory repository-intelligence policy diagnostics.
//!
//! These checks are intentionally non-blocking for now. They provide a durable
//! policy layer that can later be surfaced through `monad check` as warnings,
//! failures, or configurable governance rules.
//!
//! The policy checks consume already-built repository intelligence:
//!
//! - shallow inspection;
//! - bounded traversal;
//! - dependency signal detection.
//!
//! They do not invoke external tools and do not parse dependency contents.

use crate::{
    RepositoryBoundedTraversal, RepositoryDependencyDetection, RepositoryDependencySignalKind,
    RepositoryEntryCategory, RepositoryEntryRole, RepositoryInspection, RepositoryToolchainKind,
};

/// Severity for repository intelligence policy diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryPolicySeverity {
    /// Informational diagnostic.
    Info,

    /// Advisory diagnostic. Useful for improving repository quality, but not a failure.
    Advisory,

    /// Warning diagnostic. Stronger than advisory, but still non-blocking in this slice.
    Warning,
}

impl RepositoryPolicySeverity {
    /// Returns a stable severity label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Advisory => "advisory",
            Self::Warning => "warning",
        }
    }
}

/// One repository intelligence policy diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryPolicyDiagnostic {
    code: &'static str,
    severity: RepositoryPolicySeverity,
    message: String,
    related_paths: Vec<String>,
}

impl RepositoryPolicyDiagnostic {
    /// Creates a policy diagnostic.
    #[must_use]
    pub fn new(
        code: &'static str,
        severity: RepositoryPolicySeverity,
        message: impl Into<String>,
        related_paths: Vec<String>,
    ) -> Self {
        Self {
            code,
            severity,
            message: message.into(),
            related_paths,
        }
    }

    /// Returns the stable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    /// Returns the diagnostic severity.
    #[must_use]
    pub const fn severity(&self) -> RepositoryPolicySeverity {
        self.severity
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns related repository paths.
    #[must_use]
    pub fn related_paths(&self) -> &[String] {
        &self.related_paths
    }
}

/// Repository intelligence policy report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryPolicyReport {
    diagnostics: Vec<RepositoryPolicyDiagnostic>,
}

impl RepositoryPolicyReport {
    /// Creates a policy report from diagnostics.
    #[must_use]
    pub fn new(mut diagnostics: Vec<RepositoryPolicyDiagnostic>) -> Self {
        diagnostics.sort_by(|left, right| {
            left.severity()
                .as_str()
                .cmp(right.severity().as_str())
                .then_with(|| left.code().cmp(right.code()))
                .then_with(|| left.message().cmp(right.message()))
        });

        Self { diagnostics }
    }

    /// Returns all diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[RepositoryPolicyDiagnostic] {
        &self.diagnostics
    }

    /// Returns the total diagnostic count.
    #[must_use]
    pub fn diagnostic_count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Counts diagnostics by severity.
    #[must_use]
    pub fn severity_count(&self, severity: RepositoryPolicySeverity) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity() == severity)
            .count()
    }

    /// Returns the number of info diagnostics.
    #[must_use]
    pub fn info_count(&self) -> usize {
        self.severity_count(RepositoryPolicySeverity::Info)
    }

    /// Returns the number of advisory diagnostics.
    #[must_use]
    pub fn advisory_count(&self) -> usize {
        self.severity_count(RepositoryPolicySeverity::Advisory)
    }

    /// Returns the number of warning diagnostics.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.severity_count(RepositoryPolicySeverity::Warning)
    }

    /// Returns true when any warning exists.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.warning_count() > 0
    }
}

/// Evaluates repository intelligence policy diagnostics.
///
/// This function is advisory-only in WP-E2-012.
#[must_use]
pub fn evaluate_repository_intelligence_policy(
    inspection: &RepositoryInspection,
    bounded_traversal: &RepositoryBoundedTraversal,
    dependencies: &RepositoryDependencyDetection,
) -> RepositoryPolicyReport {
    let mut diagnostics = Vec::new();

    evaluate_basic_repository_presence_policy(inspection, &mut diagnostics);
    evaluate_dependency_lockfile_policy(dependencies, &mut diagnostics);
    evaluate_traversal_safety_policy(bounded_traversal, &mut diagnostics);

    RepositoryPolicyReport::new(diagnostics)
}

/// Checks for basic repository hygiene signals.
fn evaluate_basic_repository_presence_policy(
    inspection: &RepositoryInspection,
    diagnostics: &mut Vec<RepositoryPolicyDiagnostic>,
) {
    if inspection
        .entries_with_role(RepositoryEntryRole::Readme)
        .is_empty()
    {
        diagnostics.push(RepositoryPolicyDiagnostic::new(
            "MONAD-RI-0001",
            RepositoryPolicySeverity::Advisory,
            "repository has no top-level README signal",
            Vec::new(),
        ));
    }

    if inspection
        .entries_with_role(RepositoryEntryRole::License)
        .is_empty()
    {
        diagnostics.push(RepositoryPolicyDiagnostic::new(
            "MONAD-RI-0002",
            RepositoryPolicySeverity::Advisory,
            "repository has no top-level license signal",
            Vec::new(),
        ));
    }
}

/// Checks for dependency manifests that do not have corresponding lockfiles.
fn evaluate_dependency_lockfile_policy(
    dependencies: &RepositoryDependencyDetection,
    diagnostics: &mut Vec<RepositoryPolicyDiagnostic>,
) {
    for toolchain in [
        RepositoryToolchainKind::Rust,
        RepositoryToolchainKind::JavaScript,
        RepositoryToolchainKind::Python,
        RepositoryToolchainKind::Go,
        RepositoryToolchainKind::Php,
        RepositoryToolchainKind::Ruby,
    ] {
        let manifest_count = dependencies.signal_count_for_toolchain_and_kind(
            toolchain,
            RepositoryDependencySignalKind::Manifest,
        );

        let lockfile_count = dependencies.signal_count_for_toolchain_and_kind(
            toolchain,
            RepositoryDependencySignalKind::Lockfile,
        );

        if manifest_count > 0 && lockfile_count == 0 {
            diagnostics.push(RepositoryPolicyDiagnostic::new(
                "MONAD-RI-0100",
                RepositoryPolicySeverity::Warning,
                format!(
                    "{} dependency manifest signal exists without a lockfile signal",
                    toolchain.as_str()
                ),
                dependencies.signal_paths_for_toolchain(toolchain),
            ));
        }
    }

    let javascript_config_count = dependencies.signal_count_for_toolchain_and_kind(
        RepositoryToolchainKind::JavaScript,
        RepositoryDependencySignalKind::PackageManagerConfig,
    );

    let javascript_manifest_count = dependencies.signal_count_for_toolchain_and_kind(
        RepositoryToolchainKind::JavaScript,
        RepositoryDependencySignalKind::Manifest,
    );

    if javascript_config_count > 0 && javascript_manifest_count == 0 {
        diagnostics.push(RepositoryPolicyDiagnostic::new(
            "MONAD-RI-0101",
            RepositoryPolicySeverity::Advisory,
            "JavaScript package manager config exists without a package manifest signal",
            dependencies.signal_paths_for_toolchain(RepositoryToolchainKind::JavaScript),
        ));
    }
}

/// Checks traversal safety outcomes.
fn evaluate_traversal_safety_policy(
    bounded_traversal: &RepositoryBoundedTraversal,
    diagnostics: &mut Vec<RepositoryPolicyDiagnostic>,
) {
    let generated_or_external_count =
        bounded_traversal.category_count(RepositoryEntryCategory::GeneratedOrExternal);

    if generated_or_external_count > 0 {
        diagnostics.push(RepositoryPolicyDiagnostic::new(
            "MONAD-RI-0200",
            RepositoryPolicySeverity::Info,
            format!(
                "{generated_or_external_count} generated or external path(s) were identified for conservative traversal handling"
            ),
            Vec::new(),
        ));
    }

    if bounded_traversal.max_observed_depth() <= bounded_traversal.guardrails().max_depth() {
        diagnostics.push(RepositoryPolicyDiagnostic::new(
            "MONAD-RI-0201",
            RepositoryPolicySeverity::Info,
            "bounded traversal stayed within configured max depth",
            Vec::new(),
        ));
    } else {
        diagnostics.push(RepositoryPolicyDiagnostic::new(
            "MONAD-RI-0202",
            RepositoryPolicySeverity::Warning,
            "bounded traversal exceeded configured max depth",
            Vec::new(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::SystemTime;

    use crate::{
        WorkspaceContext, detect_repository_dependency_signals, inspect_workspace,
        traverse_workspace_bounded,
    };

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "monad-repository-policy-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_policy_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);

        fs::create_dir_all(root.join("apps/web")).expect("web directory should be created");
        fs::create_dir_all(root.join("target/debug")).expect("target directory should be created");

        fs::write(root.join("package.json"), "{}\n").expect("package.json should be written");
        fs::write(root.join("apps/web/.npmrc"), "engine-strict=true\n")
            .expect(".npmrc should be written");
        fs::write(root.join("target/debug/cache.bin"), "cache\n").expect("cache should be written");

        root
    }

    fn evaluate_from_workspace(root: &Path) -> RepositoryPolicyReport {
        let context = WorkspaceContext::new(root).expect("context should be created");
        let inspection = inspect_workspace(&context).expect("inspection should run");
        let traversal = traverse_workspace_bounded(&inspection).expect("traversal should run");
        let dependencies = detect_repository_dependency_signals(&traversal);

        evaluate_repository_intelligence_policy(&inspection, &traversal, &dependencies)
    }

    #[test]
    fn policy_severity_labels_are_stable() {
        assert_eq!(RepositoryPolicySeverity::Info.as_str(), "info");
        assert_eq!(RepositoryPolicySeverity::Advisory.as_str(), "advisory");
        assert_eq!(RepositoryPolicySeverity::Warning.as_str(), "warning");
    }

    #[test]
    fn policy_reports_missing_readme_and_license() {
        let root = create_policy_workspace("missing-basics");

        let report = evaluate_from_workspace(&root);

        assert!(
            report
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.code() == "MONAD-RI-0001")
        );
        assert!(
            report
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.code() == "MONAD-RI-0002")
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn policy_warns_when_manifest_has_no_lockfile() {
        let root = create_policy_workspace("missing-lockfile");

        let report = evaluate_from_workspace(&root);

        assert!(
            report
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.code() == "MONAD-RI-0100")
        );
        assert!(report.has_warnings());
        assert!(report.warning_count() >= 1);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn policy_reports_traversal_safety_information() {
        let root = create_policy_workspace("traversal-safety");

        let report = evaluate_from_workspace(&root);

        assert!(
            report
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.code() == "MONAD-RI-0200")
        );
        assert!(
            report
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.code() == "MONAD-RI-0201")
        );
        assert!(report.info_count() >= 1);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn policy_report_counts_by_severity() {
        let root = create_policy_workspace("severity-counts");

        let report = evaluate_from_workspace(&root);

        assert!(report.diagnostic_count() >= 4);
        assert!(report.info_count() >= 1);
        assert!(report.advisory_count() >= 2);
        assert!(report.warning_count() >= 1);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn policy_report_is_empty_when_no_diagnostics_are_supplied() {
        let report = RepositoryPolicyReport::new(Vec::new());

        assert_eq!(report.diagnostic_count(), 0);
        assert_eq!(report.info_count(), 0);
        assert_eq!(report.advisory_count(), 0);
        assert_eq!(report.warning_count(), 0);
        assert!(!report.has_warnings());
    }
}
