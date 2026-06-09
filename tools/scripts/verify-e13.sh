#!/usr/bin/env bash
set -euo pipefail

echo "==> E13 verification"

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings

if [ -x tools/scripts/verify-add.sh ]; then
  tools/scripts/verify-add.sh
fi

tools/scripts/verify-add-language.sh

echo "verify-e13: PASS"
