#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-web-workbench.sh

test -f crates/monad-core/src/web_workbench.rs
test -f docs/web-workbench/README.md
test -f docs/roadmap/epic-35-web-workbench-foundation.md

grep -q "Web Workbench Foundation" docs/web-workbench/README.md
grep -q "WP-E35-001" docs/roadmap/epic-35-web-workbench-foundation.md
grep -q "WP-E35-006" docs/roadmap/epic-35-web-workbench-foundation.md

echo "E35 verification passed."
