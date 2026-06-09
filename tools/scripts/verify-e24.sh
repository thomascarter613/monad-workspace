#!/usr/bin/env bash
set -euo pipefail

echo "[verify-e24] cargo fmt --check"
cargo fmt --check

echo "[verify-e24] cargo test"
cargo test

echo "[verify-e24] cargo clippy"
cargo clippy --all-targets --all-features -- -D warnings

echo "[verify-e24] static-analysis smoke"
tools/scripts/verify-static-analysis.sh

echo "[verify-e24] ok"
