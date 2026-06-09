---
title: E23 — Language Adapter Foundation
status: implemented
owner: Monad
last_reviewed: 2026-06-09
---

# E23 — Language Adapter Foundation

## Scope

E23 implements the first language adapter foundation for Monad.

## Work packets

- WP-E23-001 — Define language adapter interface contract
- WP-E23-002 — Add Rust adapter foundation
- WP-E23-003 — Add Node/Bun adapter foundation
- WP-E23-004 — Add Python adapter foundation
- WP-E23-005 — Add Go and Java adapter foundations
- WP-E23-006 — Add adapter registry tests and documentation

## Command surface

```bash
monad adapters --dry-run
monad adapters --dry-run --format=json
monad adapters --yes
```

`language-adapters` is accepted as an alias for `adapters`.

## Evidence

`monad adapters --yes` writes generated evidence to:

- `.monad/reports/language-adapters.md`
- `.monad/reports/language-adapters.json`

## Safety

E23 does not execute native tools. It records language adapter metadata and
command suggestions for future supervised workflows.
