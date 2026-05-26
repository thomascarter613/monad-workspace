---
title: "Context Artifact Schemas"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-26
version: 0.2.0
tags:
  - monad
  - context
  - context-bridge
  - schemas
  - ai
related:
  - docs/08-context/CONTEXT-BRIDGE.md
  - docs/08-context/CONTEXT-PACK-STANDARD.md
  - docs/08-context/CURRENT-STATE-STANDARD.md
  - docs/08-context/HANDOFF-STANDARD.md
  - docs/08-context/SESSION-CHRONICLE-STANDARD.md
  - docs/08-context/DECISION-LOG-STANDARD.md
  - docs/09-ai/BOOTSTRAP-PROMPT.md
  - docs/09-ai/FRESH-CHAT-HANDOFF.md
---

# Context Artifact Schemas

## Purpose

This document is the canonical schema reference for all Monad context artifacts.

Each context artifact has a predictable structure so that humans and AI assistants can locate, read, generate, and validate context without guessing at the format.

## Artifact Catalog

Monad defines six context artifact types:

| Artifact | Purpose | Canonical Location |
| --- | --- | --- |
| Current State | Snapshot of where the project stands right now | `.monad/context/current-state.md` |
| Latest Handoff | Instructions for the next human or AI session | `.monad/context/latest-handoff.md` |
| Context Pack | Compact bundle of all project context | `.monad/context/latest-context-pack.md` |
| Session Chronicle | Record of a single work session | `.monad/context/session-chronicles/{date}-session-{n}.md` |
| Decision Log | Accumulated project decisions | `.monad/context/decision-log.md` |
| Bootstrap Prompt | Initial prompt for a new AI session | `docs/09-ai/BOOTSTRAP-PROMPT.md` |

## Authorship Model

Context artifacts are either human-authored or generated.

| Authorship | Meaning | Frontmatter Field |
| --- | --- | --- |
| Human-authored | Written or edited by a human; considered accepted | `authored: true` |
| Generated | Produced by Monad from repository state; reviewable but not accepted until reviewed | `generated: true` |

Generated artifacts must never claim to be accepted doctrine. They should include a trust note and list their source files.

## Common Frontmatter

Every context artifact uses YAML frontmatter:

```yaml
---
title: "Artifact Title"
document_type: "context-artifact"
artifact_type: "current-state"        # one of: current-state, handoff, context-pack, session-chronicle, decision-log, bootstrap-prompt
status: "current"                     # current, stale, archived
generated: true                       # true if machine-generated
generated_at: "2026-05-26"            # ISO date of generation
reviewed: false                       # whether a human has reviewed this artifact
epic: "E2"                            # active epic at generation time
work_packet: "WP-E2-017"             # active work packet at generation time
source_files:                         # files inspected to produce this artifact
  - "Cargo.toml"
  - "docs/09-ai/CURRENT-STATE.md"
---
```

## Artifact Schemas

### Current State

**Purpose:** Answer "Where are we right now?" in a single file.

**Required sections:**

```text
# Current State

## Project                    — project name and identity
## Completed Epics            — list of finished epics
## Current Epic               — active epic identifier and title
## Current Work Packet        — active work packet identifier and title
## Active Focus               — what this work packet adds (types, modules, commands)
## Runtime Capabilities       — summary of what monad-core currently provides
## Verification               — commands to run and expected result
```

**Standard:** `docs/08-context/CURRENT-STATE-STANDARD.md`

### Latest Handoff

**Purpose:** Allow a future session to resume work without context loss.

**Required sections:**

```text
# Fresh Chat Handoff

## Start Here                 — role instructions and orientation
## Current State              — completed epics, current epic/work packet
## Read First                 — ordered list of files to read
## Runtime Foundation Available — what E1/E2 already provides
## Active Work                — what the current work packet is doing
## Verification               — commands to run
```

**Standard:** `docs/08-context/HANDOFF-STANDARD.md`

### Context Pack

**Purpose:** Compact, structured bundle of all project context for human or AI orientation.

**Required sections:**

```text
# Latest Context Pack

## Identity                   — project name and mission
## Completed                  — finished epics
## Current Epic               — active epic
## Current Work Packet        — active work packet
## Runtime Summary            — what monad-core provides today
## Active Focus               — what the current work packet adds
## Verification               — commands and expected results
```

**Standard:** `docs/08-context/CONTEXT-PACK-STANDARD.md`

### Session Chronicle

**Purpose:** Record what happened in a single work session for audit and continuity.

**Required sections:**

```text
# Session Chronicle — {date}

## Session Identity           — date, duration, epic, work packet
## Goals                      — what the session intended to accomplish
## Work Performed             — what was actually done (commits, files changed)
## Decisions Made             — decisions accepted during this session
## Verification Results       — what verification was run and its outcome
## Open Questions             — unresolved questions or blockers
## Next Session Guidance      — what the next session should do
```

**Standard:** `docs/08-context/SESSION-CHRONICLE-STANDARD.md`

### Decision Log

**Purpose:** Accumulated record of project decisions that should not be relitigated.

**Required sections:**

```text
# Decision Log

## Durable Decisions          — accepted decisions with date and source
## Provisional Decisions      — decisions that may be revisited
## Superseded Decisions       — decisions that have been replaced
```

Each decision entry should include:

```text
- decision statement
- date accepted
- source (ADR, work packet, or session)
- status (accepted, provisional, superseded)
```

**Standard:** `docs/08-context/DECISION-LOG-STANDARD.md`

### Bootstrap Prompt

**Purpose:** Initial instructions for a new AI session to orient itself from repository files.

**Required sections:**

```text
# Bootstrap Prompt

## Purpose                    — why this prompt exists
## Usage                      — how to use it
## Bootstrap Prompt           — the actual prompt text including:
                                - project identity
                                - reading order
                                - working rules
                                - operating role
## Maintenance Rule           — when to update this prompt
```

**Standard:** `docs/09-ai/BOOTSTRAP-PROMPT.md`

## Generation Rules

When Monad generates a context artifact:

1. Include `generated: true` and `generated_at` in frontmatter.
2. List all `source_files` inspected to produce the artifact.
3. Mark `reviewed: false` until a human reviews the output.
4. Do not claim generated content is accepted doctrine.
5. Be deterministic: the same repository state should produce the same artifact.
6. Prefer concise, factual statements over speculative claims.
7. Exclude secrets, credentials, and raw logs.

## Validation Rules

A context artifact is valid when:

- frontmatter includes all required fields for the artifact type;
- all required sections are present;
- the `artifact_type` matches the expected value;
- generated artifacts include `source_files`;
- no section is empty (use explicit "None" or "No known blockers" instead of blank);
- the artifact is internally consistent (e.g., current epic matches across sections).

## Staleness

A context artifact is stale when:

- the active epic has changed since the artifact was generated;
- the active work packet has changed since the artifact was generated;
- more than 7 days have passed since generation without review;
- a commit has materially changed the project state since generation.

Generated artifacts should include enough metadata for staleness detection.

## File Size Guidance

| Artifact | Target Size |
| --- | --- |
| Current State | 200–500 words |
| Latest Handoff | 300–800 words |
| Context Pack | 1,000–3,000 words |
| Session Chronicle | 200–600 words |
| Decision Log | grows over time |
| Bootstrap Prompt | 500–1,500 words |
