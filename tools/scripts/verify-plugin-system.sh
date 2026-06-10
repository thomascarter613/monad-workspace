#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib plugin_system
cargo test -p monad-cli plugin_plan

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- plugin-plan --dry-run > "$text_output"
grep -q "Monad plugin and extension system plan" "$text_output"
grep -q "Extension points:" "$text_output"
grep -q "Plugins are disabled by default" "$text_output"

cargo run -p monad-cli -- plugin-plan --dry-run --format=json > "$json_output"
grep -q '"command": "plugin-plan"' "$json_output"
grep -q 'extension_points' "$json_output"

echo "Plugin system verification passed."
