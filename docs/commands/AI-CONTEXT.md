---
title: monad ai-context
status: complete
epic: E18
---

# `monad ai-context`

`monad ai-context` prepares provider-agnostic, repo-native AI context and memory artifacts.

## Commands

```bash
monad ai-context --dry-run
monad ai-context --dry-run --format=json
monad ai-context --yes
```

## Safety contract

`ai-context` does not:

- require a paid AI subscription;
- hard-code one hosted provider;
- send repository data remotely by default;
- call AI providers;
- execute autonomous agents;
- apply patches;
- run a long-lived daemon;
- install MCP/plugin marketplace dependencies.

## Generated local artifacts

```text
.monad/ai/provider-config.example.toml
.monad/ai/memory/README.md
.monad/ai/memory/0001-project-memory-template.md
.monad/context/ai-context-snapshot.md
.monad/context/work-packet-plan.md
.monad/context/assistant-handoff.md
.monad/reports/ai-context-report.md
.monad/reports/ai-context-report.json
```
