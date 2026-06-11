#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib github_workflow
cargo test -p monad-cli github_plan

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- github-plan --dry-run > "$text_output"
grep -q "Monad GitHub integration and PR workflow foundation plan" "$text_output"
grep -q "Issue sync/export model" "$text_output"
grep -q "PR description and review-pack generation" "$text_output"

cargo run -p monad-cli -- github-plan --dry-run --format=json > "$json_output"
grep -q '"command": "github-plan"' "$json_output"
grep -q 'thomascarter613/monad-workspace' "$json_output"

echo "GitHub workflow verification passed."
