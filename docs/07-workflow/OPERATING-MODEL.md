---
title: "Operating Model"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - workflow
  - operating-model
related:
  - docs/07-workflow/WORK-HIERARCHY.md
  - docs/07-workflow/WORK-PACKET-STANDARD.md
  - docs/07-workflow/DEFINITION-OF-READY.md
  - docs/07-workflow/DEFINITION-OF-DONE.md
  - docs/01-project/01-charter/PRODUCT-CHARTER.md
---

# Operating Model

## Purpose

This document defines how Monad work is planned, executed, verified, committed, and handed off.

Monad is intended to be repo-native, AI-readable, verification-oriented, and built through small reviewable slices. The operating model exists to keep work disciplined without creating unnecessary bureaucracy.

## Core Workflow

Monad work proceeds through this chain:

```text
Vision
  → Charter
  → Requirements
  → Architecture
  → ADRs
  → Domain Model
  → Epics
  → Work Packets
  → Tasks
  → Implementation
  → Verification
  → Evidence
  → Atomic Commit
  → Context Update
  → Next Work Packet
````

This workflow keeps product intent, architecture, implementation, verification, and context aligned.

## Source of Truth

The repository is the canonical source of durable truth.

GitHub Issues and GitHub Projects support execution, but durable decisions must be promoted into repository files.

Canonical project truth may live in:

```text
docs/
work/
.monad/
Cargo.toml
monad.toml
native manifests
```

External systems may support:

```text
GitHub Issues
GitHub Projects
AppFlowy
```

External systems should not become the only place where binding project decisions live.

## Primary Delivery Unit

The primary delivery unit is the **work packet**.

A work packet is larger than a task but smaller than an epic. It is the smallest meaningful unit of planned, verifiable, commit-ready project progress.

A good work packet includes:

* product area;
* objective;
* user value;
* scope;
* tasks;
* deliverables;
* verification commands;
* expected result after verification;
* Definition of Done;
* recommended Conventional Commit;
* priority;
* size.

## Default Work Style

Monad should be built in small, high-confidence slices.

Each slice should:

1. have a clear objective;
2. affect a bounded set of files;
3. include full file contents when code is provided manually;
4. include verification commands;
5. include expected verification results;
6. end with an atomic Conventional Commit;
7. update context when needed.

## Active Work Limit

Monad should avoid too many active work packets at once.

Recommended default:

```text
One active work packet at a time.
```

A second active item is acceptable only when it unblocks or supports the first.

## Planning System

GitHub Issues are used for:

* epics;
* work packets;
* tasks;
* bugs;
* research;
* ADR candidates;
* risks.

GitHub Projects are used for:

* status;
* priority;
* size;
* product area;
* current focus;
* planning views.

The repository remains the durable source of truth for accepted standards, architecture, context, and decisions.

## Status Flow

Default work status flow:

```text
Inbox → Ready → Active → Review → Done
```

Blocked flow:

```text
Active → Blocked → Active
```

Deferred flow:

```text
Inbox → Deferred
Ready → Deferred
Deferred → Ready
```

## Definition of Ready

A work packet should not become `Ready` until it has:

* product area;
* objective;
* user value;
* scope;
* deliverables;
* verification commands;
* expected result after verification;
* priority;
* size.

## Definition of Done

A work packet is not done merely because files were edited.

A work packet is done when:

* scope is complete;
* verification passes;
* expected result is achieved;
* docs are updated if needed;
* context is updated if needed;
* changes are committed atomically;
* no known blocker remains.

## Verification First Culture

Monad should treat verification as a first-class part of work.

Verification may include:

* formatting;
* tests;
* linting;
* typechecking;
* command output;
* file existence checks;
* generated reports;
* frontmatter checks;
* context checks;
* manual review.

Implementation work should not be declared complete without evidence.

## Commit Model

Monad uses atomic Conventional Commits.

Examples:

```text
docs(meta): define documentation map and standard
docs(product): define problem users value and mvp scope
docs(architecture): define system overview principles and boundaries
feat(cli): add initial command shell
feat(core): add workspace context resolver
```

Each commit should represent one coherent change.

## Context Update Rule

When a work packet changes project state, update context.

Context updates may include:

```text
docs/09-ai/CURRENT-STATE.md
docs/09-ai/FRESH-CHAT-HANDOFF.md
.monad/context/current-state.md
.monad/context/latest-handoff.md
```

Until generation exists, context updates may be manual.

## AI Collaboration Rule

AI assistants may help plan, write, review, and explain changes, but the repository remains the source of truth.

AI-generated content must be reviewed before it is treated as accepted project guidance.

For code, Monad uses Rust Apprenticeship Mode:

* small slices;
* full file contents;
* clear comments;
* tests;
* verification commands;
* expected results;
* atomic commits.

## Review Rule

Before a work packet is marked Done, review:

* actual changed files;
* verification evidence;
* docs impact;
* context impact;
* commit atomicity;
* whether scope was exceeded.

## Current Status

This operating model is a draft. It is authoritative enough to guide initial Monad development and should be refined as the repo workflow becomes real.
