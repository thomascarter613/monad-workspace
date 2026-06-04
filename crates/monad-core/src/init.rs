//! Repository initialization planning and guarded application.
//!
//! This module contains the `monad init` foundation:
//! - dry-run planning;
//! - embedded scaffold template selection;
//! - guarded write application.
//!
//! The write path is intentionally conservative. It refuses to overwrite
//! existing paths and does not run Git commands.

use std::fs;
use std::path::PathBuf;

use crate::{
    FileOperationPlan, MonadError, MonadResult, PlannedFileOperation, TemplateDefinition,
    WorkspaceContext, evaluate_file_operation_plan, initial_template_registry, render_dry_run_plan,
};

const MINIMAL_TEMPLATE_IDS: &[&str] = &[
    "init.minimal.monad-toml",
    "init.minimal.readme",
    "init.minimal.docs-readme",
    "init.minimal.work-readme",
    "init.minimal.monad-gitignore",
];

const POLYGLOT_TEMPLATE_IDS: &[&str] = &[
    "init.polyglot.apps-gitkeep",
    "init.polyglot.packages-gitkeep",
    "init.polyglot.services-gitkeep",
    "init.polyglot.tools-gitkeep",
];

/// Built-in initialization presets.
///
/// The preset is deliberately small at this stage. It gives the future
/// template/write path something stable to target without prematurely turning
/// `monad init` into a full application generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InitPreset {
    /// Smallest useful Monad-aware repository baseline.
    #[default]
    Minimal,

    /// Minimal polyglot monorepo shape without heavy orchestration tools.
    PolyglotMinimal,
}

impl InitPreset {
    /// Parses a user-facing preset name.
    pub fn parse(value: &str) -> MonadResult<Self> {
        match value {
            "basic" | "minimal" => Ok(Self::Minimal),
            "polyglot-minimal" => Ok(Self::PolyglotMinimal),
            other => Err(MonadError::invalid_input(format!(
                "unsupported init preset `{other}`; supported presets: basic, minimal, polyglot-minimal"
            ))),
        }
    }

    /// Returns the stable user-facing preset name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::PolyglotMinimal => "polyglot-minimal",
        }
    }

    fn template_ids(self) -> Vec<&'static str> {
        let mut ids = MINIMAL_TEMPLATE_IDS.to_vec();

        if self == Self::PolyglotMinimal {
            ids.extend_from_slice(POLYGLOT_TEMPLATE_IDS);
        }

        ids
    }
}

/// Options used to build an init plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitPlanOptions {
    preset: InitPreset,
    project_name: Option<String>,
}

impl InitPlanOptions {
    /// Creates init plan options.
    #[must_use]
    pub fn new(preset: InitPreset, project_name: Option<String>) -> Self {
        Self {
            preset,
            project_name,
        }
    }

    /// Creates minimal dry-run options.
    #[must_use]
    pub fn minimal() -> Self {
        Self::new(InitPreset::Minimal, None)
    }

    /// Returns the selected preset.
    #[must_use]
    pub const fn preset(&self) -> InitPreset {
        self.preset
    }

    /// Returns the optional project name override.
    #[must_use]
    pub fn project_name(&self) -> Option<&str> {
        self.project_name.as_deref()
    }
}

impl Default for InitPlanOptions {
    fn default() -> Self {
        Self::minimal()
    }
}

/// Result of guarded init application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitApplyResult {
    preset: InitPreset,
    project_name: String,
    created_files: Vec<PathBuf>,
    total_bytes_written: usize,
}

impl InitApplyResult {
    /// Creates an init apply result.
    #[must_use]
    pub fn new(
        preset: InitPreset,
        project_name: impl Into<String>,
        created_files: Vec<PathBuf>,
        total_bytes_written: usize,
    ) -> Self {
        Self {
            preset,
            project_name: project_name.into(),
            created_files,
            total_bytes_written,
        }
    }

    /// Returns the selected preset.
    #[must_use]
    pub const fn preset(&self) -> InitPreset {
        self.preset
    }

    /// Returns the resolved project name.
    #[must_use]
    pub fn project_name(&self) -> &str {
        &self.project_name
    }

    /// Returns created files.
    #[must_use]
    pub fn created_files(&self) -> &[PathBuf] {
        &self.created_files
    }

    /// Returns created file count.
    #[must_use]
    pub fn file_count(&self) -> usize {
        self.created_files.len()
    }

    /// Returns total bytes written.
    #[must_use]
    pub const fn total_bytes_written(&self) -> usize {
        self.total_bytes_written
    }
}

/// Builds the initial init file operation plan.
///
/// This plan is reviewable and can be evaluated before any writes occur.
pub fn build_init_plan(options: &InitPlanOptions) -> MonadResult<FileOperationPlan> {
    let registry = initial_template_registry()?;
    let mut operations = Vec::new();

    for template_id in options.preset().template_ids() {
        let template = registry.get_by_str(template_id).ok_or_else(|| {
            MonadError::not_found(format!("init template `{template_id}` was not found"))
        })?;

        operations.push(planned_create_from_template(template));
    }

    Ok(FileOperationPlan::from_operations(operations))
}

fn planned_create_from_template(template: &TemplateDefinition) -> PlannedFileOperation {
    PlannedFileOperation::create(
        template.metadata().target_path().to_path_buf(),
        format!(
            "create `{}` from embedded template `{}` ({})",
            template.metadata().target_path().display(),
            template.id().as_str(),
            template.metadata().description()
        ),
    )
}

/// Applies the init scaffold after a conflict-free dry-run evaluation.
///
/// This is the first guarded write path for `monad init --yes`.
/// It refuses to overwrite existing files and performs no Git operations.
pub fn apply_init_plan(
    context: &WorkspaceContext,
    options: &InitPlanOptions,
) -> MonadResult<InitApplyResult> {
    let plan = build_init_plan(options)?;
    let dry_run = evaluate_file_operation_plan(context.root(), &plan);

    if dry_run.has_conflicts() {
        return Err(MonadError::invalid_input(
            "init plan has conflicts; run `monad init --dry-run` and resolve existing target paths before using --yes",
        ));
    }

    let registry = initial_template_registry()?;
    let project_name = resolve_project_name(context, options);
    let mut created_files = Vec::new();
    let mut total_bytes_written = 0usize;

    for template_id in options.preset().template_ids() {
        let template = registry.get_by_str(template_id).ok_or_else(|| {
            MonadError::not_found(format!("init template `{template_id}` was not found"))
        })?;
        let relative_path = template.metadata().target_path().to_path_buf();
        let target_path = context.root().join(&relative_path);

        if target_path.exists() {
            return Err(MonadError::invalid_input(format!(
                "init target `{}` already exists; refusing to overwrite",
                relative_path.display()
            )));
        }

        if let Some(parent) = target_path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).map_err(|error| {
                MonadError::internal(format!(
                    "failed to create parent directory `{}`: {error}",
                    parent.display()
                ))
            })?;
        }

        let content = render_init_template_content(template, &project_name);
        fs::write(&target_path, content.as_bytes()).map_err(|error| {
            MonadError::internal(format!(
                "failed to write init target `{}`: {error}",
                relative_path.display()
            ))
        })?;

        total_bytes_written += content.len();
        created_files.push(relative_path);
    }

    Ok(InitApplyResult::new(
        options.preset(),
        project_name,
        created_files,
        total_bytes_written,
    ))
}

fn resolve_project_name(context: &WorkspaceContext, options: &InitPlanOptions) -> String {
    options
        .project_name()
        .map(ToOwned::to_owned)
        .or_else(|| {
            context
                .root()
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "monad-project".to_string())
}

fn render_init_template_content(template: &TemplateDefinition, project_name: &str) -> String {
    template
        .content()
        .replace("monad-project", project_name)
        .replace("Monad Project", project_name)
}

/// Renders the init dry-run plan for the selected workspace.
///
/// This is the core behavior used by the CLI. Keeping the behavior in
/// `monad-core` keeps the CLI thin and makes the planning behavior testable.
pub fn render_init_dry_run(
    context: &WorkspaceContext,
    options: &InitPlanOptions,
) -> MonadResult<String> {
    let plan = build_init_plan(options)?;
    let dry_run = evaluate_file_operation_plan(context.root(), &plan);
    let project_name = resolve_project_name(context, options);

    let mut output = vec![
        "Monad init dry-run plan".to_string(),
        String::new(),
        "Workspace:".to_string(),
        format!("  root: {}", context.root().display()),
        format!("  project: {project_name}"),
        format!("  preset: {}", options.preset().as_str()),
        String::new(),
        "Template source:".to_string(),
        "  registry: embedded".to_string(),
        format!("  templates: {}", plan.len()),
        String::new(),
        "Safety:".to_string(),
        "  mode: dry-run".to_string(),
        "  writes: disabled".to_string(),
        "  apply: guarded by --yes".to_string(),
        "  approval_flag: --yes".to_string(),
        String::new(),
        render_dry_run_plan(&dry_run),
        String::new(),
        "No files were written.".to_string(),
        "Next: review this plan; rerun with --yes only if the plan is acceptable.".to_string(),
    ];

    if dry_run.has_conflicts() {
        output.push(
            "Conflicts were detected. Resolve them before using the guarded write path."
                .to_string(),
        );
    }

    Ok(output.join("\n"))
}

/// Renders the guarded init apply result.
#[must_use]
pub fn render_init_apply_result(result: &InitApplyResult) -> String {
    let mut lines = vec![
        "Monad init applied".to_string(),
        format!("  project: {}", result.project_name()),
        format!("  preset: {}", result.preset().as_str()),
        format!("  files_created: {}", result.file_count()),
        format!("  bytes_written: {}", result.total_bytes_written()),
        "  created_files:".to_string(),
    ];

    for path in result.created_files() {
        lines.push(format!("    - {}", path.display()));
    }

    lines.push("No Git commands were run.".to_string());

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{DryRunOperationKind, MonadError};

    use super::*;

    fn unique_temp_root(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);

        std::env::temp_dir().join(format!("monad-init-{name}-{unique}"))
    }

    #[test]
    fn init_preset_parses_supported_values() -> MonadResult<()> {
        assert_eq!(InitPreset::parse("minimal")?, InitPreset::Minimal);
        assert_eq!(
            InitPreset::parse("polyglot-minimal")?,
            InitPreset::PolyglotMinimal
        );

        Ok(())
    }

    #[test]
    fn init_preset_parses_basic_alias() -> MonadResult<()> {
        assert_eq!(InitPreset::parse("basic")?, InitPreset::Minimal);
        assert_eq!(InitPreset::parse("minimal")?, InitPreset::Minimal);

        Ok(())
    }

    #[test]
    fn init_preset_rejects_unknown_values() {
        let error = InitPreset::parse("everything").expect_err("unknown preset should fail");

        assert!(error.to_string().contains("unsupported init preset"));
        assert!(error.to_string().contains("minimal"));
    }

    #[test]
    fn minimal_init_plan_contains_foundation_files_from_templates() -> MonadResult<()> {
        let plan = build_init_plan(&InitPlanOptions::minimal())?;

        let targets: Vec<String> = plan
            .operations()
            .iter()
            .map(|operation| operation.target().display_path())
            .collect();

        assert!(targets.contains(&"monad.toml".to_string()));
        assert!(targets.contains(&"README.md".to_string()));
        assert!(targets.contains(&"docs/README.md".to_string()));
        assert!(targets.contains(&"work/README.md".to_string()));
        assert!(targets.contains(&".monad/.gitignore".to_string()));

        assert!(
            plan.operations()
                .iter()
                .any(|operation| operation.explanation().contains("init.minimal.monad-toml"))
        );

        Ok(())
    }

    #[test]
    fn polyglot_minimal_plan_contains_workspace_placeholders() -> MonadResult<()> {
        let plan = build_init_plan(&InitPlanOptions::new(
            InitPreset::PolyglotMinimal,
            Some("example".to_string()),
        ))?;

        let targets: Vec<String> = plan
            .operations()
            .iter()
            .map(|operation| operation.target().display_path())
            .collect();

        assert!(targets.contains(&"apps/.gitkeep".to_string()));
        assert!(targets.contains(&"packages/.gitkeep".to_string()));
        assert!(targets.contains(&"services/.gitkeep".to_string()));
        assert!(targets.contains(&"tools/.gitkeep".to_string()));

        Ok(())
    }

    #[test]
    fn init_dry_run_detects_existing_readme_conflict() -> MonadResult<()> {
        let root = unique_temp_root("existing-readme");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;
        fs::write(root.join("README.md"), "# Existing\n").map_err(|error| {
            MonadError::internal(format!("test README should be written: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let plan = build_init_plan(&InitPlanOptions::minimal())?;
        let dry_run = evaluate_file_operation_plan(context.root(), &plan);

        assert!(
            dry_run
                .operations()
                .iter()
                .any(|operation| operation.target().display_path() == "README.md"
                    && operation.outcome_kind() == DryRunOperationKind::Conflict)
        );
        assert!(dry_run.has_conflicts());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn init_dry_run_output_states_no_files_written() -> MonadResult<()> {
        let root = unique_temp_root("render");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let output = render_init_dry_run(&context, &InitPlanOptions::minimal())?;

        assert!(output.contains("Monad init dry-run plan"));
        assert!(output.contains("preset: minimal"));
        assert!(output.contains("Template source:"));
        assert!(output.contains("init.minimal.monad-toml"));
        assert!(output.contains("monad.toml"));
        assert!(output.contains("No files were written."));
        assert!(output.contains("--yes"));

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn apply_init_plan_writes_minimal_scaffold() -> MonadResult<()> {
        let root = unique_temp_root("apply");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let result = apply_init_plan(
            &context,
            &InitPlanOptions::new(InitPreset::Minimal, Some("example-app".to_string())),
        )?;

        assert_eq!(result.preset(), InitPreset::Minimal);
        assert_eq!(result.project_name(), "example-app");
        assert_eq!(result.file_count(), 5);
        assert!(root.join("monad.toml").is_file());
        assert!(root.join("README.md").is_file());
        assert!(root.join("docs/README.md").is_file());
        assert!(root.join("work/README.md").is_file());
        assert!(root.join(".monad/.gitignore").is_file());

        let manifest = fs::read_to_string(root.join("monad.toml")).map_err(|error| {
            MonadError::internal(format!("test manifest should be readable: {error}"))
        })?;
        assert!(manifest.contains("example-app"));

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn apply_init_plan_refuses_to_overwrite_existing_file() -> MonadResult<()> {
        let root = unique_temp_root("apply-conflict");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;
        fs::write(root.join("README.md"), "# Existing\n").map_err(|error| {
            MonadError::internal(format!("test README should be written: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let error = apply_init_plan(&context, &InitPlanOptions::minimal())
            .expect_err("existing target should block init apply");

        assert!(error.to_string().contains("init plan has conflicts"));
        assert!(!root.join("monad.toml").exists());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn render_init_apply_result_lists_created_files() -> MonadResult<()> {
        let result = InitApplyResult::new(
            InitPreset::Minimal,
            "example",
            vec![PathBuf::from("monad.toml"), PathBuf::from("README.md")],
            42,
        );

        let rendered = render_init_apply_result(&result);

        assert!(rendered.contains("Monad init applied"));
        assert!(rendered.contains("files_created: 2"));
        assert!(rendered.contains("monad.toml"));
        assert!(rendered.contains("No Git commands were run."));

        Ok(())
    }
}
