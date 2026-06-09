#!/usr/bin/env bash
set -euo pipefail

# Verify E12 — Component Add and Polyglot Scaffold Foundation.
#
# This script is intended as an E12 closeout check.
#
# It runs:
# - formatting check;
# - full test suite;
# - clippy with warnings denied;
# - add-command smoke verification;
# - general repo verification.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "${REPO_ROOT}"

echo "==> E12 verification: formatting"
cargo fmt --check

echo "==> E12 verification: tests"
cargo test

echo "==> E12 verification: clippy"
cargo clippy --all-targets --all-features -- -D warnings

echo "==> E12 verification: add smoke tests"
tools/scripts/verify-add.sh

echo "==> E12 verification: general repo verification"
tools/scripts/verify.sh

echo
echo "E12 verification passed."
