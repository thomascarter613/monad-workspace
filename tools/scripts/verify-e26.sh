#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-test-intelligence.sh

test -f crates/monad-core/src/test_intelligence.rs
test -f docs/test-intelligence/README.md
test -f docs/roadmap/epic-26-test-intelligence-verification-planning.md

grep -q "Test Intelligence and Verification Planning" docs/test-intelligence/README.md
grep -q "WP-E26-001" docs/roadmap/epic-26-test-intelligence-verification-planning.md
grep -q "WP-E26-006" docs/roadmap/epic-26-test-intelligence-verification-planning.md

echo "E26 verification passed."
