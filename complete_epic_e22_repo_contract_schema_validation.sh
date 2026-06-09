#!/usr/bin/env bash
set -Eeuo pipefail

SCRIPT_NAME="complete_epic_e22_repo_contract_schema_validation.sh"
EPIC="E22"
STAMP="$(date +%Y%m%d-%H%M%S)"
BACKUP_DIR=".monad/script-backups/${EPIC}-${STAMP}"

log() {
  printf '[%s] %s\n' "$SCRIPT_NAME" "$1"
}

fail() {
  printf '[%s] ERROR: %s\n' "$SCRIPT_NAME" "$1" >&2
  exit 1
}

require_repo_root() {
  [[ -f "Cargo.toml" ]] || fail "Run this script from the repository root; Cargo.toml was not found."
  [[ -d "crates/monad-core/src" ]] || fail "crates/monad-core/src was not found."
  [[ -f "crates/monad-core/src/lib.rs" ]] || fail "crates/monad-core/src/lib.rs was not found."
  [[ -f "crates/monad-cli/src/main.rs" ]] || fail "crates/monad-cli/src/main.rs was not found."
}

backup_file() {
  local path="$1"
  if [[ -e "$path" ]]; then
    mkdir -p "$BACKUP_DIR/$(dirname "$path")"
    cp "$path" "$BACKUP_DIR/$path"
  fi
}

write_file_if_changed() {
  local path="$1"
  local tmp
  tmp="$(mktemp)"
  cat > "$tmp"
  if [[ -f "$path" ]] && cmp -s "$tmp" "$path"; then
    rm -f "$tmp"
    log "unchanged: $path"
    return 0
  fi
  backup_file "$path"
  mkdir -p "$(dirname "$path")"
  mv "$tmp" "$path"
  log "wrote: $path"
}

require_repo_root
mkdir -p "$BACKUP_DIR"

log "Writing E22 repo contract schema module"
write_file_if_changed "crates/monad-core/src/contract_schema.rs" <<'RS'
//! Repo contract schema and validation foundation.
//!
//! E22 formalizes Monad's repository contract boundary around `monad.toml`,
//! generated `monad.lock` state, schema migration planning, deterministic
//! validation reports, and smoke-testable evidence. The implementation remains
//! local-first and supervised: dry-run plans do not write, and `--yes` writes
//! only generated evidence/state through E19 generated-write approval gates.

use std::fs;
use std::path::{Path, PathBuf};

use crate::diagnostics::{Diagnostic, DiagnosticReport, Severity};
use crate::manifest::{CURRENT_MANIFEST_SCHEMA_VERSION, MonadManifest};
use crate::policy::{GatedWriteRequest, GatedWriteResult, gated_generated_write};
use crate::repo_contract::check_repository_contract;
use crate::workspace::WorkspaceContext;

/// Stable path for the generated root contract lock/state file.
pub const CONTRACT_LOCK_PATH: &str = "monad.lock";

/// Stable path for generated contract state under `.monad`.
pub const CONTRACT_GENERATED_STATE_PATH: &str = ".monad/state/repository-contract-state.json";

/// Stable path for the human-readable contract schema report.
pub const CONTRACT_SCHEMA_REPORT_PATH: &str = ".monad/reports/contract-schema-report.md";

/// Stable path for the machine-readable contract schema report.
pub const CONTRACT_SCHEMA_REPORT_JSON_PATH: &str = ".monad/reports/contract-schema-report.json";

/// Stable path for the human-readable schema migration plan.
pub const CONTRACT_SCHEMA_MIGRATION_PLAN_PATH: &str = ".monad/reports/contract-schema-migration-plan.md";

/// Kind of generated contract schema artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContractSchemaArtifactKind {
    /// Root generated lock/state file.
    LockState,

    /// Generated state snapshot under `.monad`.
    GeneratedState,

    /// Human-readable validation report.
    ReportMarkdown,

    /// Machine-readable validation report.
    ReportJson,

    /// Human-readable migration plan.
    MigrationPlan,
}

impl ContractSchemaArtifactKind {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LockState => "lock-state",
            Self::GeneratedState => "generated-state",
            Self::ReportMarkdown => "report-markdown",
            Self::ReportJson => "report-json",
            Self::MigrationPlan => "migration-plan",
        }
    }
}

/// One generated artifact in a contract schema plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractSchemaArtifact {
    kind: ContractSchemaArtifactKind,
    relative_path: PathBuf,
    description: String,
    content: String,
}

impl ContractSchemaArtifact {
    /// Creates a generated contract schema artifact.
    #[must_use]
    pub fn new(
        kind: ContractSchemaArtifactKind,
        relative_path: impl Into<PathBuf>,
        description: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            relative_path: relative_path.into(),
            description: description.into(),
            content: content.into(),
        }
    }

    /// Artifact kind.
    #[must_use]
    pub const fn kind(&self) -> ContractSchemaArtifactKind {
        self.kind
    }

    /// Artifact relative path.
    #[must_use]
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// Artifact description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Artifact content.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// Schema migration step status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContractMigrationStepStatus {
    /// No migration is required.
    NotRequired,

    /// Migration can be planned later, but is not applied by E22.
    Planned,

    /// Migration is blocked because it would be destructive or unsupported.
    Blocked,
}

impl ContractMigrationStepStatus {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not-required",
            Self::Planned => "planned",
            Self::Blocked => "blocked",
        }
    }
}

/// One schema migration planning record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractMigrationStep {
    id: String,
    from_schema_version: Option<u16>,
    to_schema_version: u16,
    status: ContractMigrationStepStatus,
    description: String,
}

impl ContractMigrationStep {
    /// Creates a schema migration step.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        from_schema_version: Option<u16>,
        to_schema_version: u16,
        status: ContractMigrationStepStatus,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            from_schema_version,
            to_schema_version,
            status,
            description: description.into(),
        }
    }

    /// Stable step ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Source schema version, if it could be detected.
    #[must_use]
    pub const fn from_schema_version(&self) -> Option<u16> {
        self.from_schema_version
    }

    /// Target schema version.
    #[must_use]
    pub const fn to_schema_version(&self) -> u16 {
        self.to_schema_version
    }

    /// Migration status.
    #[must_use]
    pub const fn status(&self) -> ContractMigrationStepStatus {
        self.status
    }

    /// Human-readable description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Generated state summarized by `monad.lock` and `.monad/state`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractGeneratedState {
    schema_version: u16,
    manifest_valid: bool,
    repository_contract_valid: bool,
    generated_by: String,
}

impl ContractGeneratedState {
    /// Creates a generated state snapshot.
    #[must_use]
    pub fn new(schema_version: u16, manifest_valid: bool, repository_contract_valid: bool) -> Self {
        Self {
            schema_version,
            manifest_valid,
            repository_contract_valid,
            generated_by: "monad contract".to_string(),
        }
    }

    /// Current contract schema version recorded in state.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Whether the manifest was valid.
    #[must_use]
    pub const fn manifest_valid(&self) -> bool {
        self.manifest_valid
    }

    /// Whether required repository contract paths were valid.
    #[must_use]
    pub const fn repository_contract_valid(&self) -> bool {
        self.repository_contract_valid
    }

    /// Renders deterministic JSON for generated state files.
    #[must_use]
    pub fn render_json(&self) -> String {
        format!(
            concat!(
                "{{\n",
                "  \"schema_version\": {},\n",
                "  \"generated_by\": \"{}\",\n",
                "  \"manifest_valid\": {},\n",
                "  \"repository_contract_valid\": {},\n",
                "  \"remote_calls\": false,\n",
                "  \"destructive_migrations\": false\n",
                "}}\n"
            ),
            self.schema_version,
            json_escape(&self.generated_by),
            self.manifest_valid,
            self.repository_contract_valid
        )
    }
}

/// Dry-run plan for contract schema validation and generated state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractSchemaPlan {
    schema_version: u16,
    manifest_path: PathBuf,
    lock_path: PathBuf,
    generated_state_path: PathBuf,
    diagnostics: DiagnosticReport,
    migration_steps: Vec<ContractMigrationStep>,
    generated_state: ContractGeneratedState,
    artifacts: Vec<ContractSchemaArtifact>,
}

impl ContractSchemaPlan {
    /// Creates a contract schema plan.
    #[must_use]
    pub fn new(
        schema_version: u16,
        diagnostics: DiagnosticReport,
        migration_steps: Vec<ContractMigrationStep>,
        generated_state: ContractGeneratedState,
        artifacts: Vec<ContractSchemaArtifact>,
    ) -> Self {
        Self {
            schema_version,
            manifest_path: PathBuf::from("monad.toml"),
            lock_path: PathBuf::from(CONTRACT_LOCK_PATH),
            generated_state_path: PathBuf::from(CONTRACT_GENERATED_STATE_PATH),
            diagnostics,
            migration_steps,
            generated_state,
            artifacts,
        }
    }

    /// Contract schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Manifest path.
    #[must_use]
    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }

    /// Generated lock path.
    #[must_use]
    pub fn lock_path(&self) -> &Path {
        &self.lock_path
    }

    /// Generated state path.
    #[must_use]
    pub fn generated_state_path(&self) -> &Path {
        &self.generated_state_path
    }

    /// Diagnostics.
    #[must_use]
    pub const fn diagnostics(&self) -> &DiagnosticReport {
        &self.diagnostics
    }

    /// Migration steps.
    #[must_use]
    pub fn migration_steps(&self) -> &[ContractMigrationStep] {
        &self.migration_steps
    }

    /// Generated state.
    #[must_use]
    pub const fn generated_state(&self) -> &ContractGeneratedState {
        &self.generated_state
    }

    /// Generated artifacts.
    #[must_use]
    pub fn artifacts(&self) -> &[ContractSchemaArtifact] {
        &self.artifacts
    }

    /// Error count.
    #[must_use]
    pub fn error_count(&self) -> usize {
        count_diagnostics_with_severity(&self.diagnostics, Severity::Error)
    }

    /// Warning count.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        count_diagnostics_with_severity(&self.diagnostics, Severity::Warning)
    }

    /// Whether the plan contains validation errors.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics.has_errors()
    }
}

/// Result of writing generated contract schema evidence/state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractSchemaApplyResult {
    results: Vec<GatedWriteResult>,
}

impl ContractSchemaApplyResult {
    /// Creates an apply result.
    #[must_use]
    pub fn new(results: Vec<GatedWriteResult>) -> Self {
        Self { results }
    }

    /// Individual generated-write results.
    #[must_use]
    pub fn results(&self) -> &[GatedWriteResult] {
        &self.results
    }

    /// Result count.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Whether any generated write was blocked.
    #[must_use]
    pub fn blocked_count(&self) -> usize {
        self.results
            .iter()
            .filter(|result| matches!(result, GatedWriteResult::Blocked(_)))
            .count()
    }
}

/// Builds a local dry-run contract schema plan.
#[must_use]
pub fn build_contract_schema_plan(context: &WorkspaceContext) -> ContractSchemaPlan {
    let manifest_path = context.root().join("monad.toml");
    let mut diagnostics = DiagnosticReport::new();

    diagnostics.push(Diagnostic::info(
        "MONAD6200",
        "repo contract schema boundary is local-first and generated-state only",
    ));

    let mut detected_schema_version = None;
    let mut manifest_valid = false;

    if manifest_path.is_file() {
        match fs::read_to_string(&manifest_path) {
            Ok(text) => {
                detected_schema_version = extract_schema_version(&text);
                match MonadManifest::from_toml_str(&text) {
                    Ok(manifest) => {
                        manifest_valid = true;
                        diagnostics.push(Diagnostic::info(
                            "MONAD6201",
                            "monad.toml parsed and validated successfully",
                        ));
                        push_report(&mut diagnostics, &manifest.diagnostics());
                    }
                    Err(error) => {
                        diagnostics.push(Diagnostic::error(
                            "MONAD6202",
                            format!("monad.toml schema validation failed: {error}"),
                        ));
                    }
                }
            }
            Err(error) => diagnostics.push(Diagnostic::error(
                "MONAD6203",
                format!("failed to read monad.toml: {error}"),
            )),
        }
    } else {
        diagnostics.push(Diagnostic::error(
            "MONAD6204",
            "monad.toml is required for repository contract schema validation",
        ));
    }

    let repository_contract_report = check_repository_contract(context);
    let repository_contract_valid = !repository_contract_report.has_errors();
    push_report(&mut diagnostics, &repository_contract_report);

    if context.root().join(CONTRACT_LOCK_PATH).is_file() {
        diagnostics.push(Diagnostic::info(
            "MONAD6210",
            "monad.lock exists; E22 will not overwrite differing content silently",
        ));
    } else {
        diagnostics.push(Diagnostic::warning(
            "MONAD6211",
            "monad.lock is not present yet; --yes can write generated lock state",
        ));
    }

    let migration_steps = build_migration_steps(detected_schema_version);
    for step in &migration_steps {
        if step.status() == ContractMigrationStepStatus::Blocked {
            diagnostics.push(Diagnostic::error(
                "MONAD6220",
                format!(
                    "schema migration step {} is blocked: {}",
                    step.id(),
                    step.description()
                ),
            ));
        }
    }

    let schema_version = detected_schema_version.unwrap_or(CURRENT_MANIFEST_SCHEMA_VERSION);
    let generated_state = ContractGeneratedState::new(
        CURRENT_MANIFEST_SCHEMA_VERSION,
        manifest_valid,
        repository_contract_valid,
    );

    let mut plan = ContractSchemaPlan::new(
        schema_version,
        diagnostics,
        migration_steps,
        generated_state,
        Vec::new(),
    );

    let artifacts = build_artifacts(&plan);
    plan.artifacts = artifacts;
    plan
}

/// Applies generated contract schema evidence/state after explicit approval.
pub fn apply_contract_schema_plan(
    context: &WorkspaceContext,
) -> Result<ContractSchemaApplyResult, String> {
    let plan = build_contract_schema_plan(context);
    let mut results = Vec::new();

    for artifact in plan.artifacts() {
        let is_state_artifact = matches!(
            artifact.kind(),
            ContractSchemaArtifactKind::LockState | ContractSchemaArtifactKind::GeneratedState
        );

        if is_state_artifact && plan.has_errors() {
            results.push(GatedWriteResult::Blocked(format!(
                "refusing to write {} while contract schema validation has errors",
                artifact.relative_path().display()
            )));
            continue;
        }

        let request = GatedWriteRequest::new(artifact.relative_path(), artifact.content(), true);
        results.push(gated_generated_write(context.root(), &request)?);
    }

    Ok(ContractSchemaApplyResult::new(results))
}

/// Renders a human-readable contract schema plan.
#[must_use]
pub fn render_contract_schema_plan(plan: &ContractSchemaPlan) -> String {
    let mut lines = vec![
        "Monad repo contract schema validation plan".to_string(),
        String::new(),
        "Summary:".to_string(),
        format!("  command: contract"),
        format!("  mode: dry-run"),
        format!("  supported_schema_version: {CURRENT_MANIFEST_SCHEMA_VERSION}"),
        format!("  detected_schema_version: {}", plan.schema_version()),
        format!("  diagnostics: {}", plan.diagnostics().len()),
        format!("  warnings: {}", plan.warning_count()),
        format!("  errors: {}", plan.error_count()),
        format!("  migration_steps: {}", plan.migration_steps().len()),
        format!("  generated_artifacts: {}", plan.artifacts().len()),
        String::new(),
        "Contract paths:".to_string(),
        format!("  manifest: {}", plan.manifest_path().display()),
        format!("  lock_state: {}", plan.lock_path().display()),
        format!(
            "  generated_state: {}",
            plan.generated_state_path().display()
        ),
        String::new(),
        "Diagnostics:".to_string(),
    ];

    for line in plan.diagnostics().render_lines() {
        lines.push(format!("  - {line}"));
    }

    lines.push(String::new());
    lines.push("Migration plan:".to_string());
    for step in plan.migration_steps() {
        lines.push(format!(
            "  - {} from={} to={} status={} — {}",
            step.id(),
            format_optional_schema_version(step.from_schema_version()),
            step.to_schema_version(),
            step.status().as_str(),
            step.description()
        ));
    }

    lines.push(String::new());
    lines.push("Generated artifacts:".to_string());
    for artifact in plan.artifacts() {
        lines.push(format!(
            "  - {} [{}] {}",
            artifact.relative_path().display(),
            artifact.kind().as_str(),
            artifact.description()
        ));
    }

    lines.push(String::new());
    lines.push("Safety:".to_string());
    lines.push("  - dry-run does not write files".to_string());
    lines.push("  - --yes writes generated contract evidence/state only".to_string());
    lines.push("  - existing differing files are not silently overwritten".to_string());
    lines.push("  - no remote services, AI providers, package publishing, or destructive migrations are used".to_string());

    lines.join("\n")
}

/// Renders a machine-readable contract schema plan.
#[must_use]
pub fn render_contract_schema_plan_json(plan: &ContractSchemaPlan) -> String {
    let diagnostics = plan
        .diagnostics()
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            format!(
                "{{\"severity\":\"{}\",\"code\":\"{}\",\"message\":\"{}\"}}",
                diagnostic.severity.as_str(),
                diagnostic.code,
                json_escape(&diagnostic.message)
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let migration_steps = plan
        .migration_steps()
        .iter()
        .map(|step| {
            format!(
                "{{\"id\":\"{}\",\"from_schema_version\":{},\"to_schema_version\":{},\"status\":\"{}\",\"description\":\"{}\"}}",
                json_escape(step.id()),
                optional_schema_version_json(step.from_schema_version()),
                step.to_schema_version(),
                step.status().as_str(),
                json_escape(step.description())
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let artifacts = plan
        .artifacts()
        .iter()
        .map(|artifact| {
            format!(
                "{{\"path\":\"{}\",\"kind\":\"{}\",\"description\":\"{}\"}}",
                json_escape(&artifact.relative_path().display().to_string()),
                artifact.kind().as_str(),
                json_escape(artifact.description())
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"command\":\"contract\",\"mode\":\"dry-run\",\"supported_schema_version\":{},\"detected_schema_version\":{},\"diagnostics\":{},\"warnings\":{},\"errors\":{},\"migration_steps\":[{}],\"artifacts\":[{}],\"findings\":[{}],\"remote_calls\":false,\"destructive_migrations\":false}}",
        CURRENT_MANIFEST_SCHEMA_VERSION,
        plan.schema_version(),
        plan.diagnostics().len(),
        plan.warning_count(),
        plan.error_count(),
        migration_steps,
        artifacts,
        diagnostics
    )
}

/// Renders contract schema apply results.
#[must_use]
pub fn render_contract_schema_apply_result(result: &ContractSchemaApplyResult) -> String {
    let mut lines = vec![
        "Monad repo contract schema apply result".to_string(),
        String::new(),
        "Summary:".to_string(),
        format!("  results: {}", result.result_count()),
        format!("  blocked: {}", result.blocked_count()),
        String::new(),
        "Results:".to_string(),
    ];

    for write_result in result.results() {
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
    lines.push("No user-owned source files were rewritten.".to_string());
    lines.push("No destructive migrations were run.".to_string());
    lines.push("No remote services or AI providers were called.".to_string());

    lines.join("\n")
}

/// Renders contract schema apply results as JSON.
#[must_use]
pub fn render_contract_schema_apply_result_json(result: &ContractSchemaApplyResult) -> String {
    let results = result
        .results()
        .iter()
        .map(|write_result| match write_result {
            GatedWriteResult::Written(path) | GatedWriteResult::SkippedIdentical(path) => format!(
                "{{\"status\":\"{}\",\"path\":\"{}\"}}",
                write_result.as_str(),
                json_escape(&path.display().to_string())
            ),
            GatedWriteResult::ApprovalRequired(message) | GatedWriteResult::Blocked(message) => {
                format!(
                    "{{\"status\":\"{}\",\"message\":\"{}\"}}",
                    write_result.as_str(),
                    json_escape(message)
                )
            }
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"command\":\"contract\",\"mode\":\"apply\",\"results\":{},\"blocked\":{},\"items\":[{}],\"remote_calls\":false,\"destructive_migrations\":false}}",
        result.result_count(),
        result.blocked_count(),
        results
    )
}

fn build_migration_steps(detected_schema_version: Option<u16>) -> Vec<ContractMigrationStep> {
    let status = match detected_schema_version {
        Some(version) if version == CURRENT_MANIFEST_SCHEMA_VERSION => {
            ContractMigrationStepStatus::NotRequired
        }
        Some(version) if version < CURRENT_MANIFEST_SCHEMA_VERSION => ContractMigrationStepStatus::Planned,
        Some(_) => ContractMigrationStepStatus::Blocked,
        None => ContractMigrationStepStatus::Planned,
    };

    let description = match status {
        ContractMigrationStepStatus::NotRequired => "manifest schema is already current",
        ContractMigrationStepStatus::Planned => {
            "schema migration planning is recorded, but E22 does not mutate user manifests"
        }
        ContractMigrationStepStatus::Blocked => {
            "future or unsupported schemas are blocked until an explicit migration is implemented"
        }
    };

    vec![ContractMigrationStep::new(
        "contract-schema.v1",
        detected_schema_version,
        CURRENT_MANIFEST_SCHEMA_VERSION,
        status,
        description,
    )]
}

fn build_artifacts(plan: &ContractSchemaPlan) -> Vec<ContractSchemaArtifact> {
    let lock_json = plan.generated_state().render_json();
    let report_markdown = render_contract_schema_plan(plan);
    let report_json = render_contract_schema_plan_json(plan);
    let migration_markdown = render_migration_plan_markdown(plan);

    vec![
        ContractSchemaArtifact::new(
            ContractSchemaArtifactKind::LockState,
            CONTRACT_LOCK_PATH,
            "generated root contract lock/state file",
            lock_json.clone(),
        ),
        ContractSchemaArtifact::new(
            ContractSchemaArtifactKind::GeneratedState,
            CONTRACT_GENERATED_STATE_PATH,
            "generated repository contract state snapshot",
            lock_json,
        ),
        ContractSchemaArtifact::new(
            ContractSchemaArtifactKind::ReportMarkdown,
            CONTRACT_SCHEMA_REPORT_PATH,
            "human-readable contract schema validation report",
            report_markdown,
        ),
        ContractSchemaArtifact::new(
            ContractSchemaArtifactKind::ReportJson,
            CONTRACT_SCHEMA_REPORT_JSON_PATH,
            "machine-readable contract schema validation report",
            report_json,
        ),
        ContractSchemaArtifact::new(
            ContractSchemaArtifactKind::MigrationPlan,
            CONTRACT_SCHEMA_MIGRATION_PLAN_PATH,
            "human-readable schema migration planning report",
            migration_markdown,
        ),
    ]
}

fn render_migration_plan_markdown(plan: &ContractSchemaPlan) -> String {
    let mut lines = vec![
        "# Monad Contract Schema Migration Plan".to_string(),
        String::new(),
        "This generated report records migration planning only. E22 does not run destructive migrations and does not rewrite `monad.toml`.".to_string(),
        String::new(),
        "| Step | From | To | Status | Description |".to_string(),
        "| --- | ---: | ---: | --- | --- |".to_string(),
    ];

    for step in plan.migration_steps() {
        lines.push(format!(
            "| {} | {} | {} | {} | {} |",
            step.id(),
            format_optional_schema_version(step.from_schema_version()),
            step.to_schema_version(),
            step.status().as_str(),
            step.description()
        ));
    }

    lines.push(String::new());
    lines.push("Safety boundaries:".to_string());
    lines.push("- No destructive migrations are run.".to_string());
    lines.push("- No user-owned source files are rewritten.".to_string());
    lines.push("- No remote schema registry is contacted.".to_string());
    lines.push("- Future schema migration execution requires a later explicit epic.".to_string());
    lines.push(String::new());

    lines.join("\n")
}

fn extract_schema_version(input: &str) -> Option<u16> {
    input.lines().find_map(|line| {
        let trimmed = line.trim();
        let remainder = trimmed.strip_prefix("schema_version")?.trim();
        let value = remainder.strip_prefix('=')?.trim();
        value.parse::<u16>().ok()
    })
}

fn push_report(target: &mut DiagnosticReport, source: &DiagnosticReport) {
    for diagnostic in source.diagnostics() {
        target.push(diagnostic.clone());
    }
}

fn count_diagnostics_with_severity(report: &DiagnosticReport, severity: Severity) -> usize {
    report
        .diagnostics()
        .iter()
        .filter(|diagnostic| diagnostic.severity == severity)
        .count()
}

fn format_optional_schema_version(value: Option<u16>) -> String {
    value.map_or_else(|| "unknown".to_string(), |version| version.to_string())
}

fn optional_schema_version_json(value: Option<u16>) -> String {
    value.map_or_else(|| "null".to_string(), |version| version.to_string())
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
    use super::*;

    use std::time::{SystemTime, UNIX_EPOCH};

    const VALID_MANIFEST_TOML: &str = r#"
schema_version = 1

[project]
name = "monad"
display_name = "Monad"
description = "AI-native, repo-native, local-first Software Foundry OS."

[workspace]
root_markers = ["monad.toml", "Cargo.toml", ".monad", "work"]

[runtime]
core_crate = "monad-core"
cli_crate = "monad-cli"
execution_model = "local-first"
"#;

    fn unique_temp_dir(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "monad-contract-schema-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn create_valid_workspace(test_name: &str) -> PathBuf {
        let root = unique_temp_dir(test_name);

        fs::create_dir_all(root.join("docs")).expect("docs directory should be created");
        fs::create_dir_all(root.join("work")).expect("work directory should be created");
        fs::create_dir_all(root.join(".monad")).expect(".monad directory should be created");
        fs::create_dir_all(root.join("crates/monad-cli"))
            .expect("monad-cli directory should be created");
        fs::create_dir_all(root.join("crates/monad-core"))
            .expect("monad-core directory should be created");

        fs::write(root.join("monad.toml"), VALID_MANIFEST_TOML)
            .expect("manifest should be written");
        fs::write(root.join("Cargo.toml"), "[workspace]\n").expect("Cargo.toml should be written");

        root
    }

    #[test]
    fn valid_workspace_contract_schema_plan_has_no_errors() {
        let root = create_valid_workspace("valid-plan");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let plan = build_contract_schema_plan(&context);

        assert_eq!(plan.schema_version(), CURRENT_MANIFEST_SCHEMA_VERSION);
        assert!(!plan.has_errors());
        assert_eq!(plan.migration_steps().len(), 1);
        assert_eq!(plan.artifacts().len(), 5);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn missing_manifest_contract_schema_plan_has_errors() {
        let root = create_valid_workspace("missing-manifest");
        fs::remove_file(root.join("monad.toml")).expect("manifest should be removed");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let plan = build_contract_schema_plan(&context);

        assert!(plan.has_errors());
        assert!(render_contract_schema_plan(&plan).contains("MONAD6204"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn future_schema_version_is_blocked_for_migration() {
        let root = create_valid_workspace("future-schema");
        fs::write(
            root.join("monad.toml"),
            VALID_MANIFEST_TOML.replace("schema_version = 1", "schema_version = 999"),
        )
        .expect("future manifest should be written");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let plan = build_contract_schema_plan(&context);

        assert!(plan.has_errors());
        assert_eq!(
            plan.migration_steps()[0].status(),
            ContractMigrationStepStatus::Blocked
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn contract_schema_json_contains_stable_command() {
        let root = create_valid_workspace("json");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let plan = build_contract_schema_plan(&context);
        let json = render_contract_schema_plan_json(&plan);

        assert!(json.contains("\"command\":\"contract\""));
        assert!(json.contains("\"remote_calls\":false"));
        assert!(json.contains("\"destructive_migrations\":false"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn apply_writes_generated_contract_state_for_valid_workspace() {
        let root = create_valid_workspace("apply");
        let context = WorkspaceContext::new(&root).expect("workspace context should be created");
        let result = apply_contract_schema_plan(&context).expect("apply should succeed");

        assert_eq!(result.blocked_count(), 0);
        assert!(root.join(CONTRACT_LOCK_PATH).is_file());
        assert!(root.join(CONTRACT_SCHEMA_REPORT_PATH).is_file());
        assert!(root.join(CONTRACT_SCHEMA_REPORT_JSON_PATH).is_file());
        assert!(root.join(CONTRACT_SCHEMA_MIGRATION_PLAN_PATH).is_file());
        assert!(root.join(CONTRACT_GENERATED_STATE_PATH).is_file());

        fs::remove_dir_all(root).ok();
    }
}
RS

log "Patching monad-core lib exports"
backup_file "crates/monad-core/src/lib.rs"
python3 - <<'PY'
from pathlib import Path

path = Path("crates/monad-core/src/lib.rs")
text = path.read_text()

if "pub mod contract_schema;" not in text:
    anchor = "pub mod context;\n"
    if anchor not in text:
        raise SystemExit("Could not find lib.rs module insertion point: pub mod context;")
    text = text.replace(anchor, anchor + "pub mod contract_schema;\n", 1)

export = '''pub use contract_schema::{
    CONTRACT_GENERATED_STATE_PATH, CONTRACT_LOCK_PATH, CONTRACT_SCHEMA_MIGRATION_PLAN_PATH,
    CONTRACT_SCHEMA_REPORT_JSON_PATH, CONTRACT_SCHEMA_REPORT_PATH, ContractGeneratedState,
    ContractMigrationStep, ContractMigrationStepStatus, ContractSchemaApplyResult,
    ContractSchemaArtifact, ContractSchemaArtifactKind, ContractSchemaPlan,
    apply_contract_schema_plan, build_contract_schema_plan, render_contract_schema_apply_result,
    render_contract_schema_apply_result_json, render_contract_schema_plan,
    render_contract_schema_plan_json,
};
'''

if "pub use contract_schema::{" not in text:
    anchor = "pub use context::{\n"
    if anchor not in text:
        raise SystemExit("Could not find lib.rs export insertion point: pub use context::{")
    text = text.replace(anchor, export + anchor, 1)

path.write_text(text)
PY

log "Patching monad-cli command surface"
backup_file "crates/monad-cli/src/main.rs"
python3 - <<'PY'
from pathlib import Path

path = Path("crates/monad-cli/src/main.rs")
text = path.read_text()

def insert_before(text: str, anchor: str, addition: str, label: str) -> str:
    if addition.strip() in text:
        return text
    if anchor not in text:
        raise SystemExit(f"Could not find insertion point: {label}")
    return text.replace(anchor, addition + anchor, 1)

def insert_after(text: str, anchor: str, addition: str, label: str) -> str:
    if addition.strip() in text:
        return text
    if anchor not in text:
        raise SystemExit(f"Could not find insertion point: {label}")
    return text.replace(anchor, anchor + addition, 1)

# Add CliCommand::Contract.
contract_variant = '''
    /// Validate repository contract schema and generated state.
    Contract {
        /// Whether to run in dry-run mode.
        dry_run: bool,

        /// Whether to write generated contract schema evidence/state.
        yes: bool,

        /// Requested output format.
        output_format: OutputFormat,
    },

'''
text = insert_before(
    text,
    "    /// Generate provider-agnostic AI context and memory artifacts.\n",
    contract_variant,
    "CliCommand::Contract before AiContext",
)

# Permit --yes for contract.
if 'parts.first().copied() != Some("contract")' not in text:
    anchor = '            && parts.first().copied() != Some("policy")\n'
    if anchor not in text:
        raise SystemExit('Could not find --yes allow-list insertion point after policy')
    text = text.replace(anchor, anchor + '            && parts.first().copied() != Some("contract")\n', 1)

text = text.replace(
    "--yes is only supported for init, add, sync, upgrade, ai-context, and policy commands",
    "--yes is only supported for init, add, sync, upgrade, ai-context, policy, contract, work-packet, and patch commands",
)
text = text.replace(
    "--yes is only supported for init, add, sync, upgrade, ai-context, policy, patch, and work-packet commands",
    "--yes is only supported for init, add, sync, upgrade, ai-context, policy, contract, work-packet, and patch commands",
)

# Add parse branch.
parse_branch = '''            ["contract"] => {
                reject_write_for_non_context(write)?;
                require_contract_mode(dry_run, yes)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::Contract {
                    dry_run,
                    yes,
                    output_format,
                })
            }
            ["contract", other, ..] => {
                reject_write_for_non_context(write)?;
                Err(format!("unknown contract argument: {other}"))
            }
'''
text = insert_before(
    text,
    '            ["ai-context"] => {\n',
    parse_branch,
    'contract parse branch before ai-context',
)

# Add run match arm.
run_arm = '''        CliCommand::Contract {
            dry_run,
            yes,
            output_format,
        } => render_contract(dry_run, yes, output_format),
'''
text = insert_before(
    text,
    "        CliCommand::AiContext {\n",
    run_arm,
    "contract run match arm before AiContext",
)

# Add mode requirement helper.
require_fn = '''
/// Requires exactly one contract mode.
fn require_contract_mode(dry_run: bool, yes: bool) -> Result<(), String> {
    match (dry_run, yes) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => {
            Err("contract currently requires either --dry-run to preview or --yes to write generated contract evidence/state".to_string())
        }
        (true, true) => Err("contract accepts either --dry-run or --yes, not both".to_string()),
    }
}
'''
text = insert_before(
    text,
    "\n/// Requires exactly one AI context mode.\n",
    require_fn,
    "require_contract_mode before require_ai_context_mode",
)

# Add render function.
render_fn = '''
/// Renders or writes repository contract schema validation output.
fn render_contract(
    dry_run: bool,
    yes: bool,
    output_format: OutputFormat,
) -> Result<String, String> {
    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;

    if dry_run {
        let plan = monad_core::build_contract_schema_plan(&context);
        return match output_format {
            OutputFormat::Text => Ok(monad_core::render_contract_schema_plan(&plan)),
            OutputFormat::Json => Ok(monad_core::render_contract_schema_plan_json(&plan)),
        };
    }

    if yes {
        let result = monad_core::apply_contract_schema_plan(&context)?;
        return match output_format {
            OutputFormat::Text => Ok(monad_core::render_contract_schema_apply_result(&result)),
            OutputFormat::Json => Ok(monad_core::render_contract_schema_apply_result_json(&result)),
        };
    }

    Err("contract currently requires either --dry-run to preview or --yes to write generated contract evidence/state".to_string())
}
'''
text = insert_before(
    text,
    "\n/// Renders or writes AI context artifacts.\n",
    render_fn,
    "render_contract before render_ai_context",
)

# Help text additions.
if "  contract --dry-run" not in text:
    text = insert_after(
        text,
        '        "  policy --yes                            Write generated policy evidence",\n',
        '        "  contract --dry-run                      Preview repo contract schema validation",\n        "  contract --dry-run --format=json        Preview contract schema validation as JSON",\n        "  contract --yes                         Write generated contract evidence/state",\n',
        "contract help core commands",
    )

if '  monad contract --dry-run' not in text:
    text = insert_after(
        text,
        '        "  monad policy --dry-run --format=json",\n',
        '        "  monad contract --dry-run",\n        "  monad contract --dry-run --format=json",\n',
        "contract help examples",
    )

if "contract writes generated contract evidence/state only" not in text:
    text = insert_after(
        text,
        '        "  policy writes generated evidence only and never approves risky work automatically.",\n',
        '        "  contract writes generated contract evidence/state only and never rewrites monad.toml.",\n',
        "contract help safety note",
    )

# Add parser tests.
test_block = '''
    #[test]
    fn contract_dry_run_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "contract", "--dry-run"])
                .expect("contract dry-run should parse"),
            CliCommand::Contract {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "contract", "--dry-run", "--format=json"])
                .expect("contract json should parse"),
            CliCommand::Contract {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Json,
            }
        );
    }

    #[test]
    fn contract_yes_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "contract", "--yes"])
                .expect("contract yes should parse"),
            CliCommand::Contract {
                dry_run: false,
                yes: true,
                output_format: OutputFormat::Text,
            }
        );
    }

    #[test]
    fn contract_requires_mode() {
        let error = parse_arguments(&["monad", "contract"])
            .expect_err("contract should require a mode");

        assert!(error.contains("contract currently requires either --dry-run"));
    }

    #[test]
    fn contract_rejects_dry_run_and_yes_together() {
        let error = parse_arguments(&["monad", "contract", "--dry-run", "--yes"])
            .expect_err("contract should reject conflicting modes");

        assert!(error.contains("contract accepts either --dry-run or --yes"));
    }

'''
text = insert_before(
    text,
    "    #[test]\n    fn ai_context_dry_run_command_parses() {\n",
    test_block,
    "contract parser tests before ai_context tests",
)

path.write_text(text)
PY

log "Writing E22 docs"
write_file_if_changed "docs/contract-schema/README.md" <<'MD'
---
title: Repo Contract Schema and Validation
status: active
last_reviewed: 2026-06-09
---

# Repo Contract Schema and Validation

E22 establishes Monad's repository contract schema and validation foundation.

The goal is to make repository intent and generated state explicit without turning Monad into an autonomous migration engine.

## Command surface

```bash
monad contract --dry-run
monad contract --dry-run --format=json
monad contract --yes
```

## What E22 validates

- `monad.toml` exists and parses as the supported manifest schema.
- Manifest diagnostics from `monad-core` are included in a contract schema report.
- The initial repository contract path model remains satisfied.
- `monad.lock` is treated as generated state, not hand-authored source.
- Schema migration planning is reported but not executed destructively.

## Generated artifacts

`monad contract --yes` may write generated artifacts only:

- `monad.lock`
- `.monad/state/repository-contract-state.json`
- `.monad/reports/contract-schema-report.md`
- `.monad/reports/contract-schema-report.json`
- `.monad/reports/contract-schema-migration-plan.md`

All writes go through E19 generated-write approval gates. Existing files with different content are not silently overwritten.

## Safety boundaries

E22 does not:

- rewrite `monad.toml`;
- rewrite user-owned source files;
- run destructive migrations;
- contact a remote schema registry;
- call AI providers;
- execute arbitrary scripts;
- publish packages or releases.
MD

write_file_if_changed "docs/roadmap/epic-22-repo-contract-schema-validation.md" <<'MD'
---
title: E22 — Repo Contract Schema and Validation Foundation
status: implemented
last_reviewed: 2026-06-09
---

# E22 — Repo Contract Schema and Validation Foundation

## Scope

E22 implements the foundation for repository contract schema validation and generated state.

## Work packets

- WP-E22-001 — Define repository contract schema boundary
- WP-E22-002 — Add `monad.toml` schema validation
- WP-E22-003 — Add `monad.lock` / generated state model
- WP-E22-004 — Add schema migration planning model
- WP-E22-005 — Add contract validation fixtures
- WP-E22-006 — Add contract validation reports and smoke tests

## Delivered behavior

- Added `monad contract --dry-run` for deterministic human-readable validation plans.
- Added `monad contract --dry-run --format=json` for deterministic machine-readable plans.
- Added `monad contract --yes` for generated contract evidence/state only.
- Added a core `contract_schema` module with schema boundary, lock/generated-state model, migration planning, reports, and tests.
- Added local verification scripts for contract schema behavior and E22 closeout.

## Safety notes

E22 remains local-first and supervised. It does not mutate user-owned source, does not rewrite `monad.toml`, does not run destructive migrations, does not fetch remote schema definitions, and does not call AI providers.

## Verification

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-contract-schema.sh
tools/scripts/verify-e22.sh
```
MD

log "Writing E22 verification scripts"
write_file_if_changed "tools/scripts/verify-contract-schema.sh" <<'SH'
#!/usr/bin/env bash
set -Eeuo pipefail

printf '[verify-contract-schema] cargo test -p monad-core --lib contract_schema\n'
cargo test -p monad-core --lib contract_schema

printf '[verify-contract-schema] cargo run -p monad-cli -- contract --dry-run\n'
cargo run -p monad-cli -- contract --dry-run >/tmp/monad-contract-schema-dry-run.txt

grep -q 'Monad repo contract schema validation plan' /tmp/monad-contract-schema-dry-run.txt
grep -q 'Safety:' /tmp/monad-contract-schema-dry-run.txt

printf '[verify-contract-schema] cargo run -p monad-cli -- contract --dry-run --format=json\n'
cargo run -p monad-cli -- contract --dry-run --format=json >/tmp/monad-contract-schema-dry-run.json

grep -q '"command":"contract"' /tmp/monad-contract-schema-dry-run.json
grep -q '"remote_calls":false' /tmp/monad-contract-schema-dry-run.json
grep -q '"destructive_migrations":false' /tmp/monad-contract-schema-dry-run.json

printf '[verify-contract-schema] ok\n'
SH

write_file_if_changed "tools/scripts/verify-e22.sh" <<'SH'
#!/usr/bin/env bash
set -Eeuo pipefail

printf '[verify-e22] checking E22 files\n'
test -f crates/monad-core/src/contract_schema.rs
test -f docs/contract-schema/README.md
test -f docs/roadmap/epic-22-repo-contract-schema-validation.md
test -x tools/scripts/verify-contract-schema.sh

grep -q 'pub mod contract_schema;' crates/monad-core/src/lib.rs
grep -q 'ContractSchemaPlan' crates/monad-core/src/lib.rs
grep -q 'Contract {' crates/monad-cli/src/main.rs
grep -q 'contract --dry-run' crates/monad-cli/src/main.rs

printf '[verify-e22] running contract schema verification\n'
tools/scripts/verify-contract-schema.sh

printf '[verify-e22] ok\n'
SH

chmod +x tools/scripts/verify-contract-schema.sh tools/scripts/verify-e22.sh

log "Formatting Rust files"
cargo fmt

log "E22 script complete"
log "Backups are under: $BACKUP_DIR"
log "Next verification: cargo fmt --check && cargo test && cargo clippy --all-targets --all-features -- -D warnings && tools/scripts/verify-contract-schema.sh && tools/scripts/verify-e22.sh"
