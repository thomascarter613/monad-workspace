#!/usr/bin/env bash
set -euo pipefail

# Epic E18 — AI Context Memory and Provider-Agnostic Assistant Workflow Foundation
#
# Implements MVP-safe AI context/memory foundation:
#
#   monad ai-context --dry-run
#   monad ai-context --dry-run --format=json
#   monad ai-context --yes
#
# Safety:
# - AI is optional.
# - No paid AI subscription required.
# - No provider calls.
# - No repo data sent remotely.
# - No autonomous execution.
# - No automatic patch application.
# - No long-running agent daemon.
# - No MCP/plugin marketplace.

echo "==> Epic E18: AI Context Memory and Provider-Agnostic Assistant Workflow Foundation"

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

cd "$REPO_ROOT"

echo "==> Repo root: $REPO_ROOT"

AI_CONTEXT_FILE="crates/monad-core/src/ai_context.rs"
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
  docs/commands \
  docs/ai/memory \
  docs/architecture \
  docs/workflows \
  docs/verification \
  tools/scripts \
  work/learning/E18 \
  work/deliverables/E18 \
  .monad/script-backups/E18/EPIC-E18

BACKUP_STAMP="$(date +%Y%m%d%H%M%S)"
[ -f "$AI_CONTEXT_FILE" ] && cp "$AI_CONTEXT_FILE" ".monad/script-backups/E18/EPIC-E18/ai_context.rs.$BACKUP_STAMP.bak"
cp "$LIB_FILE" ".monad/script-backups/E18/EPIC-E18/lib.rs.$BACKUP_STAMP.bak"
cp "$CLI_FILE" ".monad/script-backups/E18/EPIC-E18/main.rs.$BACKUP_STAMP.bak"

cat > "$AI_CONTEXT_FILE" <<'EOF'
//! Provider-agnostic AI context, memory, and assistant handoff foundation.
//!
//! E18 intentionally does not call AI providers, manage secrets, run agents,
//! apply patches, or send repository data remotely. It only prepares
//! repo-native, deterministic context artifacts for supervised use.

use std::fs;
use std::path::{Path, PathBuf};

/// AI provider mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AiProviderMode {
    /// AI support is disabled.
    Disabled,

    /// Local model/runtime controlled by the user.
    Local,

    /// Self-hosted endpoint controlled by the user.
    SelfHosted,

    /// Hosted provider configured by the user.
    Hosted,
}

impl AiProviderMode {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Local => "local",
            Self::SelfHosted => "self-hosted",
            Self::Hosted => "hosted",
        }
    }
}

/// Provider configuration record.
///
/// This intentionally does not store API keys or secrets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiProviderConfig {
    name: String,
    mode: AiProviderMode,
    endpoint_hint: Option<String>,
    model_hint: Option<String>,
    secret_source_hint: Option<String>,
}

impl AiProviderConfig {
    /// Creates a provider config.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        mode: AiProviderMode,
        endpoint_hint: Option<String>,
        model_hint: Option<String>,
        secret_source_hint: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            mode,
            endpoint_hint,
            model_hint,
            secret_source_hint,
        }
    }

    /// Disabled/default config.
    #[must_use]
    pub fn disabled() -> Self {
        Self::new("disabled", AiProviderMode::Disabled, None, None, None)
    }

    /// Provider name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Provider mode.
    #[must_use]
    pub const fn mode(&self) -> AiProviderMode {
        self.mode
    }

    /// Optional endpoint hint.
    #[must_use]
    pub fn endpoint_hint(&self) -> Option<&str> {
        self.endpoint_hint.as_deref()
    }

    /// Optional model hint.
    #[must_use]
    pub fn model_hint(&self) -> Option<&str> {
        self.model_hint.as_deref()
    }

    /// Optional secret source hint.
    #[must_use]
    pub fn secret_source_hint(&self) -> Option<&str> {
        self.secret_source_hint.as_deref()
    }

    /// Validates safety constraints.
    #[must_use]
    pub fn validate(&self) -> Vec<String> {
        let mut warnings = Vec::new();

        if self.name.trim().is_empty() {
            warnings.push("provider name is empty".to_string());
        }

        for field in [
            self.endpoint_hint(),
            self.model_hint(),
            self.secret_source_hint(),
        ]
        .into_iter()
        .flatten()
        {
            if looks_like_secret(field) {
                warnings.push("provider config appears to contain a secret; store secrets outside committed config".to_string());
            }
        }

        warnings
    }
}

/// Repo-native memory record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryRecordKind {
    /// Durable decision or architectural context.
    Decision,

    /// User/team/project preference.
    Preference,

    /// Constraint or safety boundary.
    Constraint,

    /// Work status or handoff context.
    Status,

    /// Open question.
    Question,
}

impl MemoryRecordKind {
    /// Stable label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Decision => "decision",
            Self::Preference => "preference",
            Self::Constraint => "constraint",
            Self::Status => "status",
            Self::Question => "question",
        }
    }
}

/// Repo-native memory record schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRecord {
    id: String,
    kind: MemoryRecordKind,
    title: String,
    source: String,
    freshness: String,
    body: String,
}

impl MemoryRecord {
    /// Creates a memory record.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        kind: MemoryRecordKind,
        title: impl Into<String>,
        source: impl Into<String>,
        freshness: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            title: title.into(),
            source: source.into(),
            freshness: freshness.into(),
            body: body.into(),
        }
    }

    /// Record ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Record kind.
    #[must_use]
    pub const fn kind(&self) -> MemoryRecordKind {
        self.kind
    }

    /// Record title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Renders the record as Markdown with frontmatter.
    #[must_use]
    pub fn render_markdown(&self) -> String {
        format!(
            "---\nid: {}\nkind: {}\ntitle: {}\nsource: {}\nfreshness: {}\n---\n\n# {}\n\n{}\n",
            self.id,
            self.kind.as_str(),
            escape_frontmatter_scalar(&self.title),
            escape_frontmatter_scalar(&self.source),
            escape_frontmatter_scalar(&self.freshness),
            self.title,
            self.body
        )
    }
}

/// Generated AI context artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiContextArtifact {
    path: PathBuf,
    description: String,
    content: String,
}

impl AiContextArtifact {
    /// Creates an artifact.
    #[must_use]
    pub fn new(
        path: impl Into<PathBuf>,
        description: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            description: description.into(),
            content: content.into(),
        }
    }

    /// Relative output path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Content.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// AI context generation plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiContextPlan {
    provider_config: AiProviderConfig,
    artifacts: Vec<AiContextArtifact>,
    warnings: Vec<String>,
}

impl AiContextPlan {
    /// Creates a deterministic plan.
    #[must_use]
    pub fn new(
        provider_config: AiProviderConfig,
        mut artifacts: Vec<AiContextArtifact>,
        mut warnings: Vec<String>,
    ) -> Self {
        artifacts.sort_by(|left, right| left.path().cmp(right.path()));
        warnings.sort();

        Self {
            provider_config,
            artifacts,
            warnings,
        }
    }

    /// Provider config.
    #[must_use]
    pub const fn provider_config(&self) -> &AiProviderConfig {
        &self.provider_config
    }

    /// Planned artifacts.
    #[must_use]
    pub fn artifacts(&self) -> &[AiContextArtifact] {
        &self.artifacts
    }

    /// Warnings.
    #[must_use]
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Artifact count.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }
}

/// Result of applying AI context artifact generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiContextApplyResult {
    plan: AiContextPlan,
    written_paths: Vec<PathBuf>,
    skipped_paths: Vec<PathBuf>,
    conflicts: Vec<String>,
}

impl AiContextApplyResult {
    /// Creates result.
    #[must_use]
    pub fn new(
        plan: AiContextPlan,
        mut written_paths: Vec<PathBuf>,
        mut skipped_paths: Vec<PathBuf>,
        mut conflicts: Vec<String>,
    ) -> Self {
        written_paths.sort();
        skipped_paths.sort();
        conflicts.sort();

        Self {
            plan,
            written_paths,
            skipped_paths,
            conflicts,
        }
    }

    /// Original plan.
    #[must_use]
    pub const fn plan(&self) -> &AiContextPlan {
        &self.plan
    }

    /// Written paths.
    #[must_use]
    pub fn written_paths(&self) -> &[PathBuf] {
        &self.written_paths
    }

    /// Skipped paths.
    #[must_use]
    pub fn skipped_paths(&self) -> &[PathBuf] {
        &self.skipped_paths
    }

    /// Conflicts.
    #[must_use]
    pub fn conflicts(&self) -> &[String] {
        &self.conflicts
    }

    /// Whether conflicts exist.
    #[must_use]
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
}

/// Builds the AI context/memory plan.
#[must_use]
pub fn build_ai_context_plan(root: impl AsRef<Path>) -> AiContextPlan {
    let root = root.as_ref();
    let provider_config = AiProviderConfig::disabled();
    let mut warnings = provider_config.validate();

    if !root.join("monad.toml").is_file() {
        warnings.push("monad.toml is missing; AI context artifacts will still be generated as local templates".to_string());
    }

    let artifacts = vec![
        AiContextArtifact::new(
            ".monad/ai/provider-config.example.toml",
            "Provider-agnostic AI configuration example without secrets",
            provider_config_example(),
        ),
        AiContextArtifact::new(
            ".monad/ai/memory/README.md",
            "Repo-native memory directory documentation",
            memory_readme(),
        ),
        AiContextArtifact::new(
            ".monad/ai/memory/0001-project-memory-template.md",
            "Memory record template",
            MemoryRecord::new(
                "memory-0001",
                MemoryRecordKind::Decision,
                "Project memory template",
                "generated:E18",
                "template",
                "Replace this generated template with durable project memory.\n\nDo not store secrets here.\n",
            )
            .render_markdown(),
        ),
        AiContextArtifact::new(
            ".monad/context/ai-context-snapshot.md",
            "AI context snapshot",
            render_context_snapshot(root),
        ),
        AiContextArtifact::new(
            ".monad/context/work-packet-plan.md",
            "Supervised work-packet planning artifact",
            render_work_packet_plan(),
        ),
        AiContextArtifact::new(
            ".monad/context/assistant-handoff.md",
            "Provider-agnostic assistant handoff",
            render_assistant_handoff(root),
        ),
        AiContextArtifact::new(
            ".monad/reports/ai-context-report.md",
            "AI context evidence report",
            String::new(),
        ),
        AiContextArtifact::new(
            ".monad/reports/ai-context-report.json",
            "AI context evidence JSON",
            String::new(),
        ),
    ];

    AiContextPlan::new(provider_config, artifacts, warnings)
}

/// Applies generated AI context artifacts using guarded non-destructive writes.
pub fn apply_ai_context_plan(root: impl AsRef<Path>) -> Result<AiContextApplyResult, String> {
    let root = root.as_ref();
    let plan = build_ai_context_plan(root);
    let mut written_paths = Vec::new();
    let mut skipped_paths = Vec::new();
    let mut conflicts = Vec::new();

    for artifact in plan.artifacts() {
        if artifact.path() == Path::new(".monad/reports/ai-context-report.md")
            || artifact.path() == Path::new(".monad/reports/ai-context-report.json")
        {
            continue;
        }

        match guarded_write(root, artifact.path(), artifact.content()) {
            Ok(true) => written_paths.push(artifact.path().to_path_buf()),
            Ok(false) => skipped_paths.push(artifact.path().to_path_buf()),
            Err(error) => conflicts.push(format!("{}: {error}", artifact.path().display())),
        }
    }

    let mut result = AiContextApplyResult::new(plan, written_paths, skipped_paths, conflicts);

    if !result.has_conflicts() {
        write_ai_context_evidence(root, &result)?;
        result
            .written_paths
            .push(PathBuf::from(".monad/reports/ai-context-report.md"));
        result
            .written_paths
            .push(PathBuf::from(".monad/reports/ai-context-report.json"));
        result.written_paths.sort();
    }

    Ok(result)
}

fn guarded_write(root: &Path, relative_path: &Path, content: &str) -> Result<bool, String> {
    let absolute = root.join(relative_path);

    if let Some(parent) = absolute.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    if absolute.exists() {
        let existing = fs::read_to_string(&absolute).map_err(|error| error.to_string())?;
        if existing == content {
            return Ok(false);
        }

        return Err("refusing unsafe overwrite of existing file with different content".to_string());
    }

    fs::write(absolute, content).map_err(|error| error.to_string())?;
    Ok(true)
}

fn write_ai_context_evidence(root: &Path, result: &AiContextApplyResult) -> Result<(), String> {
    fs::create_dir_all(root.join(".monad/reports")).map_err(|error| error.to_string())?;

    fs::write(
        root.join(".monad/reports/ai-context-report.md"),
        render_ai_context_apply_result(result),
    )
    .map_err(|error| error.to_string())?;

    fs::write(
        root.join(".monad/reports/ai-context-report.json"),
        render_ai_context_apply_result_json(result),
    )
    .map_err(|error| error.to_string())?;

    Ok(())
}

fn provider_config_example() -> String {
    [
        "# Monad AI provider configuration example",
        "#",
        "# AI is optional. This file intentionally contains no secrets.",
        "# Store API keys outside committed config, for example in your shell,",
        "# a local secret manager, or an untracked environment file.",
        "",
        "default_provider = \"disabled\"",
        "",
        "[providers.disabled]",
        "mode = \"disabled\"",
        "model_hint = \"none\"",
        "",
        "[providers.local-example]",
        "mode = \"local\"",
        "endpoint_hint = \"http://localhost:11434\"",
        "model_hint = \"local-model\"",
        "secret_source_hint = \"none\"",
        "",
        "[providers.self-hosted-example]",
        "mode = \"self-hosted\"",
        "endpoint_hint = \"https://your-internal-ai.example\"",
        "model_hint = \"repo-assistant\"",
        "secret_source_hint = \"external-secret-manager\"",
        "",
    ]
    .join("\n")
}

fn memory_readme() -> String {
    [
        "# Monad AI Memory",
        "",
        "This directory stores repo-native memory records.",
        "",
        "Memory records are Markdown files with frontmatter.",
        "",
        "Supported memory kinds:",
        "",
        "- decision",
        "- preference",
        "- constraint",
        "- status",
        "- question",
        "",
        "Safety rules:",
        "",
        "- Do not store secrets.",
        "- Do not store private credentials.",
        "- Do not assume records are automatically sent anywhere.",
        "- Review memory before exporting to any assistant.",
        "",
    ]
    .join("\n")
}

fn render_context_snapshot(root: &Path) -> String {
    let repo_name = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown-repository");

    format!(
        "# AI Context Snapshot\n\nRepository: `{repo_name}`\n\n## Purpose\n\nThis generated snapshot gives a supervised assistant enough structure to understand the repository without relying on fragile chat history.\n\n## Safety\n\n- Local-first by default.\n- No provider calls were made.\n- No repository data was sent remotely.\n- No patches were applied.\n\n## Suggested use\n\nReview this file before sharing it with any AI assistant or provider.\n"
    )
}

fn render_work_packet_plan() -> String {
    [
        "# Work-Packet Planning Artifact",
        "",
        "Use this artifact to prepare supervised assistant work.",
        "",
        "## Work packet",
        "",
        "- ID:",
        "- Title:",
        "- Objective:",
        "- Scope:",
        "- Out of scope:",
        "- Expected files:",
        "- Verification commands:",
        "- Safety boundaries:",
        "",
        "## Assistant instructions",
        "",
        "- Propose a plan before edits.",
        "- Do not apply patches automatically.",
        "- Keep changes reviewable.",
        "- Preserve local-first behavior.",
        "- Avoid provider-specific assumptions.",
        "",
    ]
    .join("\n")
}

fn render_assistant_handoff(root: &Path) -> String {
    let repo_name = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown-repository");

    format!(
        "# Assistant Handoff\n\nRepository: `{repo_name}`\n\n## Operating mode\n\nSupervised assistant workflow only.\n\n## Boundaries\n\n- Do not execute autonomous agents.\n- Do not send repository data remotely by default.\n- Do not apply patches without human review.\n- Do not assume a paid AI subscription.\n- Do not require a specific AI provider.\n\n## Context files\n\n- `.monad/context/ai-context-snapshot.md`\n- `.monad/context/work-packet-plan.md`\n- `.monad/ai/memory/`\n\n## Verification\n\nRun project verification before accepting changes.\n"
    )
}

/// Renders dry-run text.
#[must_use]
pub fn render_ai_context_plan(plan: &AiContextPlan) -> String {
    let mut lines = vec![
        "Monad AI context dry-run plan".to_string(),
        String::new(),
        "Summary:".to_string(),
        format!("  provider_mode: {}", plan.provider_config().mode().as_str()),
        format!("  provider_name: {}", plan.provider_config().name()),
        format!("  planned_artifacts: {}", plan.artifact_count()),
        format!("  warnings: {}", plan.warnings().len()),
        String::new(),
        "Artifacts:".to_string(),
    ];

    for artifact in plan.artifacts() {
        lines.push(format!("  - {}", artifact.path().display()));
        lines.push(format!("    {}", artifact.description()));
    }

    if !plan.warnings().is_empty() {
        lines.push(String::new());
        lines.push("Warnings:".to_string());
        for warning in plan.warnings() {
            lines.push(format!("  - {warning}"));
        }
    }

    lines.push(String::new());
    lines.push("No provider calls were made.".to_string());
    lines.push("No repository data was sent remotely.".to_string());
    lines.push("No patches were applied.".to_string());
    lines.push("No autonomous agent was started.".to_string());
    lines.push("No files were written.".to_string());

    lines.join("\n")
}

/// Renders dry-run JSON.
#[must_use]
pub fn render_ai_context_plan_json(plan: &AiContextPlan) -> String {
    let artifacts = plan
        .artifacts()
        .iter()
        .map(|artifact| {
            format!(
                "{{\"path\":\"{}\",\"description\":\"{}\"}}",
                json_escape(&artifact.path().display().to_string()),
                json_escape(artifact.description())
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let warnings = plan
        .warnings()
        .iter()
        .map(|warning| format!("\"{}\"", json_escape(warning)))
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"command\":\"ai-context\",\"mode\":\"dry-run\",\"provider_mode\":\"{}\",\"provider_name\":\"{}\",\"artifacts\":[{}],\"warnings\":[{}]}}",
        plan.provider_config().mode().as_str(),
        json_escape(plan.provider_config().name()),
        artifacts,
        warnings
    )
}

/// Renders apply text.
#[must_use]
pub fn render_ai_context_apply_result(result: &AiContextApplyResult) -> String {
    let mut lines = vec![
        "Monad AI context apply result".to_string(),
        String::new(),
        format!("provider_mode: {}", result.plan().provider_config().mode().as_str()),
        format!("planned_artifacts: {}", result.plan().artifact_count()),
        format!("written_paths: {}", result.written_paths().len()),
        format!("skipped_paths: {}", result.skipped_paths().len()),
        format!("conflicts: {}", result.conflicts().len()),
        String::new(),
        "Written paths:".to_string(),
    ];

    for path in result.written_paths() {
        lines.push(format!("  - {}", path.display()));
    }

    if !result.skipped_paths().is_empty() {
        lines.push(String::new());
        lines.push("Skipped paths:".to_string());
        for path in result.skipped_paths() {
            lines.push(format!("  - {}", path.display()));
        }
    }

    if !result.conflicts().is_empty() {
        lines.push(String::new());
        lines.push("Conflicts:".to_string());
        for conflict in result.conflicts() {
            lines.push(format!("  - {conflict}"));
        }
    }

    lines.push(String::new());
    lines.push("No provider calls were made.".to_string());
    lines.push("No repository data was sent remotely.".to_string());
    lines.push("No patches were applied.".to_string());
    lines.push("No autonomous agent was started.".to_string());

    lines.join("\n")
}

/// Renders apply JSON.
#[must_use]
pub fn render_ai_context_apply_result_json(result: &AiContextApplyResult) -> String {
    let written = result
        .written_paths()
        .iter()
        .map(|path| format!("\"{}\"", json_escape(&path.display().to_string())))
        .collect::<Vec<_>>()
        .join(",");

    let skipped = result
        .skipped_paths()
        .iter()
        .map(|path| format!("\"{}\"", json_escape(&path.display().to_string())))
        .collect::<Vec<_>>()
        .join(",");

    let conflicts = result
        .conflicts()
        .iter()
        .map(|conflict| format!("\"{}\"", json_escape(conflict)))
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"command\":\"ai-context\",\"mode\":\"apply\",\"provider_mode\":\"{}\",\"written_paths\":[{}],\"skipped_paths\":[{}],\"conflicts\":[{}]}}",
        result.plan().provider_config().mode().as_str(),
        written,
        skipped,
        conflicts
    )
}

fn looks_like_secret(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("sk-") || lower.contains("api_key") || lower.contains("apikey") || lower.contains("secret")
}

fn escape_frontmatter_scalar(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\\\""))
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

        std::env::temp_dir().join(format!("monad-ai-context-{name}-{unique}"))
    }

    #[test]
    fn provider_config_defaults_to_disabled() {
        let config = AiProviderConfig::disabled();

        assert_eq!(config.mode(), AiProviderMode::Disabled);
        assert!(config.validate().is_empty());
    }

    #[test]
    fn provider_config_warns_about_secret_like_values() {
        let config = AiProviderConfig::new(
            "hosted",
            AiProviderMode::Hosted,
            Some("https://example.test".to_string()),
            Some("model".to_string()),
            Some("api_key=sk-example".to_string()),
        );

        assert_eq!(config.validate().len(), 1);
    }

    #[test]
    fn memory_record_renders_frontmatter() {
        let record = MemoryRecord::new(
            "m1",
            MemoryRecordKind::Decision,
            "Use Rust",
            "test",
            "current",
            "Rust is the runtime language.",
        );
        let output = record.render_markdown();

        assert!(output.contains("kind: decision"));
        assert!(output.contains("# Use Rust"));
    }

    #[test]
    fn ai_context_plan_contains_expected_artifacts() {
        let root = unique_temp_root("plan");
        fs::create_dir_all(&root).expect("temp root should be created");

        let plan = build_ai_context_plan(&root);

        assert!(plan.artifact_count() >= 6);
        assert!(plan
            .artifacts()
            .iter()
            .any(|artifact| artifact.path() == Path::new(".monad/context/assistant-handoff.md")));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn ai_context_apply_writes_generated_artifacts() {
        let root = unique_temp_root("apply");
        fs::create_dir_all(&root).expect("temp root should be created");

        let result = apply_ai_context_plan(&root).expect("apply should succeed");

        assert!(!result.has_conflicts());
        assert!(root.join(".monad/ai/provider-config.example.toml").is_file());
        assert!(root.join(".monad/ai/memory/README.md").is_file());
        assert!(root.join(".monad/context/assistant-handoff.md").is_file());
        assert!(root.join(".monad/reports/ai-context-report.md").is_file());

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn ai_context_apply_refuses_unsafe_overwrite() {
        let root = unique_temp_root("conflict");
        fs::create_dir_all(root.join(".monad/context")).expect("context dir should be created");
        fs::write(root.join(".monad/context/assistant-handoff.md"), "user-owned\n")
            .expect("conflicting file should be written");

        let result = apply_ai_context_plan(&root).expect("apply should return conflict result");

        assert!(result.has_conflicts());

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn ai_context_plan_json_contains_core_fields() {
        let root = unique_temp_root("json");
        fs::create_dir_all(&root).expect("temp root should be created");

        let plan = build_ai_context_plan(&root);
        let output = render_ai_context_plan_json(&plan);

        assert!(output.contains("\"command\":\"ai-context\""));
        assert!(output.contains("\"mode\":\"dry-run\""));
        assert!(output.contains("\"provider_mode\":\"disabled\""));

        fs::remove_dir_all(root).ok();
    }
}
EOF

python3 <<'PY'
from pathlib import Path
import re

LIB = Path("crates/monad-core/src/lib.rs")
CLI = Path("crates/monad-cli/src/main.rs")


def insert_before_first(text: str, markers: list[str], insertion: str, label: str) -> str:
    if insertion.strip() in text:
        return text

    for marker in markers:
        index = text.find(marker)
        if index != -1:
            return text[:index] + insertion + text[index:]

    raise SystemExit(f"ERROR: could not find insertion point for {label}")


def add_imports_to_monad_core_use(text: str, names: list[str]) -> str:
    match = re.search(r"use monad_core::\{(?P<body>.*?)\};", text, re.DOTALL)
    if not match:
        raise SystemExit("ERROR: could not find monad_core import block in CLI")

    body = match.group("body")
    missing = [name for name in names if name not in body]

    if not missing:
        return text

    addition = "\n    " + ", ".join(missing) + ","
    return text[:match.start("body")] + body + addition + text[match.end("body"):]


def ensure_cli_enum_variant_commas(text: str) -> str:
    enum_start = text.find("enum CliCommand {")
    if enum_start == -1:
        return text

    brace_start = text.find("{", enum_start)
    if brace_start == -1:
        return text

    depth = 0
    enum_end = None
    for index in range(brace_start, len(text)):
        char = text[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                enum_end = index
                break

    if enum_end is None:
        return text

    before = text[:brace_start + 1]
    body = text[brace_start + 1:enum_end]
    after = text[enum_end:]

    lines = body.splitlines(keepends=True)
    fixed = []
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped == "}":
            next_non_empty = ""
            for future in lines[i + 1:]:
                if future.strip():
                    next_non_empty = future.strip()
                    break
            if next_non_empty.startswith("///") or next_non_empty.startswith("#[") or next_non_empty[:1].isupper():
                line = line.rstrip("\n") + ",\n"
        fixed.append(line)

    return before + "".join(fixed) + after


# Patch lib.rs.
lib = LIB.read_text()

if "pub mod ai_context;" not in lib:
    lib = insert_before_first(
        lib,
        ["pub mod agents;", "pub mod checks;", "pub mod component_add;"],
        "pub mod ai_context;\n",
        "ai_context module declaration",
    )

if "pub use ai_context::" not in lib:
    export = """pub use ai_context::{
    AiContextApplyResult, AiContextArtifact, AiContextPlan, AiProviderConfig, AiProviderMode,
    MemoryRecord, MemoryRecordKind, apply_ai_context_plan, build_ai_context_plan,
    render_ai_context_apply_result, render_ai_context_apply_result_json, render_ai_context_plan,
    render_ai_context_plan_json,
};
"""
    lib = insert_before_first(
        lib,
        ["pub use checks::{", "pub use context::{", "pub use dependency_detection::{"],
        export,
        "ai_context public exports",
    )

LIB.write_text(lib)


# Patch CLI.
cli = CLI.read_text()

cli = add_imports_to_monad_core_use(
    cli,
    [
        "apply_ai_context_plan",
        "build_ai_context_plan",
        "render_ai_context_apply_result",
        "render_ai_context_apply_result_json",
        "render_ai_context_plan",
        "render_ai_context_plan_json",
    ],
)

if "AiContext {" not in cli:
    variant = """    /// Generate provider-agnostic AI context and memory artifacts.
    AiContext {
        /// Whether to run in dry-run mode.
        dry_run: bool,

        /// Whether to write generated AI context artifacts.
        yes: bool,

        /// Requested output format.
        output_format: OutputFormat,
    }

"""
    cli = insert_before_first(
        cli,
        [
            "    /// Plan or apply safe repository upgrades.\n",
            "    Upgrade {\n",
            "    /// Plan release readiness without publishing.\n",
            "    Release {\n",
            "    /// Diagnose local environment and repository readiness.\n",
            "    Doctor {\n",
        ],
        variant,
        "CliCommand::AiContext variant",
    )

# Allow --yes for ai-context.
if 'Some("ai-context")' not in cli:
    cli = cli.replace(
        'parts.first().copied() != Some("upgrade")',
        'parts.first().copied() != Some("upgrade")\n            && parts.first().copied() != Some("ai-context")',
        1,
    )
    cli = cli.replace(
        "--yes is only supported for init, add, sync, and upgrade commands",
        "--yes is only supported for init, add, sync, upgrade, and ai-context commands",
    )

if '["ai-context"] => {' not in cli:
    parse_arm = """            ["ai-context"] => {
                reject_write_for_non_context(write)?;
                require_ai_context_mode(dry_run, yes)?;
                let output_format = parse_output_format_or_default(requested_format.as_deref())?;
                Ok(Self::AiContext {
                    dry_run,
                    yes,
                    output_format,
                })
            }
            ["ai-context", other, ..] => {
                reject_write_for_non_context(write)?;
                Err(format!("unknown ai-context argument: {other}"))
            }
"""
    cli = insert_before_first(
        cli,
        [
            '            ["upgrade"] => {\n',
            '            ["release"] => {\n',
            '            ["doctor"] => {\n',
            '            ["sync"] => {\n',
        ],
        parse_arm,
        "ai-context parse arm",
    )

if "CliCommand::AiContext" not in cli.split("match command", 1)[-1]:
    run_arm = """        CliCommand::AiContext {
            dry_run,
            yes,
            output_format,
        } => render_ai_context(dry_run, yes, output_format),
"""
    cli = insert_before_first(
        cli,
        [
            "        CliCommand::Upgrade {\n",
            "        CliCommand::Release {\n",
            "        CliCommand::Doctor { output_format } => render_doctor(output_format),\n",
        ],
        run_arm,
        "ai-context run arm",
    )

if "fn require_ai_context_mode" not in cli:
    helper = """/// Requires exactly one AI context mode.
fn require_ai_context_mode(dry_run: bool, yes: bool) -> Result<(), String> {
    match (dry_run, yes) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => {
            Err("ai-context currently requires either --dry-run to preview or --yes to write generated local artifacts".to_string())
        }
        (true, true) => Err("ai-context accepts either --dry-run or --yes, not both".to_string()),
    }
}

"""
    cli = insert_before_first(
        cli,
        [
            "/// Requires exactly one upgrade mode.\n",
            "fn require_upgrade_mode",
            "/// Requires dry-run mode for the first release foundation.\n",
            "fn require_release_mode",
            "/// Requires exactly one sync mode for the guarded sync implementation.\n",
            "fn require_sync_mode",
        ],
        helper,
        "require_ai_context_mode helper",
    )

if "fn render_ai_context(" not in cli:
    renderer = """/// Renders or writes AI context artifacts.
fn render_ai_context(
    dry_run: bool,
    yes: bool,
    output_format: OutputFormat,
) -> Result<String, String> {
    let root = std::env::current_dir().map_err(|error| error.to_string())?;

    if dry_run {
        let plan = build_ai_context_plan(&root);
        return match output_format {
            OutputFormat::Text => Ok(render_ai_context_plan(&plan)),
            OutputFormat::Json => Ok(render_ai_context_plan_json(&plan)),
        };
    }

    if yes {
        let result = apply_ai_context_plan(&root)?;
        return match output_format {
            OutputFormat::Text => Ok(render_ai_context_apply_result(&result)),
            OutputFormat::Json => Ok(render_ai_context_apply_result_json(&result)),
        };
    }

    Err("ai-context currently requires either --dry-run to preview or --yes to write generated local artifacts".to_string())
}

"""
    cli = insert_before_first(
        cli,
        [
            "/// Renders or applies repository upgrade output.\n",
            "fn render_upgrade",
            "/// Renders release readiness planning output.\n",
            "fn render_release",
            "/// Renders doctor diagnostics.\n",
            "fn render_doctor",
        ],
        renderer,
        "render_ai_context helper",
    )

if "ai-context --dry-run" not in cli:
    cli = cli.replace(
        '        "  upgrade --dry-run                       Preview safe repository upgrade steps",\n',
        '        "  upgrade --dry-run                       Preview safe repository upgrade steps",\n        "  ai-context --dry-run                    Preview AI context/memory artifacts",\n        "  ai-context --dry-run --format=json      Preview AI context plan as JSON",\n        "  ai-context --yes                        Write generated local AI context artifacts",\n',
        1,
    )
    cli = cli.replace(
        '        "  monad upgrade --dry-run",\n',
        '        "  monad upgrade --dry-run",\n        "  monad ai-context --dry-run",\n        "  monad ai-context --dry-run --format=json",\n',
        1,
    )
    cli = cli.replace(
        '        "  upgrade writes generated metadata/evidence only after --yes.",\n',
        '        "  upgrade writes generated metadata/evidence only after --yes.",\n        "  ai-context never calls providers or sends repo data remotely.",\n',
        1,
    )

if "fn ai_context_dry_run_command_parses" not in cli and "parse_arguments(&" in cli:
    tests = """    #[test]
    fn ai_context_dry_run_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "ai-context", "--dry-run"])
                .expect("ai-context dry-run should parse"),
            CliCommand::AiContext {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Text,
            }
        );

        assert_eq!(
            parse_arguments(&["monad", "ai-context", "--dry-run", "--format=json"])
                .expect("ai-context json should parse"),
            CliCommand::AiContext {
                dry_run: true,
                yes: false,
                output_format: OutputFormat::Json,
            }
        );
    }

    #[test]
    fn ai_context_yes_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "ai-context", "--yes"])
                .expect("ai-context yes should parse"),
            CliCommand::AiContext {
                dry_run: false,
                yes: true,
                output_format: OutputFormat::Text,
            }
        );
    }

    #[test]
    fn ai_context_requires_mode() {
        let error = parse_arguments(&["monad", "ai-context"])
            .expect_err("ai-context should require mode");

        assert!(error.contains("ai-context currently requires either --dry-run"));
    }

"""
    cli = insert_before_first(
        cli,
        [
            "    #[test]\n    fn upgrade_dry_run_command_parses()",
            "    #[test]\n    fn release_command_parses_text_and_json_formats()",
            "    #[test]\n    fn doctor_command_parses_text_and_json_formats()",
        ],
        tests,
        "ai-context parser tests",
    )

cli = ensure_cli_enum_variant_commas(cli)
CLI.write_text(cli)
PY

cat > docs/commands/AI-CONTEXT.md <<'EOF'
---
title: monad ai-context
status: complete
epic: E18
---

# `monad ai-context`

`monad ai-context` prepares provider-agnostic, repo-native AI context and memory artifacts.

## Commands

```bash
monad ai-context --dry-run
monad ai-context --dry-run --format=json
monad ai-context --yes
```

## Safety contract

`ai-context` does not:

- require a paid AI subscription;
- hard-code one hosted provider;
- send repository data remotely by default;
- call AI providers;
- execute autonomous agents;
- apply patches;
- run a long-lived daemon;
- install MCP/plugin marketplace dependencies.

## Generated local artifacts

```text
.monad/ai/provider-config.example.toml
.monad/ai/memory/README.md
.monad/ai/memory/0001-project-memory-template.md
.monad/context/ai-context-snapshot.md
.monad/context/work-packet-plan.md
.monad/context/assistant-handoff.md
.monad/reports/ai-context-report.md
.monad/reports/ai-context-report.json
```
EOF

cat > docs/ai/PROVIDER-CONFIGURATION.md <<'EOF'
---
title: AI Provider Configuration
status: complete
epic: E18
---

# AI Provider Configuration

AI usage in Monad is optional and provider-agnostic.

Supported modes:

- disabled
- local
- self-hosted
- hosted

Provider config must not store secrets directly in committed files.

Secrets should live outside repo-committed config, such as:

- shell environment;
- local secret manager;
- untracked local file;
- organization-managed secret store.

E18 does not call providers.
EOF

cat > docs/ai/MEMORY-SCHEMA.md <<'EOF'
---
title: AI Memory Schema
status: complete
epic: E18
---

# AI Memory Schema

Monad memory records are Markdown files with frontmatter.

## Kinds

- decision
- preference
- constraint
- status
- question

## Required frontmatter

```yaml
id: memory-0001
kind: decision
title: Example decision
source: generated:E18
freshness: current
```

## Safety

Do not store secrets.

Review memory before exporting context to any assistant or provider.
EOF

cat > docs/ai/ASSISTANT-HANDOFF.md <<'EOF'
---
title: Assistant Handoff
status: complete
epic: E18
---

# Assistant Handoff

E18 assistant handoff is supervised and provider-agnostic.

The generated handoff file gives a human-reviewed assistant enough context to help without relying on fragile chat history.

Use:

```bash
monad ai-context --yes
```

Then review:

```text
.monad/context/assistant-handoff.md
```

Before sharing with any assistant, remove anything private or sensitive.
EOF

cat > docs/ai/CONTEXT-SNAPSHOT.md <<'EOF'
---
title: AI Context Snapshot
status: complete
epic: E18
---

# AI Context Snapshot

The AI context snapshot is a local Markdown artifact intended for supervised assistant sessions.

It records:

- repo name;
- safety boundaries;
- suggested use;
- generated context locations.

It does not send data anywhere.
EOF

cat > docs/ai/memory/README.md <<'EOF'
---
title: AI Memory Directory
status: complete
epic: E18
---

# AI Memory Directory

Repo-native memory records live under:

```text
.monad/ai/memory/
```

The checked-in docs explain the schema. Generated memory records are local artifacts.
EOF

cat > docs/architecture/AI-CONTEXT-MODEL.md <<'EOF'
---
title: AI Context Model
status: complete
epic: E18
---

# AI Context Model

Core types:

```text
AiProviderConfig
AiProviderMode
MemoryRecord
MemoryRecordKind
AiContextArtifact
AiContextPlan
AiContextApplyResult
```

The model is provider-agnostic and local-first.

E18 generates context artifacts only. It does not call models or run agents.
EOF

cat > docs/workflows/AI-CONTEXT-WORKFLOW.md <<'EOF'
---
title: AI Context Workflow
status: complete
epic: E18
---

# AI Context Workflow

## 1. Preview

```bash
monad ai-context --dry-run
```

## 2. Preview as JSON

```bash
monad ai-context --dry-run --format=json
```

## 3. Generate local artifacts

```bash
monad ai-context --yes
```

## 4. Review before sharing

Review:

```text
.monad/context/assistant-handoff.md
.monad/context/ai-context-snapshot.md
.monad/ai/memory/
```

Do not share secrets or private data.
EOF

cat > docs/verification/AI-CONTEXT-SMOKE-TESTS.md <<'EOF'
---
title: AI Context Smoke Tests
status: complete
epic: E18
---

# AI Context Smoke Tests

Run:

```bash
tools/scripts/verify-ai-context.sh
```

This verifies:

- dry-run writes no files;
- JSON dry-run works;
- generated local artifacts are written with `--yes`;
- no provider calls are made;
- unsafe overwrites are refused.
EOF

cat > docs/verification/E18-CLOSEOUT.md <<'EOF'
---
title: E18 Closeout
status: complete
epic: E18
---

# E18 Closeout — AI Context Memory and Provider-Agnostic Assistant Workflow Foundation

E18 is complete when:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-ai-context.sh
tools/scripts/verify-e18.sh
```

## Completed capability

```bash
monad ai-context --dry-run
monad ai-context --dry-run --format=json
monad ai-context --yes
```

## Safety retained

No provider calls.

No remote repo data transfer.

No autonomous execution.

No automatic patch application.

No paid subscription requirement.
EOF

cat > tools/scripts/verify-ai-context.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

echo "==> verify-ai-context: repo root: $REPO_ROOT"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

(
  cd "$tmpdir"

  echo "==> verify ai-context dry-run writes no files"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- ai-context --dry-run >/tmp/monad-e18-ai-context.out
  grep -q "Monad AI context dry-run plan" /tmp/monad-e18-ai-context.out
  grep -q "No provider calls were made." /tmp/monad-e18-ai-context.out
  grep -q "No files were written." /tmp/monad-e18-ai-context.out
  test ! -e .monad/ai
  test ! -e .monad/context/assistant-handoff.md

  echo "==> verify ai-context json"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- ai-context --dry-run --format=json >/tmp/monad-e18-ai-context.json
  grep -q '"command":"ai-context"' /tmp/monad-e18-ai-context.json
  grep -q '"mode":"dry-run"' /tmp/monad-e18-ai-context.json
  grep -q '"provider_mode":"disabled"' /tmp/monad-e18-ai-context.json

  echo "==> verify ai-context apply"
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- ai-context --yes >/tmp/monad-e18-ai-context-apply.out
  grep -q "Monad AI context apply result" /tmp/monad-e18-ai-context-apply.out
  grep -q "No provider calls were made." /tmp/monad-e18-ai-context-apply.out
  test -f .monad/ai/provider-config.example.toml
  test -f .monad/ai/memory/README.md
  test -f .monad/ai/memory/0001-project-memory-template.md
  test -f .monad/context/ai-context-snapshot.md
  test -f .monad/context/work-packet-plan.md
  test -f .monad/context/assistant-handoff.md
  test -f .monad/reports/ai-context-report.md
  test -f .monad/reports/ai-context-report.json

  echo "==> verify unsafe overwrite conflict"
  rm -rf .monad
  mkdir -p .monad/context
  printf 'user-owned\n' > .monad/context/assistant-handoff.md
  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- ai-context --yes >/tmp/monad-e18-ai-context-conflict.out
  grep -q "conflicts:" /tmp/monad-e18-ai-context-conflict.out
)

echo "verify-ai-context: PASS"
EOF
chmod +x tools/scripts/verify-ai-context.sh

cat > tools/scripts/verify-e18.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

echo "==> E18 verification"

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-ai-context.sh

if [ -x tools/scripts/verify.sh ]; then
  tools/scripts/verify.sh
fi

echo "verify-e18: PASS"
EOF
chmod +x tools/scripts/verify-e18.sh

cat > work/learning/E18/EPIC-E18-ai-context-memory.md <<'EOF'
---
title: Epic E18 Learning Note
epic: E18
---

# Epic E18 Learning Note: AI Context Memory

E18 makes AI-assisted development more durable without requiring a provider.

The core idea is:

```text
local repo memory + context snapshot + supervised handoff > fragile chat history
```

E18 does not call AI providers, apply patches, or run agents.
EOF

cat > work/deliverables/E18/EPIC-E18-ai-context-memory.md <<'EOF'
---
title: Epic E18 Deliverable Record
epic: E18
status: complete
---

# Epic E18 Deliverable Record

## Epic

E18 — AI Context Memory and Provider-Agnostic Assistant Workflow Foundation.

## Completed work packets

- WP-E18-001 — Define provider-agnostic AI workflow and memory contract
- WP-E18-002 — Add AI provider configuration model
- WP-E18-003 — Add repo-native memory record schema
- WP-E18-004 — Add context snapshot and work-packet planning artifacts
- WP-E18-005 — Add supervised assistant handoff/export workflow
- WP-E18-006 — Add AI context verification and smoke tests

## Implementation files

```text
crates/monad-core/src/ai_context.rs
crates/monad-core/src/lib.rs
crates/monad-cli/src/main.rs
tools/scripts/verify-ai-context.sh
tools/scripts/verify-e18.sh
```

## Verification command

```bash
tools/scripts/verify-e18.sh
```
EOF

echo "==> Formatting Rust code"
cargo fmt

echo
echo "==> Epic E18 patch complete."
echo
echo "Recommended inspection:"
echo "  git diff -- crates/monad-core/src/ai_context.rs"
echo "  git diff -- crates/monad-core/src/lib.rs"
echo "  git diff -- crates/monad-cli/src/main.rs"
echo "  git diff -- docs/commands/AI-CONTEXT.md"
echo "  git diff -- tools/scripts/verify-ai-context.sh"
echo
echo "Recommended verification:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-ai-context.sh"
echo "  tools/scripts/verify-e18.sh"
echo
echo "Commit:"
echo "  git add crates/monad-core/src/ai_context.rs crates/monad-core/src/lib.rs crates/monad-cli/src/main.rs docs/commands/AI-CONTEXT.md docs/ai/PROVIDER-CONFIGURATION.md docs/ai/MEMORY-SCHEMA.md docs/ai/ASSISTANT-HANDOFF.md docs/ai/CONTEXT-SNAPSHOT.md docs/ai/memory/README.md docs/architecture/AI-CONTEXT-MODEL.md docs/workflows/AI-CONTEXT-WORKFLOW.md docs/verification/AI-CONTEXT-SMOKE-TESTS.md docs/verification/E18-CLOSEOUT.md tools/scripts/verify-ai-context.sh tools/scripts/verify-e18.sh work/learning/E18/EPIC-E18-ai-context-memory.md work/deliverables/E18/EPIC-E18-ai-context-memory.md"
echo "  git commit -m \"feat(ai-context): add provider-agnostic memory foundation\""
echo
echo "Done."
