#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-dependency-impact.sh

test -f crates/monad-core/src/dependency_impact.rs
test -f docs/dependency-impact/README.md
test -f docs/roadmap/epic-25-dependency-graph-impact-analysis.md

grep -q "Dependency Graph and Impact Analysis" docs/dependency-impact/README.md
grep -q "WP-E25-001" docs/roadmap/epic-25-dependency-graph-impact-analysis.md
grep -q "WP-E25-006" docs/roadmap/epic-25-dependency-graph-impact-analysis.md

echo "E25 verification passed."
