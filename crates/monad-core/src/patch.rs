//! Patch planning and supervised apply foundation.
//!
//! E20 introduces a deterministic patch/change-set model, dry-run rendering,
//! validation/conflict checks, and a supervised apply path for generated local
//! patch evidence artifacts. It intentionally does not fetch remote patches,
//! execute arbitrary scripts, call AI providers, delete files, silently
//! overwrite existing content, or autonomously mutate user-owned source files.

use crate::policy::{
    ApprovalPlan, FileOperationIntent, GatedWriteRequest, GatedWriteResult, check_file_operation,
    gated_generated_write,
};
use std::fs;
use std::path::{Component, Path, PathBuf};

/// Generated marker path written by the first supervised patch apply foundation.
pub const GENERATED_PATCH_MARKER_PATH: &str = ".monad/patches/e20-supervised-apply-foundation.md";

/// Markdown evidence report path for patch planning.
pub const PATCH_EVIDENCE_MARKDOWN_PATH: &str = ".monad/reports/patch-plan.md";

/// JSON evidence report path for patch planning.
pub const PATCH_EVIDENCE_JSON_PATH: &str = ".monad/reports/patch-plan.json";

/// Kind of file change represented by a patch plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatchChangeKind {
    /// Create a generated Monad-owned file.
    CreateGeneratedFile,

    /// Update a generated Monad-owned file.
    UpdateGeneratedFile,

    /// Update user-owned source or project files.
    UpdateUserOwnedFile,

    /// Delete a file.
    DeleteFile,
}

impl PatchChangeKind {
    /// Stable label for text and JSON output.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CreateGeneratedFile => "create-generated-file",
            Self::UpdateGeneratedFile => "update-generated-file",
            Self::UpdateUserOwnedFile => "update-user-owned-file",
            Self::DeleteFile => "delete-file",
        }
    }

    /// Policy intent used by E19 approval gates.
    #[must_use]
    pub const fn file_operation_intent(self) -> FileOperationIntent {
        match self {
            Self::CreateGeneratedFile | Self::UpdateGeneratedFile => {
                FileOperationIntent::WriteGenerated
            }
            Self::UpdateUserOwnedFile => FileOperationIntent::WriteUserOwned,
            Self::DeleteFile => FileOperationIntent::Delete,
        }
    }

    /// Whether this change is allowed to use the generated-write gate.
    #[must_use]
    pub const fn is_generated_write(self) -> bool {
        matches!(self, Self::CreateGeneratedFile | Self::UpdateGeneratedFile)
    }
}

/// One file-level patch change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchFileChange {
    relative_path: PathBuf,
    kind: PatchChangeKind,
    before_hash: Option<String>,
    after_hash: Option<String>,
    after_content: Option<String>,
    rationale: String,
}

impl PatchFileChange {
    /// Creates a generated-file creation change.
    #[must_use]
    pub fn create_generated_file(
        relative_path: impl Into<PathBuf>,
        after_content: impl Into<String>,
        rationale: impl Into<String>,
    ) -> Self {
        let content = after_content.into();
        Self {
            relative_path: relative_path.into(),
            kind: PatchChangeKind::CreateGeneratedFile,
            before_hash: None,
            after_hash: Some(stable_content_hash(&content)),
            after_content: Some(content),
            rationale: rationale.into(),
        }
    }

    /// Creates a user-owned source update representation.
    ///
    /// The E20 default planner does not emit this kind, but tests and future
    /// callers can use it to confirm source mutations remain policy-gated and
    /// blocked until an explicit later policy allows them.
    #[must_use]
    pub fn update_user_owned_file(
        relative_path: impl Into<PathBuf>,
        before_content: impl AsRef<str>,
        after_content: impl Into<String>,
        rationale: impl Into<String>,
    ) -> Self {
        let before = before_content.as_ref().to_string();
        let after = after_content.into();
        Self {
            relative_path: relative_path.into(),
            kind: PatchChangeKind::UpdateUserOwnedFile,
            before_hash: Some(stable_content_hash(&before)),
            after_hash: Some(stable_content_hash(&after)),
            after_content: Some(after),
            rationale: rationale.into(),
        }
    }

    /// Relative repository path.
    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// Change kind.
    #[must_use]
    pub const fn kind(&self) -> PatchChangeKind {
        self.kind
    }

    /// Optional before-content hash.
    #[must_use]
    pub fn before_hash(&self) -> Option<&str> {
        self.before_hash.as_deref()
    }

    /// Optional after-content hash.
    #[must_use]
    pub fn after_hash(&self) -> Option<&str> {
        self.after_hash.as_deref()
    }

    /// Optional after content for generated writes.
    #[must_use]
    pub fn after_content(&self) -> Option<&str> {
        self.after_content.as_deref()
    }

    /// Human-readable rationale.
    #[must_use]
    pub fn rationale(&self) -> &str {
        &self.rationale
    }
}

/// Deterministic patch change set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchChangeSet {
    id: String,
    title: String,
    description: String,
    changes: Vec<PatchFileChange>,
}

impl PatchChangeSet {
    /// Creates a change set and sorts changes by path/kind for deterministic output.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        mut changes: Vec<PatchFileChange>,
    ) -> Self {
        changes.sort_by(|left, right| {
            left.relative_path()
                .cmp(right.relative_path())
                .then(left.kind().cmp(&right.kind()))
        });

        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            changes,
        }
    }

    /// Change-set ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Change-set title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Change-set description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Sorted changes.
    #[must_use]
    pub fn changes(&self) -> &[PatchFileChange] {
        &self.changes
    }
}

/// Patch validation severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatchValidationSeverity {
    /// Informational finding.
    Info,

    /// Human review warning.
    Warning,

    /// Blocks apply.
    Blocked,
}

impl PatchValidationSeverity {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Blocked => "blocked",
        }
    }
}

/// One deterministic patch validation finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchValidationFinding {
    id: String,
    severity: PatchValidationSeverity,
    path: Option<PathBuf>,
    message: String,
}

impl PatchValidationFinding {
    /// Creates a validation finding.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        severity: PatchValidationSeverity,
        path: Option<PathBuf>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            severity,
            path,
            message: message.into(),
        }
    }

    /// Finding ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Finding severity.
    #[must_use]
    pub const fn severity(&self) -> PatchValidationSeverity {
        self.severity
    }

    /// Optional path.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Finding message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Patch validation report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchValidationReport {
    findings: Vec<PatchValidationFinding>,
}

impl PatchValidationReport {
    /// Creates a deterministic report sorted by severity, path, then ID.
    #[must_use]
    pub fn new(mut findings: Vec<PatchValidationFinding>) -> Self {
        findings.sort_by(|left, right| {
            left.severity()
                .cmp(&right.severity())
                .then(left.path().cmp(&right.path()))
                .then(left.id().cmp(right.id()))
        });

        Self { findings }
    }

    /// Validation findings.
    #[must_use]
    pub fn findings(&self) -> &[PatchValidationFinding] {
        &self.findings
    }

    /// Count of blocked findings.
    #[must_use]
    pub fn blocked_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.severity() == PatchValidationSeverity::Blocked)
            .count()
    }

    /// Count of warning findings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.severity() == PatchValidationSeverity::Warning)
            .count()
    }

    /// Whether this validation report blocks apply.
    #[must_use]
    pub fn is_blocked(&self) -> bool {
        self.blocked_count() > 0
    }
}

/// Full patch plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchPlan {
    change_set: PatchChangeSet,
    validation: PatchValidationReport,
    approval_plan: ApprovalPlan,
    safety_boundaries: Vec<String>,
}

impl PatchPlan {
    /// Creates a patch plan.
    #[must_use]
    pub fn new(
        change_set: PatchChangeSet,
        validation: PatchValidationReport,
        approval_plan: ApprovalPlan,
        safety_boundaries: Vec<String>,
    ) -> Self {
        Self {
            change_set,
            validation,
            approval_plan,
            safety_boundaries,
        }
    }

    /// Change set.
    #[must_use]
    pub const fn change_set(&self) -> &PatchChangeSet {
        &self.change_set
    }

    /// Validation report.
    #[must_use]
    pub const fn validation(&self) -> &PatchValidationReport {
        &self.validation
    }

    /// Approval plan.
    #[must_use]
    pub const fn approval_plan(&self) -> &ApprovalPlan {
        &self.approval_plan
    }

    /// Safety boundaries.
    #[must_use]
    pub fn safety_boundaries(&self) -> &[String] {
        &self.safety_boundaries
    }

    /// Whether apply is blocked.
    #[must_use]
    pub fn is_blocked(&self) -> bool {
        self.validation().is_blocked() || self.approval_plan().is_blocked()
    }
}

/// Result from supervised patch apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchApplyResult {
    plan: PatchPlan,
    write_results: Vec<GatedWriteResult>,
}

impl PatchApplyResult {
    /// Creates an apply result.
    #[must_use]
    pub fn new(plan: PatchPlan, write_results: Vec<GatedWriteResult>) -> Self {
        Self {
            plan,
            write_results,
        }
    }

    /// Plan used for apply.
    #[must_use]
    pub const fn plan(&self) -> &PatchPlan {
        &self.plan
    }

    /// Gated generated-write results.
    #[must_use]
    pub fn write_results(&self) -> &[GatedWriteResult] {
        &self.write_results
    }
}

/// Builds the default E20 patch plan.
#[must_use]
pub fn build_patch_plan(root: impl AsRef<Path>) -> PatchPlan {
    let change_set = PatchChangeSet::new(
        "e20-supervised-apply-foundation",
        "E20 supervised patch apply foundation",
        "Create generated local patch evidence without mutating user-owned source files.",
        vec![PatchFileChange::create_generated_file(
            GENERATED_PATCH_MARKER_PATH,
            generated_patch_marker_content(),
            "record the local supervised patch foundation as generated Monad evidence",
        )],
    );

    let validation = validate_patch_change_set(root.as_ref(), &change_set);
    let approval_plan = check_file_operation(
        Path::new(GENERATED_PATCH_MARKER_PATH),
        FileOperationIntent::WriteGenerated,
    );

    PatchPlan::new(
        change_set,
        validation,
        approval_plan,
        vec![
            "No autonomous patch application.".to_string(),
            "No silent overwrites.".to_string(),
            "No destructive file deletion.".to_string(),
            "No remote patch fetching.".to_string(),
            "No arbitrary script execution.".to_string(),
            "No AI-provider patch execution.".to_string(),
            "No user-owned source mutation in this foundation slice.".to_string(),
            "Generated writes require explicit --yes approval and E19 gates.".to_string(),
        ],
    )
}

/// Validates a change set against local safety and conflict rules.
#[must_use]
pub fn validate_patch_change_set(
    root: &Path,
    change_set: &PatchChangeSet,
) -> PatchValidationReport {
    let mut findings = Vec::new();

    if change_set.changes().is_empty() {
        findings.push(PatchValidationFinding::new(
            "patch.change-set.empty",
            PatchValidationSeverity::Blocked,
            None,
            "patch change set must contain at least one change",
        ));
    }

    for change in change_set.changes() {
        let path = change.relative_path();
        let path_for_finding = Some(path.to_path_buf());

        findings.push(PatchValidationFinding::new(
            "patch.change.policy-check",
            PatchValidationSeverity::Info,
            path_for_finding.clone(),
            format!(
                "change kind {} is checked through E19 file-operation policy gates",
                change.kind().as_str()
            ),
        ));

        if !is_safe_relative_path(path) {
            findings.push(PatchValidationFinding::new(
                "patch.path.unsafe",
                PatchValidationSeverity::Blocked,
                path_for_finding.clone(),
                "patch paths must be relative repository paths and must not contain parent traversal",
            ));
        }

        let policy_plan = check_file_operation(path, change.kind().file_operation_intent());
        if policy_plan.is_blocked() {
            findings.push(PatchValidationFinding::new(
                "patch.policy.blocked",
                PatchValidationSeverity::Blocked,
                path_for_finding.clone(),
                "E19 policy blocks this file operation intent",
            ));
        }

        if matches!(change.kind(), PatchChangeKind::DeleteFile) {
            findings.push(PatchValidationFinding::new(
                "patch.delete.blocked",
                PatchValidationSeverity::Blocked,
                path_for_finding.clone(),
                "file deletion is outside the E20 supervised apply foundation",
            ));
        }

        if !change.kind().is_generated_write() {
            findings.push(PatchValidationFinding::new(
                "patch.source-mutation.blocked",
                PatchValidationSeverity::Blocked,
                path_for_finding.clone(),
                "user-owned source mutation is represented for planning but blocked in this foundation slice",
            ));
        }

        if path_is_existing_conflict(root, change) {
            findings.push(PatchValidationFinding::new(
                "patch.conflict.existing-file-differs",
                PatchValidationSeverity::Blocked,
                path_for_finding.clone(),
                "target file already exists with different content; refusing silent overwrite",
            ));
        }
    }

    PatchValidationReport::new(findings)
}

/// Applies the default patch plan under explicit approval gates.
pub fn apply_patch_plan(root: impl AsRef<Path>) -> Result<PatchApplyResult, String> {
    let plan = build_patch_plan(root.as_ref());
    let mut write_results = Vec::new();

    if plan.is_blocked() {
        return Ok(PatchApplyResult::new(plan, write_results));
    }

    for change in plan.change_set().changes() {
        if !change.kind().is_generated_write() {
            write_results.push(GatedWriteResult::Blocked(
                "source mutation is not allowed by the E20 generated-write apply foundation"
                    .to_string(),
            ));
            continue;
        }

        let Some(content) = change.after_content() else {
            write_results.push(GatedWriteResult::Blocked(
                "generated write is missing after content".to_string(),
            ));
            continue;
        };

        let request = GatedWriteRequest::new(change.relative_path().to_path_buf(), content, true);
        write_results.push(gated_generated_write(root.as_ref(), &request)?);
    }

    if !write_results.iter().any(is_blocked_write_result) {
        for request in evidence_write_requests(&plan) {
            write_results.push(gated_generated_write(root.as_ref(), &request)?);
        }
    }

    Ok(PatchApplyResult::new(plan, write_results))
}

/// Renders a patch plan as deterministic text.
#[must_use]
pub fn render_patch_plan(plan: &PatchPlan) -> String {
    let mut lines = vec![
        "Monad patch planning and supervised apply plan".to_string(),
        String::new(),
        "Summary:".to_string(),
        format!("  change_set_id: {}", plan.change_set().id()),
        format!("  title: {}", plan.change_set().title()),
        format!("  planned_changes: {}", plan.change_set().changes().len()),
        format!("  warnings: {}", plan.validation().warning_count()),
        format!("  blocked: {}", plan.validation().blocked_count()),
        format!(
            "  requires_approval: {}",
            plan.approval_plan().requires_approval()
        ),
        format!("  apply_blocked: {}", plan.is_blocked()),
        String::new(),
        "Change set:".to_string(),
        format!("  description: {}", plan.change_set().description()),
    ];

    for change in plan.change_set().changes() {
        lines.push(format!(
            "  - [{}] {}",
            change.kind().as_str(),
            change.relative_path().display()
        ));
        lines.push(format!(
            "    before_hash: {}",
            change.before_hash().unwrap_or("none")
        ));
        lines.push(format!(
            "    after_hash: {}",
            change.after_hash().unwrap_or("none")
        ));
        lines.push(format!("    rationale: {}", change.rationale()));
    }

    lines.push(String::new());
    lines.push("Validation findings:".to_string());
    for finding in plan.validation().findings() {
        let path = finding
            .path()
            .map(|value| value.display().to_string())
            .unwrap_or_else(|| "<change-set>".to_string());
        lines.push(format!(
            "  - [{}] {} {}: {}",
            finding.severity().as_str(),
            finding.id(),
            path,
            finding.message()
        ));
    }

    lines.push(String::new());
    lines.push("Safety boundaries:".to_string());
    for boundary in plan.safety_boundaries() {
        lines.push(format!("  - {boundary}"));
    }

    lines.push(String::new());
    lines.push("No commands were executed.".to_string());
    lines.push("No remote patches were fetched.".to_string());
    lines.push("No AI providers were called.".to_string());
    lines.push("No user-owned source files were rewritten.".to_string());

    lines.join("\n")
}

/// Renders a patch plan as deterministic JSON.
#[must_use]
pub fn render_patch_plan_json(plan: &PatchPlan) -> String {
    let changes = plan
        .change_set()
        .changes()
        .iter()
        .map(render_change_json)
        .collect::<Vec<_>>()
        .join(",");
    let findings = plan
        .validation()
        .findings()
        .iter()
        .map(render_validation_finding_json)
        .collect::<Vec<_>>()
        .join(",");
    let safety = plan
        .safety_boundaries()
        .iter()
        .map(|boundary| format!("\"{}\"", json_escape(boundary)))
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"command\":\"patch\",\"mode\":\"dry-run\",\"change_set_id\":\"{}\",\"title\":\"{}\",\"planned_changes\":{},\"warnings\":{},\"blocked\":{},\"requires_approval\":{},\"apply_blocked\":{},\"changes\":[{}],\"validation_findings\":[{}],\"safety_boundaries\":[{}]}}",
        json_escape(plan.change_set().id()),
        json_escape(plan.change_set().title()),
        plan.change_set().changes().len(),
        plan.validation().warning_count(),
        plan.validation().blocked_count(),
        plan.approval_plan().requires_approval(),
        plan.is_blocked(),
        changes,
        findings,
        safety,
    )
}

/// Renders supervised apply results as text.
#[must_use]
pub fn render_patch_apply_result(result: &PatchApplyResult) -> String {
    let mut lines = vec![
        "Monad supervised patch apply result".to_string(),
        String::new(),
        "Plan summary:".to_string(),
        format!("  change_set_id: {}", result.plan().change_set().id()),
        format!(
            "  planned_changes: {}",
            result.plan().change_set().changes().len()
        ),
        format!(
            "  validation_blocked: {}",
            result.plan().validation().is_blocked()
        ),
        format!(
            "  policy_blocked: {}",
            result.plan().approval_plan().is_blocked()
        ),
        String::new(),
        "Write results:".to_string(),
    ];

    if result.write_results().is_empty() {
        lines.push("  - none; apply was blocked before write attempts".to_string());
    }

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
    lines.push("Safety notes:".to_string());
    lines.push("  - Apply is limited to generated Monad evidence artifacts.".to_string());
    lines.push("  - Existing files with different content are not overwritten.".to_string());
    lines.push(
        "  - User-owned source mutation remains blocked in this foundation slice.".to_string(),
    );
    lines.push("  - E19 generated-write approval gates are used for every write.".to_string());

    lines.join("\n")
}

/// Renders supervised apply results as JSON.
#[must_use]
pub fn render_patch_apply_result_json(result: &PatchApplyResult) -> String {
    let write_results = result
        .write_results()
        .iter()
        .map(render_gated_write_result_json)
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"command\":\"patch\",\"mode\":\"apply\",\"change_set_id\":\"{}\",\"planned_changes\":{},\"validation_blocked\":{},\"policy_blocked\":{},\"write_results\":[{}]}}",
        json_escape(result.plan().change_set().id()),
        result.plan().change_set().changes().len(),
        result.plan().validation().is_blocked(),
        result.plan().approval_plan().is_blocked(),
        write_results,
    )
}

fn evidence_write_requests(plan: &PatchPlan) -> Vec<GatedWriteRequest> {
    vec![
        GatedWriteRequest::new(PATCH_EVIDENCE_MARKDOWN_PATH, render_patch_plan(plan), true),
        GatedWriteRequest::new(PATCH_EVIDENCE_JSON_PATH, render_patch_plan_json(plan), true),
    ]
}

fn generated_patch_marker_content() -> String {
    [
        "# E20 Supervised Patch Apply Foundation",
        "",
        "This generated Monad artifact records that the repository has an E20 patch planning foundation.",
        "",
        "Implemented safety posture:",
        "",
        "- patch plans are deterministic;",
        "- dry-run output is available before apply;",
        "- conflict checks prevent silent overwrites;",
        "- generated writes require explicit `--yes`;",
        "- user-owned source mutation remains blocked in this foundation slice;",
        "- no remote patch fetching, arbitrary script execution, AI-provider calls, or destructive deletion occur.",
        "",
    ]
    .join("\n")
}

fn path_is_existing_conflict(root: &Path, change: &PatchFileChange) -> bool {
    let absolute = root.join(change.relative_path());

    if !absolute.exists() {
        return false;
    }

    let Some(after_content) = change.after_content() else {
        return true;
    };

    match fs::read_to_string(absolute) {
        Ok(existing) => existing != after_content,
        Err(_) => true,
    }
}

fn is_safe_relative_path(path: &Path) -> bool {
    !path.is_absolute()
        && path.components().all(|component| {
            !matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
}

fn is_blocked_write_result(result: &GatedWriteResult) -> bool {
    matches!(
        result,
        GatedWriteResult::Blocked(_) | GatedWriteResult::ApprovalRequired(_)
    )
}

fn render_change_json(change: &PatchFileChange) -> String {
    format!(
        "{{\"path\":\"{}\",\"kind\":\"{}\",\"before_hash\":{},\"after_hash\":{},\"rationale\":\"{}\"}}",
        json_escape(&change.relative_path().display().to_string()),
        change.kind().as_str(),
        json_option(change.before_hash()),
        json_option(change.after_hash()),
        json_escape(change.rationale()),
    )
}

fn render_validation_finding_json(finding: &PatchValidationFinding) -> String {
    let path = finding
        .path()
        .map(|value| value.display().to_string())
        .unwrap_or_default();

    format!(
        "{{\"id\":\"{}\",\"severity\":\"{}\",\"path\":\"{}\",\"message\":\"{}\"}}",
        json_escape(finding.id()),
        finding.severity().as_str(),
        json_escape(&path),
        json_escape(finding.message()),
    )
}

fn render_gated_write_result_json(result: &GatedWriteResult) -> String {
    match result {
        GatedWriteResult::Written(path) | GatedWriteResult::SkippedIdentical(path) => {
            format!(
                "{{\"status\":\"{}\",\"path\":\"{}\"}}",
                result.as_str(),
                json_escape(&path.display().to_string())
            )
        }
        GatedWriteResult::ApprovalRequired(message) | GatedWriteResult::Blocked(message) => {
            format!(
                "{{\"status\":\"{}\",\"message\":\"{}\"}}",
                result.as_str(),
                json_escape(message)
            )
        }
    }
}

fn json_option(value: Option<&str>) -> String {
    value
        .map(|inner| format!("\"{}\"", json_escape(inner)))
        .unwrap_or_else(|| "null".to_string())
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn stable_content_hash(content: &str) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;

    for byte in content.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }

    format!("fnv1a64:{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn patch_plan_has_deterministic_change_set() {
        let root = unique_test_root("plan");
        let plan = build_patch_plan(&root);

        assert_eq!(plan.change_set().id(), "e20-supervised-apply-foundation");
        assert_eq!(plan.change_set().changes().len(), 1);
        assert_eq!(
            plan.change_set().changes()[0].relative_path(),
            Path::new(GENERATED_PATCH_MARKER_PATH)
        );
        assert!(!plan.is_blocked());
    }

    #[test]
    fn patch_plan_renders_safety_boundaries() {
        let root = unique_test_root("render");
        let plan = build_patch_plan(&root);
        let rendered = render_patch_plan(&plan);

        assert!(rendered.contains("No autonomous patch application"));
        assert!(rendered.contains("No user-owned source files were rewritten"));
        assert!(rendered.contains("e20-supervised-apply-foundation"));
    }

    #[test]
    fn patch_plan_json_is_command_tagged() {
        let root = unique_test_root("json");
        let plan = build_patch_plan(&root);
        let rendered = render_patch_plan_json(&plan);

        assert!(rendered.contains("\"command\":\"patch\""));
        assert!(rendered.contains("\"mode\":\"dry-run\""));
        assert!(rendered.contains("\"planned_changes\":1"));
    }

    #[test]
    fn validation_blocks_parent_traversal() {
        let root = unique_test_root("unsafe-path");
        let change_set = PatchChangeSet::new(
            "unsafe",
            "unsafe path",
            "path traversal must be blocked",
            vec![PatchFileChange::create_generated_file(
                "../outside.md",
                "unsafe",
                "test unsafe path validation",
            )],
        );

        let report = validate_patch_change_set(&root, &change_set);

        assert!(report.is_blocked());
        assert!(
            report
                .findings()
                .iter()
                .any(|finding| finding.id() == "patch.path.unsafe")
        );
    }

    #[test]
    fn validation_blocks_user_owned_source_mutation() {
        let root = unique_test_root("source-mutation");
        let change_set = PatchChangeSet::new(
            "source",
            "source mutation",
            "source mutation must remain blocked",
            vec![PatchFileChange::update_user_owned_file(
                "src/main.rs",
                "fn main() {}",
                "fn main() { println!(\"changed\"); }",
                "test blocked source mutation",
            )],
        );

        let report = validate_patch_change_set(&root, &change_set);

        assert!(report.is_blocked());
        assert!(
            report
                .findings()
                .iter()
                .any(|finding| finding.id() == "patch.source-mutation.blocked")
        );
    }

    #[test]
    fn validation_blocks_existing_file_conflict() {
        let root = unique_test_root("conflict");
        let target = root.join(GENERATED_PATCH_MARKER_PATH);
        fs::create_dir_all(target.parent().expect("target should have parent"))
            .expect("parent directory should be created");
        fs::write(&target, "different content").expect("conflict fixture should be written");

        let plan = build_patch_plan(&root);

        assert!(plan.validation().is_blocked());
        assert!(
            plan.validation()
                .findings()
                .iter()
                .any(|finding| finding.id() == "patch.conflict.existing-file-differs")
        );
    }

    #[test]
    fn supervised_apply_writes_generated_artifacts() {
        let root = unique_test_root("apply");
        let result = apply_patch_plan(&root).expect("patch apply should complete");

        assert!(!result.plan().is_blocked());
        assert!(root.join(GENERATED_PATCH_MARKER_PATH).exists());
        assert!(root.join(PATCH_EVIDENCE_MARKDOWN_PATH).exists());
        assert!(root.join(PATCH_EVIDENCE_JSON_PATH).exists());
        assert!(
            result
                .write_results()
                .iter()
                .all(|write_result| !is_blocked_write_result(write_result))
        );
    }

    #[test]
    fn supervised_apply_result_json_is_command_tagged() {
        let root = unique_test_root("apply-json");
        let result = apply_patch_plan(&root).expect("patch apply should complete");
        let rendered = render_patch_apply_result_json(&result);

        assert!(rendered.contains("\"command\":\"patch\""));
        assert!(rendered.contains("\"mode\":\"apply\""));
        assert!(rendered.contains("write_results"));
    }

    #[test]
    fn stable_hash_is_deterministic() {
        assert_eq!(stable_content_hash("abc"), stable_content_hash("abc"));
        assert_ne!(stable_content_hash("abc"), stable_content_hash("abcd"));
    }

    fn unique_test_root(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after Unix epoch")
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("monad-patch-{name}-{}-{nanos}", std::process::id()));
        fs::remove_dir_all(&root).ok();
        root
    }
}
