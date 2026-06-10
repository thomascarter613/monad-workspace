#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-report-store.sh

test -f crates/monad-core/src/report_store.rs
test -f docs/report-store/README.md
test -f docs/roadmap/epic-28-local-artifact-report-store.md

grep -q "Local Artifact and Report Store" docs/report-store/README.md
grep -q "WP-E28-001" docs/roadmap/epic-28-local-artifact-report-store.md
grep -q "WP-E28-006" docs/roadmap/epic-28-local-artifact-report-store.md

echo "E28 verification passed."
