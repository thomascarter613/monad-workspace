#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib dependency_impact
cargo test -p monad-cli impact

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- impact --dry-run > "$text_output"
grep -q "Monad dependency graph and impact analysis plan" "$text_output"
grep -q "Recommended verification" "$text_output"
grep -q "No package managers are invoked" "$text_output"

cargo run -p monad-cli -- impact --dry-run --format=json > "$json_output"
grep -q '"command": "impact"' "$json_output"
grep -q 'component:monad-core' "$json_output"

echo "Dependency impact verification passed."
