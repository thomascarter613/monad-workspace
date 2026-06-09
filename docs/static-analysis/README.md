---
title: Static Analysis and LSP Foundation
status: active
owner: Monad
last_reviewed: 2026-06-09
---

# Static Analysis and LSP Foundation

E24 establishes Monad's first local static-analysis and LSP planning foundation.

This foundation is intentionally conservative:

- it models parser abstractions;
- it models LSP server capabilities;
- it performs bounded local source scanning;
- it extracts simple symbols with deterministic heuristic parsers;
- it records source-map and ownership metadata;
- it writes generated evidence only when explicitly approved.

## Safety boundaries

E24 does not:

- launch language servers;
- install analyzers;
- run package managers;
- execute arbitrary commands;
- send source code to remote services;
- call AI providers;
- rewrite user-owned source files.

## Commands

```bash
monad analysis --dry-run
monad analysis --dry-run --format=json
monad analysis --yes
```

Alias:

```bash
monad static-analysis --dry-run
```

`--yes` writes generated evidence under `.monad/reports/` through the existing guarded generated-write policy path.
