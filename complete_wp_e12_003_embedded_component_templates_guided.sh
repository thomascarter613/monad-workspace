#!/usr/bin/env bash
set -euo pipefail

# Complete WP-E12-003 — Add embedded component scaffold templates.
#
# Learning-first script:
# - It keeps the convenience of a single script.
# - It creates a learning note that explains the design shift.
# - It keeps monad add dry-run only.
#
# Safety boundary:
# - No component files are written.
# - No directories are created.
# - No package managers are run.
# - No Git commands are run.
# - `monad add --yes` remains unsupported.

echo "==> WP-E12-003 learning checkpoint"
echo "This packet changes monad add planning from hard-coded paths to embedded component scaffold templates."
echo "Mental model: component request -> choose embedded templates -> resolve target paths -> evaluate dry-run."
echo

mkdir -p work/deliverables/E12
mkdir -p work/learning/E12

cat > crates/monad-core/src/component_add.rs <<'EOF'
//! Component add planning.
//!
//! This module backs `monad add <kind> <name> --dry-run`.
//!
//! WP-E12-003 introduces an embedded component scaffold template layer.
//!
//! The important design idea is now:
//!
//! - user input chooses a component kind and name;
//! - the component kind chooses the component root;
//! - embedded templates describe the scaffold files;
//! - the planner resolves template-relative paths into component paths;
//! - the dry-run evaluator checks the filesystem without writing files.
//!
//! WP-E12-003 remains dry-run only. It does not write component files.

use std::path::{Path, PathBuf};

use crate::{
    FileOperationPlan, MonadError, MonadResult, PlannedFileOperation, WorkspaceContext,
    evaluate_file_operation_plan, render_dry_run_plan,
};

/// Supported component kinds for the first `monad add` implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComponentKind {
    /// User-facing application component, placed under `apps/`.
    App,

    /// Shared package/library component, placed under `packages/`.
    Package,

    /// Backend/service component, placed under `services/`.
    Service,

    /// Repo-local tool component, placed under `tools/`.
    Tool,
}

impl ComponentKind {
    /// Parses a user-facing component kind.
    pub fn parse(value: &str) -> MonadResult<Self> {
        match value {
            "app" => Ok(Self::App),
            "package" => Ok(Self::Package),
            "service" => Ok(Self::Service),
            "tool" => Ok(Self::Tool),
            other => Err(MonadError::invalid_input(format!(
                "unsupported component kind `{other}`; supported kinds: app, package, service, tool"
            ))),
        }
    }

    /// Returns the stable user-facing label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::App => "app",
            Self::Package => "package",
            Self::Service => "service",
            Self::Tool => "tool",
        }
    }

    /// Returns the root directory family for the component kind.
    #[must_use]
    pub const fn base_dir(self) -> &'static str {
        match self {
            Self::App => "apps",
            Self::Package => "packages",
            Self::Service => "services",
            Self::Tool => "tools",
        }
    }
}

/// Filesystem-safe component name.
///
/// This type is deliberately stricter than a general path. Users give Monad a
/// component name, not a path. Monad decides the path from the component kind.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentName(String);

impl ComponentName {
    /// Parses and validates a component name.
    pub fn parse(value: &str) -> MonadResult<Self> {
        if value.is_empty() {
            return Err(MonadError::invalid_input(
                "component name must not be empty",
            ));
        }

        let first = value
            .chars()
            .next()
            .ok_or_else(|| MonadError::invalid_input("component name must not be empty"))?;

        if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
            return Err(MonadError::invalid_input(format!(
                "invalid component name `{value}`; names must start with a lowercase letter or number"
            )));
        }

        if !value
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-')
        {
            return Err(MonadError::invalid_input(format!(
                "invalid component name `{value}`; use lowercase letters, numbers, and hyphens only"
            )));
        }

        if value.contains("--") || value.ends_with('-') {
            return Err(MonadError::invalid_input(format!(
                "invalid component name `{value}`; avoid repeated or trailing hyphens"
            )));
        }

        Ok(Self(value.to_string()))
    }

    /// Returns the safe component name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Embedded scaffold template for a component.
///
/// This intentionally avoids a full templating engine. A small explicit
/// placeholder replacement is enough for the first E12 scaffold layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentScaffoldTemplate {
    id: &'static str,
    relative_path: &'static str,
    description: &'static str,
    content: &'static str,
}

impl ComponentScaffoldTemplate {
    /// Returns the stable template ID.
    #[must_use]
    pub const fn id(self) -> &'static str {
        self.id
    }

    /// Returns the path relative to the component root.
    #[must_use]
    pub const fn relative_path(self) -> &'static str {
        self.relative_path
    }

    /// Returns a human-readable description.
    #[must_use]
    pub const fn description(self) -> &'static str {
        self.description
    }

    /// Renders template content for a component.
    #[must_use]
    pub fn render_content(self, options: &AddPlanOptions) -> String {
        self.content
            .replace("{{component_kind}}", options.kind().as_str())
            .replace("{{component_name}}", options.name().as_str())
            .replace(
                "{{component_root}}",
                &options.component_root().display().to_string(),
            )
    }
}

const COMPONENT_SCAFFOLD_TEMPLATES: [ComponentScaffoldTemplate; 2] = [
    ComponentScaffoldTemplate {
        id: "component.readme",
        relative_path: "README.md",
        description: "component documentation entry point",
        content: "# {{component_name}}\n\nKind: `{{component_kind}}`\n\nRoot: `{{component_root}}`\n\nThis component was planned by Monad.\n",
    },
    ComponentScaffoldTemplate {
        id: "component.gitkeep",
        relative_path: ".gitkeep",
        description: "component directory placeholder",
        content: "",
    },
];

/// Returns the embedded component scaffold templates.
#[must_use]
pub const fn component_scaffold_templates() -> &'static [ComponentScaffoldTemplate] {
    &COMPONENT_SCAFFOLD_TEMPLATES
}

/// Options used to build a component add plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddPlanOptions {
    kind: ComponentKind,
    name: ComponentName,
}

impl AddPlanOptions {
    /// Creates add plan options.
    #[must_use]
    pub fn new(kind: ComponentKind, name: ComponentName) -> Self {
        Self { kind, name }
    }

    /// Returns the component kind.
    #[must_use]
    pub const fn kind(&self) -> ComponentKind {
        self.kind
    }

    /// Returns the component name.
    #[must_use]
    pub const fn name(&self) -> &ComponentName {
        &self.name
    }

    /// Returns the planned component root, relative to the workspace root.
    #[must_use]
    pub fn component_root(&self) -> PathBuf {
        Path::new(self.kind.base_dir()).join(self.name.as_str())
    }
}

/// Builds the component add file-operation plan.
///
/// This is intentionally minimal. Language-specific files are left to later
/// E12 work packets and future language adapter epics.
#[must_use]
pub fn build_add_plan(options: &AddPlanOptions) -> FileOperationPlan {
    let operations = component_scaffold_templates()
        .iter()
        .map(|template| {
            let target_path = options.component_root().join(template.relative_path());
            let rendered_content = template.render_content(options);

            PlannedFileOperation::create(
                target_path.clone(),
                format!(
                    "create `{}` from embedded component template `{}` ({}, {} bytes)",
                    target_path.display(),
                    template.id(),
                    template.description(),
                    rendered_content.len()
                ),
            )
        })
        .collect::<Vec<_>>();

    FileOperationPlan::from_operations(operations)
}

/// Renders the dry-run output for `monad add`.
pub fn render_add_dry_run(
    context: &WorkspaceContext,
    options: &AddPlanOptions,
) -> MonadResult<String> {
    let plan = build_add_plan(options);
    let dry_run = evaluate_file_operation_plan(context.root(), &plan);
    let root = options.component_root();

    let mut output = vec![
        "Monad add dry-run plan".to_string(),
        String::new(),
        "Component:".to_string(),
        format!("  kind: {}", options.kind().as_str()),
        format!("  name: {}", options.name().as_str()),
        format!("  root: {}", root.display()),
        String::new(),
        "Template source:".to_string(),
        "  registry: embedded component scaffold templates".to_string(),
        format!("  templates: {}", component_scaffold_templates().len()),
        String::new(),
        "Safety:".to_string(),
        "  mode: dry-run".to_string(),
        "  writes: disabled".to_string(),
        "  apply: not implemented in WP-E12-003".to_string(),
        "  approval_flag: --yes reserved for a later E12 work packet".to_string(),
        String::new(),
        render_dry_run_plan(&dry_run),
        String::new(),
        "No files were written.".to_string(),
        "Next: review this plan before a future guarded write path is added.".to_string(),
    ];

    if dry_run.has_conflicts() {
        output.push(
            "Conflicts were detected. Resolve existing target paths before any future write path."
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

        std::env::temp_dir().join(format!("monad-add-{name}-{unique}"))
    }

    #[test]
    fn component_kind_parses_supported_values() -> MonadResult<()> {
        assert_eq!(ComponentKind::parse("app")?, ComponentKind::App);
        assert_eq!(ComponentKind::parse("package")?, ComponentKind::Package);
        assert_eq!(ComponentKind::parse("service")?, ComponentKind::Service);
        assert_eq!(ComponentKind::parse("tool")?, ComponentKind::Tool);

        Ok(())
    }

    #[test]
    fn component_kind_rejects_unknown_values() {
        let error = ComponentKind::parse("website").expect_err("unknown kind should fail");

        assert!(error.to_string().contains("unsupported component kind"));
    }

    #[test]
    fn component_name_accepts_safe_names() -> MonadResult<()> {
        assert_eq!(ComponentName::parse("web")?.as_str(), "web");
        assert_eq!(ComponentName::parse("shared-ui")?.as_str(), "shared-ui");
        assert_eq!(ComponentName::parse("worker-1")?.as_str(), "worker-1");

        Ok(())
    }

    #[test]
    fn component_name_rejects_unsafe_names() {
        for value in ["", "../web", "apps/web", "Web", "web app", "web;", "-web", "web-"] {
            assert!(
                ComponentName::parse(value).is_err(),
                "expected `{value}` to be rejected"
            );
        }
    }

    #[test]
    fn component_templates_are_embedded_and_stable() {
        let templates = component_scaffold_templates();

        assert_eq!(templates.len(), 2);
        assert_eq!(templates[0].id(), "component.readme");
        assert_eq!(templates[0].relative_path(), "README.md");
        assert_eq!(templates[1].id(), "component.gitkeep");
        assert_eq!(templates[1].relative_path(), ".gitkeep");
    }

    #[test]
    fn component_readme_template_renders_component_values() -> MonadResult<()> {
        let options = AddPlanOptions::new(ComponentKind::Service, ComponentName::parse("api")?);
        let readme = component_scaffold_templates()[0].render_content(&options);

        assert!(readme.contains("# api"));
        assert!(readme.contains("Kind: `service`"));
        assert!(readme.contains("Root: `services/api`"));

        Ok(())
    }

    #[test]
    fn add_plan_targets_kind_specific_root_from_templates() -> MonadResult<()> {
        let options = AddPlanOptions::new(ComponentKind::Service, ComponentName::parse("api")?);
        let plan = build_add_plan(&options);
        let targets = plan
            .operations()
            .iter()
            .map(|operation| operation.target().display_path())
            .collect::<Vec<_>>();

        assert_eq!(options.component_root(), PathBuf::from("services/api"));
        assert!(targets.contains(&"services/api/README.md".to_string()));
        assert!(targets.contains(&"services/api/.gitkeep".to_string()));

        assert!(
            plan.operations()
                .iter()
                .any(|operation| operation.explanation().contains("component.readme"))
        );

        Ok(())
    }

    #[test]
    fn add_dry_run_output_states_no_files_written() -> MonadResult<()> {
        let root = unique_temp_root("render");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let options = AddPlanOptions::new(ComponentKind::App, ComponentName::parse("web")?);
        let output = render_add_dry_run(&context, &options)?;

        assert!(output.contains("Monad add dry-run plan"));
        assert!(output.contains("kind: app"));
        assert!(output.contains("name: web"));
        assert!(output.contains("Template source:"));
        assert!(output.contains("component.readme"));
        assert!(output.contains("apps/web/README.md"));
        assert!(output.contains("No files were written."));

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn add_dry_run_detects_existing_readme_conflict() -> MonadResult<()> {
        let root = unique_temp_root("conflict");
        let component_root = root.join("apps/web");
        fs::create_dir_all(&component_root).map_err(|error| {
            MonadError::internal(format!("test component root should be created: {error}"))
        })?;
        fs::write(component_root.join("README.md"), "# Existing\n").map_err(|error| {
            MonadError::internal(format!("test README should be written: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let options = AddPlanOptions::new(ComponentKind::App, ComponentName::parse("web")?);
        let plan = build_add_plan(&options);
        let dry_run = evaluate_file_operation_plan(context.root(), &plan);

        assert!(
            dry_run
                .operations()
                .iter()
                .any(|operation| operation.target().display_path() == "apps/web/README.md"
                    && operation.outcome_kind() == DryRunOperationKind::Conflict)
        );

        fs::remove_dir_all(&root).ok();

        Ok(())
    }
}
EOF

cat > work/learning/E12/WP-E12-003-embedded-component-scaffold-templates.md <<'EOF'
---
title: "Learning Note — WP-E12-003 Embedded Component Scaffold Templates"
document_type: "learning-note"
status: active
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-003
tags:
  - learning
  - rust
  - templates
  - dry-run
  - monad
---

# Learning Note — WP-E12-003 Embedded Component Scaffold Templates

## What Changed

WP-E12-002 built the `add` plan directly from hard-coded paths.

WP-E12-003 introduces a tiny embedded template layer:

```text
component.readme
component.gitkeep
```

The command still does not write files.

## Why This Matters

Hard-coded paths are fine for the first slice, but they do not scale.

Templates give us a place to attach:

- a stable ID;
- a relative path;
- a description;
- content for a future write path.

## Main File to Read

```text
crates/monad-core/src/component_add.rs
```

Start with:

```text
ComponentScaffoldTemplate
COMPONENT_SCAFFOLD_TEMPLATES
component_scaffold_templates
```

## New Mental Model

The add flow is now:

```text
component kind + name
  -> component root
  -> embedded component templates
  -> target paths
  -> dry-run file operation plan
  -> rendered dry-run output
```

## Rust Concept: Struct for Template Metadata

`ComponentScaffoldTemplate` is a struct.

It groups related template facts:

```text
id
relative_path
description
content
```

That keeps the plan builder from spreading template details across unrelated code.

## Rust Concept: Constants

The templates are stored in a constant array:

```text
COMPONENT_SCAFFOLD_TEMPLATES
```

This means the templates are compiled into the binary.

No network fetch.

No plugin install.

No runtime template discovery.

## Rust Concept: Placeholder Replacement

The README template uses simple placeholders:

```text
{{component_kind}}
{{component_name}}
{{component_root}}
```

The renderer replaces them with safe values.

This is not a full templating engine.

That is intentional.

## What to Inspect

```bash
git diff -- crates/monad-core/src/component_add.rs
```

Look for:

```text
ComponentScaffoldTemplate
component_scaffold_templates
render_content
build_add_plan
```

## Verification Commands

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- add app web --dry-run
cargo run -p monad-cli -- add service api --dry-run
```
EOF

cat > work/deliverables/E12/WP-E12-003-embedded-component-scaffold-templates.md <<'EOF'
---
title: "WP-E12-003 Embedded Component Scaffold Templates Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-003
tags:
  - monad
  - e12
  - add
  - templates
  - component
related:
  - crates/monad-core/src/component_add.rs
  - work/learning/E12/WP-E12-003-embedded-component-scaffold-templates.md
---

# WP-E12-003 Embedded Component Scaffold Templates Deliverable

## Work Packet

WP-E12-003 — Add embedded component scaffold templates.

## Outcome

Implemented.

## Summary

This work packet replaces hard-coded `monad add` planned paths with embedded component scaffold templates.

The implementation remains dry-run only.

## Embedded Templates

```text
component.readme
component.gitkeep
```

## Deliverables

- `crates/monad-core/src/component_add.rs`
- `work/learning/E12/WP-E12-003-embedded-component-scaffold-templates.md`
- `work/deliverables/E12/WP-E12-003-embedded-component-scaffold-templates.md`

## Safety Boundary

WP-E12-003 does not write files.

It only makes dry-run planning template-backed.

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- add app web --dry-run
cargo run -p monad-cli -- add service api --dry-run
tools/scripts/verify.sh
git status --short
```

## Recommended Commit

```bash
git commit -m "feat(add): add embedded component scaffold templates"
```

## Closeout Note

WP-E12-003 is complete once the template-backed dry-run plan is committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E12-004 — Add guarded add write path
```
EOF

cargo fmt

echo
echo "WP-E12-003 files updated:"
echo "  crates/monad-core/src/component_add.rs"
echo "  work/learning/E12/WP-E12-003-embedded-component-scaffold-templates.md"
echo "  work/deliverables/E12/WP-E12-003-embedded-component-scaffold-templates.md"
echo
echo "Learning checkpoint:"
echo "  Read work/learning/E12/WP-E12-003-embedded-component-scaffold-templates.md before committing."
echo
echo "Next verification:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  cargo run -p monad-cli -- add app web --dry-run"
