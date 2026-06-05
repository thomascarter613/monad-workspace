#!/usr/bin/env bash
set -euo pipefail

# Complete WP-E12-002 — Add add-command dry-run plan.
#
# Learning-first script:
# - Prints the mental model before patching.
# - Creates a learning note at work/learning/E12/WP-E12-002-add-dry-run-plan.md.
# - Performs the repo edits for the work packet.
#
# Safety boundary:
# - Adds dry-run planning only.
# - Does not implement monad add --yes.
# - Does not write scaffold components.
# - Does not install packages.
# - Does not run Git commands.

echo "==> WP-E12-002 learning checkpoint"
echo "This packet adds: monad add <kind> <name> --dry-run"
echo "Mental model: parse command -> validate component -> build file plan -> evaluate dry-run -> render output."
echo

mkdir -p work/deliverables/E12
mkdir -p work/learning/E12

cat > crates/monad-core/src/component_add.rs <<'EOF'
//! Component add planning.
//!
//! This module backs `monad add <kind> <name> --dry-run`.
//!
//! The important design idea is separation of responsibility:
//!
//! - `ComponentKind` knows the allowed component families.
//! - `ComponentName` validates a safe filesystem component name.
//! - `AddPlanOptions` carries the user's requested component.
//! - `build_add_plan` creates a reviewable file-operation plan.
//! - `render_add_dry_run` evaluates and prints that plan.
//!
//! WP-E12-002 is dry-run only. It does not write component files.

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
    let root = options.component_root();
    let readme_path = root.join("README.md");
    let gitkeep_path = root.join(".gitkeep");

    FileOperationPlan::from_operations([
        PlannedFileOperation::create(
            readme_path.clone(),
            format!(
                "create `{}` as the component documentation entry point",
                readme_path.display()
            ),
        ),
        PlannedFileOperation::create(
            gitkeep_path.clone(),
            format!(
                "create `{}` so the component directory is represented before language-specific files exist",
                gitkeep_path.display()
            ),
        ),
    ])
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
        "Safety:".to_string(),
        "  mode: dry-run".to_string(),
        "  writes: disabled".to_string(),
        "  apply: not implemented in WP-E12-002".to_string(),
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
    fn add_plan_targets_kind_specific_root() -> MonadResult<()> {
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

python3 - <<'PY'
from pathlib import Path
import re

lib_path = Path("crates/monad-core/src/lib.rs")
lib = lib_path.read_text(encoding="utf-8")

if "pub mod component_add;" not in lib:
    anchor = "pub mod checks;\n"
    if anchor not in lib:
        raise SystemExit("Could not find checks module anchor in lib.rs")
    lib = lib.replace(anchor, anchor + "pub mod component_add;\n", 1)

export = "pub use component_add::{AddPlanOptions, ComponentKind, ComponentName, build_add_plan, render_add_dry_run};\n"
if export not in lib:
    lib = lib + "\n" + export

lib_path.write_text(lib, encoding="utf-8")

main_path = Path("crates/monad-cli/src/main.rs")
main = main_path.read_text(encoding="utf-8")

match = re.search(r"use monad_core::\{(?P<body>.*?)\};", main, flags=re.S)
if not match:
    raise SystemExit("Could not find use monad_core::{...}; block in main.rs")

body = match.group("body")
for symbol in ["AddPlanOptions", "ComponentKind", "ComponentName", "render_add_dry_run"]:
    if symbol not in body:
        body = " " + symbol + "," + body
main = main[:match.start()] + "use monad_core::{" + body + "};" + main[match.end():]

if "Add {\n" not in main:
    anchor = "    Info {\n"
    if anchor not in main:
        raise SystemExit("Could not find Info variant anchor in CliCommand enum")
    add_variant = "\n".join([
        "    Add {",
        "        /// Component kind to add.",
        "        kind: ComponentKind,",
        "",
        "        /// Component name to add.",
        "        name: ComponentName,",
        "",
        "        /// Whether to run in dry-run mode.",
        "        dry_run: bool,",
        "    },",
        "",
        "",
    ])
    main = main.replace(anchor, add_variant + anchor, 1)

if '["add", kind, name] =>' not in main:
    anchor = '            ["init"] => {'
    if anchor not in main:
        raise SystemExit("Could not find init parse arm anchor")
    add_arm = "\n".join([
        '            ["add", kind, name] => {',
        "                reject_write_for_non_context(write)?;",
        "                if yes {",
        '                    return Err("add --yes is reserved for a later guarded write path; WP-E12-002 is dry-run only".to_string());',
        "                }",
        "                if !dry_run {",
        '                    return Err("add currently requires --dry-run; write behavior is not implemented".to_string());',
        "                }",
        "                if requested_format.is_some() {",
        '                    return Err("add does not support --format yet".to_string());',
        "                }",
        "                if requested_preset.is_some() {",
        '                    return Err("add does not support --preset yet".to_string());',
        "                }",
        "                if requested_project_name.is_some() {",
        '                    return Err("add does not support --name yet; pass the component name positionally".to_string());',
        "                }",
        "",
        "                Ok(Self::Add {",
        "                    kind: ComponentKind::parse(kind).map_err(|error| error.to_string())?,",
        "                    name: ComponentName::parse(name).map_err(|error| error.to_string())?,",
        "                    dry_run,",
        "                })",
        "            }",
        "",
        "",
    ])
    main = main.replace(anchor, add_arm + anchor, 1)

if "CliCommand::Add { kind, name, dry_run }" not in main:
    anchor = "        CliCommand::Init {"
    if anchor not in main:
        raise SystemExit("Could not find Init run match anchor")
    add_run = "        CliCommand::Add { kind, name, dry_run } => render_add(dry_run, kind, name),\n"
    main = main.replace(anchor, add_run + anchor, 1)

if "fn render_add(" not in main:
    anchor = "/// Renders or applies repository initialization."
    if anchor not in main:
        anchor = "/// Renders repository initialization dry-run output."
    if anchor not in main:
        raise SystemExit("Could not find render_init anchor")
    render_add = "\n".join([
        "/// Renders component add dry-run output.",
        "fn render_add(",
        "    dry_run: bool,",
        "    kind: ComponentKind,",
        "    name: ComponentName,",
        ") -> Result<String, String> {",
        "    if !dry_run {",
        '        return Err("add currently requires --dry-run; write behavior is not implemented".to_string());',
        "    }",
        "",
        '    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;',
        "    let options = AddPlanOptions::new(kind, name);",
        "",
        "    render_add_dry_run(&context, &options).map_err(|error| error.to_string())",
        "}",
        "",
        "",
    ])
    main = main.replace(anchor, render_add + anchor, 1)

help_line = '        "  add <kind> <name> --dry-run              Preview component scaffold plan",\n'
if help_line not in main:
    core_anchor = '        "Core commands:",\n'
    if core_anchor not in main:
        raise SystemExit("Could not find Core commands help anchor")
    main = main.replace(core_anchor, core_anchor + help_line, 1)

example_line = '        "  monad add app web --dry-run",\n'
if example_line not in main and '        "Examples:",\n' in main:
    main = main.replace('        "Examples:",\n', '        "Examples:",\n' + example_line, 1)

if "fn add_app_dry_run_command_parses" not in main:
    anchor = "    #[test]\n    fn info_command_parses_text_and_json_formats() {"
    if anchor not in main:
        raise SystemExit("Could not find CLI test insertion anchor")
    tests = "\n".join([
        "    #[test]",
        "    fn add_app_dry_run_command_parses() {",
        "        assert_eq!(",
        '            parse_arguments(&["monad", "add", "app", "web", "--dry-run"])',
        '                .expect("add app dry-run should parse"),',
        "            CliCommand::Add {",
        "                kind: ComponentKind::App,",
        '                name: ComponentName::parse("web").expect("test name should parse"),',
        "                dry_run: true,",
        "            }",
        "        );",
        "    }",
        "",
        "    #[test]",
        "    fn add_requires_dry_run_for_now() {",
        '        let error = parse_arguments(&["monad", "add", "app", "web"])',
        '            .expect_err("add without dry-run should fail");',
        "",
        '        assert!(error.contains("add currently requires --dry-run"));',
        "    }",
        "",
        "    #[test]",
        "    fn add_rejects_yes_until_guarded_write_exists() {",
        '        let error = parse_arguments(&["monad", "add", "app", "web", "--yes"])',
        '            .expect_err("add --yes should fail in WP-E12-002");',
        "",
        '        assert!(error.contains("add --yes is reserved"));',
        '        assert!(error.contains("dry-run only"));',
        "    }",
        "",
        "    #[test]",
        "    fn add_rejects_unsafe_component_name() {",
        '        let error = parse_arguments(&["monad", "add", "app", "../web", "--dry-run"])',
        '            .expect_err("unsafe component name should fail");',
        "",
        '        assert!(error.contains("invalid component name"));',
        "    }",
        "",
        "",
    ])
    main = main.replace(anchor, tests + anchor, 1)

if 'assert!(text.contains("add <kind> <name> --dry-run"));' not in main:
    main = main.replace(
        '        assert!(text.contains("init --dry-run"));\n',
        '        assert!(text.contains("init --dry-run"));\n'
        '        assert!(text.contains("add <kind> <name> --dry-run"));\n',
        1,
    )

main_path.write_text(main, encoding="utf-8")
PY

cat > work/learning/E12/WP-E12-002-add-dry-run-plan.md <<'EOF'
---
title: "Learning Note — WP-E12-002 Add Dry-Run Plan"
document_type: "learning-note"
status: active
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-002
tags:
  - learning
  - rust
  - cli
  - dry-run
  - monad
---

# Learning Note — WP-E12-002 Add Dry-Run Plan

## What You Are Building

You are adding the first implementation layer for:

```bash
monad add app web --dry-run
```

This command does not create files yet.

It previews what Monad would create later.

## Mental Model

The flow is:

```text
CLI arguments
  -> parse command
  -> validate component kind
  -> validate component name
  -> build file operation plan
  -> evaluate plan against the filesystem
  -> render dry-run output
```

## Main Rust File to Read First

Read:

```text
crates/monad-core/src/component_add.rs
```

Start with these types:

```text
ComponentKind
ComponentName
AddPlanOptions
```

Then read:

```text
build_add_plan
render_add_dry_run
```

## What `ComponentKind` Teaches

`ComponentKind` is an enum.

An enum is a type that can be one of several named variants:

```text
App
Package
Service
Tool
```

The enum maps user words to known component families.

## What `ComponentName` Teaches

`ComponentName` is a wrapper around `String`.

It exists so unsafe names are rejected once at the boundary.

That means the rest of the code can trust that the name is safe.

## What `AddPlanOptions` Teaches

`AddPlanOptions` is a small data object.

It carries:

```text
kind
name
```

It also knows how to compute:

```text
apps/web
packages/shared-ui
services/api
tools/repo-lint
```

## What `build_add_plan` Teaches

`build_add_plan` does not touch the filesystem.

It only builds an in-memory plan:

```text
would-create apps/web/README.md
would-create apps/web/.gitkeep
```

This is the same pattern used for `init`.

## What `render_add_dry_run` Teaches

`render_add_dry_run` evaluates the plan against the current workspace.

That is what lets Monad say:

```text
would-create
conflict
no-op
```

without writing files.

## What to Inspect After Running the Script

Run:

```bash
git diff -- crates/monad-core/src/component_add.rs
git diff -- crates/monad-cli/src/main.rs
```

Then ask yourself:

1. Where does user input enter?
2. Where is the input validated?
3. Where is the file plan built?
4. Where is the filesystem checked?
5. Where is text rendered for the user?

## Verification Commands

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- add app web --dry-run
cargo run -p monad-cli -- add service api --dry-run
cargo run -p monad-cli -- add app ../bad --dry-run
```

The last command should fail.
EOF

cat > work/deliverables/E12/WP-E12-002-add-dry-run-plan.md <<'EOF'
---
title: "WP-E12-002 Add Command Dry-Run Plan Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-002
tags:
  - monad
  - e12
  - add
  - dry-run
  - component
related:
  - crates/monad-core/src/component_add.rs
  - crates/monad-cli/src/main.rs
  - work/learning/E12/WP-E12-002-add-dry-run-plan.md
---

# WP-E12-002 Add Command Dry-Run Plan Deliverable

## Work Packet

WP-E12-002 — Add add-command dry-run plan.

## Outcome

Implemented.

## Summary

This work packet adds the first dry-run planning implementation for:

```bash
monad add <kind> <name> --dry-run
```

Supported component kinds:

```text
app
package
service
tool
```

## Deliverables

- `crates/monad-core/src/component_add.rs`
- `crates/monad-core/src/lib.rs`
- `crates/monad-cli/src/main.rs`
- `work/learning/E12/WP-E12-002-add-dry-run-plan.md`
- `work/deliverables/E12/WP-E12-002-add-dry-run-plan.md`

## Safety Boundary

WP-E12-002 is dry-run only.

It does not:

- write files;
- create directories;
- install packages;
- run Git commands;
- call remote services;
- support `--yes`.

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- add app web --dry-run
cargo run -p monad-cli -- add package shared-ui --dry-run
cargo run -p monad-cli -- add service api --dry-run
cargo run -p monad-cli -- add tool repo-lint --dry-run
cargo run -p monad-cli -- add app ../bad --dry-run
tools/scripts/verify.sh
git status --short
```

The unsafe-name command should fail.

## Recommended Commit

```bash
git commit -m "feat(add): add component dry-run plan"
```

## Closeout Note

WP-E12-002 is complete once the add dry-run plan is committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E12-003 — Add embedded component scaffold templates
```
EOF

cargo fmt

echo
echo "WP-E12-002 files updated:"
echo "  crates/monad-core/src/component_add.rs"
echo "  crates/monad-core/src/lib.rs"
echo "  crates/monad-cli/src/main.rs"
echo "  work/learning/E12/WP-E12-002-add-dry-run-plan.md"
echo "  work/deliverables/E12/WP-E12-002-add-dry-run-plan.md"
echo
echo "Learning checkpoint:"
echo "  Read work/learning/E12/WP-E12-002-add-dry-run-plan.md before committing."
echo
echo "Next verification:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  cargo run -p monad-cli -- add app web --dry-run"
