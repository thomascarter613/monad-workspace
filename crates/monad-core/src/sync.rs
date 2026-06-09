//! Repository synchronization planning and evidence.
//!
//! E14 introduces an MVP-safe `monad sync` foundation:
//!
//! - compare declared repository intent against discovered repository state;
//! - produce deterministic dry-run plans;
//! - report drift against supported native manifests;
//! - write generated evidence reports only when explicitly approved;
//! - never rewrite user-owned source files or native manifests.

use std::fs;
use std::path::{Path, PathBuf};

use crate::{MonadError, MonadResult, WorkspaceContext};

/// Severity for a sync finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyncFindingSeverity {
    /// Expected state is present.
    Match,

    /// Expected state is absent.
    Missing,

    /// State exists but is outside the first sync contract.
    Extra,

    /// State exists but appears stale or incomplete.
    Stale,

    /// State is recognized but intentionally unsupported for automatic sync.
    Unsupported,
}

impl SyncFindingSeverity {
    /// Stable user-facing label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Match => "match",
            Self::Missing => "missing",
            Self::Extra => "extra",
            Self::Stale => "stale",
            Self::Unsupported => "unsupported",
        }
    }

    /// Whether this finding means sync should surface a warning.
    #[must_use]
    pub const fn is_warning(self) -> bool {
        matches!(
            self,
            Self::Missing | Self::Extra | Self::Stale | Self::Unsupported
        )
    }
}

/// Kind of repository contract finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyncFindingKind {
    /// Monad manifest / declared repository intent.
    MonadManifest,

    /// Monad operational metadata.
    MonadState,

    /// Component directory convention.
    ComponentDirectory,

    /// Native ecosystem manifest.
    NativeManifest,

    /// Generated sync evidence.
    Evidence,
}

impl SyncFindingKind {
    /// Stable user-facing label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MonadManifest => "monad-manifest",
            Self::MonadState => "monad-state",
            Self::ComponentDirectory => "component-directory",
            Self::NativeManifest => "native-manifest",
            Self::Evidence => "evidence",
        }
    }
}

/// One deterministic sync finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncFinding {
    id: String,
    kind: SyncFindingKind,
    severity: SyncFindingSeverity,
    path: PathBuf,
    message: String,
    planned_action: String,
}

impl SyncFinding {
    /// Creates a sync finding.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        kind: SyncFindingKind,
        severity: SyncFindingSeverity,
        path: impl Into<PathBuf>,
        message: impl Into<String>,
        planned_action: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            severity,
            path: path.into(),
            message: message.into(),
            planned_action: planned_action.into(),
        }
    }

    /// Stable finding ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Finding kind.
    #[must_use]
    pub const fn kind(&self) -> SyncFindingKind {
        self.kind
    }

    /// Finding severity.
    #[must_use]
    pub const fn severity(&self) -> SyncFindingSeverity {
        self.severity
    }

    /// Repository-relative path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Human-readable finding message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Human-readable planned action.
    #[must_use]
    pub fn planned_action(&self) -> &str {
        &self.planned_action
    }
}

/// Deterministic repository sync plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncPlan {
    findings: Vec<SyncFinding>,
    generated_report_paths: Vec<PathBuf>,
}

impl SyncPlan {
    /// Creates a plan from findings.
    #[must_use]
    pub fn new(mut findings: Vec<SyncFinding>) -> Self {
        findings.sort_by(|left, right| {
            left.kind()
                .cmp(&right.kind())
                .then(left.severity().cmp(&right.severity()))
                .then(left.path().cmp(right.path()))
                .then(left.id().cmp(right.id()))
        });

        Self {
            findings,
            generated_report_paths: vec![
                PathBuf::from(".monad/reports/sync-report.md"),
                PathBuf::from(".monad/reports/sync-report.json"),
            ],
        }
    }

    /// Findings in deterministic order.
    #[must_use]
    pub fn findings(&self) -> &[SyncFinding] {
        &self.findings
    }

    /// Generated evidence paths that `monad sync --yes` may write.
    #[must_use]
    pub fn generated_report_paths(&self) -> &[PathBuf] {
        &self.generated_report_paths
    }

    /// Number of findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }

    /// Number of warnings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.severity().is_warning())
            .count()
    }

    /// Whether the plan has warnings.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.warning_count() > 0
    }
}

/// Result of approved generated sync evidence writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncApplyResult {
    written_files: Vec<PathBuf>,
    bytes_written: usize,
    warning_count: usize,
}

impl SyncApplyResult {
    /// Creates an apply result.
    #[must_use]
    pub fn new(written_files: Vec<PathBuf>, bytes_written: usize, warning_count: usize) -> Self {
        Self {
            written_files,
            bytes_written,
            warning_count,
        }
    }

    /// Generated files written by sync.
    #[must_use]
    pub fn written_files(&self) -> &[PathBuf] {
        &self.written_files
    }

    /// Number of generated files written.
    #[must_use]
    pub fn file_count(&self) -> usize {
        self.written_files.len()
    }

    /// Total bytes written.
    #[must_use]
    pub const fn bytes_written(&self) -> usize {
        self.bytes_written
    }

    /// Warning count from the source sync plan.
    #[must_use]
    pub const fn warning_count(&self) -> usize {
        self.warning_count
    }
}

/// Builds the current repository sync plan.
pub fn build_sync_plan(context: &WorkspaceContext) -> MonadResult<SyncPlan> {
    let root = context.root();
    let mut findings = Vec::new();

    add_core_contract_findings(root, &mut findings);
    add_component_directory_findings(root, &mut findings)?;
    add_native_manifest_findings(root, &mut findings)?;
    add_evidence_findings(&mut findings);

    Ok(SyncPlan::new(findings))
}

fn add_core_contract_findings(root: &Path, findings: &mut Vec<SyncFinding>) {
    let manifest_path = root.join("monad.toml");

    if manifest_path.is_file() {
        findings.push(SyncFinding::new(
            "monad.manifest.present",
            SyncFindingKind::MonadManifest,
            SyncFindingSeverity::Match,
            "monad.toml",
            "Monad manifest is present.",
            "No manifest write is planned.",
        ));
    } else {
        findings.push(SyncFinding::new(
            "monad.manifest.missing",
            SyncFindingKind::MonadManifest,
            SyncFindingSeverity::Missing,
            "monad.toml",
            "Monad manifest is missing.",
            "Run `monad init --yes` before relying on sync.",
        ));
    }

    let state_path = root.join(".monad");

    if state_path.is_dir() {
        findings.push(SyncFinding::new(
            "monad.state.present",
            SyncFindingKind::MonadState,
            SyncFindingSeverity::Match,
            ".monad",
            "Monad state directory is present.",
            "No state directory write is planned.",
        ));
    } else {
        findings.push(SyncFinding::new(
            "monad.state.missing",
            SyncFindingKind::MonadState,
            SyncFindingSeverity::Missing,
            ".monad",
            "Monad state directory is missing.",
            "Run `monad init --yes` before relying on sync.",
        ));
    }
}

fn add_component_directory_findings(
    root: &Path,
    findings: &mut Vec<SyncFinding>,
) -> MonadResult<()> {
    let families = [
        ("apps", "app"),
        ("packages", "package"),
        ("services", "service"),
        ("tools", "tool"),
    ];

    for (directory, kind) in families {
        let family_path = root.join(directory);

        if !family_path.exists() {
            findings.push(SyncFinding::new(
                format!("components.{directory}.missing"),
                SyncFindingKind::ComponentDirectory,
                SyncFindingSeverity::Missing,
                directory,
                format!("Component family directory `{directory}` is missing."),
                "No directory write is planned by sync; use `monad add` for components.",
            ));
            continue;
        }

        if !family_path.is_dir() {
            findings.push(SyncFinding::new(
                format!("components.{directory}.stale"),
                SyncFindingKind::ComponentDirectory,
                SyncFindingSeverity::Stale,
                directory,
                format!("Component family path `{directory}` exists but is not a directory."),
                "Resolve the path manually; sync will not overwrite it.",
            ));
            continue;
        }

        findings.push(SyncFinding::new(
            format!("components.{directory}.present"),
            SyncFindingKind::ComponentDirectory,
            SyncFindingSeverity::Match,
            directory,
            format!("Component family directory `{directory}` is present."),
            "No component family write is planned.",
        ));

        for component in first_level_directories(&family_path)? {
            let component_path = Path::new(directory).join(&component);
            findings.push(SyncFinding::new(
                format!("components.{directory}.{component}.present"),
                SyncFindingKind::ComponentDirectory,
                SyncFindingSeverity::Match,
                component_path.clone(),
                format!("Discovered {kind} component `{component}`."),
                "Record as discovered state; no source write is planned.",
            ));
        }
    }

    Ok(())
}

fn add_native_manifest_findings(root: &Path, findings: &mut Vec<SyncFinding>) -> MonadResult<()> {
    let cargo_toml = root.join("Cargo.toml");
    if cargo_toml.is_file() {
        let text = fs::read_to_string(&cargo_toml).unwrap_or_default();
        if text.contains("[workspace]") {
            findings.push(SyncFinding::new(
                "native.cargo.workspace.present",
                SyncFindingKind::NativeManifest,
                SyncFindingSeverity::Match,
                "Cargo.toml",
                "Root Cargo workspace manifest is present.",
                "No Cargo workspace rewrite is planned.",
            ));
        } else {
            findings.push(SyncFinding::new(
                "native.cargo.workspace.not-detected",
                SyncFindingKind::NativeManifest,
                SyncFindingSeverity::Stale,
                "Cargo.toml",
                "Root Cargo.toml exists but `[workspace]` was not detected.",
                "Report only; sync does not rewrite Cargo.toml.",
            ));
        }
    }

    let package_json = root.join("package.json");
    if package_json.is_file() {
        let text = fs::read_to_string(&package_json).unwrap_or_default();
        if text.contains("\"workspaces\"") {
            findings.push(SyncFinding::new(
                "native.node.workspaces.present",
                SyncFindingKind::NativeManifest,
                SyncFindingSeverity::Match,
                "package.json",
                "Root package.json workspace configuration was detected.",
                "No package.json rewrite is planned.",
            ));
        } else {
            findings.push(SyncFinding::new(
                "native.node.workspaces.not-detected",
                SyncFindingKind::NativeManifest,
                SyncFindingSeverity::Unsupported,
                "package.json",
                "Root package.json exists but workspace configuration was not detected.",
                "Report only; sync does not rewrite package.json.",
            ));
        }
    }

    if root.join("go.work").is_file() {
        findings.push(SyncFinding::new(
            "native.go.work.present",
            SyncFindingKind::NativeManifest,
            SyncFindingSeverity::Match,
            "go.work",
            "Root go.work file is present.",
            "No go.work rewrite is planned.",
        ));
    }

    for component_manifest in discover_component_manifests(root)? {
        let id_path = component_manifest.display().to_string().replace('/', ".");
        findings.push(SyncFinding::new(
            format!("native.component-manifest.{id_path}"),
            SyncFindingKind::NativeManifest,
            SyncFindingSeverity::Match,
            component_manifest.clone(),
            format!(
                "Discovered native component manifest `{}`.",
                component_manifest.display()
            ),
            "Record as discovered state; no native manifest rewrite is planned.",
        ));
    }

    Ok(())
}

fn add_evidence_findings(findings: &mut Vec<SyncFinding>) {
    findings.push(SyncFinding::new(
        "sync.evidence.markdown",
        SyncFindingKind::Evidence,
        SyncFindingSeverity::Match,
        ".monad/reports/sync-report.md",
        "Approved generated Markdown sync evidence path is defined.",
        "`monad sync --yes` may write this generated evidence file.",
    ));

    findings.push(SyncFinding::new(
        "sync.evidence.json",
        SyncFindingKind::Evidence,
        SyncFindingSeverity::Match,
        ".monad/reports/sync-report.json",
        "Approved generated JSON sync evidence path is defined.",
        "`monad sync --yes` may write this generated evidence file.",
    ));
}

fn first_level_directories(path: &Path) -> MonadResult<Vec<String>> {
    let mut directories = Vec::new();

    for entry in fs::read_dir(path).map_err(|error| {
        MonadError::internal(format!(
            "failed to read component family directory `{}`: {error}",
            path.display()
        ))
    })? {
        let entry = entry.map_err(|error| {
            MonadError::internal(format!(
                "failed to read component family entry under `{}`: {error}",
                path.display()
            ))
        })?;

        let entry_path = entry.path();
        if entry_path.is_dir()
            && let Some(name) = entry.file_name().to_str()
        {
            directories.push(name.to_string());
        }
    }

    directories.sort();
    Ok(directories)
}

fn discover_component_manifests(root: &Path) -> MonadResult<Vec<PathBuf>> {
    let mut manifests = Vec::new();
    let families = ["apps", "packages", "services", "tools"];
    let names = ["Cargo.toml", "package.json", "pyproject.toml", "go.mod"];

    for family in families {
        let family_path = root.join(family);
        if !family_path.is_dir() {
            continue;
        }

        for component in first_level_directories(&family_path)? {
            for manifest_name in names {
                let relative = Path::new(family).join(&component).join(manifest_name);
                if root.join(&relative).is_file() {
                    manifests.push(relative);
                }
            }
        }
    }

    manifests.sort();
    Ok(manifests)
}

/// Renders a human-readable sync dry-run plan.
#[must_use]
pub fn render_sync_plan(plan: &SyncPlan) -> String {
    let mut lines = vec![
        "Monad sync dry-run plan".to_string(),
        String::new(),
        "Summary:".to_string(),
        format!("  findings: {}", plan.finding_count()),
        format!("  warnings: {}", plan.warning_count()),
        "  writes: disabled".to_string(),
        "  apply: guarded by --yes".to_string(),
        String::new(),
        "Approved generated write targets:".to_string(),
    ];

    for path in plan.generated_report_paths() {
        lines.push(format!("  - {}", path.display()));
    }

    lines.push(String::new());
    lines.push("Findings:".to_string());

    for finding in plan.findings() {
        lines.push(format!(
            "  - [{}] {} {}",
            finding.severity().as_str(),
            finding.kind().as_str(),
            finding.path().display()
        ));
        lines.push(format!("    id: {}", finding.id()));
        lines.push(format!("    message: {}", finding.message()));
        lines.push(format!("    planned_action: {}", finding.planned_action()));
    }

    lines.push(String::new());
    lines.push("No files were written.".to_string());
    lines.push(
        "Native manifests and user source files will not be rewritten by this command.".to_string(),
    );

    lines.join("\n")
}

/// Renders sync plan JSON without introducing a new dependency surface.
#[must_use]
pub fn render_sync_plan_json(plan: &SyncPlan) -> String {
    let findings = plan
        .findings()
        .iter()
        .map(|finding| {
            format!(
                "{{\"id\":\"{}\",\"kind\":\"{}\",\"severity\":\"{}\",\"path\":\"{}\",\"message\":\"{}\",\"planned_action\":\"{}\"}}",
                json_escape(finding.id()),
                finding.kind().as_str(),
                finding.severity().as_str(),
                json_escape(&finding.path().display().to_string()),
                json_escape(finding.message()),
                json_escape(finding.planned_action())
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let write_targets = plan
        .generated_report_paths()
        .iter()
        .map(|path| format!("\"{}\"", json_escape(&path.display().to_string())))
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"command\":\"sync\",\"mode\":\"dry-run\",\"findings\":{},\"warnings\":{},\"writes_enabled\":false,\"approved_generated_write_targets\":[{}],\"items\":[{}]}}",
        plan.finding_count(),
        plan.warning_count(),
        write_targets,
        findings
    )
}

/// Applies approved generated sync evidence writes.
pub fn apply_sync_plan(context: &WorkspaceContext) -> MonadResult<SyncApplyResult> {
    let plan = build_sync_plan(context)?;
    let reports_dir = context.root().join(".monad/reports");

    if reports_dir.exists() && !reports_dir.is_dir() {
        return Err(MonadError::invalid_input(
            ".monad/reports exists but is not a directory; refusing to write sync evidence",
        ));
    }

    fs::create_dir_all(&reports_dir).map_err(|error| {
        MonadError::internal(format!(
            "failed to create sync evidence directory `{}`: {error}",
            reports_dir.display()
        ))
    })?;

    let markdown_path = context.root().join(".monad/reports/sync-report.md");
    let json_path = context.root().join(".monad/reports/sync-report.json");

    let markdown = render_sync_evidence_markdown(&plan);
    let json = render_sync_plan_json(&plan);

    fs::write(&markdown_path, &markdown).map_err(|error| {
        MonadError::internal(format!(
            "failed to write sync evidence `{}`: {error}",
            markdown_path.display()
        ))
    })?;

    fs::write(&json_path, &json).map_err(|error| {
        MonadError::internal(format!(
            "failed to write sync evidence `{}`: {error}",
            json_path.display()
        ))
    })?;

    Ok(SyncApplyResult::new(
        vec![
            PathBuf::from(".monad/reports/sync-report.md"),
            PathBuf::from(".monad/reports/sync-report.json"),
        ],
        markdown.len() + json.len(),
        plan.warning_count(),
    ))
}

/// Renders sync apply result.
#[must_use]
pub fn render_sync_apply_result(result: &SyncApplyResult) -> String {
    let mut lines = vec![
        "Monad sync evidence written".to_string(),
        format!("  files_written: {}", result.file_count()),
        format!("  bytes_written: {}", result.bytes_written()),
        format!("  warnings: {}", result.warning_count()),
        "  written_files:".to_string(),
    ];

    for path in result.written_files() {
        lines.push(format!("    - {}", path.display()));
    }

    lines.push("No native manifests were rewritten.".to_string());
    lines.push("No user source files were rewritten.".to_string());
    lines.push("No package managers were run.".to_string());

    lines.join("\n")
}

/// Renders Markdown sync evidence.
#[must_use]
pub fn render_sync_evidence_markdown(plan: &SyncPlan) -> String {
    let mut lines = vec![
        "# Monad Sync Evidence Report".to_string(),
        String::new(),
        "## Summary".to_string(),
        String::new(),
        format!("- Findings: {}", plan.finding_count()),
        format!("- Warnings: {}", plan.warning_count()),
        "- Native manifest rewrites: none".to_string(),
        "- User source rewrites: none".to_string(),
        "- Package manager execution: none".to_string(),
        String::new(),
        "## Findings".to_string(),
        String::new(),
    ];

    for finding in plan.findings() {
        lines.push(format!(
            "### `{}` — `{}`",
            finding.id(),
            finding.severity().as_str()
        ));
        lines.push(String::new());
        lines.push(format!("- Kind: `{}`", finding.kind().as_str()));
        lines.push(format!("- Path: `{}`", finding.path().display()));
        lines.push(format!("- Message: {}", finding.message()));
        lines.push(format!("- Planned action: {}", finding.planned_action()));
        lines.push(String::new());
    }

    lines.join("\n")
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

        std::env::temp_dir().join(format!("monad-sync-{name}-{unique}"))
    }

    fn create_initialized_workspace(name: &str) -> MonadResult<(PathBuf, WorkspaceContext)> {
        let root = unique_temp_root(name);
        fs::create_dir_all(root.join(".monad")).map_err(|error| {
            MonadError::internal(format!("test .monad dir should be created: {error}"))
        })?;
        fs::write(root.join("monad.toml"), "[project]\nname = \"example\"\n").map_err(|error| {
            MonadError::internal(format!("test monad.toml should be written: {error}"))
        })?;
        WorkspaceContext::new(&root).map(|context| (root, context))
    }

    #[test]
    fn sync_plan_detects_initialized_workspace_contract() -> MonadResult<()> {
        let (root, context) = create_initialized_workspace("initialized")?;
        let plan = build_sync_plan(&context)?;

        assert!(
            plan.findings()
                .iter()
                .any(|finding| finding.id() == "monad.manifest.present")
        );
        assert!(
            plan.findings()
                .iter()
                .any(|finding| finding.id() == "monad.state.present")
        );

        fs::remove_dir_all(root).ok();

        Ok(())
    }

    #[test]
    fn sync_plan_detects_component_directories_and_manifests() -> MonadResult<()> {
        let (root, context) = create_initialized_workspace("components")?;
        fs::create_dir_all(root.join("services/api/src")).map_err(|error| {
            MonadError::internal(format!("test service dir should be created: {error}"))
        })?;
        fs::write(
            root.join("services/api/Cargo.toml"),
            "[package]\nname = \"api\"\n",
        )
        .map_err(|error| {
            MonadError::internal(format!(
                "test service Cargo.toml should be written: {error}"
            ))
        })?;

        let plan = build_sync_plan(&context)?;

        assert!(
            plan.findings()
                .iter()
                .any(|finding| finding.path() == Path::new("services/api"))
        );
        assert!(
            plan.findings()
                .iter()
                .any(|finding| finding.path() == Path::new("services/api/Cargo.toml"))
        );

        fs::remove_dir_all(root).ok();

        Ok(())
    }

    #[test]
    fn sync_plan_is_deterministic() -> MonadResult<()> {
        let (root, context) = create_initialized_workspace("deterministic")?;
        fs::create_dir_all(root.join("apps/web")).map_err(|error| {
            MonadError::internal(format!("test app dir should be created: {error}"))
        })?;
        fs::create_dir_all(root.join("tools/repo-lint")).map_err(|error| {
            MonadError::internal(format!("test tool dir should be created: {error}"))
        })?;

        let first = build_sync_plan(&context)?;
        let second = build_sync_plan(&context)?;

        let first_ids = first
            .findings()
            .iter()
            .map(SyncFinding::id)
            .collect::<Vec<_>>();
        let second_ids = second
            .findings()
            .iter()
            .map(SyncFinding::id)
            .collect::<Vec<_>>();

        assert_eq!(first_ids, second_ids);

        fs::remove_dir_all(root).ok();

        Ok(())
    }

    #[test]
    fn sync_dry_run_text_states_no_files_written() -> MonadResult<()> {
        let (root, context) = create_initialized_workspace("dry-run")?;
        let plan = build_sync_plan(&context)?;
        let output = render_sync_plan(&plan);

        assert!(output.contains("Monad sync dry-run plan"));
        assert!(output.contains("No files were written."));
        assert!(output.contains("Native manifests and user source files will not be rewritten"));

        fs::remove_dir_all(root).ok();

        Ok(())
    }

    #[test]
    fn sync_plan_json_contains_core_fields() -> MonadResult<()> {
        let (root, context) = create_initialized_workspace("json")?;
        let plan = build_sync_plan(&context)?;
        let output = render_sync_plan_json(&plan);

        assert!(output.contains("\"command\":\"sync\""));
        assert!(output.contains("\"mode\":\"dry-run\""));
        assert!(output.contains("\"writes_enabled\":false"));
        assert!(output.contains("\"items\""));

        fs::remove_dir_all(root).ok();

        Ok(())
    }

    #[test]
    fn sync_apply_writes_generated_evidence_only() -> MonadResult<()> {
        let (root, context) = create_initialized_workspace("apply")?;
        let result = apply_sync_plan(&context)?;

        assert_eq!(result.file_count(), 2);
        assert!(root.join(".monad/reports/sync-report.md").is_file());
        assert!(root.join(".monad/reports/sync-report.json").is_file());
        assert!(!root.join("Cargo.lock").exists());
        assert!(!root.join("package-lock.json").exists());

        let report =
            fs::read_to_string(root.join(".monad/reports/sync-report.md")).map_err(|error| {
                MonadError::internal(format!("test sync report should be readable: {error}"))
            })?;
        assert!(report.contains("Monad Sync Evidence Report"));
        assert!(report.contains("Native manifest rewrites: none"));

        fs::remove_dir_all(root).ok();

        Ok(())
    }

    #[test]
    fn sync_apply_refuses_when_reports_path_is_file() -> MonadResult<()> {
        let (root, context) = create_initialized_workspace("apply-conflict")?;
        fs::write(root.join(".monad/reports"), "not a directory").map_err(|error| {
            MonadError::internal(format!("test reports file should be written: {error}"))
        })?;

        let error = apply_sync_plan(&context).expect_err("reports file should block sync apply");

        assert!(
            error
                .to_string()
                .contains(".monad/reports exists but is not a directory")
        );

        fs::remove_dir_all(root).ok();

        Ok(())
    }
}
