#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib agent_sandbox
cargo test -p monad-cli sandbox_plan

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- sandbox-plan --dry-run > "$text_output"
grep -q "Monad agent workflow sandbox plan" "$text_output"
grep -q "Verification command path" "$text_output"
grep -q "No agent actions are executed by Monad" "$text_output"

cargo run -p monad-cli -- sandbox-plan --dry-run --format=json > "$json_output"
grep -q '"command": "sandbox-plan"' "$json_output"
grep -q 'agent-workflow-default' "$json_output"

echo "Agent sandbox verification passed."
