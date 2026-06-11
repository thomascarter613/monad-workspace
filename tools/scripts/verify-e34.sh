#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-interactive-workbench.sh

test -f crates/monad-core/src/interactive_workbench.rs
test -f docs/interactive-workbench/README.md
test -f docs/roadmap/epic-34-interactive-workbench-tui.md

grep -q "Interactive Workbench / TUI Foundation" docs/interactive-workbench/README.md
grep -q "WP-E34-001" docs/roadmap/epic-34-interactive-workbench-tui.md
grep -q "WP-E34-006" docs/roadmap/epic-34-interactive-workbench-tui.md

echo "E34 verification passed."
