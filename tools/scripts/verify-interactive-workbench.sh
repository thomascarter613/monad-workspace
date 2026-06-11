#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib interactive_workbench
cargo test -p monad-cli workbench_plan

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- workbench-plan --dry-run > "$text_output"
grep -q "Monad interactive workbench / TUI foundation plan" "$text_output"
grep -q "TUI shell proof of concept" "$text_output"
grep -q "Approval review screen" "$text_output"

cargo run -p monad-cli -- workbench-plan --dry-run --format=json > "$json_output"
grep -q '"command": "workbench-plan"' "$json_output"
grep -q 'nav:dashboard' "$json_output"

echo "Interactive workbench verification passed."
