---
title: "WP-E12-001 Add UX and Safety Contract Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-001
tags:
  - monad
  - e12
  - add
  - safety
  - command-contract
related:
  - docs/commands/ADD.md
---

# WP-E12-001 Add UX and Safety Contract Deliverable

## Work Packet

WP-E12-001 — Define `monad add` UX and safety contract.

## Outcome

Completed.

## Summary

This work packet establishes the design and safety contract for `monad add` before implementation begins.

The contract defines:

- dry-run-first behavior;
- no silent overwrite rule;
- explicit `--yes` approval boundary for future writes;
- initial component kinds;
- component name validation rules;
- initial scaffold shape;
- relationship to `monad init`;
- non-goals and command boundaries.

## Initial Component Kinds

```text
app
package
service
tool
```

## Initial Target Path Families

```text
apps/<name>
packages/<name>
services/<name>
tools/<name>
```

## Safety Boundary

`monad add` must:

- preview before writes;
- require `--yes` for future writes;
- refuse to overwrite existing component paths;
- avoid Git mutation;
- avoid package installation;
- avoid remote service calls.

## Deliverables

- `docs/commands/ADD.md`
- `work/deliverables/E12/WP-E12-001-add-ux-safety-contract.md`

## Verification

Run:

```bash
git status --short
test -f docs/commands/ADD.md
grep -n "Dry-run before writes" docs/commands/ADD.md
grep -n "No silent overwrite" docs/commands/ADD.md
grep -n "Initial Component Kinds" docs/commands/ADD.md
tools/scripts/verify.sh
git status --short
```

## Recommended Commit

```bash
git commit -m "docs(add): define add ux and safety contract"
```

## Closeout Note

WP-E12-001 is complete once the command contract is committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E12-002 — Add add-command dry-run plan
```
