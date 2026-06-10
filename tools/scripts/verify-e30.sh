#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-plugin-system.sh

test -f crates/monad-core/src/plugin_system.rs
test -f docs/plugin-system/README.md
test -f docs/roadmap/epic-30-plugin-extension-system.md

grep -q "Plugin and Extension System" docs/plugin-system/README.md
grep -q "WP-E30-001" docs/roadmap/epic-30-plugin-extension-system.md
grep -q "WP-E30-006" docs/roadmap/epic-30-plugin-extension-system.md

echo "E30 verification passed."
