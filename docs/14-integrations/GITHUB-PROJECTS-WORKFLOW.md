---
title: "GitHub Projects Workflow"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - integrations
  - github
  - projects
  - workflow
related:
  - docs/07-workflow/OPERATING-MODEL.md
  - docs/07-workflow/WORK-HIERARCHY.md
  - docs/14-integrations/GITHUB-ISSUES-WORKFLOW.md
---

# GitHub Projects Workflow

## Purpose

This document defines how Monad uses GitHub Projects.

GitHub Projects provide planning views, status tracking, prioritization, and execution visibility.

GitHub Projects do not replace repository documentation.

## Core Rule

GitHub Projects show the state of work.

The repository records durable truth.

## Recommended Project Purpose

The Monad GitHub Project should answer:

- What is active?
- What is ready?
- What is blocked?
- What is deferred?
- Which epic does this work belong to?
- Which product area does it affect?
- What priority and size does it have?
- What is the next practical work packet?

## Recommended Fields

### Status

Allowed values:

```text
Inbox
Ready
Active
Blocked
Review
Done
Deferred
```

### Type

Allowed values:

```text
Epic
Work Packet
Task
Bug
Research
ADR Candidate
Risk
```

### Product Area

Allowed values:

```text
Core Runtime
CLI
Repo Intelligence
Context Bridge
Verification
Evolution Engine
Agent Supervision
Policy / Governance
Documentation
Workflow
Business / Monetization
Integrations
Security
Operations
Reference
```

### Priority

Allowed values:

```text
P0 Critical
P1 High
P2 Normal
P3 Low
```

### Size

Allowed values:

```text
XS
S
M
L
XL
Unknown
```

### Epic

The parent epic ID.

Examples:

```text
E0
E1
E2
```

### Work Packet ID

The work packet ID.

Examples:

```text
WP-E0-001
WP-E1-003
WP-E5-006
```

## Status Definitions

### Inbox

Captured but not triaged.

### Ready

Clear enough to start.

### Active

Currently being worked.

Monad should normally have only one active work packet.

### Blocked

Cannot continue until a blocker is resolved.

### Review

Work is substantially complete and awaiting verification, review, acceptance, or closeout.

### Done

Complete, verified, committed, and accepted.

### Deferred

Valid work, intentionally postponed.

## Recommended Views

### Current Focus

Shows:

```text
Status = Active OR Review OR Blocked
```

Purpose:

- identify immediate work;
- avoid scattered attention;
- keep active work visible.

### Ready Work

Shows:

```text
Status = Ready
```

Purpose:

- identify next work;
- choose the next work packet without re-planning.

### Epic Roadmap

Grouped by Epic.

Purpose:

- see all work under each epic;
- confirm roadmap coverage.

### Product Area View

Grouped by Product Area.

Purpose:

- see balance across architecture, core, verification, context, and agents.

### Deferred / Later

Shows:

```text
Status = Deferred
```

Purpose:

- retain ideas without distracting active work.

## Recommended Initial Statuses

After seeding epics and work packets:

```text
E0 — Active
WP-E0-001 — Active or Ready
E0 remaining work packets — Ready
E1 — Ready
E2 through E6 — Deferred
```

As documentation foundation work proceeds, status may be adjusted.

## Project Maintenance Rule

When an issue changes state, update the project status.

When a durable decision appears in an issue, promote it into repo docs or ADRs.

When a work packet is completed, update:

- issue status;
- project status;
- related docs if needed;
- context handoff if needed;
- commit reference if known.

## GitHub CLI and Projects

The GitHub CLI may add issues to a project.

Example:

```bash
gh project item-add PROJECT_NUMBER \
  --owner OWNER \
  --url ISSUE_URL
```

Project field setting may require additional manual or scripted steps.

If project commands fail, refresh project scope:

```bash
gh auth refresh -s project
```

## Avoiding Project Overhead

GitHub Projects should not become a bureaucratic burden.

The minimum useful practice is:

- one project board;
- clear statuses;
- product area;
- priority;
- size;
- epic;
- work packet ID;
- one active work packet.

## Current Status

This GitHub Projects workflow is a draft. It is authoritative enough for early Monad planning and should be refined after the project board is used in real work.
