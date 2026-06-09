---
title: Release Model
status: complete
epic: E16
---

# Release Model

E16 adds a bounded release planning model.

## Core types

```text
ReleasePlan
ReleaseCheck
ReleaseDecision
ReleaseCheckSeverity
ReleaseCheckCategory
ReleasePlanOptions
```

## Decision model

A release plan is either:

- `go`
- `no-go`

Any blocker produces `no-go`.

Warnings do not block, but should be reviewed.

## Boundaries

The first release foundation is intentionally not a publishing system.

It prepares evidence and local artifacts only.
