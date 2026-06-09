#!/usr/bin/env bash
set -euo pipefail

echo "==> E18 verification"

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-ai-context.sh

if [ -x tools/scripts/verify.sh ]; then
  tools/scripts/verify.sh
fi

echo "verify-e18: PASS"
