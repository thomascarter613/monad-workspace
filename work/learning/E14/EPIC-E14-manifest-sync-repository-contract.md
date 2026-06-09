---
title: Epic E14 Learning Note
epic: E14
---

# Epic E14 Learning Note: Manifest Sync and Repository Contract Foundation

E14 adds the first safe form of repository synchronization.

The important concept is that sync does not mean "rewrite everything."

In Monad, sync means:

1. discover declared repository intent;
2. discover actual repository state;
3. compare the two;
4. produce deterministic findings;
5. write generated evidence only when approved.

## Why it is safe

`monad sync --dry-run` writes nothing.

`monad sync --yes` writes only:

```text
.monad/reports/sync-report.md
.monad/reports/sync-report.json
```

It does not rewrite native manifests or source files.

## What to inspect

```bash
git diff -- crates/monad-core/src/sync.rs
git diff -- crates/monad-core/src/lib.rs
git diff -- crates/monad-cli/src/main.rs
git diff -- docs/commands/SYNC.md
git diff -- docs/architecture/REPOSITORY-CONTRACT.md
```

## Why this prepares E15

E15 is doctor diagnostics. Doctor can reuse E14's contract concepts to report whether the repo is healthy and ready.
