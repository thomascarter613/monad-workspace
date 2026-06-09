#!/usr/bin/env bash
set -euo pipefail

echo "==> E16 verification"

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-release.sh

if [ -x tools/scripts/verify.sh ]; then
  tools/scripts/verify.sh
fi

echo "verify-e16: PASS"
