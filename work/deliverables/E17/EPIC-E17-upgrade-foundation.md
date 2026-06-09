---
title: Epic E17 Deliverable Record
epic: E17
status: complete
---

# Epic E17 Deliverable Record

## Epic

E17 — Upgrade and Repository Evolution Foundation.

## Completed work packets

- WP-E17-001 — Define `monad upgrade` contract and safety model
- WP-E17-002 — Add repository version and upgrade target model
- WP-E17-003 — Add upgrade dry-run plan output
- WP-E17-004 — Add upgrade step registry foundation
- WP-E17-005 — Add guarded non-destructive upgrade writes
- WP-E17-006 — Add upgrade evidence reports and smoke tests

## Implementation files

```text
crates/monad-core/src/upgrade.rs
crates/monad-core/src/lib.rs
crates/monad-cli/src/main.rs
tools/scripts/verify-upgrade.sh
tools/scripts/verify-e17.sh
```

## Verification command

```bash
tools/scripts/verify-e17.sh
```
