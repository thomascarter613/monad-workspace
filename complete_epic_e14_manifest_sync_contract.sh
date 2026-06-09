#!/usr/bin/env bash
set -euo pipefail

# Epic E14 — Manifest Sync and Repository Contract Foundation
#
# This epic-level script implements the MVP-safe `monad sync` foundation:
#
#   monad sync --dry-run
#   monad sync --dry-run --format=json
#   monad sync --yes
#
# Scope:
# - Define and document the sync contract.
# - Add a deterministic repository contract / drift model.
# - Add `monad sync --dry-run` CLI output.
# - Add guarded generated metadata/evidence writes with `--yes`.
# - Add native manifest reconciliation checks.
# - Add sync evidence reports and smoke tests.
#
# Safety boundaries:
# - No destructive overwrites.
# - No package-manager rewrites.
# - No dependency installation.
# - No lockfile generation.
# - No package publishing.
# - No cross-repo or cloud sync.
# - No autonomous agent-driven changes.
# - Writes are limited to generated `.monad/reports/sync-report.*` evidence files.

echo "==> Epic E14: Manifest Sync and Repository Contract Foundation"

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

cd "$REPO_ROOT"

echo "==> Repo root: $REPO_ROOT"

CORE_SYNC_FILE="crates/monad-core/src/sync.rs"
LIB_FILE="crates/monad-core/src/lib.rs"
CLI_FILE="crates/monad-cli/src/main.rs"

for required in "$LIB_FILE" "$CLI_FILE"; do
  if [ ! -f "$required" ]; then
    echo "ERROR: expected file not found: $required" >&2
    echo "Run this from the Monad repository root." >&2
    exit 1
  fi
done

mkdir -p \
  crates/monad-core/src \
  docs/commands \
  docs/architecture \
  docs/workflows \
  docs/verification \
  tools/scripts \
  work/learning/E14 \
  work/deliverables/E14 \
  .monad/script-backups/E14/EPIC-E14

BACKUP_STAMP="$(date +%Y%m%d%H%M%S)"
[ -f "$CORE_SYNC_FILE" ] && cp "$CORE_SYNC_FILE" ".monad/script-backups/E14/EPIC-E14/sync.rs.$BACKUP_STAMP.bak"
cp "$LIB_FILE" ".monad/script-backups/E14/EPIC-E14/lib.rs.$BACKUP_STAMP.bak"
cp "$CLI_FILE" ".monad/script-backups/E14/EPIC-E14/main.rs.$BACKUP_STAMP.bak"

echo "==> Backups written under .monad/script-backups/E14/EPIC-E14"

cat > "$CORE_SYNC_FILE" <<'EOF'
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
        matches!(self, Self::Missing | Self::Extra | Self::Stale | Self::Unsupported)
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
            format!("Discovered native component manifest `{}`.", component_manifest.display()),
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
        if entry_path.is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                directories.push(name.to_string());
            }
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
    lines.push("Native manifests and user source files will not be rewritten by this command.".to_string());

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
        fs::write(root.join("services/api/Cargo.toml"), "[package]\nname = \"api\"\n")
            .map_err(|error| {
                MonadError::internal(format!("test service Cargo.toml should be written: {error}"))
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

        let report = fs::read_to_string(root.join(".monad/reports/sync-report.md")).map_err(|error| {
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

        assert!(error.to_string().contains(".monad/reports exists but is not a directory"));

        fs::remove_dir_all(root).ok();

        Ok(())
    }
}
EOF

python3 <<'PY'
from pathlib import Path
import re

LIB = Path("crates/monad-core/src/lib.rs")
CLI = Path("crates/monad-cli/src/main.rs")


def replace_once(text: str, old: str, new: str, label: str) -> str:
    if old not in text:
        raise SystemExit(f"ERROR: could not find expected block for {label}")
    return text.replace(old, new, 1)


# ---------------------------------------------------------------------------
# Patch lib.rs.
# ---------------------------------------------------------------------------
lib = LIB.read_text()

if "pub mod sync;" not in lib:
    if "pub mod templates;" in lib:
        lib = lib.replace("pub mod templates;", "pub mod sync;\npub mod templates;", 1)
    else:
        raise SystemExit("ERROR: could not find module insertion point in lib.rs")

if "pub use sync::" not in lib:
    export = '''pub use sync::{
    SyncApplyResult, SyncFinding, SyncFindingKind, SyncFindingSeverity, SyncPlan, apply_sync_plan,
    build_sync_plan, render_sync_apply_result, render_sync_evidence_markdown, render_sync_plan,
    render_sync_plan_json,
};
'''
    marker = "pub use templates::{"
    if marker not in lib:
        raise SystemExit("ERROR: could not find sync export insertion point in lib.rs")
    lib = lib.replace(marker, export + marker, 1)

LIB.write_text(lib)


# ---------------------------------------------------------------------------
# Patch CLI main.rs.
# ---------------------------------------------------------------------------
cli = CLI.read_text()

if "build_sync_plan" not in cli.split("use monad_core::", 1)[-1].split("};", 1)[0]:
    # Insert sync functions near existing render/init/check imports.
    cli = cli.replace(
        "run_monad_workspace_checks, traverse_workspace_bounded, verify_context,",
        "run_monad_workspace_checks, apply_sync_plan, build_sync_plan, render_sync_apply_result,\n    render_sync_plan, render_sync_plan_json, traverse_workspace_bounded, verify_context,",
        1,
    )

if "Sync {" not in cli.split("enum CliCommand", 1)[-1].split("/// Render or export AI-readable", 1)[0]:
    marker = '''    /// Run workspace checks.
    Check {
        /// Requested output format.
        output_format: OutputFormat,
    }

'''
    insertion = '''    /// Synchronize declared repository intent with discovered state.
    Sync {
        /// Whether to run in dry-run mode.
        dry_run: bool,

        /// Whether to write approved generated sync evidence.
        yes: bool,

        /// Requested output format.
        output_format: OutputFormat,
    }

'''
    if marker not in cli:
        raise SystemExit("ERROR: could not find CliCommand Sync insertion point")
    cli = cli.replace(marker, marker + insertion, 1)

# Permit --yes for sync.
cli = cli.replace(
    'if yes && parts.first().copied() != Some("init") && parts.first().copied() != Some("add") {',
    'if yes\n            && parts.first().copied() != Some("init")\n            && parts.first().copied() != Some("add")\n            && parts.first().copied() != Some("sync")\n        {',
)

cli = cli.replace(
    '"--yes is only supported for init and add commands"',
    '"--yes is only supported for init, add, and sync commands"',
)

if '["sync"] => {' not in cli:
    marker = '''            ["check"] => {
                reject_write_for_non_context(write)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::Check { output_format })
            }
'''
    insertion = '''            ["sync"] => {
                reject_write_for_non_context(write)?;
                require_sync_mode(dry_run, yes)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::Sync {
                    dry_run,
                    yes,
                    output_format,
                })
            }
'''
    if marker not in cli:
        raise SystemExit("ERROR: could not find sync parse insertion point")
    cli = cli.replace(marker, marker + insertion, 1)

if "CliCommand::Sync" not in cli.split("match command", 1)[-1].split("CliCommand::Context", 1)[0]:
    marker = '''        CliCommand::Check { output_format } => render_check(output_format),
'''
    insertion = '''        CliCommand::Sync {
            dry_run,
            yes,
            output_format,
        } => render_sync(dry_run, yes, output_format),
'''
    if marker not in cli:
        raise SystemExit("ERROR: could not find sync run insertion point")
    cli = cli.replace(marker, marker + insertion, 1)

if "fn require_sync_mode" not in cli:
    marker = '''/// Requires exactly one init mode for the guarded init implementation.
fn require_init_mode(dry_run: bool, yes: bool) -> Result<(), String> {
'''
    insertion = '''/// Requires exactly one sync mode for the guarded sync implementation.
fn require_sync_mode(dry_run: bool, yes: bool) -> Result<(), String> {
    match (dry_run, yes) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => {
            Err("sync currently requires either --dry-run to preview or --yes to write generated evidence".to_string())
        }
        (true, true) => Err("sync accepts either --dry-run or --yes, not both".to_string()),
    }
}

'''
    if marker not in cli:
        raise SystemExit("ERROR: could not find require_sync_mode insertion point")
    cli = cli.replace(marker, insertion + marker, 1)

if "fn render_sync(" not in cli:
    marker = '''/// Renders workspace checks.
fn render_check(output_format: OutputFormat) -> Result<String, String> {
'''
    insertion = '''/// Renders or applies repository sync output.
fn render_sync(dry_run: bool, yes: bool, output_format: OutputFormat) -> Result<String, String> {
    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;

    if dry_run {
        let plan = build_sync_plan(&context).map_err(|error| error.to_string())?;
        return match output_format {
            OutputFormat::Text => Ok(render_sync_plan(&plan)),
            OutputFormat::Json => Ok(render_sync_plan_json(&plan)),
        };
    }

    if yes {
        let result = apply_sync_plan(&context).map_err(|error| error.to_string())?;
        return Ok(render_sync_apply_result(&result));
    }

    Err("sync currently requires either --dry-run to preview or --yes to write generated evidence".to_string())
}

'''
    if marker not in cli:
        raise SystemExit("ERROR: could not find render_sync insertion point")
    cli = cli.replace(marker, insertion + marker, 1)

# Help text additions.
if "sync --dry-run" not in cli.split("fn help_text()", 1)[-1]:
    cli = cli.replace(
        '        "  check                                     Run workspace checks",\n',
        '        "  check                                     Run workspace checks",\n        "  sync --dry-run                            Preview repository sync plan",\n        "  sync --dry-run --format=json              Preview repository sync plan as JSON",\n        "  sync --yes                                Write generated sync evidence reports",\n',
        1,
    )
    cli = cli.replace(
        '        "  monad check --format=json",\n',
        '        "  monad check --format=json",\n        "  monad sync --dry-run",\n        "  monad sync --dry-run --format=json",\n',
        1,
    )
    cli = cli.replace(
        '        "  --write is only supported for the context command.",\n',
        '        "  sync writes generated evidence reports only.",\n        "  --write is only supported for the context command.",\n',
        1,
    )

# CLI tests.
if "fn sync_dry_run_command_parses" not in cli:
    tests = '''    #[test]
    fn sync_dry_run_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "sync", "--dry-run"]).expect("sync dry-run should parse"),
            CliCommand::Sync {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            }
        );
    }

    #[test]
    fn sync_dry_run_json_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "sync", "--dry-run", "--format=json"])
                .expect("sync dry-run json should parse"),
            CliCommand::Sync {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Json,
            }
        );
    }

    #[test]
    fn sync_yes_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "sync", "--yes"]).expect("sync yes should parse"),
            CliCommand::Sync {
                dry_run: false,
                yes: true,
                output_format: OutputFormat::Text,
            }
        );
    }

    #[test]
    fn sync_requires_dry_run_or_yes() {
        let error = parse_arguments(&["monad", "sync"]).expect_err("sync without mode should fail");

        assert!(error.contains("sync currently requires either --dry-run"));
        assert!(error.contains("--yes"));
    }

    #[test]
    fn sync_rejects_dry_run_and_yes_together() {
        let error = parse_arguments(&["monad", "sync", "--dry-run", "--yes"])
            .expect_err("sync should reject conflicting modes");

        assert!(error.contains("either --dry-run or --yes"));
        assert!(error.contains("not both"));
    }

'''
    marker = "    #[test]\n    fn info_command_parses_text_and_json_formats()"
    if marker not in cli:
        raise SystemExit("ERROR: could not find CLI test insertion point")
    cli = cli.replace(marker, tests + marker, 1)

CLI.write_text(cli)
PY

echo "==> Writing sync command docs"
cat > docs/commands/SYNC.md <<'EOF'
---
title: monad sync
status: complete
epic: E14
---

# `monad sync`

`monad sync` compares Monad's declared repository intent with discovered repository state and produces a reviewable synchronization plan.

## Commands

```bash
monad sync --dry-run
monad sync --dry-run --format=json
monad sync --yes
```

## Safety contract

`monad sync` is MVP-safe and non-destructive.

It does not:

- rewrite native manifests such as `Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`, or `go.work`;
- install dependencies;
- generate lockfiles;
- run package managers;
- run language toolchains;
- overwrite user-owned source files;
- publish packages;
- synchronize with cloud services;
- perform autonomous agent-driven changes.

## Dry-run behavior

```bash
monad sync --dry-run
```

Dry-run:

- discovers repository state;
- checks core Monad paths such as `monad.toml` and `.monad/`;
- checks component family directories;
- discovers first-level components under `apps/`, `packages/`, `services/`, and `tools/`;
- discovers supported component-native manifests;
- reports mismatches and unsupported automatic changes;
- writes no files.

## JSON dry-run

```bash
monad sync --dry-run --format=json
```

JSON output is intended for future dashboards, automation, and AI-readable evidence.

## Guarded writes

```bash
monad sync --yes
```

The guarded write path writes generated evidence only:

```text
.monad/reports/sync-report.md
.monad/reports/sync-report.json
```

It does not rewrite source files or native manifests.
EOF

cat > docs/architecture/REPOSITORY-CONTRACT.md <<'EOF'
---
title: Repository Contract
status: complete
epic: E14
---

# Repository Contract

The repository contract is Monad's bounded understanding of what a healthy repository should look like.

E14 establishes the first MVP-safe version of that contract.

## Contract sources

The initial contract uses:

- `monad.toml` as declared Monad intent;
- `.monad/` as Monad operational state;
- component family directories:
  - `apps/`
  - `packages/`
  - `services/`
  - `tools/`
- supported native manifests discovered in components:
  - `Cargo.toml`
  - `package.json`
  - `pyproject.toml`
  - `go.mod`

## Finding severities

| Severity | Meaning |
| --- | --- |
| `match` | Expected state exists. |
| `missing` | Expected state is absent. |
| `extra` | State exists but is outside the first sync contract. |
| `stale` | State exists but appears incomplete or inconsistent. |
| `unsupported` | State is recognized but not automatically rewritten by sync. |

## Non-destructive rule

The contract may report drift, but sync does not silently fix user-owned files.

E14 only permits generated evidence writes under:

```text
.monad/reports/
```

Native manifest rewriting is intentionally deferred.
EOF

cat > docs/workflows/SYNC-WORKFLOW.md <<'EOF'
---
title: Sync Workflow
status: complete
epic: E14
---

# Sync Workflow

Use sync to understand repository drift before taking action.

## 1. Preview

```bash
monad sync --dry-run
```

## 2. Review

Read the findings:

- matches show expected state;
- missing items show expected state that was not found;
- stale items show incomplete or inconsistent state;
- unsupported items show recognized state that Monad will not rewrite automatically.

## 3. Write evidence

```bash
monad sync --yes
```

This writes generated reports only:

```text
.monad/reports/sync-report.md
.monad/reports/sync-report.json
```

## 4. Fix manually or with later approved commands

E14 does not rewrite native manifests. Future epics may add narrowly approved reconciliation commands.
EOF

cat > docs/verification/SYNC-SMOKE-TESTS.md <<'EOF'
---
title: Sync Smoke Tests
status: complete
epic: E14
---

# Sync Smoke Tests

Run:

```bash
tools/scripts/verify-sync.sh
```

This verifies:

- `monad sync --dry-run` writes no sync evidence;
- `monad sync --dry-run --format=json` renders JSON;
- `monad sync --yes` writes only generated sync evidence reports;
- duplicate or unsafe source rewrites are not part of sync;
- missing command mode fails safely.

Full E14 verification:

```bash
tools/scripts/verify-e14.sh
```
EOF

cat > docs/verification/E14-CLOSEOUT.md <<'EOF'
---
title: E14 Closeout
status: complete
epic: E14
---

# E14 Closeout — Manifest Sync and Repository Contract Foundation

E14 is complete when:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-sync.sh
tools/scripts/verify-e14.sh
```

## Completed capability

`monad sync` now supports:

```bash
monad sync --dry-run
monad sync --dry-run --format=json
monad sync --yes
```

## Safety retained

Sync does not:

- rewrite native manifests;
- rewrite user source files;
- install dependencies;
- generate lockfiles;
- run package managers;
- publish packages;
- call cloud services.

## Generated evidence

Approved sync writes are limited to:

```text
.monad/reports/sync-report.md
.monad/reports/sync-report.json
```

## Next epic

E15 — Doctor and Environment Diagnostics Foundation.
EOF

echo "==> Writing verification scripts"
cat > tools/scripts/verify-sync.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

echo "==> verify-sync: repo root: $REPO_ROOT"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

(
  cd "$tmpdir"

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- init --yes >/tmp/monad-e14-init.out

  echo "==> verify sync dry-run"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync --dry-run >/tmp/monad-e14-sync-dry.out
  grep -q "Monad sync dry-run plan" /tmp/monad-e14-sync-dry.out
  grep -q "No files were written." /tmp/monad-e14-sync-dry.out
  test ! -e .monad/reports/sync-report.md
  test ! -e .monad/reports/sync-report.json

  echo "==> verify sync json dry-run"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync --dry-run --format=json >/tmp/monad-e14-sync-json.out
  grep -q '"command":"sync"' /tmp/monad-e14-sync-json.out
  grep -q '"mode":"dry-run"' /tmp/monad-e14-sync-json.out
  grep -q '"writes_enabled":false' /tmp/monad-e14-sync-json.out

  echo "==> verify sync generated evidence writes"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync --yes >/tmp/monad-e14-sync-yes.out
  grep -q "Monad sync evidence written" /tmp/monad-e14-sync-yes.out
  grep -q "No native manifests were rewritten." /tmp/monad-e14-sync-yes.out
  test -f .monad/reports/sync-report.md
  test -f .monad/reports/sync-report.json
  grep -q "Monad Sync Evidence Report" .monad/reports/sync-report.md
  grep -q "Native manifest rewrites: none" .monad/reports/sync-report.md
  grep -q '"command":"sync"' .monad/reports/sync-report.json

  echo "==> verify sync mode guard"
  if cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync >/tmp/monad-e14-sync-no-mode.out 2>&1; then
    echo "Expected sync without mode to fail" >&2
    exit 1
  fi
  grep -q "sync currently requires either --dry-run" /tmp/monad-e14-sync-no-mode.out

  echo "==> verify conflicting mode guard"
  if cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- sync --dry-run --yes >/tmp/monad-e14-sync-conflict-mode.out 2>&1; then
    echo "Expected sync with conflicting modes to fail" >&2
    exit 1
  fi
  grep -q "sync accepts either --dry-run or --yes" /tmp/monad-e14-sync-conflict-mode.out
)

echo "verify-sync: PASS"
EOF
chmod +x tools/scripts/verify-sync.sh

cat > tools/scripts/verify-e14.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

echo "==> E14 verification"

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-sync.sh

if [ -x tools/scripts/verify.sh ]; then
  tools/scripts/verify.sh
fi

echo "verify-e14: PASS"
EOF
chmod +x tools/scripts/verify-e14.sh

echo "==> Writing learning and deliverable records"
cat > work/learning/E14/EPIC-E14-manifest-sync-repository-contract.md <<'EOF'
---
title: Epic E14 Learning Note
epic: E14
---

# Epic E14 Learning Note: Manifest Sync and Repository Contract Foundation

E14 adds the first safe form of repository synchronization.

The important concept is that sync does not mean "rewrite everything."

In Monad, sync means:

1. discover declared repository intent;
2. discover actual repository state;
3. compare the two;
4. produce deterministic findings;
5. write generated evidence only when approved.

## Why it is safe

`monad sync --dry-run` writes nothing.

`monad sync --yes` writes only:

```text
.monad/reports/sync-report.md
.monad/reports/sync-report.json
```

It does not rewrite native manifests or source files.

## What to inspect

```bash
git diff -- crates/monad-core/src/sync.rs
git diff -- crates/monad-core/src/lib.rs
git diff -- crates/monad-cli/src/main.rs
git diff -- docs/commands/SYNC.md
git diff -- docs/architecture/REPOSITORY-CONTRACT.md
```

## Why this prepares E15

E15 is doctor diagnostics. Doctor can reuse E14's contract concepts to report whether the repo is healthy and ready.
EOF

cat > work/deliverables/E14/EPIC-E14-manifest-sync-repository-contract.md <<'EOF'
---
title: Epic E14 Deliverable Record
epic: E14
status: complete
---

# Epic E14 Deliverable Record

## Epic

E14 — Manifest Sync and Repository Contract Foundation.

## Completed work packets

- WP-E14-001 — Define `monad sync` contract and repo intent model
- WP-E14-002 — Add repository contract diff model
- WP-E14-003 — Add `monad sync --dry-run` plan output
- WP-E14-004 — Add non-destructive manifest/context sync writes
- WP-E14-005 — Add native manifest reconciliation checks
- WP-E14-006 — Add sync evidence reports and smoke tests

## Implementation files

```text
crates/monad-core/src/sync.rs
crates/monad-core/src/lib.rs
crates/monad-cli/src/main.rs
```

## Documentation files

```text
docs/commands/SYNC.md
docs/architecture/REPOSITORY-CONTRACT.md
docs/workflows/SYNC-WORKFLOW.md
docs/verification/SYNC-SMOKE-TESTS.md
docs/verification/E14-CLOSEOUT.md
```

## Verification files

```text
tools/scripts/verify-sync.sh
tools/scripts/verify-e14.sh
```

## Verification command

```bash
tools/scripts/verify-e14.sh
```

## Next epic

E15 — Doctor and Environment Diagnostics Foundation.
EOF

echo "==> Formatting Rust code"
cargo fmt

echo
echo "==> Epic E14 patch complete."
echo
echo "Recommended inspection:"
echo "  git diff -- crates/monad-core/src/sync.rs"
echo "  git diff -- crates/monad-core/src/lib.rs"
echo "  git diff -- crates/monad-cli/src/main.rs"
echo "  git diff -- docs/commands/SYNC.md"
echo "  git diff -- docs/architecture/REPOSITORY-CONTRACT.md"
echo "  git diff -- tools/scripts/verify-sync.sh"
echo "  git diff -- tools/scripts/verify-e14.sh"
echo
echo "Recommended verification:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-sync.sh"
echo "  tools/scripts/verify-e14.sh"
echo
echo "Commit:"
echo "  git add crates/monad-core/src/sync.rs crates/monad-core/src/lib.rs crates/monad-cli/src/main.rs docs/commands/SYNC.md docs/architecture/REPOSITORY-CONTRACT.md docs/workflows/SYNC-WORKFLOW.md docs/verification/SYNC-SMOKE-TESTS.md docs/verification/E14-CLOSEOUT.md tools/scripts/verify-sync.sh tools/scripts/verify-e14.sh work/learning/E14/EPIC-E14-manifest-sync-repository-contract.md work/deliverables/E14/EPIC-E14-manifest-sync-repository-contract.md"
echo "  git commit -m \"feat(sync): add repository contract sync foundation\""
echo
echo "Done."
