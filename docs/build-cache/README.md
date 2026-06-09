# Build Cache and Incremental Execution

E27 adds Monad's first local-only build-cache and incremental execution planning
foundation.

## What this foundation does

- Defines a cache and incremental execution contract.
- Computes deterministic task fingerprints.
- Reads an optional local execution metadata store.
- Produces cache-aware dry-run plans.
- Models an incremental execution proof of concept without executing tasks.
- Writes generated cache evidence only when explicitly approved.

## Command surface

```bash
monad cache-plan --dry-run
monad cache-plan --dry-run --format=json
monad cache-plan --yes
```

Aliases:

```bash
monad build-cache --dry-run
monad incremental-plan --dry-run
```

## Local metadata store

The local execution metadata store is:

```text
.monad/cache/execution-metadata.tsv
```

Format:

```text
task_id<TAB>fingerprint<TAB>status<TAB>evidence_path
```

## Safety boundaries

This foundation does **not**:

- execute build commands;
- execute test commands;
- invoke package managers;
- restore cache artifacts;
- contact remote cache services;
- call AI providers;
- rewrite user-owned source files.

`--yes` writes generated evidence only under `.monad/reports` and `.monad/cache`.
