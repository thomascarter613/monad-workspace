#!/usr/bin/env bash
set -euo pipefail

# Complete WP-E12-004 — Add guarded add write path.
#
# Guided/learning-first packet.
#
# Safety:
# - `monad add ... --dry-run` still previews.
# - `monad add ... --yes` writes only after the same plan is checked.
# - existing files block the write path.
# - no Git commands, package installs, or remote calls are made.

echo "==> WP-E12-004"
echo "Goal: add guarded writes for monad add <kind> <name> --yes."
echo "Mental model: build plan -> evaluate conflicts -> write embedded template files."
echo

mkdir -p work/learning/E12 work/deliverables/E12

python3 - <<'PY'
from pathlib import Path
import re

component_path = Path("crates/monad-core/src/component_add.rs")
lib_path = Path("crates/monad-core/src/lib.rs")
main_path = Path("crates/monad-cli/src/main.rs")

component = component_path.read_text(encoding="utf-8")

if "use std::fs;" not in component:
    component = component.replace(
        "use std::path::{Path, PathBuf};",
        "use std::fs;\nuse std::path::{Path, PathBuf};",
        1,
    )

component = component.replace("  apply: not implemented in WP-E12-003", "  apply: guarded by --yes")
component = component.replace(
    "  approval_flag: --yes reserved for a later E12 work packet",
    "  approval_flag: --yes",
)

core_add_write = r'''
/// Result of guarded component add application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddApplyResult {
    kind: ComponentKind,
    name: ComponentName,
    component_root: PathBuf,
    created_files: Vec<PathBuf>,
    total_bytes_written: usize,
}

impl AddApplyResult {
    /// Creates an add apply result.
    #[must_use]
    pub fn new(
        kind: ComponentKind,
        name: ComponentName,
        component_root: PathBuf,
        created_files: Vec<PathBuf>,
        total_bytes_written: usize,
    ) -> Self {
        Self {
            kind,
            name,
            component_root,
            created_files,
            total_bytes_written,
        }
    }

    /// Returns component kind.
    #[must_use]
    pub const fn kind(&self) -> ComponentKind {
        self.kind
    }

    /// Returns component name.
    #[must_use]
    pub const fn name(&self) -> &ComponentName {
        &self.name
    }

    /// Returns component root.
    #[must_use]
    pub fn component_root(&self) -> &Path {
        &self.component_root
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

/// Applies the component scaffold after a conflict-free dry-run evaluation.
///
/// This is the guarded write path for `monad add <kind> <name> --yes`.
/// It refuses to overwrite existing files and performs no Git operations.
pub fn apply_add_plan(
    context: &WorkspaceContext,
    options: &AddPlanOptions,
) -> MonadResult<AddApplyResult> {
    let plan = build_add_plan(options);
    let dry_run = evaluate_file_operation_plan(context.root(), &plan);

    if dry_run.has_conflicts() {
        return Err(MonadError::invalid_input(
            "add plan has conflicts; run `monad add <kind> <name> --dry-run` and resolve existing target paths before using --yes",
        ));
    }

    let mut created_files = Vec::new();
    let mut total_bytes_written = 0usize;

    for template in component_scaffold_templates() {
        let relative_path = options.component_root().join(template.relative_path());
        let target_path = context.root().join(&relative_path);

        if target_path.exists() {
            return Err(MonadError::invalid_input(format!(
                "add target `{}` already exists; refusing to overwrite",
                relative_path.display()
            )));
        }

        if let Some(parent) = target_path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).map_err(|error| {
                MonadError::internal(format!(
                    "failed to create parent directory `{}`: {error}",
                    parent.display()
                ))
            })?;
        }

        let content = template.render_content(options);
        fs::write(&target_path, content.as_bytes()).map_err(|error| {
            MonadError::internal(format!(
                "failed to write add target `{}`: {error}",
                relative_path.display()
            ))
        })?;

        total_bytes_written += content.len();
        created_files.push(relative_path);
    }

    Ok(AddApplyResult::new(
        options.kind(),
        options.name().clone(),
        options.component_root(),
        created_files,
        total_bytes_written,
    ))
}

/// Renders the guarded component add apply result.
#[must_use]
pub fn render_add_apply_result(result: &AddApplyResult) -> String {
    let mut lines = vec![
        "Monad add applied".to_string(),
        format!("  kind: {}", result.kind().as_str()),
        format!("  name: {}", result.name().as_str()),
        format!("  root: {}", result.component_root().display()),
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
'''

if "pub struct AddApplyResult" not in component:
    marker = "#[cfg(test)]"
    idx = component.find(marker)
    if idx == -1:
        raise SystemExit("Could not find test module marker in component_add.rs")
    component = component[:idx].rstrip() + "\n\n" + core_add_write.strip() + "\n\n" + component[idx:]

if "fn apply_add_plan_writes_component_scaffold" not in component:
    insert = r'''
    #[test]
    fn apply_add_plan_writes_component_scaffold() -> MonadResult<()> {
        let root = unique_temp_root("apply");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let options = AddPlanOptions::new(ComponentKind::App, ComponentName::parse("web")?);
        let result = apply_add_plan(&context, &options)?;

        assert_eq!(result.kind(), ComponentKind::App);
        assert_eq!(result.name().as_str(), "web");
        assert_eq!(result.component_root(), Path::new("apps/web"));
        assert_eq!(result.file_count(), 2);
        assert!(root.join("apps/web/README.md").is_file());
        assert!(root.join("apps/web/.gitkeep").is_file());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn apply_add_plan_refuses_existing_component_file() -> MonadResult<()> {
        let root = unique_temp_root("apply-conflict");
        fs::create_dir_all(root.join("apps/web")).map_err(|error| {
            MonadError::internal(format!("test component root should be created: {error}"))
        })?;
        fs::write(root.join("apps/web/README.md"), "# Existing\n").map_err(|error| {
            MonadError::internal(format!("test README should be written: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let options = AddPlanOptions::new(ComponentKind::App, ComponentName::parse("web")?);
        let error = apply_add_plan(&context, &options)
            .expect_err("existing component file should block add apply");

        assert!(error.to_string().contains("add plan has conflicts"));
        assert!(!root.join("apps/web/.gitkeep").exists());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn render_add_apply_result_lists_created_files() -> MonadResult<()> {
        let result = AddApplyResult::new(
            ComponentKind::Tool,
            ComponentName::parse("repo-lint")?,
            PathBuf::from("tools/repo-lint"),
            vec![
                PathBuf::from("tools/repo-lint/README.md"),
                PathBuf::from("tools/repo-lint/.gitkeep"),
            ],
            99,
        );

        let rendered = render_add_apply_result(&result);

        assert!(rendered.contains("Monad add applied"));
        assert!(rendered.contains("kind: tool"));
        assert!(rendered.contains("name: repo-lint"));
        assert!(rendered.contains("files_created: 2"));
        assert!(rendered.contains("No Git commands were run."));

        Ok(())
    }
'''
    last = component.rfind("\n}")
    if last == -1:
        raise SystemExit("Could not find end of component_add.rs")
    component = component[:last] + "\n" + insert.rstrip() + component[last:]

component_path.write_text(component, encoding="utf-8")

lib = lib_path.read_text(encoding="utf-8")
lib = re.sub(r"\n*pub use component_add::\{.*?\};\n*", "\n", lib, flags=re.S)

if "pub mod component_add;" not in lib:
    anchor = "pub mod checks;\n"
    if anchor not in lib:
        raise SystemExit("Could not find checks module anchor in lib.rs")
    lib = lib.replace(anchor, anchor + "pub mod component_add;\n", 1)

export_block = "\n".join([
    "pub use component_add::{",
    "    AddApplyResult, AddPlanOptions, ComponentKind, ComponentName, apply_add_plan, build_add_plan,",
    "    component_scaffold_templates, render_add_apply_result, render_add_dry_run,",
    "};",
    "",
])

idx = lib.find("#[cfg(test)]")
if idx == -1:
    lib = lib.rstrip() + "\n\n" + export_block
else:
    lib = lib[:idx].rstrip() + "\n\n" + export_block + lib[idx:].lstrip()

lib_path.write_text(lib, encoding="utf-8")

main = main_path.read_text(encoding="utf-8")

match = re.search(r"use monad_core::\{(?P<body>.*?)\};", main, flags=re.S)
if not match:
    raise SystemExit("Could not find use monad_core import block")
body = match.group("body")
for symbol in ["apply_add_plan", "render_add_apply_result"]:
    if symbol not in body:
        body = " " + symbol + "," + body
main = main[:match.start()] + "use monad_core::{" + body + "};" + main[match.end():]

signature = re.search(r"fn parse_arguments\((?P<name>\w+):\s*&\[[^\]]+\]\)", main)
arg_name = signature.group("name") if signature else "args"
error_text = '"--yes is only supported for init command"'
if error_text in main:
    err_idx = main.find(error_text)
    if_start = main.rfind("if yes", 0, err_idx)
    if if_start != -1:
        brace = main.find("{", if_start)
        depth = 0
        end = None
        for i in range(brace, len(main)):
            if main[i] == "{":
                depth += 1
            elif main[i] == "}":
                depth -= 1
                if depth == 0:
                    end = i + 1
                    break
        if end is not None:
            replacement = "\n".join([
                "if yes",
                f"        && {arg_name}.get(1).copied() != Some(\"init\")",
                f"        && {arg_name}.get(1).copied() != Some(\"add\")",
                "    {",
                "        return Err(\"--yes is only supported for init and add commands\".to_string());",
                "    }",
            ])
            main = main[:if_start] + replacement + main[end:]

add_variant_start = main.find("    Add {\n")
if add_variant_start != -1:
    info_start = main.find("    Info {", add_variant_start)
    if info_start != -1:
        add_block = main[add_variant_start:info_start]
        if "yes: bool" not in add_block:
            add_block = add_block.replace(
                "        /// Whether to run in dry-run mode.\n        dry_run: bool,\n",
                "        /// Whether to run in dry-run mode.\n        dry_run: bool,\n\n        /// Whether to apply after explicit approval.\n        yes: bool,\n",
            )
            main = main[:add_variant_start] + add_block + main[info_start:]

marker = '["add", kind, name] => {'
arm_idx = main.find(marker)
if arm_idx == -1:
    raise SystemExit("Could not find add parse arm")
start = main.rfind("\n", 0, arm_idx) + 1
brace = main.find("{", arm_idx)
depth = 0
end = None
for i in range(brace, len(main)):
    if main[i] == "{":
        depth += 1
    elif main[i] == "}":
        depth -= 1
        if depth == 0:
            end = i + 1
            break
while end is not None and end < len(main) and main[end] == "\n":
    end += 1

new_arm = "\n".join([
    '            ["add", kind, name] => {',
    "                reject_write_for_non_context(write)?;",
    "                require_add_mode(dry_run, yes)?;",
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
    "                    yes,",
    "                })",
    "            }",
    "",
])
main = main[:start] + new_arm + main[end:]

if "fn require_add_mode(" not in main:
    helper = "\n".join([
        "/// Requires exactly one add mode for the guarded add implementation.",
        "fn require_add_mode(dry_run: bool, yes: bool) -> Result<(), String> {",
        "    match (dry_run, yes) {",
        "        (true, false) | (false, true) => Ok(()),",
        "        (false, false) => {",
        '            Err("add currently requires either --dry-run to preview or --yes to apply".to_string())',
        "        }",
        '        (true, true) => Err("add accepts either --dry-run or --yes, not both".to_string()),',
        "    }",
        "}",
        "",
        "",
    ])
    anchor = "/// Requires exactly one init mode"
    if anchor not in main:
        anchor = "/// Rejects output-format flags"
    if anchor not in main:
        raise SystemExit("Could not find helper insertion anchor")
    main = main.replace(anchor, helper + anchor, 1)

main = main.replace(
    "CliCommand::Add { kind, name, dry_run } => render_add(dry_run, kind, name),",
    "CliCommand::Add { kind, name, dry_run, yes } => render_add(dry_run, yes, kind, name),",
)

fn_idx = main.find("fn render_add(")
if fn_idx == -1:
    raise SystemExit("Could not find render_add function")
doc_start = main.rfind("\n///", 0, fn_idx)
start = doc_start + 1 if doc_start != -1 else fn_idx
brace = main.find("{", fn_idx)
depth = 0
end = None
for i in range(brace, len(main)):
    if main[i] == "{":
        depth += 1
    elif main[i] == "}":
        depth -= 1
        if depth == 0:
            end = i + 1
            break
while end is not None and end < len(main) and main[end] == "\n":
    end += 1

render_add = "\n".join([
    "/// Renders or applies component add output.",
    "fn render_add(",
    "    dry_run: bool,",
    "    yes: bool,",
    "    kind: ComponentKind,",
    "    name: ComponentName,",
    ") -> Result<String, String> {",
    '    let context = WorkspaceContext::discover_from(".").map_err(|error| error.to_string())?;',
    "    let options = AddPlanOptions::new(kind, name);",
    "",
    "    if dry_run {",
    "        return render_add_dry_run(&context, &options).map_err(|error| error.to_string());",
    "    }",
    "",
    "    if yes {",
    "        let result = apply_add_plan(&context, &options).map_err(|error| error.to_string())?;",
    "        return Ok(render_add_apply_result(&result));",
    "    }",
    "",
    '    Err("add currently requires either --dry-run to preview or --yes to apply".to_string())',
    "}",
    "",
    "",
])
main = main[:start] + render_add + main[end:]

yes_line = '        "  add <kind> <name> --yes                  Apply component scaffold after review",\n'
if yes_line not in main:
    dry_line = '        "  add <kind> <name> --dry-run              Preview component scaffold plan",\n'
    if dry_line in main:
        main = main.replace(dry_line, dry_line + yes_line, 1)

def remove_test(source: str, name: str) -> str:
    idx = source.find(f"fn {name}(")
    if idx == -1:
        return source
    attr = source.rfind("#[test]", 0, idx)
    start = source.rfind("\n", 0, attr) + 1
    brace = source.find("{", idx)
    depth = 0
    end = None
    for j in range(brace, len(source)):
        if source[j] == "{":
            depth += 1
        elif source[j] == "}":
            depth -= 1
            if depth == 0:
                end = j + 1
                break
    while end is not None and end < len(source) and source[end] == "\n":
        end += 1
    return source[:start] + source[end:]

for test_name in [
    "add_requires_dry_run_for_now",
    "add_rejects_yes_until_guarded_write_exists",
]:
    main = remove_test(main, test_name)

if "fn add_yes_command_parses" not in main:
    anchor = "    #[test]\n    fn info_command_parses_text_and_json_formats() {"
    if anchor not in main:
        raise SystemExit("Could not find CLI test anchor")
    tests = "\n".join([
        "    #[test]",
        "    fn add_requires_dry_run_or_yes() {",
        '        let error = parse_arguments(&["monad", "add", "app", "web"])',
        '            .expect_err("add without mode should fail");',
        "",
        '        assert!(error.contains("add currently requires either --dry-run"));',
        '        assert!(error.contains("--yes"));',
        "    }",
        "",
        "    #[test]",
        "    fn add_yes_command_parses() {",
        "        assert_eq!(",
        '            parse_arguments(&["monad", "add", "app", "web", "--yes"])',
        '                .expect("add --yes should parse"),',
        "            CliCommand::Add {",
        "                kind: ComponentKind::App,",
        '                name: ComponentName::parse("web").expect("test name should parse"),',
        "                dry_run: false,",
        "                yes: true,",
        "            }",
        "        );",
        "    }",
        "",
        "    #[test]",
        "    fn add_rejects_dry_run_and_yes_together() {",
        '        let error = parse_arguments(&["monad", "add", "app", "web", "--dry-run", "--yes"])',
        '            .expect_err("add should reject conflicting modes");',
        "",
        '        assert!(error.contains("either --dry-run or --yes"));',
        '        assert!(error.contains("not both"));',
        "    }",
        "",
        "",
    ])
    main = main.replace(anchor, tests + anchor, 1)

main = main.replace(
    "CliCommand::Add {\n                kind: ComponentKind::App,\n                name: ComponentName::parse(\"web\").expect(\"test name should parse\"),\n                dry_run: true,\n            }",
    "CliCommand::Add {\n                kind: ComponentKind::App,\n                name: ComponentName::parse(\"web\").expect(\"test name should parse\"),\n                dry_run: true,\n                yes: false,\n            }",
)
main = main.replace("yes: false,\n                yes: false,", "yes: false,")
main = main.replace("yes: true,\n                yes: true,", "yes: true,")

main_path.write_text(main, encoding="utf-8")
PY

cat > work/learning/E12/WP-E12-004-guarded-add-write-path.md <<'EOF'
---
title: "Learning Note — WP-E12-004 Guarded Add Write Path"
document_type: "learning-note"
status: active
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-004
tags:
  - learning
  - rust
  - write-path
  - safety
  - monad
---

# Learning Note — WP-E12-004 Guarded Add Write Path

## What You Are Building

You are adding:

```bash
monad add app web --yes
```

This is the first write path for `monad add`.

## Important Mental Model

The write path does not invent a second workflow.

It reuses the dry-run plan:

```text
build plan
  -> evaluate plan
  -> if conflicts exist, stop
  -> if no conflicts exist, write files
```

That keeps the write path consistent with the preview path.

## Main Rust Functions

Read:

```text
apply_add_plan
render_add_apply_result
render_add_dry_run
```

in:

```text
crates/monad-core/src/component_add.rs
```

## Why This Is Safer

The write path checks the exact same file-operation plan that dry-run shows.

That means the user can preview before writing.

The same target paths are used for both modes.

## What `apply_add_plan` Does

It:

1. Builds the add plan.
2. Evaluates it against the filesystem.
3. Refuses to continue if conflicts exist.
4. Creates parent directories for approved targets.
5. Writes embedded template contents.
6. Returns a result listing created files.

## What It Does Not Do

It does not:

- run Git;
- commit;
- push;
- install packages;
- modify package manager lockfiles;
- call remote services.

## CLI Change

The CLI now supports two modes:

```bash
monad add app web --dry-run
monad add app web --yes
```

It rejects:

```bash
monad add app web
monad add app web --dry-run --yes
```

## What to Inspect

```bash
git diff -- crates/monad-core/src/component_add.rs
git diff -- crates/monad-cli/src/main.rs
```

Look for:

```text
AddApplyResult
apply_add_plan
render_add_apply_result
require_add_mode
```

## Verification

Test `--yes` in a temporary directory:

```bash
tmpdir="$(mktemp -d)"
(
  cd "$tmpdir"
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- add app web --yes
  test -f apps/web/README.md
  test -f apps/web/.gitkeep
)
rm -rf "$tmpdir"
```
EOF

cat > work/deliverables/E12/WP-E12-004-guarded-add-write-path.md <<'EOF'
---
title: "WP-E12-004 Guarded Add Write Path Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-004
tags:
  - monad
  - e12
  - add
  - guarded-write
  - safety
related:
  - crates/monad-core/src/component_add.rs
  - crates/monad-cli/src/main.rs
  - work/learning/E12/WP-E12-004-guarded-add-write-path.md
---

# WP-E12-004 Guarded Add Write Path Deliverable

## Work Packet

WP-E12-004 — Add guarded add write path.

## Outcome

Implemented.

## Summary

This work packet adds the guarded write path for:

```bash
monad add <kind> <name> --yes
```

The write path reuses the dry-run plan, refuses conflicts, and writes only embedded component scaffold files.

## Deliverables

- `crates/monad-core/src/component_add.rs`
- `crates/monad-core/src/lib.rs`
- `crates/monad-cli/src/main.rs`
- `work/learning/E12/WP-E12-004-guarded-add-write-path.md`
- `work/deliverables/E12/WP-E12-004-guarded-add-write-path.md`

## Safety Boundary

The guarded write path:

- requires `--yes`;
- refuses conflicts;
- refuses overwrites;
- writes only approved component scaffold files;
- runs no Git commands;
- installs no packages;
- calls no remote services.

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- add app web --dry-run

tmpdir="$(mktemp -d)"
(
  cd "$tmpdir"
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- add app web --yes
  test -f apps/web/README.md
  test -f apps/web/.gitkeep
)
rm -rf "$tmpdir"

tools/scripts/verify.sh
git status --short
```

Adjust `/data/monad-workspace/Cargo.toml` if your repo path differs.

## Recommended Commit

```bash
git commit -m "feat(add): add guarded component write path"
```

## Closeout Note

WP-E12-004 is complete once the guarded add write path is committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E12-005 — Add add-command smoke verification
```
EOF

cargo fmt

echo
echo "WP-E12-004 files updated:"
echo "  crates/monad-core/src/component_add.rs"
echo "  crates/monad-core/src/lib.rs"
echo "  crates/monad-cli/src/main.rs"
echo "  work/learning/E12/WP-E12-004-guarded-add-write-path.md"
echo "  work/deliverables/E12/WP-E12-004-guarded-add-write-path.md"
echo
echo "Learning checkpoint:"
echo "  Read work/learning/E12/WP-E12-004-guarded-add-write-path.md before committing."
echo
echo "Next verification:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  cargo run -p monad-cli -- add app web --dry-run"
