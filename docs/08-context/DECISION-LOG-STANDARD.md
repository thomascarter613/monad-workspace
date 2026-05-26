---
title: "Decision Log Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-26
version: 0.2.0
tags:
  - monad
  - context
  - decision-log
  - standard
  - ai
related:
  - docs/08-context/CONTEXT-ARTIFACT-SCHEMAS.md
  - docs/08-context/CONTEXT-BRIDGE.md
  - docs/06-adrs/README.md
---

# Decision Log Standard

## Purpose

This document defines the standard for the Monad decision log.

The decision log is a running record of project decisions that should not be relitigated unless a new ADR or explicit review supersedes them.

## Core Rule

A decision log must:

```text
record the decision clearly
record when and why it was made
distinguish accepted from provisional decisions
track superseded decisions
be a single file that grows over time
```

## Canonical Location

```text
.monad/context/decision-log.md
```

## Structure

The decision log has three sections:

### Durable Decisions

Decisions that are accepted and should not be revisited without a new ADR.

Each entry:

```text
- **{ID}**: {decision statement}
  - Date: {YYYY-MM-DD}
  - Source: {ADR-NNNN | WP-EX-NNN | session}
  - Status: accepted
```

Example:

```text
- **D-001**: Monad uses Rust for its core runtime.
  - Date: 2026-05-23
  - Source: ADR-0001
  - Status: accepted
```

### Provisional Decisions

Decisions that are in use but may be revisited.

Same entry format with `Status: provisional`.

### Superseded Decisions

Decisions that have been replaced. Include a reference to the superseding decision.

```text
- **D-005**: Product name is AionX.
  - Date: 2026-05-20
  - Source: early planning
  - Status: superseded by D-006
```

## Decision Sources

Decisions can originate from:

- **ADRs** — formal Architecture Decision Records in `docs/06-adrs/`;
- **Work packets** — decisions made during implementation;
- **Sessions** — decisions made during a work session and recorded in a session chronicle.

ADR-sourced decisions have the highest weight. Session-sourced decisions should be promoted to ADRs if they affect architecture.

## Generation Strategy

When Monad generates the decision log, it should:

1. Read all ADRs in `docs/06-adrs/` and extract accepted decisions.
2. Read session chronicles for session-sourced decisions.
3. Merge and deduplicate entries.
4. Sort by date within each section.

## Update Triggers

Update the decision log when:

- a new ADR is accepted;
- a provisional decision is promoted to accepted;
- a decision is superseded.
