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

        return Err(
            "refusing unsafe overwrite of existing file with different content".to_string(),
        );
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
        format!(
            "  provider_mode: {}",
            plan.provider_config().mode().as_str()
        ),
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
        format!(
            "provider_mode: {}",
            result.plan().provider_config().mode().as_str()
        ),
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
    lower.contains("sk-")
        || lower.contains("api_key")
        || lower.contains("apikey")
        || lower.contains("secret")
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
        assert!(
            root.join(".monad/ai/provider-config.example.toml")
                .is_file()
        );
        assert!(root.join(".monad/ai/memory/README.md").is_file());
        assert!(root.join(".monad/context/assistant-handoff.md").is_file());
        assert!(root.join(".monad/reports/ai-context-report.md").is_file());

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn ai_context_apply_refuses_unsafe_overwrite() {
        let root = unique_temp_root("conflict");
        fs::create_dir_all(root.join(".monad/context")).expect("context dir should be created");
        fs::write(
            root.join(".monad/context/assistant-handoff.md"),
            "user-owned\n",
        )
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
