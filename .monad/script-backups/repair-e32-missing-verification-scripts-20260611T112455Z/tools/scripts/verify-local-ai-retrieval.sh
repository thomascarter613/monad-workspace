#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib local_ai_retrieval
cargo test -p monad-cli retrieval_plan

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- retrieval-plan --dry-run > "$text_output"
grep -q "Monad local AI retrieval and vector memory plan" "$text_output"
grep -q "deterministic-local" "$text_output"
grep -q "No AI model provider is called by Monad" "$text_output"

cargo run -p monad-cli -- retrieval-plan --dry-run --format=json > "$json_output"
grep -q '"command": "retrieval-plan"' "$json_output"
grep -q 'deterministic-local' "$json_output"

echo "Local AI retrieval verification passed."
