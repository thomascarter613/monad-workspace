---
title: "WP-E11-001 Init UX and Safety Contract Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E11
work_packet: WP-E11-001
tags:
  - monad
  - e11
  - init
  - safety
  - command-contract
related:
  - docs/commands/INIT.md
---

# WP-E11-001 Init UX and Safety Contract Deliverable

## Work Packet

WP-E11-001 — Define `monad init` UX and safety contract.

## Outcome

Completed.

## Summary

This work packet establishes the design and safety contract for `monad init` before code implementation begins.

The contract defines:

- dry-run-first implementation order;
- no silent overwrite rule;
- explicit approval requirement for writes;
- existing-repository support;
- Git non-mutation boundary;
- generated-file visibility;
- initial `minimal` and `polyglot-minimal` presets;
- relationship to future commands.

## Deliverables

- `docs/commands/INIT.md`
- `work/deliverables/E11/WP-E11-001-init-ux-safety-contract.md`

## Decision

`monad init` should begin as a dry-run planning feature and only later gain explicit, guarded write behavior.

The accepted write approval flag for later implementation is:

```bash
--yes
```

The accepted first presets are:

```text
minimal
polyglot-minimal
```

## Verification

Run:

```bash
git status --short
test -f docs/commands/INIT.md
grep -n "No silent overwrite" docs/commands/INIT.md
grep -n "Dry-run before writes" docs/commands/INIT.md
grep -n "Write path requires explicit approval" docs/commands/INIT.md
tools/scripts/verify.sh
git status --short
```

## Recommended Commit

```bash
git commit -m "docs(init): define init ux and safety contract"
```

## Closeout Note

WP-E11-001 is complete once the command contract is committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E11-002 — Add init dry-run plan
```
