---
title: monad sync
status: complete
epic: E14
---

# `monad sync`

`monad sync` compares Monad's declared repository intent with discovered repository state and produces a reviewable synchronization plan.

## Commands

```bash
monad sync --dry-run
monad sync --dry-run --format=json
monad sync --yes
```

## Safety contract

`monad sync` is MVP-safe and non-destructive.

It does not:

- rewrite native manifests such as `Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`, or `go.work`;
- install dependencies;
- generate lockfiles;
- run package managers;
- run language toolchains;
- overwrite user-owned source files;
- publish packages;
- synchronize with cloud services;
- perform autonomous agent-driven changes.

## Dry-run behavior

```bash
monad sync --dry-run
```

Dry-run:

- discovers repository state;
- checks core Monad paths such as `monad.toml` and `.monad/`;
- checks component family directories;
- discovers first-level components under `apps/`, `packages/`, `services/`, and `tools/`;
- discovers supported component-native manifests;
- reports mismatches and unsupported automatic changes;
- writes no files.

## JSON dry-run

```bash
monad sync --dry-run --format=json
```

JSON output is intended for future dashboards, automation, and AI-readable evidence.

## Guarded writes

```bash
monad sync --yes
```

The guarded write path writes generated evidence only:

```text
.monad/reports/sync-report.md
.monad/reports/sync-report.json
```

It does not rewrite source files or native manifests.
