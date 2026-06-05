#!/usr/bin/env bash
set -euo pipefail

# Fix WP-E12-002 add --yes test for actual parser validation order.
#
# Current parser behavior:
#   monad add app web --yes
# fails early with:
#   --yes is only supported for init command
#
# That is acceptable for WP-E12-002 because the safety boundary is:
#   monad add has no write path yet.
#
# This patch changes the test to assert that --yes is rejected, without requiring
# the error to mention the add command specifically.

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
    '            error.contains("--yes"),',
    '            "expected --yes-related safety error, got: {error}"',
    "        );",
    "        assert!(",
    '            error.contains("only supported")',
    '                || error.contains("reserved")',
    '                || error.contains("write behavior")',
    '                || error.contains("dry-run"),',
    '            "expected add write-safety boundary error, got: {error}"',
    "        );",
    "    }",
])

text = replace_test_function(text, "add_rejects_yes_until_guarded_write_exists", replacement)
path.write_text(text, encoding="utf-8")

learning_path = Path("work/learning/E12/WP-E12-002-add-dry-run-plan.md")
if learning_path.exists():
    learning = learning_path.read_text(encoding="utf-8")
    marker = "## Fix Note — Parser Validation Order"
    if marker not in learning:
        learning += "\n\n" + "\n".join([
            marker,
            "",
            "The `monad add app web --yes` parser test revealed a useful parser-design detail.",
            "",
            "The command fails before the parser reaches the `add` command arm because `--yes` is currently globally allowed only for `init`.",
            "",
            "That means the actual error is:",
            "",
            "```text",
            "--yes is only supported for init command",
            "```",
            "",
            "For WP-E12-002, this is still correct because `monad add` has no write path yet.",
            "",
            "The test should therefore verify the safety boundary:",
            "",
            "```text",
            "--yes is rejected for monad add",
            "```",
            "",
            "It should not require the parser to reach the `add` command-specific error path.",
            "",
        ])
        learning_path.write_text(learning, encoding="utf-8")
PY

cargo fmt

echo
echo "Applied WP-E12-002 add --yes validation-order test fix."
echo
echo "Run:"
echo "  cargo test -p monad-cli --bin monad"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
