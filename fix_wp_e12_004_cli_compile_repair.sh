#!/usr/bin/env bash
set -euo pipefail

# Fix WP-E12-004 CLI compile issues.
#
# Fixes:
# 1. Replace the bad `arguments.get(1)` parser guard with the actual command-vector variable.
# 2. Update the stale CliCommand::Add run arm to include the new `yes` field.
# 3. Remove accidental `fs = ...` dependency from crates/monad-core/Cargo.toml if present,
#    because this packet uses `std::fs`, not the external `fs` crate.
# 4. Run cargo generate-lockfile and cargo fmt.

python3 - <<'PY'
from pathlib import Path
import re

main_path = Path("crates/monad-cli/src/main.rs")
main = main_path.read_text(encoding="utf-8")

bad_guard_pattern = re.compile(
    r'if yes\s*\n'
    r'\s*&& arguments\.get\(1\)\.copied\(\) != Some\("init"\)\s*\n'
    r'\s*&& arguments\.get\(1\)\.copied\(\) != Some\("add"\)\s*\n'
    r'\s*\{\s*\n'
    r'\s*return Err\("--yes is only supported for init and add commands"\.to_string\(\)\);\s*\n'
    r'\s*\}',
    re.S,
)

bad_match = bad_guard_pattern.search(main)
if bad_match:
    following = main[bad_match.end(): bad_match.end() + 3000]
    command_var_match = re.search(r"match\s+([A-Za-z_][A-Za-z0-9_]*)\.as_slice\(\)", following)
    command_var = command_var_match.group(1) if command_var_match else "command"
    replacement = "\n".join([
        "if yes",
        f"        && {command_var}.first().copied() != Some(\"init\")",
        f"        && {command_var}.first().copied() != Some(\"add\")",
        "    {",
        "        return Err(\"--yes is only supported for init and add commands\".to_string());",
        "    }",
    ])
    main = bad_guard_pattern.sub(replacement, main, count=1)
elif "arguments.get(1).copied()" in main:
    following = main[main.find("arguments.get(1).copied()"): main.find("arguments.get(1).copied()") + 3000]
    command_var_match = re.search(r"match\s+([A-Za-z_][A-Za-z0-9_]*)\.as_slice\(\)", following)
    command_var = command_var_match.group(1) if command_var_match else "command"
    main = main.replace("arguments.get(1).copied()", f"{command_var}.first().copied()")

# Fix stale CliCommand::Add match arm.
main = re.sub(
    r"CliCommand::Add\s*\{\s*kind,\s*name,\s*dry_run,\s*\}\s*=>\s*render_add\(dry_run,\s*kind,\s*name\)",
    "CliCommand::Add { kind, name, dry_run, yes } => render_add(dry_run, yes, kind, name)",
    main,
    count=1,
)

main = re.sub(
    r"CliCommand::Add\s*\{\s*kind,\s*name,\s*dry_run,\s*yes\s*\}\s*=>\s*render_add\(dry_run,\s*kind,\s*name\)",
    "CliCommand::Add { kind, name, dry_run, yes } => render_add(dry_run, yes, kind, name)",
    main,
    count=1,
)

main_path.write_text(main, encoding="utf-8")

# Remove accidental external fs crate dependency.
toml_path = Path("crates/monad-core/Cargo.toml")
if toml_path.exists():
    toml = toml_path.read_text(encoding="utf-8")
    cleaned = re.sub(r"(?m)^fs\s*=\s*['\"][^'\"]+['\"]\s*\n", "", toml)
    if cleaned != toml:
        toml_path.write_text(cleaned, encoding="utf-8")

learning_path = Path("work/learning/E12/WP-E12-004-guarded-add-write-path.md")
if learning_path.exists():
    learning = learning_path.read_text(encoding="utf-8")
    marker = "## Fix Note — Parser Guard and Match Arm Drift"
    if marker not in learning:
        learning += "\n\n" + "\n".join([
            marker,
            "",
            "The guarded-write patch had two integration mistakes:",
            "",
            "1. It guessed a parser variable named `arguments`, but that variable was not in scope.",
            "2. It added `yes` to `CliCommand::Add`, but one match arm still destructured the old shape.",
            "",
            "The compiler errors were useful:",
            "",
            "```text",
            "cannot find value `arguments` in this scope",
            "pattern does not mention field `yes`",
            "this function takes 4 arguments but 3 arguments were supplied",
            "```",
            "",
            "The fix is to align all layers:",
            "",
            "```text",
            "parser guard",
            "CliCommand::Add variant",
            "run match arm",
            "render_add function signature",
            "```",
            "",
            "Also note: `std::fs` does not require adding an external `fs` crate to Cargo.toml.",
            "",
        ])
        learning_path.write_text(learning, encoding="utf-8")
PY

cargo generate-lockfile
cargo fmt

echo
echo "Applied WP-E12-004 compile repair."
echo
echo "Run:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  cargo run -p monad-cli -- add app web --dry-run"
