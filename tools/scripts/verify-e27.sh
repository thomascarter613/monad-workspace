#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-build-cache.sh

test -f crates/monad-core/src/build_cache.rs
test -f docs/build-cache/README.md
test -f docs/roadmap/epic-27-build-cache-incremental-execution.md

grep -q "Build Cache and Incremental Execution" docs/build-cache/README.md
grep -q "WP-E27-001" docs/roadmap/epic-27-build-cache-incremental-execution.md
grep -q "WP-E27-006" docs/roadmap/epic-27-build-cache-incremental-execution.md

echo "E27 verification passed."
