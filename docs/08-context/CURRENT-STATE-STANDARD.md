---
title: "Current State Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-26
version: 0.2.0
tags:
  - monad
  - context
  - current-state
  - standard
  - ai
related:
  - docs/08-context/CONTEXT-ARTIFACT-SCHEMAS.md
  - docs/08-context/CONTEXT-BRIDGE.md
  - docs/08-context/HANDOFF-STANDARD.md
  - docs/09-ai/CURRENT-STATE.md
---

# Current State Standard

## Purpose

This document defines the standard for Monad current-state artifacts.

A current-state artifact is the simplest context artifact. It answers one question: "Where are we right now?"

## Core Rule

A current-state artifact must be:

```text
short enough to scan in seconds
specific enough to orient a new session
grounded in repository state
clear about what is active
```

## Canonical Location

Human-authored or reviewed:

```text
docs/09-ai/CURRENT-STATE.md
```

Generated:

```text
.monad/context/current-state.md
```

## Required Sections

### Project

State the project identity in one sentence.

```text
Monad is an AI-native, repo-native, local-first Software Foundry OS.
```

### Completed Epics

List completed epics with their titles.

```text
E0 — Project Foundation is complete.
E1 — Runtime Foundation is complete.
```

### Current Epic

State the active epic.

```text
E2 — Repository Intelligence Foundation
```

### Current Work Packet

State the active work packet.

```text
WP-E2-017 — Add Generated Context Artifact Policy Foundation
```

### Active Focus

Describe what the current work packet adds. List new types, modules, commands, or artifacts.

### Runtime Capabilities

Summarize what `monad-core` currently provides. This section grows as epics complete.

### Verification

Include commands to verify the current state and the expected result.

```bash
tools/scripts/verify.sh
```

Expected result:

```text
Verification baseline passed.
```

## Generation Strategy

When Monad generates a current-state artifact, it should:

1. Read `Cargo.toml` and `monad.toml` for project identity.
2. Read `work/epics/` to determine completed and active epics.
3. Read `work/packets/` to determine the active work packet.
4. Read `crates/monad-core/src/lib.rs` exports to determine runtime capabilities.
5. Include the standard verification commands.

## Size Guidance

A current-state artifact should be 200–500 words. If it exceeds 500 words, move detail into the context pack.

## Update Triggers

Regenerate or update the current state when:

- a work packet is completed;
- the active epic changes;
- a new module is added to `monad-core`;
- a new CLI command is added.
