---
title: Repo Contract Schema and Validation
status: active
last_reviewed: 2026-06-09
---

# Repo Contract Schema and Validation

E22 establishes Monad's repository contract schema and validation foundation.

The goal is to make repository intent and generated state explicit without turning Monad into an autonomous migration engine.

## Command surface

```bash
monad contract --dry-run
monad contract --dry-run --format=json
monad contract --yes
```

## What E22 validates

- `monad.toml` exists and parses as the supported manifest schema.
- Manifest diagnostics from `monad-core` are included in a contract schema report.
- The initial repository contract path model remains satisfied.
- `monad.lock` is treated as generated state, not hand-authored source.
- Schema migration planning is reported but not executed destructively.

## Generated artifacts

`monad contract --yes` may write generated artifacts only:

- `monad.lock`
- `.monad/state/repository-contract-state.json`
- `.monad/reports/contract-schema-report.md`
- `.monad/reports/contract-schema-report.json`
- `.monad/reports/contract-schema-migration-plan.md`

All writes go through E19 generated-write approval gates. Existing files with different content are not silently overwritten.

## Safety boundaries

E22 does not:

- rewrite `monad.toml`;
- rewrite user-owned source files;
- run destructive migrations;
- contact a remote schema registry;
- call AI providers;
- execute arbitrary scripts;
- publish packages or releases.
