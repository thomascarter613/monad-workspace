//! Build cache and incremental execution planning foundation.
//!
//! E27 introduces a supervised local cache-planning model. This module computes
//! stable task fingerprints, reads optional local execution metadata, and renders
//! cache-aware dry-run plans. It does not execute tasks, restore cache artifacts,
//! invoke build tools, call package managers, or rewrite user-owned source files.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{GatedWriteRequest, GatedWriteResult, gated_generated_write};

/// Cache decision for a planned task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CacheDecision {
    /// No matching local metadata was found.
    Miss,
    /// Matching local metadata was found.
    Hit,
    /// Matching metadata was found, but safety/evidence requires revalidation.
    Revalidate,
}

impl CacheDecision {
    /// Stable decision label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Miss => "miss",
            Self::Hit => "hit",
            Self::Revalidate => "revalidate",
        }
    }
}

/// Incremental execution action Monad recommends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum IncrementalAction {
    /// Run the task if the human chooses to execute it outside this planner.
    Run,
    /// The task can be skipped because the fingerprint matches completed local metadata.
    Skip,
    /// Re-run or review the task because evidence confidence is not enough.
    Revalidate,
}

impl IncrementalAction {
    /// Stable action label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::Skip => "skip",
            Self::Revalidate => "revalidate",
        }
    }
}

/// A cacheable task planned by Monad.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CacheTask {
    id: String,
    command: String,
    inputs: Vec<PathBuf>,
    outputs: Vec<PathBuf>,
}

impl CacheTask {
    /// Creates a deterministic cache task.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        command: impl Into<String>,
        mut inputs: Vec<PathBuf>,
        mut outputs: Vec<PathBuf>,
    ) -> Self {
        inputs.sort();
        inputs.dedup();
        outputs.sort();
        outputs.dedup();

        Self {
            id: id.into(),
            command: command.into(),
            inputs,
            outputs,
        }
    }

    /// Stable task ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Command text. It is modeled only and is not executed by Monad.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Inputs participating in the fingerprint.
    #[must_use]
    pub fn inputs(&self) -> &[PathBuf] {
        &self.inputs
    }

    /// Conceptual outputs.
    #[must_use]
    pub fn outputs(&self) -> &[PathBuf] {
        &self.outputs
    }
}

/// Stable task fingerprint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskFingerprint {
    task_id: String,
    fingerprint: String,
    input_count: usize,
    missing_inputs: Vec<PathBuf>,
}

impl TaskFingerprint {
    /// Creates a fingerprint record.
    #[must_use]
    pub fn new(
        task_id: impl Into<String>,
        fingerprint: impl Into<String>,
        input_count: usize,
        mut missing_inputs: Vec<PathBuf>,
    ) -> Self {
        missing_inputs.sort();
        missing_inputs.dedup();

        Self {
            task_id: task_id.into(),
            fingerprint: fingerprint.into(),
            input_count,
            missing_inputs,
        }
    }

    /// Task ID.
    #[must_use]
    pub fn task_id(&self) -> &str {
        &self.task_id
    }

    /// Stable hexadecimal fingerprint.
    #[must_use]
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    /// Count of declared inputs.
    #[must_use]
    pub const fn input_count(&self) -> usize {
        self.input_count
    }

    /// Missing inputs.
    #[must_use]
    pub fn missing_inputs(&self) -> &[PathBuf] {
        &self.missing_inputs
    }
}

/// Local execution metadata entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExecutionMetadata {
    task_id: String,
    fingerprint: String,
    status: String,
    evidence_path: PathBuf,
}

impl ExecutionMetadata {
    /// Creates local execution metadata.
    #[must_use]
    pub fn new(
        task_id: impl Into<String>,
        fingerprint: impl Into<String>,
        status: impl Into<String>,
        evidence_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            task_id: task_id.into(),
            fingerprint: fingerprint.into(),
            status: status.into(),
            evidence_path: evidence_path.into(),
        }
    }

    /// Task ID.
    #[must_use]
    pub fn task_id(&self) -> &str {
        &self.task_id
    }

    /// Fingerprint.
    #[must_use]
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    /// Status string.
    #[must_use]
    pub fn status(&self) -> &str {
        &self.status
    }

    /// Evidence path.
    #[must_use]
    pub fn evidence_path(&self) -> &Path {
        &self.evidence_path
    }
}

/// Cache decision for one planned task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CachePlanEntry {
    task: CacheTask,
    fingerprint: TaskFingerprint,
    decision: CacheDecision,
    action: IncrementalAction,
    reason: String,
}

impl CachePlanEntry {
    /// Creates a cache-plan entry.
    #[must_use]
    pub fn new(
        task: CacheTask,
        fingerprint: TaskFingerprint,
        decision: CacheDecision,
        action: IncrementalAction,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            task,
            fingerprint,
            decision,
            action,
            reason: reason.into(),
        }
    }

    /// Planned task.
    #[must_use]
    pub const fn task(&self) -> &CacheTask {
        &self.task
    }

    /// Fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> &TaskFingerprint {
        &self.fingerprint
    }

    /// Cache decision.
    #[must_use]
    pub const fn decision(&self) -> CacheDecision {
        self.decision
    }

    /// Incremental action.
    #[must_use]
    pub const fn action(&self) -> IncrementalAction {
        self.action
    }

    /// Rationale.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// Full cache-aware plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BuildCachePlan {
    command: String,
    entries: Vec<CachePlanEntry>,
    metadata_entries: Vec<ExecutionMetadata>,
    evidence_paths: Vec<PathBuf>,
    safety_notes: Vec<String>,
}

impl BuildCachePlan {
    /// Creates a deterministic plan.
    #[must_use]
    pub fn new(
        mut entries: Vec<CachePlanEntry>,
        mut metadata_entries: Vec<ExecutionMetadata>,
    ) -> Self {
        entries.sort_by(|left, right| left.task().id().cmp(right.task().id()));
        metadata_entries.sort_by(|left, right| left.task_id().cmp(right.task_id()));

        Self {
            command: "cache-plan".to_string(),
            entries,
            metadata_entries,
            evidence_paths: vec![
                PathBuf::from(".monad/reports/build-cache-plan.md"),
                PathBuf::from(".monad/reports/build-cache-plan.json"),
                PathBuf::from(".monad/cache/execution-metadata.tsv"),
            ],
            safety_notes: vec![
                "No build or test commands are executed by Monad.".to_string(),
                "No cache artifacts are restored by Monad.".to_string(),
                "No package managers are invoked by Monad.".to_string(),
                "No remote cache service is contacted.".to_string(),
                "No user-owned source files are rewritten.".to_string(),
                "Only generated local evidence is written when --yes is used.".to_string(),
            ],
        }
    }

    /// Planned task entries.
    #[must_use]
    pub fn entries(&self) -> &[CachePlanEntry] {
        &self.entries
    }

    /// Local execution metadata entries.
    #[must_use]
    pub fn metadata_entries(&self) -> &[ExecutionMetadata] {
        &self.metadata_entries
    }

    /// Evidence paths.
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

/// Apply result for generated cache evidence writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildCacheApplyResult {
    write_results: Vec<GatedWriteResult>,
}

impl BuildCacheApplyResult {
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

/// Builds the default local cache plan.
#[must_use]
pub fn build_cache_plan(root: impl AsRef<Path>) -> BuildCachePlan {
    let root = root.as_ref();
    let tasks = default_cache_tasks(root);
    let metadata_entries = read_execution_metadata(root);

    let entries = tasks
        .into_iter()
        .map(|task| {
            let fingerprint = fingerprint_task(root, &task);
            let (decision, action, reason) = decide_cache_action(&fingerprint, &metadata_entries);
            CachePlanEntry::new(task, fingerprint, decision, action, reason)
        })
        .collect::<Vec<_>>();

    BuildCachePlan::new(entries, metadata_entries)
}

/// Computes a deterministic task fingerprint.
#[must_use]
pub fn fingerprint_task(root: &Path, task: &CacheTask) -> TaskFingerprint {
    let mut hash = Fnv64::default();
    hash.update(task.id().as_bytes());
    hash.update(b"\n");
    hash.update(task.command().as_bytes());
    hash.update(b"\n");

    let mut missing_inputs = Vec::new();

    for input in task.inputs() {
        hash.update(input.to_string_lossy().as_bytes());
        hash.update(b"\0");

        let absolute = root.join(input);
        match fs::read(&absolute) {
            Ok(bytes) => {
                hash.update(&bytes);
                hash.update(b"\0");
            }
            Err(_) => {
                missing_inputs.push(input.clone());
                hash.update(b"<missing>");
                hash.update(b"\0");
            }
        }
    }

    TaskFingerprint::new(
        task.id().to_string(),
        format!("{:016x}", hash.finish()),
        task.inputs().len(),
        missing_inputs,
    )
}

/// Writes generated cache plan evidence.
pub fn write_build_cache_evidence(root: impl AsRef<Path>) -> Result<BuildCacheApplyResult, String> {
    let root = root.as_ref();
    let plan = build_cache_plan(root);
    let markdown = render_build_cache_plan(&plan);
    let json = render_build_cache_plan_json(&plan);
    let metadata = render_execution_metadata_snapshot(&plan);

    let requests = [
        GatedWriteRequest::new(".monad/reports/build-cache-plan.md", markdown, true),
        GatedWriteRequest::new(".monad/reports/build-cache-plan.json", json, true),
        GatedWriteRequest::new(".monad/cache/execution-metadata.tsv", metadata, true),
    ];

    let write_results = requests
        .iter()
        .map(|request| gated_generated_write(root, request))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(BuildCacheApplyResult::new(write_results))
}

/// Renders a text cache plan.
#[must_use]
pub fn render_build_cache_plan(plan: &BuildCachePlan) -> String {
    let mut lines = vec![
        "Monad build cache and incremental execution plan".to_string(),
        String::new(),
        "Summary:".to_string(),
        format!("  tasks: {}", plan.entries().len()),
        format!("  metadata_entries: {}", plan.metadata_entries().len()),
        String::new(),
        "Task decisions:".to_string(),
    ];

    if plan.entries().is_empty() {
        lines.push("  - no cacheable tasks discovered".to_string());
    } else {
        for entry in plan.entries() {
            lines.push(format!(
                "  - {} decision={} action={} fingerprint={}",
                entry.task().id(),
                entry.decision().as_str(),
                entry.action().as_str(),
                entry.fingerprint().fingerprint()
            ));
            lines.push(format!("    command: {}", entry.task().command()));
            lines.push(format!("    reason: {}", entry.reason()));
            if !entry.fingerprint().missing_inputs().is_empty() {
                let missing = entry
                    .fingerprint()
                    .missing_inputs()
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                lines.push(format!("    missing_inputs: {missing}"));
            }
        }
    }

    lines.push(String::new());
    lines.push("Evidence outputs:".to_string());
    for path in plan.evidence_paths() {
        lines.push(format!("  - {}", path.display()));
    }

    lines.push(String::new());
    lines.push("Safety notes:".to_string());
    for note in plan.safety_notes() {
        lines.push(format!("  - {note}"));
    }

    lines.join("\n")
}

/// Renders a JSON cache plan.
#[must_use]
pub fn render_build_cache_plan_json(plan: &BuildCachePlan) -> String {
    serde_json::to_string_pretty(plan).unwrap_or_else(|_| {
        "{\n  \"command\": \"cache-plan\",\n  \"error\": \"build cache plan serialization failed\"\n}".to_string()
    })
}

/// Renders generated evidence write results.
#[must_use]
pub fn render_build_cache_apply_result(result: &BuildCacheApplyResult) -> String {
    let mut lines = vec![
        "Monad build-cache evidence write result".to_string(),
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
    lines.push("No build or test commands were executed by Monad.".to_string());
    lines.push("No cache artifacts were restored.".to_string());
    lines.push("No remote cache service was contacted.".to_string());
    lines.push("No user-owned source files were rewritten.".to_string());

    lines.join("\n")
}

fn default_cache_tasks(root: &Path) -> Vec<CacheTask> {
    let mut tasks = Vec::new();

    if root.join("Cargo.toml").is_file() {
        tasks.push(CacheTask::new(
            "rust:fmt",
            "cargo fmt --check",
            vec![PathBuf::from("Cargo.toml"), PathBuf::from("rustfmt.toml")],
            vec![PathBuf::from(".monad/cache/rust-fmt.ok")],
        ));
        tasks.push(CacheTask::new(
            "rust:test",
            "cargo test",
            vec![PathBuf::from("Cargo.toml"), PathBuf::from("Cargo.lock")],
            vec![PathBuf::from(".monad/cache/rust-test.ok")],
        ));
        tasks.push(CacheTask::new(
            "rust:clippy",
            "cargo clippy --all-targets --all-features -- -D warnings",
            vec![PathBuf::from("Cargo.toml"), PathBuf::from("Cargo.lock")],
            vec![PathBuf::from(".monad/cache/rust-clippy.ok")],
        ));
    }

    if root.join("tools/scripts/verify-e27.sh").is_file() {
        tasks.push(CacheTask::new(
            "script:e27",
            "tools/scripts/verify-e27.sh",
            vec![PathBuf::from("tools/scripts/verify-e27.sh")],
            vec![PathBuf::from(".monad/cache/e27-verify.ok")],
        ));
    }

    tasks.sort_by(|left, right| left.id().cmp(right.id()));
    tasks
}

fn decide_cache_action(
    fingerprint: &TaskFingerprint,
    metadata_entries: &[ExecutionMetadata],
) -> (CacheDecision, IncrementalAction, String) {
    if !fingerprint.missing_inputs().is_empty() {
        return (
            CacheDecision::Revalidate,
            IncrementalAction::Revalidate,
            "one or more declared inputs are missing; human review is required".to_string(),
        );
    }

    let matching = metadata_entries.iter().find(|entry| {
        entry.task_id() == fingerprint.task_id() && entry.fingerprint() == fingerprint.fingerprint()
    });

    match matching {
        Some(entry) if entry.status() == "completed" => (
            CacheDecision::Hit,
            IncrementalAction::Skip,
            format!(
                "matching completed metadata found at {}",
                entry.evidence_path().display()
            ),
        ),
        Some(entry) => (
            CacheDecision::Revalidate,
            IncrementalAction::Revalidate,
            format!(
                "matching metadata found with non-completed status `{}`",
                entry.status()
            ),
        ),
        None => (
            CacheDecision::Miss,
            IncrementalAction::Run,
            "no matching local execution metadata was found".to_string(),
        ),
    }
}

fn read_execution_metadata(root: &Path) -> Vec<ExecutionMetadata> {
    let path = root.join(".monad/cache/execution-metadata.tsv");
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };

    let mut entries = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with('#'))
        .filter_map(parse_metadata_line)
        .collect::<Vec<_>>();

    entries.sort_by(|left, right| left.task_id().cmp(right.task_id()));
    entries
}

fn parse_metadata_line(line: &str) -> Option<ExecutionMetadata> {
    let parts = line.split('\t').collect::<Vec<_>>();
    if parts.len() != 4 {
        return None;
    }

    Some(ExecutionMetadata::new(
        parts[0],
        parts[1],
        parts[2],
        PathBuf::from(parts[3]),
    ))
}

fn render_execution_metadata_snapshot(plan: &BuildCachePlan) -> String {
    let mut lines = vec!["# task_id\tfingerprint\tstatus\tevidence_path".to_string()];

    for entry in plan.entries() {
        let status = match entry.action() {
            IncrementalAction::Skip => "completed",
            IncrementalAction::Run => "planned",
            IncrementalAction::Revalidate => "revalidate",
        };
        lines.push(format!(
            "{}\t{}\t{}\t.monad/reports/build-cache-plan.md",
            entry.task().id(),
            entry.fingerprint().fingerprint(),
            status
        ));
    }

    lines.join("\n")
}

#[derive(Debug, Clone, Copy)]
struct Fnv64 {
    value: u64,
}

impl Fnv64 {
    fn update(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.value ^= u64::from(*byte);
            self.value = self.value.wrapping_mul(1_099_511_628_211);
        }
    }

    const fn finish(self) -> u64 {
        self.value
    }
}

impl Default for Fnv64 {
    fn default() -> Self {
        Self {
            value: 14_695_981_039_346_656_037,
        }
    }
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
            "monad-build-cache-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);
        assert!(fs::create_dir_all(root.join(".monad/cache")).is_ok());
        assert!(fs::create_dir_all(root.join("tools/scripts")).is_ok());
        assert!(fs::write(root.join("Cargo.toml"), "[workspace]\n").is_ok());
        assert!(fs::write(root.join("Cargo.lock"), "# lock\n").is_ok());
        root
    }

    #[test]
    fn fingerprint_is_deterministic_for_same_inputs() {
        let root = create_workspace("fingerprint");
        let task = CacheTask::new(
            "rust:test",
            "cargo test",
            vec![PathBuf::from("Cargo.toml"), PathBuf::from("Cargo.lock")],
            Vec::new(),
        );

        let left = fingerprint_task(&root, &task);
        let right = fingerprint_task(&root, &task);

        assert_eq!(left.fingerprint(), right.fingerprint());
        assert!(left.missing_inputs().is_empty());

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn missing_inputs_force_revalidation() {
        let root = create_workspace("missing");
        let task = CacheTask::new(
            "custom",
            "custom command",
            vec![PathBuf::from("missing.file")],
            Vec::new(),
        );
        let fingerprint = fingerprint_task(&root, &task);
        let (decision, action, reason) = decide_cache_action(&fingerprint, &[]);

        assert_eq!(decision, CacheDecision::Revalidate);
        assert_eq!(action, IncrementalAction::Revalidate);
        assert!(reason.contains("missing"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn completed_matching_metadata_is_cache_hit() {
        let root = create_workspace("hit");
        let task = CacheTask::new(
            "rust:test",
            "cargo test",
            vec![PathBuf::from("Cargo.toml"), PathBuf::from("Cargo.lock")],
            Vec::new(),
        );
        let fingerprint = fingerprint_task(&root, &task);
        let metadata = vec![ExecutionMetadata::new(
            "rust:test",
            fingerprint.fingerprint().to_string(),
            "completed",
            ".monad/reports/example.md",
        )];

        let (decision, action, _) = decide_cache_action(&fingerprint, &metadata);

        assert_eq!(decision, CacheDecision::Hit);
        assert_eq!(action, IncrementalAction::Skip);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn plan_contains_default_rust_tasks() {
        let root = create_workspace("plan");
        let plan = build_cache_plan(&root);
        let ids = plan
            .entries()
            .iter()
            .map(|entry| entry.task().id().to_string())
            .collect::<Vec<_>>();

        assert!(ids.contains(&"rust:fmt".to_string()));
        assert!(ids.contains(&"rust:test".to_string()));
        assert!(ids.contains(&"rust:clippy".to_string()));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn text_render_mentions_no_execution_safety() {
        let root = create_workspace("text");
        let plan = build_cache_plan(&root);
        let text = render_build_cache_plan(&plan);

        assert!(text.contains("Monad build cache and incremental execution plan"));
        assert!(text.contains("No build or test commands are executed by Monad"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn json_render_contains_cache_plan_command() {
        let root = create_workspace("json");
        let plan = build_cache_plan(&root);
        let json = render_build_cache_plan_json(&plan);

        assert!(json.contains("\"command\": \"cache-plan\""));
        assert!(json.contains("rust:test"));

        fs::remove_dir_all(root).ok();
    }
}
