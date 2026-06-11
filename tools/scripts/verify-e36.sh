#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-github-workflow.sh

test -f crates/monad-core/src/github_workflow.rs
test -f docs/github-workflow/README.md
test -f docs/roadmap/epic-36-github-integration-pr-workflow.md

grep -q "GitHub Integration and PR Workflow Foundation" docs/github-workflow/README.md
grep -q "WP-E36-001" docs/roadmap/epic-36-github-integration-pr-workflow.md
grep -q "WP-E36-006" docs/roadmap/epic-36-github-integration-pr-workflow.md

echo "E36 verification passed."
