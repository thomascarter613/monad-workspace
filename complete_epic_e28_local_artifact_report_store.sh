#!/usr/bin/env bash
set -euo pipefail

# Complete Epic E28 — Local Artifact and Report Store Foundation
#
# Safety:
# - local-first and deterministic
# - no network access
# - no upload/sync/delete of artifacts or reports
# - no package-manager invocation
# - no external command execution by Monad
# - no user-owned source rewrites by Monad runtime behavior
# - generated evidence writes only under .monad/reports through E19 approval gates
# - backs up touched files under .monad/script-backups/...

if [[ ! -f "Cargo.toml" || ! -d "crates/monad-core/src" || ! -d "crates/monad-cli/src" ]]; then
  echo "Run this script from the monad-workspace repository root." >&2
  exit 1
fi

BACKUP_DIR=".monad/script-backups/complete-epic-e28-local-artifact-report-store-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$BACKUP_DIR"

backup_if_exists() {
  local path="$1"
  if [[ -e "$path" ]]; then
    mkdir -p "$BACKUP_DIR/$(dirname "$path")"
    cp -a "$path" "$BACKUP_DIR/$path"
  fi
}

backup_if_exists "crates/monad-core/src/lib.rs"
backup_if_exists "crates/monad-cli/src/main.rs"
backup_if_exists "crates/monad-core/src/report_store.rs"
backup_if_exists "docs/report-store/README.md"
backup_if_exists "docs/roadmap/epic-28-local-artifact-report-store.md"
backup_if_exists "tools/scripts/verify-report-store.sh"
backup_if_exists "tools/scripts/verify-e28.sh"

mkdir -p docs/report-store docs/roadmap tools/scripts crates/monad-core/src

cat > crates/monad-core/src/report_store.rs <<'RS'
//! Local artifact and report store foundation.
//!
//! E28 defines Monad's local `.monad/reports` and `.monad/artifacts` contract,
//! report/artifact metadata schemas, deterministic indexing, and generated
//! evidence writes. This module never uploads, deletes, syncs, or rewrites
//! existing reports/artifacts. It only reads local metadata and writes generated
//! evidence when explicitly approved by the caller.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{GatedWriteRequest, GatedWriteResult, gated_generated_write};

/// Local store object kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum StoreObjectKind {
    /// A human or machine-readable report under `.monad/reports`.
    Report,

    /// A generated or captured artifact under `.monad/artifacts`.
    Artifact,
}

impl StoreObjectKind {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Report => "report",
            Self::Artifact => "artifact",
        }
    }
}

/// Retention class for local store objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum StoreRetentionClass {
    /// Keep until replaced by a newer generated record.
    Rolling,

    /// Keep as closeout or audit evidence.
    Evidence,

    /// Keep until human review decides whether to retain it.
    Review,
}

impl StoreRetentionClass {
    /// Stable retention label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rolling => "rolling",
            Self::Evidence => "evidence",
            Self::Review => "review",
        }
    }
}

/// Metadata for one local report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReportMetadata {
    id: String,
    path: PathBuf,
    title: String,
    retention: StoreRetentionClass,
    byte_len: u64,
}

impl ReportMetadata {
    /// Creates report metadata.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        path: impl Into<PathBuf>,
        title: impl Into<String>,
        retention: StoreRetentionClass,
        byte_len: u64,
    ) -> Self {
        Self {
            id: id.into(),
            path: path.into(),
            title: title.into(),
            retention,
            byte_len,
        }
    }

    /// Stable report ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Report path relative to repository root.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Report title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Retention class.
    #[must_use]
    pub const fn retention(&self) -> StoreRetentionClass {
        self.retention
    }

    /// Report size in bytes.
    #[must_use]
    pub const fn byte_len(&self) -> u64 {
        self.byte_len
    }
}

/// Metadata for one local artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArtifactMetadata {
    id: String,
    path: PathBuf,
    label: String,
    retention: StoreRetentionClass,
    byte_len: u64,
}

impl ArtifactMetadata {
    /// Creates artifact metadata.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        path: impl Into<PathBuf>,
        label: impl Into<String>,
        retention: StoreRetentionClass,
        byte_len: u64,
    ) -> Self {
        Self {
            id: id.into(),
            path: path.into(),
            label: label.into(),
            retention,
            byte_len,
        }
    }

    /// Stable artifact ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Artifact path relative to repository root.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Artifact label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Retention class.
    #[must_use]
    pub const fn retention(&self) -> StoreRetentionClass {
        self.retention
    }

    /// Artifact size in bytes.
    #[must_use]
    pub const fn byte_len(&self) -> u64 {
        self.byte_len
    }
}

/// Store contract for local reports and artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalStoreContract {
    reports_dir: PathBuf,
    artifacts_dir: PathBuf,
    index_path: PathBuf,
    retention_policy: Vec<String>,
}

impl LocalStoreContract {
    /// Creates the default local store contract.
    #[must_use]
    pub fn default_contract() -> Self {
        Self {
            reports_dir: PathBuf::from(".monad/reports"),
            artifacts_dir: PathBuf::from(".monad/artifacts"),
            index_path: PathBuf::from(".monad/reports/report-store-index.json"),
            retention_policy: vec![
                "Generated indexes are rolling evidence.".to_string(),
                "Epic closeout reports are evidence records.".to_string(),
                "Unknown artifacts are review records until classified.".to_string(),
                "Retention planning does not delete files automatically.".to_string(),
            ],
        }
    }

    /// Reports directory.
    #[must_use]
    pub fn reports_dir(&self) -> &Path {
        &self.reports_dir
    }

    /// Artifacts directory.
    #[must_use]
    pub fn artifacts_dir(&self) -> &Path {
        &self.artifacts_dir
    }

    /// Index path.
    #[must_use]
    pub fn index_path(&self) -> &Path {
        &self.index_path
    }

    /// Retention policy notes.
    #[must_use]
    pub fn retention_policy(&self) -> &[String] {
        &self.retention_policy
    }
}

/// Local report/artifact index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalStoreIndex {
    command: String,
    contract: LocalStoreContract,
    reports: Vec<ReportMetadata>,
    artifacts: Vec<ArtifactMetadata>,
    evidence_paths: Vec<PathBuf>,
    safety_notes: Vec<String>,
}

impl LocalStoreIndex {
    /// Creates a deterministic local store index.
    #[must_use]
    pub fn new(
        contract: LocalStoreContract,
        mut reports: Vec<ReportMetadata>,
        mut artifacts: Vec<ArtifactMetadata>,
    ) -> Self {
        reports.sort_by(|left, right| left.path().cmp(right.path()));
        reports.dedup_by(|left, right| left.path() == right.path());

        artifacts.sort_by(|left, right| left.path().cmp(right.path()));
        artifacts.dedup_by(|left, right| left.path() == right.path());

        Self {
            command: "report-store".to_string(),
            contract,
            reports,
            artifacts,
            evidence_paths: vec![
                PathBuf::from(".monad/reports/report-store-index.md"),
                PathBuf::from(".monad/reports/report-store-index.json"),
            ],
            safety_notes: vec![
                "No reports or artifacts are uploaded.".to_string(),
                "No reports or artifacts are deleted.".to_string(),
                "No existing report or artifact content is rewritten.".to_string(),
                "No remote store is contacted.".to_string(),
                "Generated store evidence is written only when --yes is used.".to_string(),
            ],
        }
    }

    /// Store contract.
    #[must_use]
    pub const fn contract(&self) -> &LocalStoreContract {
        &self.contract
    }

    /// Report metadata.
    #[must_use]
    pub fn reports(&self) -> &[ReportMetadata] {
        &self.reports
    }

    /// Artifact metadata.
    #[must_use]
    pub fn artifacts(&self) -> &[ArtifactMetadata] {
        &self.artifacts
    }

    /// Evidence output paths.
    #[must_use]
    pub fn evidence_paths(&self) -> &[PathBuf] {
        &self.evidence_paths
    }

    /// Safety notes.
    #[must_use]
    pub fn safety_notes(&self) -> &[String] {
        &self.safety_notes
    }
}

/// Apply result for generated report-store evidence writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalStoreApplyResult {
    write_results: Vec<GatedWriteResult>,
}

impl LocalStoreApplyResult {
    /// Creates an apply result.
    #[must_use]
    pub fn new(write_results: Vec<GatedWriteResult>) -> Self {
        Self { write_results }
    }

    /// Generated evidence write results.
    #[must_use]
    pub fn write_results(&self) -> &[GatedWriteResult] {
        &self.write_results
    }
}

/// Builds the local store index.
#[must_use]
pub fn build_local_store_index(root: impl AsRef<Path>) -> LocalStoreIndex {
    let root = root.as_ref();
    let contract = LocalStoreContract::default_contract();
    let reports = discover_reports(root, contract.reports_dir());
    let artifacts = discover_artifacts(root, contract.artifacts_dir());

    LocalStoreIndex::new(contract, reports, artifacts)
}

/// Discovers local reports without modifying them.
#[must_use]
pub fn discover_reports(root: &Path, reports_dir: &Path) -> Vec<ReportMetadata> {
    discover_files(root, reports_dir)
        .into_iter()
        .map(|path| {
            let absolute = root.join(&path);
            let byte_len = fs::metadata(&absolute).map_or(0, |metadata| metadata.len());
            let title = derive_report_title(&absolute, &path);
            let retention = classify_retention(&path);

            ReportMetadata::new(make_store_id(StoreObjectKind::Report, &path), path, title, retention, byte_len)
        })
        .collect()
}

/// Discovers local artifacts without modifying them.
#[must_use]
pub fn discover_artifacts(root: &Path, artifacts_dir: &Path) -> Vec<ArtifactMetadata> {
    discover_files(root, artifacts_dir)
        .into_iter()
        .map(|path| {
            let absolute = root.join(&path);
            let byte_len = fs::metadata(&absolute).map_or(0, |metadata| metadata.len());
            let label = path
                .file_name()
                .map_or_else(|| "artifact".to_string(), |name| name.to_string_lossy().to_string());
            let retention = classify_retention(&path);

            ArtifactMetadata::new(
                make_store_id(StoreObjectKind::Artifact, &path),
                path,
                label,
                retention,
                byte_len,
            )
        })
        .collect()
}

/// Writes generated report-store evidence reports.
pub fn write_local_store_evidence(root: impl AsRef<Path>) -> Result<LocalStoreApplyResult, String> {
    let root = root.as_ref();
    let index = build_local_store_index(root);
    let markdown = render_local_store_index(&index);
    let json = render_local_store_index_json(&index);

    let requests = [
        GatedWriteRequest::new(".monad/reports/report-store-index.md", markdown, true),
        GatedWriteRequest::new(".monad/reports/report-store-index.json", json, true),
    ];

    let write_results = requests
        .iter()
        .map(|request| gated_generated_write(root, request))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(LocalStoreApplyResult::new(write_results))
}

/// Renders a text local store index.
#[must_use]
pub fn render_local_store_index(index: &LocalStoreIndex) -> String {
    let mut lines = vec![
        "Monad local artifact and report store index".to_string(),
        String::new(),
        "Contract:".to_string(),
        format!("  reports_dir: {}", index.contract().reports_dir().display()),
        format!("  artifacts_dir: {}", index.contract().artifacts_dir().display()),
        format!("  index_path: {}", index.contract().index_path().display()),
        String::new(),
        "Retention policy:".to_string(),
    ];

    for note in index.contract().retention_policy() {
        lines.push(format!("  - {note}"));
    }

    lines.push(String::new());
    lines.push(format!("Reports: {}", index.reports().len()));
    if index.reports().is_empty() {
        lines.push("  - no reports discovered".to_string());
    } else {
        for report in index.reports() {
            lines.push(format!(
                "  - {} id={} retention={} bytes={} title={}",
                report.path().display(),
                report.id(),
                report.retention().as_str(),
                report.byte_len(),
                report.title()
            ));
        }
    }

    lines.push(String::new());
    lines.push(format!("Artifacts: {}", index.artifacts().len()));
    if index.artifacts().is_empty() {
        lines.push("  - no artifacts discovered".to_string());
    } else {
        for artifact in index.artifacts() {
            lines.push(format!(
                "  - {} id={} retention={} bytes={} label={}",
                artifact.path().display(),
                artifact.id(),
                artifact.retention().as_str(),
                artifact.byte_len(),
                artifact.label()
            ));
        }
    }

    lines.push(String::new());
    lines.push("Evidence outputs:".to_string());
    for path in index.evidence_paths() {
        lines.push(format!("  - {}", path.display()));
    }

    lines.push(String::new());
    lines.push("Safety notes:".to_string());
    for note in index.safety_notes() {
        lines.push(format!("  - {note}"));
    }

    lines.join("\n")
}

/// Renders a JSON local store index.
#[must_use]
pub fn render_local_store_index_json(index: &LocalStoreIndex) -> String {
    serde_json::to_string_pretty(index).unwrap_or_else(|_| {
        "{\n  \"command\": \"report-store\",\n  \"error\": \"local store index serialization failed\"\n}".to_string()
    })
}

/// Renders generated evidence write results.
#[must_use]
pub fn render_local_store_apply_result(result: &LocalStoreApplyResult) -> String {
    let mut lines = vec![
        "Monad local artifact/report store evidence write result".to_string(),
        String::new(),
        "Results:".to_string(),
    ];

    for write_result in result.write_results() {
        match write_result {
            GatedWriteResult::Written(path) | GatedWriteResult::SkippedIdentical(path) => {
                lines.push(format!("  - [{}] {}", write_result.as_str(), path.display()));
            }
            GatedWriteResult::ApprovalRequired(message) | GatedWriteResult::Blocked(message) => {
                lines.push(format!("  - [{}] {message}", write_result.as_str()));
            }
        }
    }

    lines.push(String::new());
    lines.push("No existing reports or artifacts were rewritten.".to_string());
    lines.push("No reports or artifacts were deleted.".to_string());
    lines.push("No remote store was contacted.".to_string());

    lines.join("\n")
}

fn discover_files(root: &Path, relative_dir: &Path) -> Vec<PathBuf> {
    let start = root.join(relative_dir);
    let mut files = Vec::new();

    collect_files(root, &start, &mut files);
    files.sort();
    files
}

fn collect_files(root: &Path, dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            collect_files(root, &path, files);
        } else if path.is_file() {
            files.push(path.strip_prefix(root).map_or(path.clone(), Path::to_path_buf));
        }
    }
}

fn derive_report_title(absolute_path: &Path, relative_path: &Path) -> String {
    if absolute_path.extension().and_then(|value| value.to_str()) == Some("md")
        && let Ok(text) = fs::read_to_string(absolute_path)
        && let Some(title) = text
            .lines()
            .map(str::trim)
            .find(|line| line.starts_with("# "))
    {
        return title.trim_start_matches("# ").trim().to_string();
    }

    relative_path
        .file_stem()
        .map_or_else(|| "report".to_string(), |stem| stem.to_string_lossy().to_string())
}

fn classify_retention(path: &Path) -> StoreRetentionClass {
    let normalized = path.to_string_lossy();

    if normalized.contains("closeout") || normalized.contains("epic-") {
        StoreRetentionClass::Evidence
    } else if normalized.ends_with(".tmp") || normalized.contains("scratch") {
        StoreRetentionClass::Review
    } else {
        StoreRetentionClass::Rolling
    }
}

fn make_store_id(kind: StoreObjectKind, path: &Path) -> String {
    let normalized = path.to_string_lossy().replace('\\', "/");
    let slug = normalized
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    format!("{}:{slug}", kind.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos());

        std::env::temp_dir().join(format!(
            "monad-report-store-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);
        assert!(fs::create_dir_all(root.join(".monad/reports")).is_ok());
        assert!(fs::create_dir_all(root.join(".monad/artifacts/logs")).is_ok());
        assert!(
            fs::write(root.join(".monad/reports/epic-closeout.md"), "# Epic Closeout\n\nDone.\n")
                .is_ok()
        );
        assert!(fs::write(root.join(".monad/artifacts/logs/run.log"), "ok\n").is_ok());
        root
    }

    #[test]
    fn default_contract_points_to_monad_store_paths() {
        let contract = LocalStoreContract::default_contract();

        assert_eq!(contract.reports_dir(), Path::new(".monad/reports"));
        assert_eq!(contract.artifacts_dir(), Path::new(".monad/artifacts"));
        assert_eq!(
            contract.index_path(),
            Path::new(".monad/reports/report-store-index.json")
        );
    }

    #[test]
    fn reports_are_discovered_with_titles() {
        let root = create_workspace("reports");
        let reports = discover_reports(&root, Path::new(".monad/reports"));

        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].title(), "Epic Closeout");
        assert_eq!(reports[0].retention(), StoreRetentionClass::Evidence);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn artifacts_are_discovered_with_labels() {
        let root = create_workspace("artifacts");
        let artifacts = discover_artifacts(&root, Path::new(".monad/artifacts"));

        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].label(), "run.log");
        assert_eq!(artifacts[0].retention(), StoreRetentionClass::Rolling);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn local_store_index_contains_reports_and_artifacts() {
        let root = create_workspace("index");
        let index = build_local_store_index(&root);

        assert_eq!(index.reports().len(), 1);
        assert_eq!(index.artifacts().len(), 1);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn text_render_mentions_safety_notes() {
        let root = create_workspace("text");
        let index = build_local_store_index(&root);
        let text = render_local_store_index(&index);

        assert!(text.contains("Monad local artifact and report store index"));
        assert!(text.contains("No reports or artifacts are deleted"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn json_render_contains_report_store_command() {
        let root = create_workspace("json");
        let index = build_local_store_index(&root);
        let json = render_local_store_index_json(&index);

        assert!(json.contains("\"command\": \"report-store\""));
        assert!(json.contains("epic-closeout.md"));

        fs::remove_dir_all(root).ok();
    }
}
RS

cat > docs/report-store/README.md <<'MD'
# Local Artifact and Report Store

E28 adds Monad's local artifact/report store foundation.

## What this foundation does

- Defines the `.monad/reports` and `.monad/artifacts` contract.
- Adds report metadata schema.
- Adds artifact metadata schema.
- Adds report writing and retention policy documentation.
- Adds a local report/artifact index foundation.
- Adds artifact/report store smoke tests.

## Command surface

```bash
monad report-store --dry-run
monad report-store --dry-run --format=json
monad report-store --yes
```

Aliases:

```bash
monad reports --dry-run
monad artifacts --dry-run
```

## Store paths

```text
.monad/reports
.monad/artifacts
.monad/reports/report-store-index.md
.monad/reports/report-store-index.json
```

## Safety boundaries

This foundation does **not**:

- upload reports or artifacts;
- delete reports or artifacts;
- rewrite existing report or artifact content;
- contact remote storage;
- call AI providers;
- invoke package managers.

`--yes` writes generated index evidence only under `.monad/reports`.
MD

cat > docs/roadmap/epic-28-local-artifact-report-store.md <<'MD'
# E28 — Local Artifact and Report Store Foundation

## Product Area

Local Artifact and Report Store Foundation

## Objective

Add Monad's local `.monad/reports` and `.monad/artifacts` store contract,
metadata schemas, retention policy, deterministic index foundation, and smoke
tests.

## Work Packets

- WP-E28-001 — Define `.monad/reports` and `.monad/artifacts` contract
- WP-E28-002 — Add report metadata schema
- WP-E28-003 — Add artifact metadata schema
- WP-E28-004 — Add report writing and retention policy
- WP-E28-005 — Add report index and lookup command foundation
- WP-E28-006 — Add artifact/report store smoke tests

## Delivered Behavior

- `crates/monad-core/src/report_store.rs`
- `monad report-store --dry-run`
- `monad report-store --dry-run --format=json`
- `monad report-store --yes`
- `monad reports --dry-run`
- `monad artifacts --dry-run`
- `tools/scripts/verify-report-store.sh`
- `tools/scripts/verify-e28.sh`

## Safety

E28 indexes local store metadata and writes generated index evidence only. It
does not upload, delete, sync, or rewrite existing report/artifact content.
MD

cat > tools/scripts/verify-report-store.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib report_store
cargo test -p monad-cli report_store

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- report-store --dry-run > "$text_output"
grep -q "Monad local artifact and report store index" "$text_output"
grep -q "Contract:" "$text_output"
grep -q "No reports or artifacts are deleted" "$text_output"

cargo run -p monad-cli -- report-store --dry-run --format=json > "$json_output"
grep -q '"command": "report-store"' "$json_output"
grep -q '.monad/reports' "$json_output"

echo "Report store verification passed."
SH
chmod +x tools/scripts/verify-report-store.sh

cat > tools/scripts/verify-e28.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-report-store.sh

test -f crates/monad-core/src/report_store.rs
test -f docs/report-store/README.md
test -f docs/roadmap/epic-28-local-artifact-report-store.md

grep -q "Local Artifact and Report Store" docs/report-store/README.md
grep -q "WP-E28-001" docs/roadmap/epic-28-local-artifact-report-store.md
grep -q "WP-E28-006" docs/roadmap/epic-28-local-artifact-report-store.md

echo "E28 verification passed."
SH
chmod +x tools/scripts/verify-e28.sh

python3 - <<'PY'
from pathlib import Path

lib = Path("crates/monad-core/src/lib.rs")
text = lib.read_text()

if "pub mod report_store;" not in text:
    anchors = [
        "pub mod build_cache;\n",
        "pub mod test_intelligence;\n",
        "pub mod dependency_impact;\n",
        "pub mod static_analysis;\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, anchor + "pub mod report_store;\n", 1)
            break
    else:
        raise SystemExit("Could not find lib.rs module insertion point for report_store.")

pub_use = '''pub use report_store::{
    ArtifactMetadata, LocalStoreApplyResult, LocalStoreContract, LocalStoreIndex, ReportMetadata,
    StoreObjectKind, StoreRetentionClass, build_local_store_index, discover_artifacts,
    discover_reports, render_local_store_apply_result, render_local_store_index,
    render_local_store_index_json, write_local_store_evidence,
};
'''
if "pub use report_store::" not in text:
    anchors = [
        "pub use build_cache::{\n",
        "pub use test_intelligence::{\n",
        "pub use dependency_impact::{\n",
        "pub use static_analysis::{\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, pub_use + anchor, 1)
            break
    else:
        raise SystemExit("Could not find lib.rs pub use insertion point for report_store.")

lib.write_text(text)

main = Path("crates/monad-cli/src/main.rs")
text = main.read_text()

variant = '''    /// Index the local report/artifact store without uploading or deleting objects.
    ReportStore {
        /// Whether to run in dry-run mode.
        dry_run: bool,

        /// Whether to write generated report-store evidence.
        yes: bool,

        /// Requested output format.
        output_format: OutputFormat,
    },

'''
if "ReportStore {" not in text:
    anchors = [
        "    /// Plan build-cache and incremental execution decisions without executing tasks.\n    CachePlan {\n",
        "    /// Plan targeted verification without executing test commands.\n    VerifyPlan {\n",
        "    /// Produce a supervised plan from a user intent.\n    Plan {\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, variant + anchor, 1)
            break
    else:
        raise SystemExit("Could not find CliCommand enum insertion point for ReportStore.")

if 'Some("report-store")' not in text:
    anchor = '            && parts.first().copied() != Some("sync")'
    if anchor in text:
        text = text.replace(
            anchor,
            anchor
            + '\n            && parts.first().copied() != Some("report-store")'
            + '\n            && parts.first().copied() != Some("reports")'
            + '\n            && parts.first().copied() != Some("artifacts")',
            1,
        )

parse_arm = '''            ["report-store"] | ["reports"] | ["artifacts"] => {
                reject_write_for_non_context(write)?;
                require_report_store_mode(dry_run, yes)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::ReportStore {
                    dry_run,
                    yes,
                    output_format,
                })
            }
            ["report-store", other, ..] | ["reports", other, ..] | ["artifacts", other, ..] => {
                reject_write_for_non_context(write)?;
                Err(format!("unknown report-store argument: {other}"))
            }
'''
if '["report-store"] | ["reports"] | ["artifacts"]' not in text:
    anchors = [
        '            ["cache-plan"] | ["build-cache"] | ["incremental-plan"] => {\n',
        '            ["verify-plan"] | ["test-intelligence"] | ["verification-plan"] => {\n',
        '            ["impact"] => {\n',
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, parse_arm + anchor, 1)
            break
    else:
        raise SystemExit("Could not find command parse insertion point for report-store.")

run_arm = '''        CliCommand::ReportStore {
            dry_run,
            yes,
            output_format,
        } => render_report_store(dry_run, yes, output_format),
'''
if "render_report_store(dry_run, yes, output_format)" not in text:
    anchors = [
        "        CliCommand::CachePlan {\n",
        "        CliCommand::VerifyPlan {\n",
        "        CliCommand::Impact {\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, run_arm + anchor, 1)
            break
    else:
        raise SystemExit("Could not find run match insertion point for report-store.")

helper = '''/// Requires exactly one report-store mode.
fn require_report_store_mode(dry_run: bool, yes: bool) -> Result<(), String> {
    match (dry_run, yes) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => Err(
            "report-store currently requires either --dry-run to preview or --yes to write generated report-store evidence".to_string(),
        ),
        (true, true) => Err("report-store accepts either --dry-run or --yes, not both".to_string()),
    }
}

'''
if "fn require_report_store_mode" not in text:
    anchors = [
        "/// Requires exactly one cache-plan mode.\n",
        "/// Requires exactly one verify-plan mode.\n",
        "/// Requires exactly one dependency-impact mode.\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, helper + anchor, 1)
            break
    else:
        raise SystemExit("Could not find helper insertion point for report-store.")

render_fn = '''/// Renders or writes local report/artifact store evidence.
fn render_report_store(
    dry_run: bool,
    yes: bool,
    output_format: OutputFormat,
) -> Result<String, String> {
    let root = std::env::current_dir().map_err(|error| error.to_string())?;

    if dry_run {
        let index = monad_core::build_local_store_index(&root);
        return match output_format {
            OutputFormat::Text => Ok(monad_core::render_local_store_index(&index)),
            OutputFormat::Json => Ok(monad_core::render_local_store_index_json(&index)),
        };
    }

    if yes {
        let result =
            monad_core::write_local_store_evidence(&root).map_err(|error| error.to_string())?;
        return Ok(monad_core::render_local_store_apply_result(&result));
    }

    Err("report-store currently requires either --dry-run to preview or --yes to write generated report-store evidence".to_string())
}

'''
if "fn render_report_store(" not in text:
    anchors = [
        "/// Renders or writes build-cache planning evidence.\n",
        "/// Renders or writes test-intelligence verification-planning evidence.\n",
        "/// Renders or writes dependency-impact evidence.\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, render_fn + anchor, 1)
            break
    else:
        raise SystemExit("Could not find render function insertion point for report-store.")

if "  report-store --dry-run" not in text:
    anchors = [
        '        "  cache-plan --yes                          Write generated cache-plan evidence",\n',
        '        "  verify-plan --yes                         Write generated verification-plan evidence",\n',
        '        "  impact --yes                              Write generated dependency-impact evidence",\n',
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(
                anchor,
                anchor
                + '        "  report-store --dry-run                    Preview local report/artifact index",\n'
                + '        "  report-store --dry-run --format=json      Preview local report/artifact index as JSON",\n'
                + '        "  report-store --yes                        Write generated report-store evidence",\n',
                1,
            )
            break

if "monad reports --dry-run" not in text:
    anchors = [
        '        "  monad build-cache --dry-run",\n',
        '        "  monad test-intelligence --dry-run",\n',
        '        "  monad dependency-impact --dry-run",\n',
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(
                anchor,
                anchor
                + '        "  monad report-store --dry-run",\n'
                + '        "  monad report-store --dry-run --format=json",\n'
                + '        "  monad reports --dry-run",\n'
                + '        "  monad artifacts --dry-run",\n',
                1,
            )
            break

if "report-store writes generated evidence only" not in text:
    anchors = [
        '        "  cache-plan writes generated evidence only and does not execute tasks.",\n',
        '        "  verify-plan writes generated evidence only and does not execute tests.",\n',
        '        "  impact writes generated evidence only and does not execute tools.",\n',
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(
                anchor,
                anchor
                + '        "  report-store writes generated evidence only and does not delete artifacts.",\n',
                1,
            )
            break

test_block = '''    #[test]
    fn report_store_dry_run_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "report-store", "--dry-run"])
                .expect("report-store dry-run should parse"),
            CliCommand::ReportStore {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "report-store", "--dry-run", "--format=json"])
                .expect("report-store dry-run json should parse"),
            CliCommand::ReportStore {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Json,
            }
        );
    }

    #[test]
    fn report_store_aliases_parse() {
        assert_eq!(
            parse_arguments(&["monad", "reports", "--dry-run"])
                .expect("reports alias should parse"),
            CliCommand::ReportStore {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "artifacts", "--dry-run"])
                .expect("artifacts alias should parse"),
            CliCommand::ReportStore {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            }
        );
    }

    #[test]
    fn report_store_yes_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "report-store", "--yes"])
                .expect("report-store yes should parse"),
            CliCommand::ReportStore {
                dry_run: false,
                yes: true,
                output_format: OutputFormat::Text,
            }
        );
    }

    #[test]
    fn report_store_requires_mode() {
        let error = parse_arguments(&["monad", "report-store"])
            .expect_err("report-store should require mode");

        assert!(error.contains("report-store currently requires either --dry-run"));
    }

'''
if "fn report_store_dry_run_command_parses" not in text:
    anchors = [
        "    #[test]\n    fn cache_plan_dry_run_command_parses() {\n",
        "    #[test]\n    fn verify_plan_dry_run_command_parses() {\n",
        "    #[test]\n    fn impact_dry_run_command_parses() {\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, test_block + anchor, 1)
            break
    else:
        raise SystemExit("Could not find CLI test insertion point for report-store.")

main.write_text(text)
PY

cargo fmt

echo "Applied E28 local artifact/report store foundation."
echo "Backups written under: $BACKUP_DIR"
echo
echo "Run verification:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-report-store.sh"
echo "  tools/scripts/verify-e28.sh"
