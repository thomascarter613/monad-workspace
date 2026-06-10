#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-template-registry.sh

test -f crates/monad-core/src/template_registry.rs
test -f docs/template-registry/README.md
test -f docs/roadmap/epic-29-template-registry-preset-evolution.md

grep -q "Template Registry and Preset Evolution" docs/template-registry/README.md
grep -q "WP-E29-001" docs/roadmap/epic-29-template-registry-preset-evolution.md
grep -q "WP-E29-006" docs/roadmap/epic-29-template-registry-preset-evolution.md

echo "E29 verification passed."
