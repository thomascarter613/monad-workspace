#!/usr/bin/env bash
set -euo pipefail

# Complete WP-E11-004 — Add guarded init write path.
#
# This script adds the guarded `monad init --yes` write path.
#
# Safety boundary:
# - `monad init --dry-run` remains available.
# - `monad init --yes` writes only after a conflict-free dry-run evaluation.
# - existing target files block writes.
# - no Git commands are run.
# - no remote services are called.

mkdir -p work/deliverables/E11

cat > crates/monad-core/src/init.rs <<'EOF'
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

        if let Some(parent) = target_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|error| {
                    MonadError::internal(format!(
                        "failed to create parent directory `{}`: {error}",
                        parent.display()
                    ))
                })?;
            }
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
EOF

python3 - <<'PY'
from pathlib import Path
import re

lib_path = Path("crates/monad-core/src/lib.rs")
lib = lib_path.read_text(encoding="utf-8")

lib = lib.replace(
    "pub use init::{InitPlanOptions, InitPreset, build_init_plan, render_init_dry_run};",
    "pub use init::{\n"
    "    InitApplyResult, InitPlanOptions, InitPreset, apply_init_plan, build_init_plan,\n"
    "    render_init_apply_result, render_init_dry_run,\n"
    "};",
)

lib_path.write_text(lib, encoding="utf-8")

main_path = Path("crates/monad-cli/src/main.rs")
main = main_path.read_text(encoding="utf-8")

main = main.replace(
    "InitPlanOptions, InitPreset, RepositoryGraphRenderFormat, WorkspaceContext,\n    build_local_agent_plan, build_repository_graph,",
    "InitPlanOptions, InitPreset, RepositoryGraphRenderFormat, WorkspaceContext, apply_init_plan,\n    build_local_agent_plan, build_repository_graph,",
)

main = main.replace(
    "render_context_baseline_dry_run, render_context_verify_summary, render_init_dry_run,",
    "render_context_baseline_dry_run, render_context_verify_summary, render_init_apply_result,\n    render_init_dry_run,",
)

# Add yes field to enum if missing.
main = main.replace(
    "    Init {\n        /// Whether to run in dry-run mode.\n        dry_run: bool,\n\n        /// Selected init preset.",
    "    Init {\n        /// Whether to run in dry-run mode.\n        dry_run: bool,\n\n        /// Whether to apply after explicit approval.\n        yes: bool,\n\n        /// Selected init preset.",
)

# Replace init parse arm from E11-002/E11-003.
old_arm = """            [\"init\"] => {
                reject_write_for_non_context(write)?;
                require_dry_run_for_init(dry_run)?;
                reject_yes_for_init(yes)?;
                reject_format_for_init(requested_format.as_deref())?;
                let preset = parse_init_preset_or_default(requested_preset.as_deref())?;
                Ok(Self::Init {
                    dry_run,
                    preset,
                    project_name: requested_project_name,
                })
            }
"""
new_arm = """            [\"init\"] => {
                reject_write_for_non_context(write)?;
                require_init_mode(dry_run, yes)?;
                reject_format_for_init(requested_format.as_deref())?;
                let preset = parse_init_preset_or_default(requested_preset.as_deref())?;
                Ok(Self::Init {
                    dry_run,
                    yes,
                    preset,
                    project_name: requested_project_name,
                })
            }
"""
main = main.replace(old_arm, new_arm)

# Replace run match init arm.
old_match = """        CliCommand::Init {
            dry_run,
            preset,
            project_name,
        } => render_init(dry_run, preset, project_name),
"""
new_match = """        CliCommand::Init {
            dry_run,
            yes,
            preset,
            project_name,
        } => render_init(dry_run, yes, preset, project_name),
"""
main = main.replace(old_match, new_match)

# Replace old init safety helper functions.
main = re.sub(
    r"""/// Requires dry-run mode for the first init implementation\.
fn require_dry_run_for_init\(dry_run: bool\) -> Result<\(\), String> \{
    if dry_run \{
        Ok\(\(\)\)
    \} else \{
        Err\("init currently requires --dry-run; write behavior is not implemented"\.to_string\(\)\)
    \}
\}

/// Rejects init write approval until the guarded write path exists\.
fn reject_yes_for_init\(yes: bool\) -> Result<\(\), String> \{
    if yes \{
        Err\("init --yes is reserved for the guarded write path; WP-E11-002 is dry-run only"\.to_string\(\)\)
    \} else \{
        Ok\(\(\)\)
    \}
\}
""",
    """/// Requires exactly one init mode for the guarded init implementation.
fn require_init_mode(dry_run: bool, yes: bool) -> Result<(), String> {
    match (dry_run, yes) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => {
            Err("init currently requires either --dry-run to preview or --yes to apply".to_string())
        }
        (true, true) => Err("init accepts either --dry-run or --yes, not both".to_string()),
    }
}
""",
    main,
)

# Replace render_init function.
main = re.sub(
    r"""/// Renders repository initialization dry-run output\.
fn render_init\(
    dry_run: bool,
    preset: InitPreset,
    project_name: Option<String>,
\) -> Result<String, String> \{
    if !dry_run \{
        return Err\("init currently requires --dry-run; write behavior is not implemented"\.to_string\(\)\);
    \}

    let context = WorkspaceContext::discover_from\("\."\)\.map_err\(\|error\| error\.to_string\(\)\)\?;
    let options = InitPlanOptions::new\(preset, project_name\);

    render_init_dry_run\(&context, &options\)\.map_err\(\|error\| error\.to_string\(\)\)
\}
""",
    """/// Renders or applies repository initialization.
fn render_init(
    dry_run: bool,
    yes: bool,
    preset: InitPreset,
    project_name: Option<String>,
) -> Result<String, String> {
    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;
    let options = InitPlanOptions::new(preset, project_name);

    if dry_run {
        return render_init_dry_run(&context, &options).map_err(|error| error.to_string());
    }

    if yes {
        let result = apply_init_plan(&context, &options).map_err(|error| error.to_string())?;
        return Ok(render_init_apply_result(&result));
    }

    Err("init currently requires either --dry-run to preview or --yes to apply".to_string())
}
""",
    main,
)

# Update help text lines.
main = main.replace(
    '        "  init --dry-run                            Preview repository initialization plan",\n',
    '        "  init --dry-run                            Preview repository initialization plan",\n'
    '        "  init --yes                                Apply repository initialization after review",\n',
)

main = main.replace(
    '        "  monad init --dry-run\\n',
    '        "  monad init --dry-run",\n'
    '        "  monad init --yes",\n',
)

main = main.replace(
    "  init is dry-run only until guarded write support is implemented.\n",
    "  init writes only with --yes after a conflict-free plan.\n",
)

# Update tests: add yes field to expected Init structs where missing.
main = main.replace(
    "CliCommand::Init {\n                dry_run: true,\n                preset:",
    "CliCommand::Init {\n                dry_run: true,\n                yes: false,\n                preset:",
)
main = main.replace(
    "CliCommand::Init {\n                dry_run: true,\n                yes: false,\n                preset: InitPreset::PolyglotMinimal,",
    "CliCommand::Init {\n                dry_run: true,\n                yes: false,\n                preset: InitPreset::PolyglotMinimal,",
)

# Replace outdated init tests if present.
main = re.sub(
    r"""    #\[test\]
    fn init_requires_dry_run_for_now\(\) \{
        let error = parse_arguments\(&\["monad", "init"\]\)
            \.expect_err\("init without dry-run should fail"\);

        assert!\(error\.contains\("init currently requires --dry-run"\)\);
        assert!\(error\.contains\("write behavior is not implemented"\)\);
    \}

    #\[test\]
    fn init_rejects_yes_until_guarded_write_exists\(\) \{
        let error = parse_arguments\(&\["monad", "init", "--dry-run", "--yes"\]\)
            \.expect_err\("init --yes should fail for now"\);

        assert!\(error\.contains\("init --yes is reserved"\)\);
        assert!\(error\.contains\("dry-run only"\)\);
    \}
""",
    """    #[test]
    fn init_requires_dry_run_or_yes() {
        let error =
            parse_arguments(&["monad", "init"]).expect_err("init without mode should fail");

        assert!(error.contains("init currently requires either --dry-run"));
        assert!(error.contains("--yes"));
    }

    #[test]
    fn init_yes_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "init", "--yes"]).expect("init --yes should parse"),
            CliCommand::Init {
                dry_run: false,
                yes: true,
                preset: InitPreset::Minimal,
                project_name: None,
            }
        );
    }

    #[test]
    fn init_rejects_dry_run_and_yes_together() {
        let error = parse_arguments(&["monad", "init", "--dry-run", "--yes"])
            .expect_err("init should reject conflicting modes");

        assert!(error.contains("either --dry-run or --yes"));
        assert!(error.contains("not both"));
    }
""",
    main,
)

# If the previous regex did not match due to formatting, append replacement tests after init_dry_run_command_parses.
if "fn init_yes_command_parses" not in main:
    main = main.replace(
        "    #[test]\n    fn info_command_parses_text_and_json_formats() {\n",
        """    #[test]
    fn init_requires_dry_run_or_yes() {
        let error =
            parse_arguments(&["monad", "init"]).expect_err("init without mode should fail");

        assert!(error.contains("init currently requires either --dry-run"));
        assert!(error.contains("--yes"));
    }

    #[test]
    fn init_yes_command_parses() {
        assert_eq!(
            parse_arguments(&["monad", "init", "--yes"]).expect("init --yes should parse"),
            CliCommand::Init {
                dry_run: false,
                yes: true,
                preset: InitPreset::Minimal,
                project_name: None,
            }
        );
    }

    #[test]
    fn init_rejects_dry_run_and_yes_together() {
        let error = parse_arguments(&["monad", "init", "--dry-run", "--yes"])
            .expect_err("init should reject conflicting modes");

        assert!(error.contains("either --dry-run or --yes"));
        assert!(error.contains("not both"));
    }

    #[test]
    fn info_command_parses_text_and_json_formats() {
""",
    )

# Help test should mention yes too.
main = main.replace(
    '        assert!(text.contains("init --dry-run"));\n        assert!(text.contains("plan \\"<intent>\\""));\n',
    '        assert!(text.contains("init --dry-run"));\n        assert!(text.contains("init --yes"));\n        assert!(text.contains("plan \\"<intent>\\""));\n',
)

main_path.write_text(main, encoding="utf-8")
PY

python3 - <<'PY'
from pathlib import Path

# Documentation updates.
cmd_path = Path("docs/commands/INIT.md")
if cmd_path.exists():
    text = cmd_path.read_text(encoding="utf-8")
    if "## WP-E11-004 Implementation Note" not in text:
        text += """

## WP-E11-004 Implementation Note

WP-E11-004 adds the guarded write path:

```bash
monad init --yes
```

The write path is conservative:

- it runs through the same planned template set as dry-run;
- it refuses to continue if conflicts exist;
- it refuses to overwrite existing files;
- it creates parent directories only for approved scaffold targets;
- it runs no Git commands;
- it reports created files and bytes written.

Dry-run remains available:

```bash
monad init --dry-run
```
"""
    text = text.replace(
        "No files are written until the guarded write path is implemented in WP-E11-004.",
        "Guarded write behavior is implemented in WP-E11-004 through `monad init --yes`.",
    )
    cmd_path.write_text(text, encoding="utf-8")

ref_path = Path("docs/project/MVP-COMMAND-REFERENCE.md")
if ref_path.exists():
    text = ref_path.read_text(encoding="utf-8")
    if "cargo run -p monad-cli -- init --yes" not in text:
        text = text.replace(
            "cargo run -p monad-cli -- init --name=my-project --dry-run\n",
            "cargo run -p monad-cli -- init --name=my-project --dry-run\n"
            "cargo run -p monad-cli -- init --yes\n",
        )
    text = text.replace(
        "* rejects `--yes` until guarded write behavior is implemented\n",
        "* applies the scaffold only when `--yes` is provided and no conflicts exist\n",
    )
    ref_path.write_text(text, encoding="utf-8")

readme_path = Path("README.md")
if readme_path.exists():
    text = readme_path.read_text(encoding="utf-8")
    if "monad init --yes" not in text:
        text = text.replace(
            "monad init --dry-run\n",
            "monad init --dry-run\nmonad init --yes\n",
            1,
        )
    text = text.replace(
        "Safety boundary: `init` and `evolve` commands are dry-run only, `plan` is no-write, and write behavior is currently limited to explicit context export/generation commands.",
        "Safety boundary: `init --yes` writes only after conflict checks, `evolve` commands are dry-run only, `plan` is no-write, and other write behavior is limited to explicit context export/generation commands.",
    )
    readme_path.write_text(text, encoding="utf-8")
PY

cat > work/deliverables/E11/WP-E11-004-guarded-init-write-path.md <<'EOF'
---
title: "WP-E11-004 Guarded Init Write Path Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E11
work_packet: WP-E11-004
tags:
  - monad
  - e11
  - init
  - guarded-write
  - safety
related:
  - crates/monad-core/src/init.rs
  - crates/monad-core/src/lib.rs
  - crates/monad-cli/src/main.rs
  - docs/commands/INIT.md
---

# WP-E11-004 Guarded Init Write Path Deliverable

## Work Packet

WP-E11-004 — Add guarded init write path.

## Outcome

Implemented.

## Summary

This work packet adds the first guarded write path for `monad init`.

The command now supports:

```bash
cargo run -p monad-cli -- init --dry-run
cargo run -p monad-cli -- init --yes
```

The write path is conservative and aborts if the planned operation set has conflicts.

## Deliverables

- `crates/monad-core/src/init.rs`
- `crates/monad-core/src/lib.rs`
- `crates/monad-cli/src/main.rs`
- `docs/commands/INIT.md`
- `docs/project/MVP-COMMAND-REFERENCE.md`
- `README.md`
- `work/deliverables/E11/WP-E11-004-guarded-init-write-path.md`

## Safety Boundary

`monad init --yes`:

- writes only the selected embedded scaffold templates;
- refuses to overwrite existing target files;
- creates parent directories only for approved scaffold targets;
- runs no Git commands;
- does not commit;
- does not push;
- does not install packages;
- does not call remote services.

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings

tmpdir="$(mktemp -d)"
(
  cd "$tmpdir"
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- init --dry-run
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- init --yes
  test -f monad.toml
  test -f README.md
  test -f docs/README.md
  test -f work/README.md
  test -f .monad/.gitignore
)
rm -rf "$tmpdir"

tools/scripts/verify.sh
git status --short
```

Adjust `/data/monad-workspace/Cargo.toml` if your repository path differs.

## Recommended Commit

```bash
git commit -m "feat(init): add guarded init write path"
```

## Closeout Note

WP-E11-004 is complete once the guarded init write path is committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E11-005 — Add basic/polyglot-minimal preset
```
EOF

cargo fmt

echo
echo "WP-E11-004 files updated:"
echo "  crates/monad-core/src/init.rs"
echo "  crates/monad-core/src/lib.rs"
echo "  crates/monad-cli/src/main.rs"
echo "  docs/commands/INIT.md"
echo "  docs/project/MVP-COMMAND-REFERENCE.md"
echo "  README.md"
echo "  work/deliverables/E11/WP-E11-004-guarded-init-write-path.md"
echo
echo "Next verification:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  cargo run -p monad-cli -- init --dry-run"
echo "  cargo run -p monad-cli -- init --yes   # run only in an empty temp directory"
