#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-mcp-integration.sh

test -f crates/monad-core/src/mcp_integration.rs
test -f docs/mcp-integration/README.md
test -f docs/roadmap/epic-31-mcp-external-tool-integration.md

grep -q "MCP and External Tool Integration" docs/mcp-integration/README.md
grep -q "WP-E31-001" docs/roadmap/epic-31-mcp-external-tool-integration.md
grep -q "WP-E31-006" docs/roadmap/epic-31-mcp-external-tool-integration.md

echo "E31 verification passed."
