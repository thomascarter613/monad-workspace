# MCP and External Tool Integration

E31 adds Monad's MCP and external tool integration foundation.

## What this foundation does

- Defines the MCP integration boundary.
- Adds an MCP/local tool capability model.
- Adds an MCP context export proof of concept.
- Adds external tool invocation policy checks.
- Adds MCP/local tool documentation.
- Adds MCP integration smoke tests.

## Command surface

```bash
monad mcp-plan --dry-run
monad mcp-plan --dry-run --format=json
monad mcp-plan --yes
```

Aliases:

```bash
monad mcp --dry-run
monad external-tools --dry-run
```

## Safety boundaries

This foundation does **not**:

- connect to MCP servers;
- invoke external tools;
- execute subprocesses;
- access the network;
- install packages;
- mutate user-owned source files.

`--yes` writes generated MCP integration evidence only under `.monad/reports`.
