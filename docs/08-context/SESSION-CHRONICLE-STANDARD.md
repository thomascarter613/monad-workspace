---
title: "Session Chronicle Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-26
version: 0.2.0
tags:
  - monad
  - context
  - session-chronicle
  - standard
  - ai
related:
  - docs/08-context/CONTEXT-ARTIFACT-SCHEMAS.md
  - docs/08-context/CONTEXT-BRIDGE.md
  - docs/08-context/HANDOFF-STANDARD.md
---

# Session Chronicle Standard

## Purpose

This document defines the standard for Monad session chronicle artifacts.

A session chronicle records what happened in a single work session. It serves as an audit trail and a continuity bridge between sessions.

## Core Rule

A session chronicle must:

```text
record what was intended
record what was done
record what was decided
record what remains
be short enough to skim
```

## Canonical Location

```text
.monad/context/session-chronicles/{YYYY-MM-DD}-session-{NNN}.md
```

Example:

```text
.monad/context/session-chronicles/2026-05-26-session-001.md
```

## Required Sections

### Session Identity

```yaml
date: 2026-05-26
session: 001
epic: E2
work_packet: WP-E2-017
```

### Goals

What the session intended to accomplish. Write these at the start of a session.

### Work Performed

What was actually done. Include:

- files created or modified;
- commits made (with messages);
- commands run;
- tests added.

### Decisions Made

Decisions accepted during this session. Each decision should include:

- the decision statement;
- the reasoning;
- whether it should be promoted to the decision log.

### Verification Results

What verification was run and the outcome.

```text
cargo fmt --check: passed
cargo test: 104 passed, 0 failed
tools/scripts/verify.sh: passed
```

### Open Questions

Unresolved questions or items that need attention in a future session.

### Next Session Guidance

What the next session should start with. Include recommended reading order and next action.

## Generation Strategy

When Monad generates a session chronicle, it should:

1. Read recent git commits since the last chronicle.
2. Read changed files to summarize work performed.
3. Read the decision log for new entries.
4. Run verification and record results.

## Size Guidance

A session chronicle should be 200–600 words. Long chronicles suggest the session covered too many topics; prefer shorter, focused sessions.

## Retention

Session chronicles accumulate. Old chronicles should not be deleted. They serve as an audit trail and can help understand why decisions were made.
