---
title: "Handoff Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - context
  - handoff
  - standard
  - ai
related:
  - docs/08-context/CONTEXT-BRIDGE.md
  - docs/08-context/CONTEXT-PACK-STANDARD.md
  - docs/09-ai/FRESH-CHAT-HANDOFF.md
  - docs/09-ai/BOOTSTRAP-PROMPT.md
  - docs/07-workflow/CONTEXT-UPDATE-STANDARD.md
---

# Handoff Standard

## Purpose

This document defines the standard for Monad handoff artifacts.

A handoff artifact allows a future human or AI assistant to resume work without reconstructing project state from memory or scattered conversation history.

## Core Rule

A handoff must answer:

```text
Where are we?
What changed?
What is active?
What matters next?
What should not be redone?
What evidence exists?
```

## Handoff Types

Monad may use several types of handoff.

### Fresh Chat Handoff

Used when starting a new AI conversation.

Canonical path:

```text
docs/09-ai/FRESH-CHAT-HANDOFF.md
```

Generated or current path:

```text
.monad/context/latest-handoff.md
```

### Session Handoff

Used at the end of a work session.

Possible path:

```text
.monad/context/session-chronicles/YYYY-MM-DD-session.md
```

### Work Packet Handoff

Used when a work packet is paused, blocked, or completed.

Possible paths:

```text
work/packets/WP-E1-003.md
.monad/context/work-packet-handoffs/WP-E1-003.md
```

### Release Handoff

Used when preparing a release or milestone transition.

Possible path:

```text
.monad/context/release-handoffs/
```

## Required Handoff Sections

A handoff should include these sections:

```text
Project
Current Status
Active Epic
Active Work Packet
Recently Completed
Current Files of Interest
Decisions Already Made
Verification Status
Known Blockers
Next Recommended Action
Do Not Redo
Instructions for Next Assistant
```

## Section: Project

State the project name and product identity.

Example:

```text
Project: Monad
Product: AI-native, repo-native Software Foundry OS
```

## Section: Current Status

Summarize the state in a few paragraphs.

This should be direct and practical.

## Section: Active Epic

Identify the active epic.

Example:

```text
E0 — Project Foundation
```

## Section: Active Work Packet

Identify the active work packet or pre-work item.

Example:

```text
Documentation foundation before WP-E0-001
```

## Section: Recently Completed

List recent work completed in the current or previous session.

Include commits if known.

## Section: Current Files of Interest

List files the next session should inspect first.

Examples:

```text
docs/01-project/01-charter/PRODUCT-CHARTER.md
docs/05-architecture/SYSTEM-OVERVIEW.md
docs/07-workflow/WORK-PACKET-STANDARD.md
```

## Section: Decisions Already Made

List accepted decisions so the next session does not reopen them unnecessarily.

Examples:

- Monad is the unified product name.
- Rust is the core runtime language.
- The repository is the source of truth.
- Work packets are the primary delivery unit.

## Section: Verification Status

List verification commands run and results.

Example:

```text
python3 frontmatter check: passed
find docs -type f: passed
```

If verification has not been run, say so explicitly.

## Section: Known Blockers

List blockers clearly.

If there are no blockers, write:

```text
No known blockers.
```

## Section: Next Recommended Action

State the next action clearly.

Example:

```text
Continue filling the E0 foundation docs, then begin WP-E0-001 repository foundation.
```

## Section: Do Not Redo

List work that should not be repeated.

Examples:

- Do not rename Monad back to AionX, Foundry, or Charon.
- Do not switch from Rust core unless a new ADR supersedes ADR-0001.
- Do not introduce Bazel/Pants/Buck2/Nx as default dependencies.
- Do not start implementation before the current documentation slice is committed.

## Section: Instructions for Next Assistant

Include guidance for AI behavior.

Example:

```text
Operate as a principal-level software engineering partner.
Prefer forward progress.
Use repo files as source of truth.
Provide full file contents for implementation changes.
Use Rust Apprenticeship Mode for Rust code.
```

## Handoff Quality Checklist

A good handoff is:

- current;
- specific;
- short enough to read;
- complete enough to resume;
- clear about uncertainty;
- clear about next action;
- clear about what not to repeat;
- grounded in repository files;
- free of secrets.

## Handoff Update Triggers

Update a handoff when:

- the conversation is getting long;
- a work packet is completed;
- a commit changes project direction;
- the active work packet changes;
- a major decision is accepted;
- a blocker appears;
- a blocker is resolved;
- the next session would otherwise be confused.

## Generated Handoffs

When Monad later generates handoffs, generated handoffs should include:

- generation time;
- source files inspected;
- confidence or trust status;
- known omissions;
- whether the handoff has been human-reviewed.

Generated handoffs should not pretend to be accepted doctrine unless reviewed.

## Current Status

This handoff standard is a draft. It is authoritative enough for manual E0 handoffs and should guide the generated handoff work in E3.
