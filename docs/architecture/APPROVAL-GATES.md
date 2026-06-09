---
title: Approval Gates
status: complete
epic: E19
---

# Approval Gates

Approval gates are the boundary between planning and mutation.

## Gate types

| Gate | Meaning |
|---|---|
| none | Safe read-only operation |
| dry-run-review | Must be reviewed before any future apply path |
| explicit-yes | Requires explicit caller approval marker |
| forbidden | Blocked in the MVP policy foundation |

## Examples

- `doctor` uses `none`.
- `sync --yes`, `upgrade --yes`, and `ai-context --yes` require `explicit-yes`.
- release publishing remains outside MVP.
- patch/source mutation is `forbidden`.
