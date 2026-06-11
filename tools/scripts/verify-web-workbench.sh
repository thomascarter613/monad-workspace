#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib web_workbench
cargo test -p monad-cli web_workbench_plan

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- web-workbench-plan --dry-run > "$text_output"
grep -q "Monad web workbench foundation plan" "$text_output"
grep -q "Local server/API foundation" "$text_output"
grep -q "Approval/context viewer foundation" "$text_output"

cargo run -p monad-cli -- web-workbench-plan --dry-run --format=json > "$json_output"
grep -q '"command": "web-workbench-plan"' "$json_output"
grep -q '/api/repository-graph' "$json_output"

echo "Web workbench verification passed."
