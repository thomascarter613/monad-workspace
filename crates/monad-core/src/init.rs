//! Repository initialization planning.
//!
//! This module contains the first `monad init` foundation: a dry-run plan.
//! It intentionally does not write files. Guarded write behavior belongs to a
//! later E11 work packet.

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
            "minimal" => Ok(Self::Minimal),
            "polyglot-minimal" => Ok(Self::PolyglotMinimal),
            other => Err(MonadError::invalid_input(format!(
                "unsupported init preset `{other}`; supported presets: minimal, polyglot-minimal"
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

/// Options used to build an init dry-run plan.
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

/// Builds the initial init file operation plan.
///
/// This plan is intentionally review-only. It lists what Monad would create
/// for the selected preset but does not write anything.
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
    let project_name = options
        .project_name()
        .map(ToOwned::to_owned)
        .or_else(|| {
            context
                .root()
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "monad-project".to_string());

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
        "  apply: not implemented in WP-E11-003".to_string(),
        "  approval_flag: --yes reserved for WP-E11-004".to_string(),
        String::new(),
        render_dry_run_plan(&dry_run),
        String::new(),
        "No files were written.".to_string(),
        "Next: review this plan; guarded write support begins in WP-E11-004.".to_string(),
    ];

    if dry_run.has_conflicts() {
        output.push(
            "Conflicts were detected. Resolve or accept them explicitly before any future write path."
                .to_string(),
        );
    }

    Ok(output.join("\n"))
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
        assert!(output.contains("WP-E11-003"));

        fs::remove_dir_all(&root).ok();

        Ok(())
    }
}
