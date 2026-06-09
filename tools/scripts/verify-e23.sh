#!/usr/bin/env bash
set -euo pipefail

echo "[verify-e23] checking expected files"
test -f crates/monad-core/src/language_adapters.rs
test -f docs/language-adapters/README.md
test -f docs/roadmap/epic-23-language-adapter-foundation.md
test -x tools/scripts/verify-language-adapters.sh

echo "[verify-e23] cargo fmt --check"
cargo fmt --check

echo "[verify-e23] cargo test"
cargo test

echo "[verify-e23] cargo clippy"
cargo clippy --all-targets --all-features -- -D warnings

echo "[verify-e23] language adapter verification"
tools/scripts/verify-language-adapters.sh

echo "[verify-e23] ok"
