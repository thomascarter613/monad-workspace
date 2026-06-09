#!/usr/bin/env bash
set -euo pipefail

echo "[verify-e20] cargo fmt --check"
cargo fmt --check

echo "[verify-e20] cargo test"
cargo test

echo "[verify-e20] cargo clippy --all-targets --all-features -- -D warnings"
cargo clippy --all-targets --all-features -- -D warnings

echo "[verify-e20] tools/scripts/verify-patch.sh"
tools/scripts/verify-patch.sh

echo "[verify-e20] ok"
