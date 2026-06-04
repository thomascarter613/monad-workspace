#!/usr/bin/env bash
set -euo pipefail

# Fix WP-E11-004 stale CLI tests and dead helper functions.
#
# Current expected behavior after WP-E11-004:
# - `monad init` without a mode fails.
# - `monad init --dry-run` parses.
# - `monad init --yes` parses.
# - `monad init --dry-run --yes` fails because the modes conflict.

python3 - <<'PY'
from pathlib import Path

path = Path("crates/monad-cli/src/main.rs")
text = path.read_text(encoding="utf-8")

def remove_function(source: str, name: str) -> str:
    marker = f"fn {name}("
    fn_start = source.find(marker)
    if fn_start == -1:
        return source

    start = fn_start
    cursor = source.rfind("\n", 0, fn_start)
    while cursor != -1:
        line_start = source.rfind("\n", 0, cursor)
        line = source[line_start + 1:cursor]
        if line.strip().startswith("///") or line.strip() == "":
            start = line_start + 1
            cursor = line_start
        else:
            break

    brace = source.find("{", fn_start)
    if brace == -1:
        raise SystemExit(f"Could not find opening brace for function {name}")

    depth = 0
    end = None
    for i in range(brace, len(source)):
        if source[i] == "{":
            depth += 1
        elif source[i] == "}":
            depth -= 1
            if depth == 0:
                end = i + 1
                break

    if end is None:
        raise SystemExit(f"Could not find closing brace for function {name}")

    while end < len(source) and source[end] == "\n":
        end += 1

    return source[:start] + source[end:]

def remove_test_function(source: str, name: str) -> str:
    marker = f"fn {name}("
    fn_start = source.find(marker)
    if fn_start == -1:
        return source

    test_attr = source.rfind("#[test]", 0, fn_start)
    if test_attr == -1:
        return remove_function(source, name)

    start = source.rfind("\n", 0, test_attr)
    start = 0 if start == -1 else start + 1

    brace = source.find("{", fn_start)
    if brace == -1:
        raise SystemExit(f"Could not find opening brace for test {name}")

    depth = 0
    end = None
    for i in range(brace, len(source)):
        if source[i] == "{":
            depth += 1
        elif source[i] == "}":
            depth -= 1
            if depth == 0:
                end = i + 1
                break

    if end is None:
        raise SystemExit(f"Could not find closing brace for test {name}")

    while end < len(source) and source[end] == "\n":
        end += 1

    return source[:start] + source[end:]

for fn_name in ["require_dry_run_for_init", "reject_yes_for_init"]:
    text = remove_function(text, fn_name)

for test_name in [
    "init_requires_dry_run_for_now",
    "init_rejects_yes_until_guarded_write_exists",
]:
    text = remove_test_function(text, test_name)

new_tests = """
    #[test]
    fn init_requires_dry_run_or_yes() {
        let error =
            parse_arguments(&[\"monad\", \"init\"]).expect_err(\"init without mode should fail\");

        assert!(error.contains(\"init currently requires either --dry-run\"));
        assert!(error.contains(\"--yes\"));
    }

    #[test]
    fn init_yes_command_parses() {
        assert_eq!(
            parse_arguments(&[\"monad\", \"init\", \"--yes\"]).expect(\"init --yes should parse\"),
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
        let error = parse_arguments(&[\"monad\", \"init\", \"--dry-run\", \"--yes\"])
            .expect_err(\"init should reject conflicting modes\");

        assert!(error.contains(\"either --dry-run or --yes\"));
        assert!(error.contains(\"not both\"));
    }

"""

if "fn init_requires_dry_run_or_yes" not in text:
    anchor = "    #[test]\n    fn info_command_parses_text_and_json_formats() {"
    if anchor not in text:
        raise SystemExit("Could not find test insertion anchor before info_command_parses_text_and_json_formats")
    text = text.replace(anchor, new_tests + anchor, 1)

text = text.replace(
    "CliCommand::Init {\n                dry_run: true,\n                preset:",
    "CliCommand::Init {\n                dry_run: true,\n                yes: false,\n                preset:",
)
text = text.replace(
    "CliCommand::Init {\n                dry_run: false,\n                preset:",
    "CliCommand::Init {\n                dry_run: false,\n                yes: true,\n                preset:",
)
text = text.replace("yes: false,\n                yes: false,", "yes: false,")
text = text.replace("yes: true,\n                yes: true,", "yes: true,")

path.write_text(text, encoding="utf-8")
PY

cargo fmt

echo
echo "Applied WP-E11-004 stale-test cleanup."
echo
echo "Run:"
echo "  cargo test -p monad-cli --bin monad"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
