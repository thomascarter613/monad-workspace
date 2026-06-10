# E31 — MCP and External Tool Integration Foundation

## Product Area

MCP and External Tool Integration Foundation

## Objective

Add Monad's MCP integration boundary, MCP/local tool capability model, context
export proof of concept, external tool invocation policy checks, documentation,
and smoke tests.

## Work Packets

- WP-E31-001 — Define MCP integration boundary
- WP-E31-002 — Add MCP tool capability model
- WP-E31-003 — Add MCP context export proof of concept
- WP-E31-004 — Add external tool invocation policy checks
- WP-E31-005 — Add MCP/local tool documentation
- WP-E31-006 — Add MCP integration smoke tests

## Delivered Behavior

- `crates/monad-core/src/mcp_integration.rs`
- `monad mcp-plan --dry-run`
- `monad mcp-plan --dry-run --format=json`
- `monad mcp-plan --yes`
- `monad mcp --dry-run`
- `monad external-tools --dry-run`
- `tools/scripts/verify-mcp-integration.sh`
- `tools/scripts/verify-e31.sh`

## Safety

E31 is export/planning-only. Monad models MCP capabilities and external-tool
policies, but it does not connect to MCP servers, invoke tools, execute
subprocesses, access networks, or mutate user-owned source files.
