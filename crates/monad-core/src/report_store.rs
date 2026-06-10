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

            ReportMetadata::new(
                make_store_id(StoreObjectKind::Report, &path),
                path,
                title,
                retention,
                byte_len,
            )
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
            let label = path.file_name().map_or_else(
                || "artifact".to_string(),
                |name| name.to_string_lossy().to_string(),
            );
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
        format!(
            "  reports_dir: {}",
            index.contract().reports_dir().display()
        ),
        format!(
            "  artifacts_dir: {}",
            index.contract().artifacts_dir().display()
        ),
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
                lines.push(format!(
                    "  - [{}] {}",
                    write_result.as_str(),
                    path.display()
                ));
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
            files.push(
                path.strip_prefix(root)
                    .map_or(path.clone(), Path::to_path_buf),
            );
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

    relative_path.file_stem().map_or_else(
        || "report".to_string(),
        |stem| stem.to_string_lossy().to_string(),
    )
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
            fs::write(
                root.join(".monad/reports/epic-closeout.md"),
                "# Epic Closeout\n\nDone.\n"
            )
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
