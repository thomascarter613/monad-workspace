---
title: "Context Bridge"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - context
  - context-bridge
  - handoff
  - ai
related:
  - docs/01-project/00-vision/PRODUCT-VISION.md
  - docs/05-architecture/CONTEXT-BRIDGE-ARCHITECTURE.md
  - docs/08-context/HANDOFF-STANDARD.md
  - docs/08-context/CONTEXT-PACK-STANDARD.md
  - docs/09-ai/BOOTSTRAP-PROMPT.md
  - docs/09-ai/FRESH-CHAT-HANDOFF.md
---

# Context Bridge

## Purpose

This document defines Monad’s Context Bridge concept.

The Context Bridge is Monad’s repo-native system for preserving project state, active work, architectural decisions, handoff information, and AI-readable continuity across sessions.

It replaces reliance on hidden chat memory with reviewable repository artifacts.

## Background

Monad absorbed earlier concepts formerly discussed as Charon, Context Bridge, repo-native memory, and AI handoff infrastructure.

Those concepts are now part of Monad.

The current name for this capability is:

```text
Context Bridge
```

## Core Problem

AI-assisted development sessions lose context.

Human developers also lose context when:

- a conversation gets too long;
- work spans multiple days;
- issues and docs drift;
- decisions remain in chat;
- implementation details are not committed;
- generated output is not recorded;
- the next session cannot tell what happened.

Monad solves this by treating context as project infrastructure.

## Context Bridge Thesis

A serious software repository should be able to explain its current state.

A future human or AI assistant should be able to enter the repository, read a small set of context files, and understand:

- what the project is;
- what is currently active;
- what was recently completed;
- what decisions govern the work;
- what files matter next;
- what verification has passed;
- what remains blocked or incomplete;
- what the next recommended action is.

## Source of Truth

The repository is the source of truth.

The Context Bridge may use generated artifacts, but durable truth should be grounded in repo files:

```text
docs/
work/
.monad/
Cargo.toml
monad.toml
native manifests
Git history
```

Chat history may help reconstruct context, but it is not the canonical source of truth.

## Human-Authored vs Generated Context

Monad must distinguish between:

```text
Human-authored canonical docs
Generated context artifacts
Temporary session notes
Accepted ADRs
Draft plans
Verification evidence
```

Generated context should not be treated as automatically accepted truth.

Generated context is useful, but it must identify its status and source.

## Context Artifact Locations

Recommended locations:

```text
docs/08-context/     Standards for context artifacts
docs/09-ai/          Human-authored AI collaboration docs and bootstrap prompts
work/                Epics, work packets, tasks, and delivery records
.monad/context/      Generated or Monad-maintained context artifacts
.monad/reports/      Generated reports and evidence packets
```

## Core Context Artifacts

### Current State

A current-state artifact summarizes the present project situation.

It should include:

- active epic;
- active work packet;
- recent completed work;
- next recommended action;
- known blockers;
- verification status;
- important files.

Possible paths:

```text
docs/09-ai/CURRENT-STATE.md
.monad/context/current-state.md
```

### Fresh Chat Handoff

A fresh-chat handoff helps a new AI session resume without relying on the prior conversation.

It should include:

- what project this is;
- where to read first;
- what has already been decided;
- what is active now;
- what should happen next;
- how the assistant should behave;
- what not to redo.

Possible paths:

```text
docs/09-ai/FRESH-CHAT-HANDOFF.md
.monad/context/latest-handoff.md
```

### Bootstrap Prompt

A bootstrap prompt is a ready-to-use instruction block for starting a new AI session.

It tells the assistant:

- the project name;
- the repository source-of-truth rule;
- which files to read first;
- the current workflow;
- the active work packet;
- response expectations.

Canonical path:

```text
docs/09-ai/BOOTSTRAP-PROMPT.md
```

### Context Pack

A context pack is a bundled summary of the most important project context.

It should be short enough to fit into a model context window but complete enough to orient a session.

Possible path:

```text
.monad/context/latest-context-pack.md
```

### Session Chronicle

A session chronicle records what happened during a work session.

It should include:

- start state;
- work performed;
- files changed;
- verification run;
- commit made;
- next step.

Possible path:

```text
.monad/context/session-chronicles/
```

### Decision Log

A decision log records decisions that are not yet formal ADRs.

Important decisions should eventually be promoted into ADRs or accepted docs.

Possible path:

```text
.monad/context/decision-log.md
```

## Context Trust Levels

Monad should distinguish trust levels:

| Trust Level | Meaning |
|---|---|
| Canonical | Accepted docs, accepted ADRs, committed source code. |
| Draft | Useful but not final guidance. |
| Generated | Produced by Monad or an assistant; review before relying on it. |
| Observed | Derived from current repo inspection. |
| Assumed | Inferred or proposed; must be verified. |
| Historical | Retained for reference, not necessarily current. |

## Context Bridge Workflow

Recommended workflow:

```text
Start session
  → Read bootstrap prompt
  → Read current state
  → Read active work packet
  → Read relevant docs/ADRs
  → Perform work
  → Verify work
  → Commit work
  → Update current state / handoff
  → End session
```

## Context Update Triggers

Update context when:

- a work packet is completed;
- an epic status changes;
- a major decision is made;
- a new ADR is accepted;
- implementation changes architecture;
- verification status changes;
- a blocker appears or is resolved;
- a new session handoff is needed.

## What Context Must Not Contain

Context artifacts should not contain:

- secrets;
- API keys;
- passwords;
- private credentials;
- unnecessary personal data;
- unreviewed claims presented as accepted truth;
- stale instructions that conflict with current docs;
- provider-specific assumptions unless clearly scoped.

## MVP Context Bridge

The MVP Context Bridge should support:

- manual current-state docs;
- manual fresh-chat handoff;
- bootstrap prompt;
- context artifact standards;
- later generated current-state artifact;
- later generated context pack;
- later context verification checks.

## Future Context Bridge

Later Monad may generate context by inspecting:

- docs;
- ADRs;
- Git history;
- GitHub issues;
- work packets;
- verification reports;
- source code;
- manifests;
- project graph;
- recent commits.

The long-term goal is for Monad to produce accurate, compact, reviewable context packs automatically.

## Current Status

This Context Bridge document is a draft. It is authoritative enough to guide initial context and AI handoff docs for E0, and it should be refined when E3 begins implementing context generation.
