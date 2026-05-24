---
title: "Work Hierarchy"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - workflow
  - hierarchy
related:
  - docs/07-workflow/OPERATING-MODEL.md
  - docs/07-workflow/WORK-PACKET-STANDARD.md
  - docs/07-workflow/TASK-STANDARD.md
  - docs/07-workflow/DEFINITION-OF-READY.md
  - docs/07-workflow/DEFINITION-OF-DONE.md
---

# Work Hierarchy

## Purpose

This document defines Monad’s work hierarchy.

The hierarchy exists so large product goals can be broken into reviewable, verifiable, atomic units of progress.

## Canonical Hierarchy

Monad uses this hierarchy:

```text
Program
  → Product Area
  → Epic
  → Capability
  → Work Packet
  → Task
  → Deliverable
  → Verification Evidence
  → Atomic Commit
````

## Program

The program is the overall Monad initiative.

Current program:

```text
Monad — AI-native, repo-native Software Foundry OS
```

The program contains the complete product vision.

## Product Area

A product area is a major capability area.

Canonical product areas:

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

Product Area should appear before Objective in work packet records.

## Epic

An epic is a large outcome composed of multiple work packets.

Initial epics:

```text
E0 — Project Foundation
E1 — Rust Core Foundation
E2 — Repo Intelligence
E3 — Context Bridge
E4 — Verification Engine
E5 — Evolution Engine
E6 — Agent Supervision
```

An epic should define:

* product area;
* objective;
* user value;
* scope;
* expected work packets;
* deliverables;
* verification strategy;
* risks;
* priority;
* size.

## Capability

A capability is a coherent functional area inside an epic.

Examples:

```text
Workspace resolution
Toolchain detection
Context generation
Evidence packet reporting
Dry-run file planning
Model provider abstraction
```

Capabilities may be documented explicitly when helpful, but they do not always need separate GitHub issues.

## Work Packet

A work packet is the primary delivery unit.

A work packet is:

* bounded;
* actionable;
* verifiable;
* commit-sized or a small sequence of commits;
* clear enough to start;
* small enough to review.

Example:

```text
WP-E1-003 — Add core error and diagnostic model
```

## Task

A task is a checklist item inside a work packet.

Tasks should be concrete actions.

Examples:

```text
Create diagnostics module.
Define DiagnosticSeverity enum.
Add tests for warning diagnostics.
Export diagnostics module from monad-core.
Run cargo test.
```

Tasks normally do not need separate issues unless they become large, blocked, or independently trackable.

## Deliverable

A deliverable is a concrete output of a work packet.

Examples:

```text
A Markdown standard document.
A Rust module.
A test file.
A CLI command.
A generated report.
A verification script.
```

Deliverables should be inspectable.

## Verification Evidence

Verification evidence proves whether the deliverable works.

Examples:

```text
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- --help
python3 scripts/check-doc-frontmatter.py
find docs -type f | sort
```

Evidence may be command output, generated reports, passing tests, or documented manual review.

## Atomic Commit

An atomic commit records one coherent unit of change.

Each completed work packet should end with a recommended Conventional Commit.

Examples:

```text
docs(workflow): define operating model and work hierarchy
feat(core): add diagnostic model
feat(cli): add initial command shell
```

## ID Standards

### Epic IDs

```text
E0
E1
E2
```

### Work Packet IDs

```text
WP-E0-001
WP-E1-003
WP-E5-006
```

### Task IDs

Use task IDs only when needed:

```text
TASK-WP-E1-003-001
```

Most tasks can remain checklist items.

## GitHub Issue Mapping

Recommended mapping:

| Work Object   | GitHub Issue? | Notes                                      |
| ------------- | ------------: | ------------------------------------------ |
| Epic          |           Yes | Parent issue or issue with sub-issues.     |
| Work Packet   |           Yes | Primary tracked execution unit.            |
| Task          |    Usually no | Checklist inside work packet unless large. |
| Bug           |           Yes | Independent issue.                         |
| Research      |           Yes | When investigation is needed.              |
| ADR Candidate |           Yes | Before accepted ADR exists.                |

## Project Field Mapping

Recommended GitHub Project fields:

| Field          | Purpose                                                      |
| -------------- | ------------------------------------------------------------ |
| Status         | Inbox, Ready, Active, Blocked, Review, Done, Deferred.       |
| Type           | Epic, Work Packet, Task, Bug, Research, ADR Candidate, Risk. |
| Product Area   | Major product/work area.                                     |
| Priority       | P0, P1, P2, P3.                                              |
| Size           | XS, S, M, L, XL, Unknown.                                    |
| Epic           | Parent epic ID.                                              |
| Work Packet ID | Work packet identifier.                                      |

## Priority Values

```text
P0 — Critical
P1 — High
P2 — Normal
P3 — Low
```

Priority describes importance and sequencing urgency.

## Size Values

```text
XS — tiny, obvious change
S — small focused task
M — normal work packet
L — large or caution-worthy
XL — should probably be split
Unknown — needs discovery
```

Size describes effort and complexity, not importance.

Priority and Size should appear at the end of work packet records.

## Current Status

This work hierarchy is a draft. It is authoritative enough for initial GitHub planning and E0/E1 implementation.
