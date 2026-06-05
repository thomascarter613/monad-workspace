#!/usr/bin/env bash
set -euo pipefail

# Fix WP-E12-002 brittle add --yes test.
#
# This patch updates the test to protect the safety boundary instead of one exact
# error sentence.

python3 - <<'PY'
from pathlib import Path

path = Path("crates/monad-cli/src/main.rs")
text = path.read_text(encoding="utf-8")

def replace_test_function(source: str, name: str, replacement: str) -> str:
    marker = f"fn {name}("
    fn_start = source.find(marker)
    if fn_start == -1:
        raise SystemExit(f"Could not find test function `{name}`")

    test_attr = source.rfind("#[test]", 0, fn_start)
    if test_attr == -1:
        raise SystemExit(f"Could not find #[test] attribute for `{name}`")

    start = source.rfind("\n", 0, test_attr)
    start = 0 if start == -1 else start + 1

    brace = source.find("{", fn_start)
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

    while end < len(source) and source[end] == "\n":
        end += 1

    return source[:start] + replacement.rstrip() + "\n\n" + source[end:]

replacement = "\n".join([
    "    #[test]",
    "    fn add_rejects_yes_until_guarded_write_exists() {",
    '        let error = parse_arguments(&["monad", "add", "app", "web", "--yes"])',
    '            .expect_err("add --yes should fail in WP-E12-002");',
    "",
    "        assert!(",
    '            error.contains("add"),',
    '            "expected add-related error, got: {error}"',
    "        );",
    "        assert!(",
    '            error.contains("--yes")',
    '                || error.contains("--dry-run")',
    '                || error.contains("write behavior"),',
    '            "expected add write-safety error, got: {error}"',
    "        );",
    "    }",
])

text = replace_test_function(text, "add_rejects_yes_until_guarded_write_exists", replacement)
path.write_text(text, encoding="utf-8")

learning_path = Path("work/learning/E12/WP-E12-002-add-dry-run-plan.md")
if learning_path.exists():
    learning = learning_path.read_text(encoding="utf-8")
    marker = "## Fix Note — Testing Safety Properties, Not Brittle Strings"
    if marker not in learning:
        lines = [
            "",
            "",
            marker,
            "",
            "The first test for `monad add app web --yes` expected one exact error phrase.",
            "",
            "That was too brittle.",
            "",
            "The important behavior is not the exact sentence.",
            "",
            "The important safety property is that `monad add` remains dry-run only in WP-E12-002.",
            "",
            "The improved test checks that the command fails and that the error mentions the add/write-safety boundary.",
            "",
            "This makes the test less fragile while still protecting the safety rule.",
            "",
        ]
        learning_path.write_text(learning + "\n".join(lines), encoding="utf-8")
PY

cargo fmt

echo
echo "Applied WP-E12-002 test expectation fix."
echo
echo "Run:"
echo "  cargo test -p monad-cli --bin monad"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
