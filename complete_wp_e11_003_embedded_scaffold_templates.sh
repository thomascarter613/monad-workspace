#!/usr/bin/env bash
set -euo pipefail

# Complete WP-E11-003 — Add minimal embedded scaffold templates.
#
# This script extends the existing embedded template registry with init scaffold
# templates and updates the init dry-run plan to source its planned files from
# the registry.
#
# Safety boundary:
# - still dry-run only
# - no scaffold files are written
# - guarded write path remains WP-E11-004

mkdir -p work/deliverables/E11

cat > crates/monad-core/src/templates/registry.rs <<'EOF'
//! Local template registry.

use std::collections::BTreeMap;

use crate::{MonadError, MonadResult, TemplateDefinition, TemplateId};

/// Registry of known local templates.
///
/// The registry is deterministic because it uses a `BTreeMap`. This keeps
/// iteration and tests stable.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TemplateRegistry {
    templates: BTreeMap<TemplateId, TemplateDefinition>,
}

impl TemplateRegistry {
    /// Creates an empty template registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry from templates.
    pub fn from_templates(
        templates: impl IntoIterator<Item = TemplateDefinition>,
    ) -> MonadResult<Self> {
        let mut registry = Self::new();

        for template in templates {
            registry.register(template)?;
        }

        Ok(registry)
    }

    /// Registers a template.
    ///
    /// Duplicate IDs are rejected so evolution commands cannot silently replace
    /// known template source material.
    pub fn register(&mut self, template: TemplateDefinition) -> MonadResult<()> {
        let id = template.id().clone();

        if self.templates.contains_key(&id) {
            return Err(MonadError::invalid_input(format!(
                "template `{}` is already registered",
                id.as_str()
            )));
        }

        self.templates.insert(id, template);

        Ok(())
    }

    /// Returns true if a template ID is registered.
    #[must_use]
    pub fn contains(&self, id: &TemplateId) -> bool {
        self.templates.contains_key(id)
    }

    /// Looks up a template by ID.
    #[must_use]
    pub fn get(&self, id: &TemplateId) -> Option<&TemplateDefinition> {
        self.templates.get(id)
    }

    /// Looks up a template by string ID.
    #[must_use]
    pub fn get_by_str(&self, id: &str) -> Option<&TemplateDefinition> {
        self.get(&TemplateId::new(id))
    }

    /// Returns registered templates in deterministic ID order.
    #[must_use]
    pub fn templates(&self) -> Vec<&TemplateDefinition> {
        self.templates.values().collect()
    }

    /// Returns registered template IDs in deterministic order.
    #[must_use]
    pub fn ids(&self) -> Vec<&TemplateId> {
        self.templates.keys().collect()
    }

    /// Returns the number of registered templates.
    #[must_use]
    pub fn len(&self) -> usize {
        self.templates.len()
    }

    /// Returns true when no templates are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }
}

/// Builds the initial embedded template registry.
///
/// The registry remains local and deterministic. It now includes the E5
/// baseline templates plus the E11 init scaffold templates used by
/// `monad init --dry-run`.
pub fn initial_template_registry() -> MonadResult<TemplateRegistry> {
    TemplateRegistry::from_templates([
        TemplateDefinition::embedded(
            "verify-baseline.readme",
            "Verification Baseline README",
            "1",
            "Creates a placeholder README for future verification baseline documentation.",
            "docs/verification/README.md",
            "# Verification\n\nThis directory documents repository verification practices.\n",
        ),
        TemplateDefinition::embedded(
            "context-baseline.readme",
            "Context Baseline README",
            "1",
            "Creates a placeholder README for future context bridge documentation.",
            "docs/ai/README.md",
            "# AI Context\n\nThis directory contains AI-readable project context.\n",
        ),
        TemplateDefinition::embedded(
            "init.minimal.monad-toml",
            "Minimal Monad Manifest",
            "1",
            "Creates the initial Monad manifest for a repository.",
            "monad.toml",
            r#"# Monad repository manifest.
schema_version = 1

[project]
name = "monad-project"
display_name = "Monad Project"
description = "A Monad-aware software repository."

[workspace]
members = []

[runtime]
core = "monad-core"
cli = "monad-cli"
execution_model = "local-first"
"#,
        ),
        TemplateDefinition::embedded(
            "init.minimal.readme",
            "Minimal Repository README",
            "1",
            "Creates a minimal repository README entry point.",
            "README.md",
            "# Monad Project\n\nThis repository is initialized for Monad-aware local development.\n",
        ),
        TemplateDefinition::embedded(
            "init.minimal.docs-readme",
            "Minimal Documentation README",
            "1",
            "Creates a documentation directory entry point.",
            "docs/README.md",
            "# Documentation\n\nThis directory contains project documentation.\n",
        ),
        TemplateDefinition::embedded(
            "init.minimal.work-readme",
            "Minimal Work README",
            "1",
            "Creates a work-tracking directory entry point.",
            "work/README.md",
            "# Work\n\nThis directory contains repo-native work records.\n",
        ),
        TemplateDefinition::embedded(
            "init.minimal.monad-gitignore",
            "Minimal Monad State Gitignore",
            "1",
            "Creates a local/generated Monad state ignore policy.",
            ".monad/.gitignore",
            "# Local/generated Monad state.\n*\n!.gitignore\n",
        ),
        TemplateDefinition::embedded(
            "init.polyglot.apps-gitkeep",
            "Polyglot Apps Placeholder",
            "1",
            "Creates an apps directory placeholder.",
            "apps/.gitkeep",
            "",
        ),
        TemplateDefinition::embedded(
            "init.polyglot.packages-gitkeep",
            "Polyglot Packages Placeholder",
            "1",
            "Creates a packages directory placeholder.",
            "packages/.gitkeep",
            "",
        ),
        TemplateDefinition::embedded(
            "init.polyglot.services-gitkeep",
            "Polyglot Services Placeholder",
            "1",
            "Creates a services directory placeholder.",
            "services/.gitkeep",
            "",
        ),
        TemplateDefinition::embedded(
            "init.polyglot.tools-gitkeep",
            "Polyglot Tools Placeholder",
            "1",
            "Creates a tools directory placeholder.",
            "tools/.gitkeep",
            "",
        ),
    ])
}

#[cfg(test)]
mod tests {
    use crate::TemplateSourceKind;

    use super::*;

    fn example_template(id: &str) -> TemplateDefinition {
        TemplateDefinition::embedded(
            id,
            "Example Template",
            "1",
            "Example local embedded template.",
            "docs/example.md",
            "# Example\n",
        )
    }

    #[test]
    fn registry_starts_empty() {
        let registry = TemplateRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn registry_registers_and_retrieves_template() -> MonadResult<()> {
        let mut registry = TemplateRegistry::new();
        let template = example_template("example.template");

        registry.register(template)?;

        let retrieved = registry
            .get_by_str("example.template")
            .ok_or_else(|| MonadError::not_found("template should be registered"))?;

        assert_eq!(retrieved.id().as_str(), "example.template");
        assert_eq!(retrieved.metadata().version(), "1");
        assert_eq!(
            retrieved.metadata().source_kind(),
            TemplateSourceKind::Embedded
        );
        assert_eq!(retrieved.content(), "# Example\n");

        Ok(())
    }

    #[test]
    fn registry_rejects_duplicate_template_ids() -> MonadResult<()> {
        let mut registry = TemplateRegistry::new();

        registry.register(example_template("duplicate.template"))?;
        let error = registry
            .register(example_template("duplicate.template"))
            .expect_err("duplicate template registration should fail");

        assert_eq!(error.code(), "MONAD2001");
        assert!(error.message().contains("already registered"));

        Ok(())
    }

    #[test]
    fn registry_lists_templates_in_deterministic_order() -> MonadResult<()> {
        let registry = TemplateRegistry::from_templates([
            example_template("z.template"),
            example_template("a.template"),
            example_template("m.template"),
        ])?;

        let ids = registry
            .ids()
            .iter()
            .map(|id| id.as_str())
            .collect::<Vec<_>>();

        assert_eq!(ids, vec!["a.template", "m.template", "z.template"]);

        Ok(())
    }

    #[test]
    fn initial_registry_contains_baseline_templates() -> MonadResult<()> {
        let registry = initial_template_registry()?;

        assert!(registry.contains(&TemplateId::new("verify-baseline.readme")));
        assert!(registry.contains(&TemplateId::new("context-baseline.readme")));

        Ok(())
    }

    #[test]
    fn initial_registry_contains_minimal_init_templates() -> MonadResult<()> {
        let registry = initial_template_registry()?;

        assert!(registry.contains(&TemplateId::new("init.minimal.monad-toml")));
        assert!(registry.contains(&TemplateId::new("init.minimal.readme")));
        assert!(registry.contains(&TemplateId::new("init.minimal.docs-readme")));
        assert!(registry.contains(&TemplateId::new("init.minimal.work-readme")));
        assert!(registry.contains(&TemplateId::new("init.minimal.monad-gitignore")));

        Ok(())
    }

    #[test]
    fn initial_registry_contains_polyglot_init_templates() -> MonadResult<()> {
        let registry = initial_template_registry()?;

        assert!(registry.contains(&TemplateId::new("init.polyglot.apps-gitkeep")));
        assert!(registry.contains(&TemplateId::new("init.polyglot.packages-gitkeep")));
        assert!(registry.contains(&TemplateId::new("init.polyglot.services-gitkeep")));
        assert!(registry.contains(&TemplateId::new("init.polyglot.tools-gitkeep")));

        Ok(())
    }
}
EOF

cat > crates/monad-core/src/init.rs <<'EOF'
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
EOF

python3 - <<'PY'
from pathlib import Path

# Keep docs current for WP-E11-003.
cmd_path = Path("docs/commands/INIT.md")
if cmd_path.exists():
    text = cmd_path.read_text(encoding="utf-8")
    if "## WP-E11-003 Implementation Note" not in text:
        text += """

## WP-E11-003 Implementation Note

WP-E11-003 adds embedded scaffold templates to the existing local template registry.

`monad init --dry-run` now previews planned file operations sourced from embedded templates rather than hard-coded target paths.

The command remains dry-run only.

No files are written until the guarded write path is implemented in WP-E11-004.
"""
    cmd_path.write_text(text, encoding="utf-8")

ref_path = Path("docs/project/MVP-COMMAND-REFERENCE.md")
if ref_path.exists():
    text = ref_path.read_text(encoding="utf-8")
    if "previews embedded scaffold-template file operations" not in text:
        text = text.replace(
            "* previews repository initialization file operations\n",
            "* previews embedded scaffold-template file operations\n",
        )
    ref_path.write_text(text, encoding="utf-8")
PY

cat > work/deliverables/E11/WP-E11-003-minimal-embedded-scaffold-templates.md <<'EOF'
---
title: "WP-E11-003 Minimal Embedded Scaffold Templates Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E11
work_packet: WP-E11-003
tags:
  - monad
  - e11
  - init
  - templates
  - scaffold
related:
  - crates/monad-core/src/templates/registry.rs
  - crates/monad-core/src/init.rs
  - docs/commands/INIT.md
---

# WP-E11-003 Minimal Embedded Scaffold Templates Deliverable

## Work Packet

WP-E11-003 — Add minimal embedded scaffold templates.

## Outcome

Implemented.

## Summary

This work packet extends the existing embedded template registry with init scaffold templates and updates the `monad init --dry-run` plan to source planned file operations from those templates.

The implementation remains dry-run only.

## Deliverables

- `crates/monad-core/src/templates/registry.rs`
- `crates/monad-core/src/init.rs`
- `docs/commands/INIT.md`
- `docs/project/MVP-COMMAND-REFERENCE.md`
- `work/deliverables/E11/WP-E11-003-minimal-embedded-scaffold-templates.md`

## Embedded Init Templates

Minimal preset templates:

```text
init.minimal.monad-toml
init.minimal.readme
init.minimal.docs-readme
init.minimal.work-readme
init.minimal.monad-gitignore
```

Polyglot-minimal additional templates:

```text
init.polyglot.apps-gitkeep
init.polyglot.packages-gitkeep
init.polyglot.services-gitkeep
init.polyglot.tools-gitkeep
```

## Safety Boundary

WP-E11-003 adds template source material and planning integration only.

It does not:

- write files;
- create directories;
- apply templates;
- initialize Git;
- commit changes;
- enable `--yes`.

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- init --dry-run
cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run
cargo run -p monad-cli -- init --dry-run | grep "Template source"
cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run | grep "apps/.gitkeep"
tools/scripts/verify.sh
git status --short
```

## Recommended Commit

```bash
git commit -m "feat(init): add embedded scaffold templates"
```

## Closeout Note

WP-E11-003 is complete once the embedded scaffold templates are committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E11-004 — Add guarded init write path
```
EOF

cargo fmt

echo
echo "WP-E11-003 files updated:"
echo "  crates/monad-core/src/templates/registry.rs"
echo "  crates/monad-core/src/init.rs"
echo "  docs/commands/INIT.md"
echo "  docs/project/MVP-COMMAND-REFERENCE.md"
echo "  work/deliverables/E11/WP-E11-003-minimal-embedded-scaffold-templates.md"
echo
echo "Next verification:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  cargo run -p monad-cli -- init --dry-run"
echo "  cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run"
echo "  tools/scripts/verify.sh"
