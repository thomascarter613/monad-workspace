#!/usr/bin/env bash
set -euo pipefail

echo "[verify-static-analysis] cargo test -p monad-core --lib static_analysis"
cargo test -p monad-core --lib static_analysis

echo "[verify-static-analysis] monad analysis --dry-run"
cargo run -q -p monad-cli -- analysis --dry-run >/tmp/monad-analysis-dry-run.txt
grep -q "Monad LSP and static-analysis plan" /tmp/monad-analysis-dry-run.txt
grep -q "No language servers were launched" /tmp/monad-analysis-dry-run.txt

echo "[verify-static-analysis] monad analysis --dry-run --format=json"
cargo run -q -p monad-cli -- analysis --dry-run --format=json >/tmp/monad-analysis-dry-run.json
grep -q '"command":"analysis"' /tmp/monad-analysis-dry-run.json
grep -q '"lsp_servers"' /tmp/monad-analysis-dry-run.json

echo "[verify-static-analysis] ok"
