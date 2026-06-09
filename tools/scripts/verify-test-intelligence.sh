#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib test_intelligence
cargo test -p monad-cli verify_plan

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- verify-plan --dry-run > "$text_output"
grep -q "Monad test intelligence and verification plan" "$text_output"
grep -q "Recommended verification" "$text_output"
grep -q "No test commands are executed by Monad" "$text_output"

cargo run -p monad-cli -- verify-plan --dry-run --format=json > "$json_output"
grep -q '"command": "verify-plan"' "$json_output"
grep -q 'cargo test' "$json_output"

echo "Test intelligence verification passed."
