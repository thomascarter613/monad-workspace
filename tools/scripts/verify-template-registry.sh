#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib template_registry
cargo test -p monad-cli template_registry

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- template-registry --dry-run > "$text_output"
grep -q "Monad template registry and preset evolution plan" "$text_output"
grep -q "Templates:" "$text_output"
grep -q "No templates were rendered or applied" "$text_output"

cargo run -p monad-cli -- template-registry --dry-run --format=json > "$json_output"
grep -q '"command": "template-registry"' "$json_output"
grep -q 'template' "$json_output"

echo "Template registry verification passed."
