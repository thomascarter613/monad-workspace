---
title: "Rehydration Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-26
version: 0.2.0
tags:
  - monad
  - context
  - rehydration
  - standard
  - ai
related:
  - docs/08-context/CONTEXT-ARTIFACT-SCHEMAS.md
  - docs/08-context/CONTEXT-BRIDGE.md
  - docs/08-context/HANDOFF-STANDARD.md
  - docs/09-ai/BOOTSTRAP-PROMPT.md
---

# Rehydration Standard

## Purpose

This document defines the standard for context rehydration in Monad.

Rehydration is the process of restoring project understanding from repository files at the start of a new session. It replaces reliance on hidden chat memory.

## Core Rule

Rehydration must:

```text
use repository files as the source of truth
follow a predictable reading order
be fast enough to complete before work begins
surface staleness warnings
never trust chat memory over repo state
```

## Rehydration Reading Order

A new session should rehydrate by reading files in this order:

### Tier 1 — Orientation (required)

```text
1. docs/09-ai/CURRENT-STATE.md          — where we are
2. .monad/context/latest-context-pack.md — full context bundle
3. .monad/context/latest-handoff.md      — session continuity
```

### Tier 2 — Active Work (recommended)

```text
4. work/epics/{active-epic}.md           — epic scope and goals
5. work/packets/{epic}/{active-wp}.md    — current work packet details
6. .monad/context/decision-log.md        — decisions not to revisit
```

### Tier 3 — Architecture (as needed)

```text
7. docs/05-architecture/SYSTEM-OVERVIEW.md
8. docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
9. docs/05-architecture/MODULE-BOUNDARIES.md
10. docs/06-adrs/README.md
```

### Tier 4 — Product and Workflow (reference)

```text
11. docs/01-project/01-charter/PRODUCT-CHARTER.md
12. docs/07-workflow/OPERATING-MODEL.md
13. docs/07-workflow/WORK-PACKET-STANDARD.md
```

## Rehydration Output

After rehydration, a session should be able to state:

- the current active epic;
- the current active work packet or slice;
- the next recommended action;
- any blockers;
- which files were used for orientation.

This matches the bootstrap prompt expectation.

## Staleness Warnings

During rehydration, check for staleness:

- If `current-state.md` references a different epic than the latest commit messages, warn.
- If `latest-handoff.md` was generated more than 7 days ago, warn.
- If `latest-context-pack.md` references a work packet that no longer exists, warn.

## Conflict Resolution

If repository files conflict with chat memory or assumptions:

1. Trust the repository.
2. Identify the conflict explicitly.
3. Do not silently override repo state with chat assumptions.

## Future Automation

Monad should eventually support:

```bash
monad rehydrate
```

This command would:

1. Read context artifacts in the standard order.
2. Check for staleness.
3. Output a rehydration summary.
4. Warn about conflicts or missing artifacts.

This is out of scope for E3 but should be planned for a future epic.
