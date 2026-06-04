#!/usr/bin/env bash
set -euo pipefail

# Fix WP-E11-004 compile/clippy failures:
# - add missing require_init_mode(...)
# - replace old render_init(...) with guarded render_init(..., yes, ...)
# - collapse nested parent-directory if in monad-core init write path
# - normalize Init expected structs in CLI tests if needed

python3 - <<'PY'
from pathlib import Path
import re

main_path = Path("crates/monad-cli/src/main.rs")
init_path = Path("crates/monad-core/src/init.rs")

main = main_path.read_text(encoding="utf-8")

def replace_rust_function(source: str, name: str, replacement: str) -> str:
    marker = f"fn {name}("
    start = source.find(marker)
    if start == -1:
        raise SystemExit(f"Could not find function `{name}`")

    # Include preceding doc comments directly above the function when present.
    doc_start = source.rfind("\n///", 0, start)
    if doc_start != -1:
        between = source[doc_start:start]
        lines = [line for line in between.splitlines() if line.strip()]
        if all(line.strip().startswith("///") for line in lines):
            start = doc_start + 1

    brace = source.find("{", source.find(marker))
    if brace == -1:
        raise SystemExit(f"Could not find opening brace for `{name}`")

    depth = 0
    end = None
    for index in range(brace, len(source)):
        char = source[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                end = index + 1
                break

    if end is None:
        raise SystemExit(f"Could not find closing brace for `{name}`")

    return source[:start] + replacement.rstrip() + "\n" + source[end:]

# Ensure the mode helper exists.
if "fn require_init_mode(" not in main:
    helper = '''
/// Requires exactly one init mode for the guarded init implementation.
fn require_init_mode(dry_run: bool, yes: bool) -> Result<(), String> {
    match (dry_run, yes) {
        (true, false) | (false, true) => Ok(()),
        (false, false) => {
            Err("init currently requires either --dry-run to preview or --yes to apply".to_string())
        }
        (true, true) => Err("init accepts either --dry-run or --yes, not both".to_string()),
    }
}

'''
    anchor = "/// Rejects output-format flags for the first init implementation."
    if anchor not in main:
        anchor = "/// Parses an init preset or returns the default minimal preset."
    if anchor not in main:
        raise SystemExit("Could not find insertion anchor for require_init_mode")
    main = main.replace(anchor, helper + anchor, 1)

# Remove the obsolete reject_yes_for_init helper if it still exists.
main = re.sub(
    r'''/// Rejects init write approval until the guarded write path exists\.
fn reject_yes_for_init\(yes: bool\) -> Result<\(\), String> \{
    if yes \{
        Err\("init --yes is reserved for the guarded write path; WP-E11-002 is dry-run only"\.to_string\(\)\)
    \} else \{
        Ok\(\(\)\)
    \}
\}

''',
    "\n",
    main,
)

# Replace render_init with the guarded version accepting yes.
guarded_render_init = '''
/// Renders or applies repository initialization.
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
'''
main = replace_rust_function(main, "render_init", guarded_render_init)

# Make sure init expected structs include yes field in tests if needed.
main = main.replace(
    "CliCommand::Init {\n                dry_run: true,\n                preset:",
    "CliCommand::Init {\n                dry_run: true,\n                yes: false,\n                preset:",
)
main = main.replace(
    "CliCommand::Init {\n                dry_run: false,\n                preset:",
    "CliCommand::Init {\n                dry_run: false,\n                yes: true,\n                preset:",
)
main = main.replace("yes: false,\n                yes: false,", "yes: false,")
main = main.replace("yes: true,\n                yes: true,", "yes: true,")

# Ensure help text mentions init --yes if present command surface tests expect it.
help_yes_line = '        "  init --yes                                Apply repository initialization after review",\n'
if help_yes_line not in main and '        "  init --dry-run                            Preview repository initialization plan",\n' in main:
    main = main.replace(
        '        "  init --dry-run                            Preview repository initialization plan",\n',
        '        "  init --dry-run                            Preview repository initialization plan",\n' + help_yes_line,
        1,
    )

main_path.write_text(main, encoding="utf-8")

init_text = init_path.read_text(encoding="utf-8")

# Collapse the nested parent-directory if that Clippy reported.
nested = '''        if let Some(parent) = target_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|error| {
                    MonadError::internal(format!(
                        "failed to create parent directory `{}`: {error}",
                        parent.display()
                    ))
                })?;
            }
        }
'''
collapsed = '''        if let Some(parent) = target_path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).map_err(|error| {
                MonadError::internal(format!(
                    "failed to create parent directory `{}`: {error}",
                    parent.display()
                ))
            })?;
        }
'''
if nested in init_text:
    init_text = init_text.replace(nested, collapsed, 1)
else:
    init_text = re.sub(
        r'''        if let Some\(parent\) = target_path\.parent\(\) \{\n            if !parent\.as_os_str\(\)\.is_empty\(\) \{\n(?P<body>                fs::create_dir_all\(parent\)\.map_err\(\|error\| \{\n                    MonadError::internal\(format!\(\n                        "failed to create parent directory `\{\}`: \{error\}",\n                        parent\.display\(\)\n                    \)\)\n                \}\)\?;\n)            \}\n        \}\n''',
        r'''        if let Some(parent) = target_path.parent()
            && !parent.as_os_str().is_empty()
        {
\g<body>        }
''',
        init_text,
    )

init_path.write_text(init_text, encoding="utf-8")
PY

cargo fmt

echo
echo "Applied WP-E11-004 compile/clippy fix."
echo
echo "Run:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  cargo run -p monad-cli -- init --dry-run"
