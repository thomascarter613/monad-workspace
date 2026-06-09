#!/usr/bin/env bash
set -euo pipefail

echo "[verify-e21] cargo fmt --check"
cargo fmt --check

echo "[verify-e21] cargo test"
cargo test

echo "[verify-e21] cargo clippy --all-targets --all-features -- -D warnings"
cargo clippy --all-targets --all-features -- -D warnings

echo "[verify-e21] tools/scripts/verify-work-packet.sh"
tools/scripts/verify-work-packet.sh

echo "[verify-e21] ok"
