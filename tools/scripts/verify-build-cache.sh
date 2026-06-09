#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib build_cache
cargo test -p monad-cli cache_plan

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- cache-plan --dry-run > "$text_output"
grep -q "Monad build cache and incremental execution plan" "$text_output"
grep -q "Task decisions" "$text_output"
grep -q "No build or test commands are executed by Monad" "$text_output"

cargo run -p monad-cli -- cache-plan --dry-run --format=json > "$json_output"
grep -q '"command": "cache-plan"' "$json_output"
grep -q 'rust:test' "$json_output"

echo "Build cache verification passed."
