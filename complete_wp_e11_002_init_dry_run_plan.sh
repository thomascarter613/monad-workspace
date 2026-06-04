#!/usr/bin/env bash
set -euo pipefail

# Complete WP-E11-002 — Add init dry-run plan.
#
# This script adds the first code implementation for `monad init`:
# - core init dry-run plan model in monad-core
# - thin CLI parser/renderer wiring in monad-cli
# - command docs and deliverable record updates
#
# Safety boundary:
# - `monad init` requires --dry-run
# - no write/apply behavior is added
# - --yes is rejected until guarded write behavior arrives in WP-E11-004

mkdir -p crates/monad-core/src
mkdir -p docs/commands
mkdir -p work/deliverables/E11

cat > crates/monad-core/src/init.rs <<'EOF'
//! Repository initialization planning.
//!
//! This module contains the first `monad init` foundation: a dry-run plan.
//! It intentionally does not write files. Guarded write behavior belongs to a
//! later E11 work packet.

use crate::{
    FileOperationPlan, MonadError, MonadResult, PlannedFileOperation, WorkspaceContext,
    evaluate_file_operation_plan, render_dry_run_plan,
};

/// Built-in initialization presets.
///
/// The preset is deliberately small at this stage. It gives the future
/// template/write path something stable to target without prematurely turning
/// `monad init` into a full application generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitPreset {
    /// Smallest useful Monad-aware repository baseline.
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
}

impl Default for InitPreset {
    fn default() -> Self {
        Self::Minimal
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
#[must_use]
pub fn build_init_plan(options: &InitPlanOptions) -> FileOperationPlan {
    let mut operations = vec![
        PlannedFileOperation::create(
            "monad.toml",
            "create Monad manifest describing repository intent",
        ),
        PlannedFileOperation::create(
            "README.md",
            "create repository README entry point if one does not already exist",
        ),
        PlannedFileOperation::create(
            "docs/README.md",
            "create documentation directory entry point",
        ),
        PlannedFileOperation::create(
            "work/README.md",
            "create work-tracking directory entry point",
        ),
        PlannedFileOperation::create(
            ".monad/.gitignore",
            "create Monad local/generated state ignore policy",
        ),
    ];

    if options.preset() == InitPreset::PolyglotMinimal {
        operations.extend([
            PlannedFileOperation::create(
                "apps/.gitkeep",
                "create apps directory placeholder for polyglot workspace layout",
            ),
            PlannedFileOperation::create(
                "packages/.gitkeep",
                "create packages directory placeholder for shared libraries",
            ),
            PlannedFileOperation::create(
                "services/.gitkeep",
                "create services directory placeholder for service workloads",
            ),
            PlannedFileOperation::create(
                "tools/.gitkeep",
                "create tools directory placeholder for repository tooling",
            ),
        ]);
    }

    FileOperationPlan::from_operations(operations)
}

/// Renders the init dry-run plan for the selected workspace.
///
/// This is the core behavior used by the CLI. Keeping the behavior in
/// `monad-core` keeps the CLI thin and makes the planning behavior testable.
pub fn render_init_dry_run(
    context: &WorkspaceContext,
    options: &InitPlanOptions,
) -> MonadResult<String> {
    let plan = build_init_plan(options);
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
        "Safety:".to_string(),
        "  mode: dry-run".to_string(),
        "  writes: disabled".to_string(),
        "  apply: not implemented in WP-E11-002".to_string(),
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
    fn minimal_init_plan_contains_foundation_files() {
        let plan = build_init_plan(&InitPlanOptions::minimal());

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
    }

    #[test]
    fn polyglot_minimal_plan_contains_workspace_placeholders() {
        let plan = build_init_plan(&InitPlanOptions::new(
            InitPreset::PolyglotMinimal,
            Some("example".to_string()),
        ));

        let targets: Vec<String> = plan
            .operations()
            .iter()
            .map(|operation| operation.target().display_path())
            .collect();

        assert!(targets.contains(&"apps/.gitkeep".to_string()));
        assert!(targets.contains(&"packages/.gitkeep".to_string()));
        assert!(targets.contains(&"services/.gitkeep".to_string()));
        assert!(targets.contains(&"tools/.gitkeep".to_string()));
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
        let plan = build_init_plan(&InitPlanOptions::minimal());
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
        assert!(output.contains("monad.toml"));
        assert!(output.contains("No files were written."));
        assert!(output.contains("WP-E11-002"));

        fs::remove_dir_all(&root).ok();

        Ok(())
    }
}
EOF

python3 - <<'PY'
from pathlib import Path

lib_path = Path("crates/monad-core/src/lib.rs")
lib = lib_path.read_text(encoding="utf-8")

if "pub mod init;" not in lib:
    lib = lib.replace("pub mod git;\n", "pub mod git;\npub mod init;\n")

if "pub use init::{" not in lib:
    lib = lib.replace(
        "pub use git::{\n    GitEvolutionSafety, GitWorkingTreeStatus, inspect_git_working_tree, parse_git_status_porcelain,\n};\n",
        "pub use git::{\n    GitEvolutionSafety, GitWorkingTreeStatus, inspect_git_working_tree, parse_git_status_porcelain,\n};\n"
        "pub use init::{InitPlanOptions, InitPreset, build_init_plan, render_init_dry_run};\n",
    )

lib_path.write_text(lib, encoding="utf-8")

main_path = Path("crates/monad-cli/src/main.rs")
main = main_path.read_text(encoding="utf-8")

main = main.replace(
    "RepositoryGraphRenderFormat, WorkspaceContext, build_local_agent_plan, build_repository_graph,\n",
    "InitPlanOptions, InitPreset, RepositoryGraphRenderFormat, WorkspaceContext,\n    build_local_agent_plan, build_repository_graph,\n",
)

main = main.replace(
    "render_context_baseline_dry_run, render_context_verify_summary,\n",
    "render_context_baseline_dry_run, render_context_verify_summary, render_init_dry_run,\n",
)

if "Init {" not in main:
    main = main.replace(
        "    /// Print runtime identity.\n    Version,\n\n",
        "    /// Print runtime identity.\n    Version,\n\n"
        "    /// Preview repository initialization plan.\n"
        "    Init {\n"
        "        /// Whether to run in dry-run mode.\n"
        "        dry_run: bool,\n\n"
        "        /// Selected init preset.\n"
        "        preset: InitPreset,\n\n"
        "        /// Optional project name override.\n"
        "        project_name: Option<String>,\n"
        "    },\n\n",
    )

if "let mut requested_preset" not in main:
    main = main.replace(
        "        let mut requested_format: Option<String> = None;\n"
        "        let mut positional: Vec<String> = Vec::new();\n"
        "        let mut write = false;\n"
        "        let mut dry_run = false;\n",
        "        let mut requested_format: Option<String> = None;\n"
        "        let mut requested_preset: Option<String> = None;\n"
        "        let mut requested_project_name: Option<String> = None;\n"
        "        let mut positional: Vec<String> = Vec::new();\n"
        "        let mut write = false;\n"
        "        let mut dry_run = false;\n"
        "        let mut yes = false;\n",
    )

if 'argument == "--yes"' not in main:
    main = main.replace(
        "            if argument == \"--dry-run\" {\n"
        "                dry_run = true;\n"
        "                continue;\n"
        "            }\n\n",
        "            if argument == \"--dry-run\" {\n"
        "                dry_run = true;\n"
        "                continue;\n"
        "            }\n\n"
        "            if argument == \"--yes\" {\n"
        "                yes = true;\n"
        "                continue;\n"
        "            }\n\n",
    )

if 'strip_prefix("--preset=")' not in main:
    main = main.replace(
        "            if let Some(value) = argument.strip_prefix(\"--format=\") {\n"
        "                requested_format = Some(value.to_string());\n"
        "                continue;\n"
        "            }\n\n"
        "            if argument == \"--format\" {\n"
        "                return Err(\"expected a value after --format, such as --format=json\".to_string());\n"
        "            }\n\n",
        "            if let Some(value) = argument.strip_prefix(\"--format=\") {\n"
        "                requested_format = Some(value.to_string());\n"
        "                continue;\n"
        "            }\n\n"
        "            if argument == \"--format\" {\n"
        "                return Err(\"expected a value after --format, such as --format=json\".to_string());\n"
        "            }\n\n"
        "            if let Some(value) = argument.strip_prefix(\"--preset=\") {\n"
        "                requested_preset = Some(value.to_string());\n"
        "                continue;\n"
        "            }\n\n"
        "            if argument == \"--preset\" {\n"
        "                return Err(\"expected a value after --preset, such as --preset=minimal\".to_string());\n"
        "            }\n\n"
        "            if let Some(value) = argument.strip_prefix(\"--name=\") {\n"
        "                requested_project_name = Some(value.to_string());\n"
        "                continue;\n"
        "            }\n\n"
        "            if argument == \"--name\" {\n"
        "                return Err(\"expected a value after --name, such as --name=my-project\".to_string());\n"
        "            }\n\n",
    )

if "requested_preset.is_some()" not in main:
    main = main.replace(
        "        // Convert positional args to string slices for pattern matching.\n"
        "        let parts: Vec<&str> = positional.iter().map(|s| s.as_str()).collect();\n\n"
        "        match parts.as_slice() {\n",
        "        // Convert positional args to string slices for pattern matching.\n"
        "        let parts: Vec<&str> = positional.iter().map(|s| s.as_str()).collect();\n\n"
        "        if yes && parts.first().copied() != Some(\"init\") {\n"
        "            return Err(\"--yes is only supported for init command\".to_string());\n"
        "        }\n\n"
        "        if (requested_preset.is_some() || requested_project_name.is_some())\n"
        "            && parts.first().copied() != Some(\"init\")\n"
        "        {\n"
        "            return Err(\"--preset and --name are only supported for init command\".to_string());\n"
        "        }\n\n"
        "        match parts.as_slice() {\n",
    )

if '["init"]' not in main:
    main = main.replace(
        "            [\"version\"] => {\n"
        "                reject_write_for_non_context(write)?;\n"
        "                Ok(Self::Version)\n"
        "            }\n",
        "            [\"version\"] => {\n"
        "                reject_write_for_non_context(write)?;\n"
        "                Ok(Self::Version)\n"
        "            }\n"
        "            [\"init\"] => {\n"
        "                reject_write_for_non_context(write)?;\n"
        "                require_dry_run_for_init(dry_run)?;\n"
        "                reject_yes_for_init(yes)?;\n"
        "                reject_format_for_init(requested_format.as_deref())?;\n"
        "                let preset = parse_init_preset_or_default(requested_preset.as_deref())?;\n"
        "                Ok(Self::Init {\n"
        "                    dry_run,\n"
        "                    preset,\n"
        "                    project_name: requested_project_name,\n"
        "                })\n"
        "            }\n"
        "            [\"init\", other, ..] => {\n"
        "                reject_write_for_non_context(write)?;\n"
        "                Err(format!(\"unknown init argument: {other}\"))\n"
        "            }\n",
    )

if "CliCommand::Init" not in main:
    main = main.replace(
        "        CliCommand::Version => render_version(),\n",
        "        CliCommand::Version => render_version(),\n"
        "        CliCommand::Init {\n"
        "            dry_run,\n"
        "            preset,\n"
        "            project_name,\n"
        "        } => render_init(dry_run, preset, project_name),\n",
    )

if "fn require_dry_run_for_init" not in main:
    main = main.replace(
        "/// Requires dry-run mode for early evolution commands.\n",
        "/// Requires dry-run mode for the first init implementation.\n"
        "fn require_dry_run_for_init(dry_run: bool) -> Result<(), String> {\n"
        "    if dry_run {\n"
        "        Ok(())\n"
        "    } else {\n"
        "        Err(\"init currently requires --dry-run; write behavior is not implemented\".to_string())\n"
        "    }\n"
        "}\n\n"
        "/// Rejects init write approval until the guarded write path exists.\n"
        "fn reject_yes_for_init(yes: bool) -> Result<(), String> {\n"
        "    if yes {\n"
        "        Err(\"init --yes is reserved for the guarded write path; WP-E11-002 is dry-run only\".to_string())\n"
        "    } else {\n"
        "        Ok(())\n"
        "    }\n"
        "}\n\n"
        "/// Rejects output-format flags for the first init implementation.\n"
        "fn reject_format_for_init(requested_format: Option<&str>) -> Result<(), String> {\n"
        "    if requested_format.is_some() {\n"
        "        Err(\"--format is not supported for init yet\".to_string())\n"
        "    } else {\n"
        "        Ok(())\n"
        "    }\n"
        "}\n\n"
        "/// Parses an init preset or returns the default minimal preset.\n"
        "fn parse_init_preset_or_default(value: Option<&str>) -> Result<InitPreset, String> {\n"
        "    match value {\n"
        "        Some(value) => InitPreset::parse(value).map_err(|error| error.to_string()),\n"
        "        None => Ok(InitPreset::Minimal),\n"
        "    }\n"
        "}\n\n"
        "/// Requires dry-run mode for early evolution commands.\n",
    )

if "fn render_init(" not in main:
    main = main.replace(
        "/// Renders verification baseline evolution dry-run output.\n",
        "/// Renders repository initialization dry-run output.\n"
        "fn render_init(\n"
        "    dry_run: bool,\n"
        "    preset: InitPreset,\n"
        "    project_name: Option<String>,\n"
        ") -> Result<String, String> {\n"
        "    if !dry_run {\n"
        "        return Err(\"init currently requires --dry-run; write behavior is not implemented\".to_string());\n"
        "    }\n\n"
        "    let context = WorkspaceContext::discover_from(\".\").map_err(|error| error.to_string())?;\n"
        "    let options = InitPlanOptions::new(preset, project_name);\n\n"
        "    render_init_dry_run(&context, &options).map_err(|error| error.to_string())\n"
        "}\n\n"
        "/// Renders verification baseline evolution dry-run output.\n",
    )

main = main.replace(
    "  info                                      Show workspace summary\n",
    "  init --dry-run                            Preview repository initialization plan\n"
    "  info                                      Show workspace summary\n",
)

main = main.replace(
    "  monad inspect\n",
    "  monad init --dry-run\n"
    "  monad inspect\n",
)

main = main.replace(
    "  evolve commands are dry-run only in this MVP hardening phase.\n",
    "  init is dry-run only until guarded write support is implemented.\n"
    "  evolve commands are dry-run only in this MVP hardening phase.\n",
)

if "fn init_dry_run_command_parses" not in main:
    main = main.replace(
        "    #[test]\n    fn info_command_parses_text_and_json_formats() {\n",
        "    #[test]\n"
        "    fn init_dry_run_command_parses() {\n"
        "        assert_eq!(\n"
        "            parse_arguments(&[\"monad\", \"init\", \"--dry-run\"]).expect(\"init should parse\"),\n"
        "            CliCommand::Init {\n"
        "                dry_run: true,\n"
        "                preset: InitPreset::Minimal,\n"
        "                project_name: None,\n"
        "            }\n"
        "        );\n\n"
        "        assert_eq!(\n"
        "            parse_arguments(&[\n"
        "                \"monad\",\n"
        "                \"init\",\n"
        "                \"--dry-run\",\n"
        "                \"--preset=polyglot-minimal\",\n"
        "                \"--name=example\",\n"
        "            ])\n"
        "            .expect(\"init preset should parse\"),\n"
        "            CliCommand::Init {\n"
        "                dry_run: true,\n"
        "                preset: InitPreset::PolyglotMinimal,\n"
        "                project_name: Some(\"example\".to_string()),\n"
        "            }\n"
        "        );\n"
        "    }\n\n"
        "    #[test]\n"
        "    fn init_requires_dry_run_for_now() {\n"
        "        let error = parse_arguments(&[\"monad\", \"init\"])\n"
        "            .expect_err(\"init without dry-run should fail\");\n\n"
        "        assert!(error.contains(\"init currently requires --dry-run\"));\n"
        "        assert!(error.contains(\"write behavior is not implemented\"));\n"
        "    }\n\n"
        "    #[test]\n"
        "    fn init_rejects_yes_until_guarded_write_exists() {\n"
        "        let error = parse_arguments(&[\"monad\", \"init\", \"--dry-run\", \"--yes\"])\n"
        "            .expect_err(\"init --yes should fail for now\");\n\n"
        "        assert!(error.contains(\"init --yes is reserved\"));\n"
        "        assert!(error.contains(\"dry-run only\"));\n"
        "    }\n\n"
        "    #[test]\n    fn info_command_parses_text_and_json_formats() {\n",
    )

main = main.replace(
    "        assert!(text.contains(\"plan \\\"<intent>\\\"\"));\n",
    "        assert!(text.contains(\"init --dry-run\"));\n"
    "        assert!(text.contains(\"plan \\\"<intent>\\\"\"));\n",
)

main_path.write_text(main, encoding="utf-8")

cmd_path = Path("docs/commands/INIT.md")
if cmd_path.exists():
    cmd = cmd_path.read_text(encoding="utf-8")
    cmd = cmd.replace("work_packet: WP-E11-001", "work_packet: WP-E11-002")
    cmd = cmd.replace(
        "status: accepted",
        "status: accepted",
        1,
    )
    if "## WP-E11-002 Implementation Note" not in cmd:
        cmd += """

## WP-E11-002 Implementation Note

WP-E11-002 adds the first implemented `monad init` behavior:

```bash
monad init --dry-run
monad init --preset=minimal --dry-run
monad init --preset=polyglot-minimal --dry-run
monad init --name=my-project --dry-run
```

The command is dry-run only in this slice.

It renders a safe file-operation plan and writes no files.
"""
    cmd_path.write_text(cmd, encoding="utf-8")

readme_path = Path("README.md")
if readme_path.exists():
    readme = readme_path.read_text(encoding="utf-8")
    readme = readme.replace(
        "monad info\nmonad inspect",
        "monad init --dry-run\nmonad info\nmonad inspect",
    )
    readme = readme.replace(
        "monad init\nmonad add",
        "monad add",
    )
    readme = readme.replace(
        "Safety boundary: `plan` is no-write, `evolve` commands are dry-run only, and write behavior is currently limited to explicit context export/generation commands.",
        "Safety boundary: `init` and `evolve` commands are dry-run only, `plan` is no-write, and write behavior is currently limited to explicit context export/generation commands.",
    )
    readme_path.write_text(readme, encoding="utf-8")

reference_path = Path("docs/project/MVP-COMMAND-REFERENCE.md")
if reference_path.exists():
    reference = reference_path.read_text(encoding="utf-8")
    if "### Init dry-run" not in reference:
        reference = reference.replace(
            "## 3. Current core commands\n\n",
            "## 3. Current core commands\n\n"
            "### Init dry-run\n\n"
            "```bash\n"
            "cargo run -p monad-cli -- init --dry-run\n"
            "cargo run -p monad-cli -- init --preset=minimal --dry-run\n"
            "cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run\n"
            "cargo run -p monad-cli -- init --name=my-project --dry-run\n"
            "```\n\n"
            "Expected behavior:\n\n"
            "* previews repository initialization file operations\n"
            "* writes no files\n"
            "* rejects `--yes` until guarded write behavior is implemented\n"
            "* detects conflicts such as existing target files\n\n",
        )
    reference = reference.replace(
        "cargo run -p monad-cli -- --help\ncargo run -p monad-cli -- version\ncargo run -p monad-cli -- inspect",
        "cargo run -p monad-cli -- --help\ncargo run -p monad-cli -- version\ncargo run -p monad-cli -- init --dry-run\ncargo run -p monad-cli -- inspect",
    )
    reference_path.write_text(reference, encoding="utf-8")
PY

cat > work/deliverables/E11/WP-E11-002-init-dry-run-plan.md <<'EOF'
---
title: "WP-E11-002 Init Dry-Run Plan Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E11
work_packet: WP-E11-002
tags:
  - monad
  - e11
  - init
  - dry-run
  - rust
related:
  - crates/monad-core/src/init.rs
  - crates/monad-core/src/lib.rs
  - crates/monad-cli/src/main.rs
  - docs/commands/INIT.md
---

# WP-E11-002 Init Dry-Run Plan Deliverable

## Work Packet

WP-E11-002 — Add init dry-run plan.

## Outcome

Implemented.

## Summary

This work packet adds the first implemented `monad init` behavior.

The command is intentionally dry-run only:

```bash
cargo run -p monad-cli -- init --dry-run
```

It supports:

```bash
cargo run -p monad-cli -- init --dry-run
cargo run -p monad-cli -- init --preset=minimal --dry-run
cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run
cargo run -p monad-cli -- init --name=my-project --dry-run
```

It rejects write approval for now:

```bash
cargo run -p monad-cli -- init --dry-run --yes
```

## Deliverables

- `crates/monad-core/src/init.rs`
- `crates/monad-core/src/lib.rs`
- `crates/monad-cli/src/main.rs`
- `docs/commands/INIT.md`
- `docs/project/MVP-COMMAND-REFERENCE.md`
- `README.md`
- `work/deliverables/E11/WP-E11-002-init-dry-run-plan.md`

## Safety Boundary

WP-E11-002 adds no write behavior.

The command:

- writes no files;
- does not create directories;
- does not initialize Git;
- does not commit;
- does not apply templates;
- rejects `--yes`;
- uses the existing file-operation dry-run evaluator.

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- init --dry-run
cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run
cargo run -p monad-cli -- init --name=my-project --dry-run
cargo run -p monad-cli -- init --dry-run --yes
tools/scripts/verify.sh
git status --short
```

The `--yes` command is expected to fail with an actionable dry-run-only error.

## Recommended Commit

```bash
git commit -m "feat(init): add init dry-run plan"
```

## Closeout Note

WP-E11-002 is complete once the init dry-run plan is committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E11-003 — Add minimal embedded scaffold templates
```
EOF

cargo fmt

echo
echo "WP-E11-002 files updated:"
echo "  crates/monad-core/src/init.rs"
echo "  crates/monad-core/src/lib.rs"
echo "  crates/monad-cli/src/main.rs"
echo "  docs/commands/INIT.md"
echo "  docs/project/MVP-COMMAND-REFERENCE.md"
echo "  README.md"
echo "  work/deliverables/E11/WP-E11-002-init-dry-run-plan.md"
echo
echo "Next verification:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  cargo run -p monad-cli -- init --dry-run"
echo "  tools/scripts/verify.sh"
