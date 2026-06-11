#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-agent-sandbox.sh

test -f crates/monad-core/src/agent_sandbox.rs
test -f docs/agent-sandbox/README.md
test -f docs/roadmap/epic-33-agent-workflow-sandbox.md

grep -q "Agent Workflow Sandbox" docs/agent-sandbox/README.md
grep -q "WP-E33-001" docs/roadmap/epic-33-agent-workflow-sandbox.md
grep -q "WP-E33-006" docs/roadmap/epic-33-agent-workflow-sandbox.md

echo "E33 verification passed."
