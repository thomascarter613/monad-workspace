//! Non-mutating environment and repository diagnostics.
//!
//! E15 introduces `monad doctor`, a read-only command that checks local
//! readiness without installing tools, editing configuration, running package
//! manager installs, uploading telemetry, or mutating the repository.

use std::collections::BTreeSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Severity for one doctor check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DoctorSeverity {
    /// Required readiness check passed.
    Pass,

    /// Non-blocking issue or optional tool missing while repository signals need.
    Warn,

    /// Required readiness check failed.
    Fail,

    /// Informational state.
    Info,

    /// Optional check was skipped because it was not required by current state.
    Skipped,
}

impl DoctorSeverity {
    /// Stable user-facing label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Warn => "warn",
            Self::Fail => "fail",
            Self::Info => "info",
            Self::Skipped => "skipped",
        }
    }

    /// Whether this severity should be treated as actionable.
    #[must_use]
    pub const fn is_actionable(self) -> bool {
        matches!(self, Self::Warn | Self::Fail)
    }
}

/// Diagnostic category for one doctor check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DoctorCategory {
    /// Operating environment and process context.
    Environment,

    /// Required core tooling.
    CoreTooling,

    /// Optional ecosystem tools.
    EcosystemTooling,

    /// Git/repository readiness.
    Repository,

    /// Monad manifest/state/context readiness.
    MonadContext,

    /// Sync/repository-contract evidence readiness.
    RepositoryContract,
}

impl DoctorCategory {
    /// Stable user-facing label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Environment => "environment",
            Self::CoreTooling => "core-tooling",
            Self::EcosystemTooling => "ecosystem-tooling",
            Self::Repository => "repository",
            Self::MonadContext => "monad-context",
            Self::RepositoryContract => "repository-contract",
        }
    }
}

/// One doctor diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorCheck {
    id: String,
    category: DoctorCategory,
    severity: DoctorSeverity,
    subject: String,
    message: String,
    remediation: Option<String>,
}

impl DoctorCheck {
    /// Creates a doctor check.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        category: DoctorCategory,
        severity: DoctorSeverity,
        subject: impl Into<String>,
        message: impl Into<String>,
        remediation: Option<String>,
    ) -> Self {
        Self {
            id: id.into(),
            category,
            severity,
            subject: subject.into(),
            message: message.into(),
            remediation,
        }
    }

    /// Stable check ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Check category.
    #[must_use]
    pub const fn category(&self) -> DoctorCategory {
        self.category
    }

    /// Check severity.
    #[must_use]
    pub const fn severity(&self) -> DoctorSeverity {
        self.severity
    }

    /// Check subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Check message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Optional remediation hint.
    #[must_use]
    pub fn remediation(&self) -> Option<&str> {
        self.remediation.as_deref()
    }
}

/// Doctor report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorReport {
    checks: Vec<DoctorCheck>,
}

impl DoctorReport {
    /// Creates a deterministic doctor report.
    #[must_use]
    pub fn new(mut checks: Vec<DoctorCheck>) -> Self {
        checks.sort_by(|left, right| {
            left.category()
                .cmp(&right.category())
                .then(left.severity().cmp(&right.severity()))
                .then(left.id().cmp(right.id()))
        });

        Self { checks }
    }

    /// All checks.
    #[must_use]
    pub fn checks(&self) -> &[DoctorCheck] {
        &self.checks
    }

    /// Total check count.
    #[must_use]
    pub fn check_count(&self) -> usize {
        self.checks.len()
    }

    /// Number of pass checks.
    #[must_use]
    pub fn pass_count(&self) -> usize {
        self.count_by_severity(DoctorSeverity::Pass)
    }

    /// Number of warnings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.count_by_severity(DoctorSeverity::Warn)
    }

    /// Number of failures.
    #[must_use]
    pub fn failure_count(&self) -> usize {
        self.count_by_severity(DoctorSeverity::Fail)
    }

    /// Number of skipped checks.
    #[must_use]
    pub fn skipped_count(&self) -> usize {
        self.count_by_severity(DoctorSeverity::Skipped)
    }

    /// Whether the report contains failures.
    #[must_use]
    pub fn has_failures(&self) -> bool {
        self.failure_count() > 0
    }

    fn count_by_severity(&self, severity: DoctorSeverity) -> usize {
        self.checks
            .iter()
            .filter(|check| check.severity() == severity)
            .count()
    }
}

/// Runs doctor against the current directory.
#[must_use]
pub fn run_doctor() -> DoctorReport {
    let root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    run_doctor_for_root(root)
}

/// Runs doctor against a specific root path.
#[must_use]
pub fn run_doctor_for_root(root: impl AsRef<Path>) -> DoctorReport {
    let root = root.as_ref();
    let needs = RepositoryNeeds::detect(root);
    let mut checks = Vec::new();

    add_environment_checks(root, &mut checks);
    add_core_tool_checks(root, &mut checks);
    add_git_repository_checks(root, &mut checks);
    add_ecosystem_tool_checks(&needs, &mut checks);
    add_monad_context_checks(root, &mut checks);
    add_repository_contract_checks(root, &mut checks);

    DoctorReport::new(checks)
}

fn add_environment_checks(root: &Path, checks: &mut Vec<DoctorCheck>) {
    checks.push(DoctorCheck::new(
        "doctor.environment.current-directory",
        DoctorCategory::Environment,
        DoctorSeverity::Info,
        "current directory",
        format!("Doctor inspected `{}`.", root.display()),
        None,
    ));

    if env::var_os("PATH").is_some() {
        checks.push(DoctorCheck::new(
            "doctor.environment.path-present",
            DoctorCategory::Environment,
            DoctorSeverity::Pass,
            "PATH",
            "PATH is available for local command discovery.",
            None,
        ));
    } else {
        checks.push(DoctorCheck::new(
            "doctor.environment.path-missing",
            DoctorCategory::Environment,
            DoctorSeverity::Fail,
            "PATH",
            "PATH is not available; command discovery cannot run.",
            Some("Restore PATH in the shell before running Monad commands.".to_string()),
        ));
    }
}

fn add_core_tool_checks(root: &Path, checks: &mut Vec<DoctorCheck>) {
    add_tool_check(
        checks,
        ToolSpec::required(
            "git",
            &["--version"],
            DoctorCategory::CoreTooling,
            "Install Git and ensure it is available on PATH.",
        ),
        true,
    );
    add_tool_check(
        checks,
        ToolSpec::required(
            "rustc",
            &["--version"],
            DoctorCategory::CoreTooling,
            "Install Rust with rustup or ensure rustc is available on PATH.",
        ),
        true,
    );
    add_tool_check(
        checks,
        ToolSpec::required(
            "cargo",
            &["--version"],
            DoctorCategory::CoreTooling,
            "Install Rust/Cargo with rustup or ensure cargo is available on PATH.",
        ),
        true,
    );

    let cargo_manifest = root.join("Cargo.toml");
    if cargo_manifest.is_file() {
        checks.push(DoctorCheck::new(
            "doctor.repository.cargo-manifest-present",
            DoctorCategory::Repository,
            DoctorSeverity::Pass,
            "Cargo.toml",
            "Root Cargo.toml is present.",
            None,
        ));
    }
}

fn add_git_repository_checks(root: &Path, checks: &mut Vec<DoctorCheck>) {
    if !command_exists("git") {
        checks.push(DoctorCheck::new(
            "doctor.repository.git-skipped",
            DoctorCategory::Repository,
            DoctorSeverity::Skipped,
            "git repository",
            "Git repository checks were skipped because git is not available.",
            Some("Install Git and rerun `monad doctor`.".to_string()),
        ));
        return;
    }

    let inside_repo = command_output(
        "git",
        &[
            "-C",
            &root.display().to_string(),
            "rev-parse",
            "--is-inside-work-tree",
        ],
    )
    .map(|output| output.trim() == "true")
    .unwrap_or(false);

    if inside_repo {
        checks.push(DoctorCheck::new(
            "doctor.repository.git-inside-work-tree",
            DoctorCategory::Repository,
            DoctorSeverity::Pass,
            "git repository",
            "Current directory is inside a Git work tree.",
            None,
        ));

        match command_output(
            "git",
            &["-C", &root.display().to_string(), "status", "--porcelain"],
        ) {
            Ok(output) if output.trim().is_empty() => checks.push(DoctorCheck::new(
                "doctor.repository.git-clean",
                DoctorCategory::Repository,
                DoctorSeverity::Pass,
                "git status",
                "Git working tree appears clean.",
                None,
            )),
            Ok(_) => checks.push(DoctorCheck::new(
                "doctor.repository.git-dirty",
                DoctorCategory::Repository,
                DoctorSeverity::Warn,
                "git status",
                "Git working tree has uncommitted changes.",
                Some(
                    "Review `git status --short` before applying further generated changes."
                        .to_string(),
                ),
            )),
            Err(error) => checks.push(DoctorCheck::new(
                "doctor.repository.git-status-unavailable",
                DoctorCategory::Repository,
                DoctorSeverity::Warn,
                "git status",
                format!("Git status could not be read: {error}"),
                Some("Run `git status --short` manually.".to_string()),
            )),
        }
    } else {
        checks.push(DoctorCheck::new(
            "doctor.repository.git-not-work-tree",
            DoctorCategory::Repository,
            DoctorSeverity::Warn,
            "git repository",
            "Current directory is not inside a Git work tree.",
            Some("Initialize Git or run doctor from the repository root.".to_string()),
        ));
    }
}

fn add_ecosystem_tool_checks(needs: &RepositoryNeeds, checks: &mut Vec<DoctorCheck>) {
    add_tool_check(
        checks,
        ToolSpec::optional(
            "node",
            &["--version"],
            "Install Node.js if this repository uses Node or TypeScript.",
        ),
        needs.node,
    );
    add_tool_check(
        checks,
        ToolSpec::optional(
            "bun",
            &["--version"],
            "Install Bun if this repository uses Bun.",
        ),
        false,
    );
    add_tool_check(
        checks,
        ToolSpec::optional(
            "npm",
            &["--version"],
            "Install npm if this repository uses npm.",
        ),
        needs.node,
    );
    add_tool_check(
        checks,
        ToolSpec::optional(
            "pnpm",
            &["--version"],
            "Install pnpm if this repository uses pnpm workspaces.",
        ),
        false,
    );
    add_tool_check(
        checks,
        ToolSpec::optional(
            "yarn",
            &["--version"],
            "Install Yarn if this repository uses Yarn workspaces.",
        ),
        false,
    );
    add_tool_check(
        checks,
        ToolSpec::optional(
            "python3",
            &["--version"],
            "Install Python 3 if this repository uses Python components.",
        ),
        needs.python,
    );
    add_tool_check(
        checks,
        ToolSpec::optional(
            "go",
            &["version"],
            "Install Go if this repository uses Go components.",
        ),
        needs.go,
    );
    add_tool_check(
        checks,
        ToolSpec::optional(
            "java",
            &["-version"],
            "Install Java if this repository uses JVM components.",
        ),
        needs.java,
    );
}

fn add_monad_context_checks(root: &Path, checks: &mut Vec<DoctorCheck>) {
    let monad_toml = root.join("monad.toml");
    if monad_toml.is_file() {
        match fs::read_to_string(&monad_toml) {
            Ok(content) if content.trim().is_empty() => checks.push(DoctorCheck::new(
                "doctor.monad.manifest-empty",
                DoctorCategory::MonadContext,
                DoctorSeverity::Fail,
                "monad.toml",
                "monad.toml exists but is empty.",
                Some("Regenerate or repair monad.toml.".to_string()),
            )),
            Ok(content) => {
                let severity =
                    if content.contains("[project]") || content.contains("schema_version") {
                        DoctorSeverity::Pass
                    } else {
                        DoctorSeverity::Warn
                    };
                let message = if severity == DoctorSeverity::Pass {
                    "monad.toml is present and contains recognizable Monad manifest structure."
                } else {
                    "monad.toml is present, but expected manifest markers were not detected."
                };

                checks.push(DoctorCheck::new(
                    "doctor.monad.manifest-readable",
                    DoctorCategory::MonadContext,
                    severity,
                    "monad.toml",
                    message,
                    Some(
                        "Run `monad sync --dry-run` to inspect repository-contract drift."
                            .to_string(),
                    ),
                ));
            }
            Err(error) => checks.push(DoctorCheck::new(
                "doctor.monad.manifest-unreadable",
                DoctorCategory::MonadContext,
                DoctorSeverity::Fail,
                "monad.toml",
                format!("monad.toml exists but could not be read: {error}"),
                Some("Check file permissions and encoding.".to_string()),
            )),
        }
    } else {
        checks.push(DoctorCheck::new(
            "doctor.monad.manifest-missing",
            DoctorCategory::MonadContext,
            DoctorSeverity::Warn,
            "monad.toml",
            "monad.toml is missing.",
            Some("Run `monad init --yes` when ready to initialize a Monad workspace.".to_string()),
        ));
    }

    add_path_presence_check(
        checks,
        root,
        PathPresenceCheckSpec {
            relative_path: ".monad",
            present_id: "doctor.monad.state-present",
            missing_id: "doctor.monad.state-missing",
            category: DoctorCategory::MonadContext,
            subject: "Monad state directory",
            remediation: "Run `monad init --yes` to create the Monad state directory.",
        },
    );
    add_path_presence_check(
        checks,
        root,
        PathPresenceCheckSpec {
            relative_path: ".monad/context",
            present_id: "doctor.monad.context-present",
            missing_id: "doctor.monad.context-missing",
            category: DoctorCategory::MonadContext,
            subject: "Monad context directory",
            remediation: "Run `monad context pack` or the current context-generation workflow when ready.",
        },
    );
    add_path_presence_check(
        checks,
        root,
        PathPresenceCheckSpec {
            relative_path: "docs/ai/BOOTSTRAP-PROMPT.md",
            present_id: "doctor.monad.bootstrap-present",
            missing_id: "doctor.monad.bootstrap-missing",
            category: DoctorCategory::MonadContext,
            subject: "AI bootstrap prompt",
            remediation: "Run the context bootstrap generation workflow when ready.",
        },
    );
}

fn add_repository_contract_checks(root: &Path, checks: &mut Vec<DoctorCheck>) {
    add_path_presence_check(
        checks,
        root,
        PathPresenceCheckSpec {
            relative_path: ".monad/reports/sync-report.md",
            present_id: "doctor.contract.sync-report-present",
            missing_id: "doctor.contract.sync-report-missing",
            category: DoctorCategory::RepositoryContract,
            subject: "sync evidence report",
            remediation: "Run `monad sync --yes` after reviewing `monad sync --dry-run`.",
        },
    );
    add_path_presence_check(
        checks,
        root,
        PathPresenceCheckSpec {
            relative_path: ".monad/reports/sync-report.json",
            present_id: "doctor.contract.sync-json-present",
            missing_id: "doctor.contract.sync-json-missing",
            category: DoctorCategory::RepositoryContract,
            subject: "sync JSON evidence",
            remediation: "Run `monad sync --yes` after reviewing `monad sync --dry-run`.",
        },
    );
}

struct PathPresenceCheckSpec<'a> {
    relative_path: &'a str,
    present_id: &'a str,
    missing_id: &'a str,
    category: DoctorCategory,
    subject: &'a str,
    remediation: &'a str,
}

fn add_path_presence_check(
    checks: &mut Vec<DoctorCheck>,
    root: &Path,
    spec: PathPresenceCheckSpec<'_>,
) {
    let path = root.join(spec.relative_path);

    if path.exists() {
        checks.push(DoctorCheck::new(
            spec.present_id,
            spec.category,
            DoctorSeverity::Pass,
            spec.subject,
            format!("`{}` is present.", spec.relative_path),
            None,
        ));
    } else {
        checks.push(DoctorCheck::new(
            spec.missing_id,
            spec.category,
            DoctorSeverity::Warn,
            spec.subject,
            format!("`{}` is missing.", spec.relative_path),
            Some(spec.remediation.to_string()),
        ));
    }
}

fn add_tool_check(checks: &mut Vec<DoctorCheck>, spec: ToolSpec<'_>, required_by_repo: bool) {
    if command_exists(spec.command) {
        let version = command_output(spec.command, spec.version_args)
            .ok()
            .and_then(|output| output.lines().next().map(str::to_string))
            .unwrap_or_else(|| "version unavailable".to_string());

        checks.push(DoctorCheck::new(
            format!("doctor.tool.{}.available", spec.command),
            spec.category,
            DoctorSeverity::Pass,
            spec.command,
            format!("`{}` is available: {version}", spec.command),
            None,
        ));
        return;
    }

    if spec.required || required_by_repo {
        let severity = if spec.required {
            DoctorSeverity::Fail
        } else {
            DoctorSeverity::Warn
        };

        checks.push(DoctorCheck::new(
            format!("doctor.tool.{}.missing", spec.command),
            spec.category,
            severity,
            spec.command,
            format!("`{}` was not found on PATH.", spec.command),
            Some(spec.remediation.to_string()),
        ));
    } else {
        checks.push(DoctorCheck::new(
            format!("doctor.tool.{}.skipped", spec.command),
            spec.category,
            DoctorSeverity::Skipped,
            spec.command,
            format!(
                "`{}` was not found on PATH and is not required by detected repo state.",
                spec.command
            ),
            Some(spec.remediation.to_string()),
        ));
    }
}

struct ToolSpec<'a> {
    command: &'a str,
    version_args: &'a [&'a str],
    category: DoctorCategory,
    required: bool,
    remediation: &'a str,
}

impl<'a> ToolSpec<'a> {
    fn required(
        command: &'a str,
        version_args: &'a [&'a str],
        category: DoctorCategory,
        remediation: &'a str,
    ) -> Self {
        Self {
            command,
            version_args,
            category,
            required: true,
            remediation,
        }
    }

    fn optional(command: &'a str, version_args: &'a [&'a str], remediation: &'a str) -> Self {
        Self {
            command,
            version_args,
            category: DoctorCategory::EcosystemTooling,
            required: false,
            remediation,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct RepositoryNeeds {
    node: bool,
    python: bool,
    go: bool,
    java: bool,
}

impl RepositoryNeeds {
    fn detect(root: &Path) -> Self {
        let mut needs = Self::default();

        for relative_path in collect_relevant_files(root, 4, 2_000) {
            let Some(file_name) = relative_path.file_name().and_then(OsStr::to_str) else {
                continue;
            };

            match file_name {
                "package.json" | "tsconfig.json" | "bun.lockb" | "pnpm-lock.yaml" | "yarn.lock" => {
                    needs.node = true
                }
                "pyproject.toml" | "requirements.txt" => needs.python = true,
                "go.mod" | "go.work" => needs.go = true,
                "pom.xml" | "build.gradle" | "build.gradle.kts" => needs.java = true,
                _ => {}
            }
        }

        needs
    }
}

fn collect_relevant_files(root: &Path, max_depth: usize, max_files: usize) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![(root.to_path_buf(), PathBuf::new(), 0_usize)];
    let ignored_dirs = BTreeSet::from([
        ".git",
        "target",
        "node_modules",
        ".venv",
        "venv",
        ".monad/script-backups",
    ]);

    while let Some((absolute, relative, depth)) = stack.pop() {
        if files.len() >= max_files || depth > max_depth {
            break;
        }

        let Ok(entries) = fs::read_dir(&absolute) else {
            continue;
        };

        for entry in entries.flatten() {
            let entry_name = entry.file_name();
            let entry_name_string = entry_name.to_string_lossy().to_string();
            let child_relative = relative.join(&entry_name);
            let child_absolute = entry.path();

            if child_absolute.is_dir() {
                if ignored_dirs.contains(entry_name_string.as_str()) {
                    continue;
                }

                stack.push((child_absolute, child_relative, depth + 1));
            } else if child_absolute.is_file() {
                files.push(child_relative);
            }

            if files.len() >= max_files {
                break;
            }
        }
    }

    files.sort();
    files
}

fn command_exists(command: &str) -> bool {
    let Some(path) = env::var_os("PATH") else {
        return false;
    };

    env::split_paths(&path).any(|directory| directory.join(command).is_file())
}

fn command_output(command: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|error| format!("failed to run `{command}`: {error}"))?;

    let bytes = if output.stdout.is_empty() {
        output.stderr
    } else {
        output.stdout
    };

    Ok(String::from_utf8_lossy(&bytes).trim().to_string())
}

/// Renders a human-readable doctor report.
#[must_use]
pub fn render_doctor_report(report: &DoctorReport) -> String {
    let mut lines = vec![
        "Monad doctor report".to_string(),
        String::new(),
        "Summary:".to_string(),
        format!("  checks: {}", report.check_count()),
        format!("  passed: {}", report.pass_count()),
        format!("  warnings: {}", report.warning_count()),
        format!("  failures: {}", report.failure_count()),
        format!("  skipped: {}", report.skipped_count()),
        String::new(),
        "Checks:".to_string(),
    ];

    for check in report.checks() {
        lines.push(format!(
            "  - [{}] {} {}",
            check.severity().as_str(),
            check.category().as_str(),
            check.subject()
        ));
        lines.push(format!("    id: {}", check.id()));
        lines.push(format!("    message: {}", check.message()));
        if let Some(remediation) = check.remediation() {
            lines.push(format!("    remediation: {remediation}"));
        }
    }

    lines.push(String::new());
    lines.push("No tools were installed.".to_string());
    lines.push("No environment files were modified.".to_string());
    lines.push("No package managers were run.".to_string());
    lines.push("No telemetry was uploaded.".to_string());

    lines.join("\n")
}

/// Renders a JSON doctor report without adding a JSON dependency.
#[must_use]
pub fn render_doctor_report_json(report: &DoctorReport) -> String {
    let checks = report
        .checks()
        .iter()
        .map(|check| {
            let remediation = check
                .remediation()
                .map(|value| format!("\"{}\"", json_escape(value)))
                .unwrap_or_else(|| "null".to_string());

            format!(
                "{{\"id\":\"{}\",\"category\":\"{}\",\"severity\":\"{}\",\"subject\":\"{}\",\"message\":\"{}\",\"remediation\":{}}}",
                json_escape(check.id()),
                check.category().as_str(),
                check.severity().as_str(),
                json_escape(check.subject()),
                json_escape(check.message()),
                remediation
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"command\":\"doctor\",\"checks\":{},\"passed\":{},\"warnings\":{},\"failures\":{},\"skipped\":{},\"items\":[{}]}}",
        report.check_count(),
        report.pass_count(),
        report.warning_count(),
        report.failure_count(),
        report.skipped_count(),
        checks
    )
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    fn unique_temp_root(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);

        env::temp_dir().join(format!("monad-doctor-{name}-{unique}"))
    }

    #[test]
    fn severity_labels_are_stable() {
        assert_eq!(DoctorSeverity::Pass.as_str(), "pass");
        assert_eq!(DoctorSeverity::Warn.as_str(), "warn");
        assert_eq!(DoctorSeverity::Fail.as_str(), "fail");
        assert_eq!(DoctorSeverity::Info.as_str(), "info");
        assert_eq!(DoctorSeverity::Skipped.as_str(), "skipped");
    }

    #[test]
    fn doctor_report_counts_severities() {
        let report = DoctorReport::new(vec![
            DoctorCheck::new(
                "pass",
                DoctorCategory::Environment,
                DoctorSeverity::Pass,
                "pass",
                "passed",
                None,
            ),
            DoctorCheck::new(
                "warn",
                DoctorCategory::Environment,
                DoctorSeverity::Warn,
                "warn",
                "warned",
                None,
            ),
            DoctorCheck::new(
                "fail",
                DoctorCategory::Environment,
                DoctorSeverity::Fail,
                "fail",
                "failed",
                None,
            ),
        ]);

        assert_eq!(report.check_count(), 3);
        assert_eq!(report.pass_count(), 1);
        assert_eq!(report.warning_count(), 1);
        assert_eq!(report.failure_count(), 1);
        assert!(report.has_failures());
    }

    #[test]
    fn doctor_detects_missing_monad_manifest() {
        let root = unique_temp_root("missing-manifest");
        fs::create_dir_all(&root).expect("temp root should be created");

        let report = run_doctor_for_root(&root);

        assert!(
            report
                .checks()
                .iter()
                .any(|check| check.id() == "doctor.monad.manifest-missing")
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn doctor_detects_readable_monad_manifest() {
        let root = unique_temp_root("readable-manifest");
        fs::create_dir_all(&root).expect("temp root should be created");
        fs::write(root.join("monad.toml"), "[project]\nname = \"example\"\n")
            .expect("manifest should be written");

        let report = run_doctor_for_root(&root);

        assert!(
            report
                .checks()
                .iter()
                .any(|check| check.id() == "doctor.monad.manifest-readable")
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn doctor_text_report_states_non_mutation_contract() {
        let report = DoctorReport::new(vec![DoctorCheck::new(
            "info",
            DoctorCategory::Environment,
            DoctorSeverity::Info,
            "info",
            "message",
            None,
        )]);
        let output = render_doctor_report(&report);

        assert!(output.contains("Monad doctor report"));
        assert!(output.contains("No tools were installed."));
        assert!(output.contains("No environment files were modified."));
        assert!(output.contains("No package managers were run."));
    }

    #[test]
    fn doctor_json_report_contains_core_fields() {
        let report = DoctorReport::new(vec![DoctorCheck::new(
            "info",
            DoctorCategory::Environment,
            DoctorSeverity::Info,
            "info",
            "message",
            None,
        )]);
        let output = render_doctor_report_json(&report);

        assert!(output.contains("\"command\":\"doctor\""));
        assert!(output.contains("\"checks\":1"));
        assert!(output.contains("\"items\""));
    }
}
