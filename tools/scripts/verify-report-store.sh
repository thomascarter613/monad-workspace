#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib report_store
cargo test -p monad-cli report_store

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- report-store --dry-run > "$text_output"
grep -q "Monad local artifact and report store index" "$text_output"
grep -q "Contract:" "$text_output"
grep -q "No reports or artifacts are deleted" "$text_output"

cargo run -p monad-cli -- report-store --dry-run --format=json > "$json_output"
grep -q '"command": "report-store"' "$json_output"
grep -q '.monad/reports' "$json_output"

echo "Report store verification passed."
