#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib mcp_integration
cargo test -p monad-cli mcp_plan

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- mcp-plan --dry-run > "$text_output"
grep -q "Monad MCP and external tool integration plan" "$text_output"
grep -q "Boundary:" "$text_output"
grep -q "No external tools are invoked by Monad" "$text_output"

cargo run -p monad-cli -- mcp-plan --dry-run --format=json > "$json_output"
grep -q '"command": "mcp-plan"' "$json_output"
grep -q 'context:repo-summary' "$json_output"

echo "MCP integration verification passed."
