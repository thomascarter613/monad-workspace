---
title: "Architecture Decision Records"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - architecture
  - adr
  - decisions
related:
  - docs/05-architecture/SYSTEM-OVERVIEW.md
  - docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
  - docs/05-architecture/MODULE-BOUNDARIES.md
  - docs/00-meta/DOCUMENTATION-STANDARD.md
---

# Architecture Decision Records

## Purpose

This directory contains Monad’s Architecture Decision Records.

ADRs record consequential decisions that affect Monad’s product direction, architecture, implementation strategy, workflow, safety model, or long-term maintainability.

ADR files are part of the canonical project record.

## Why ADRs Matter

Monad is intended to be a repo-native, AI-readable, verification-oriented software foundry.

That requires durable decision memory.

Without ADRs, important reasoning gets lost in:

- chat history;
- issue comments;
- private notes;
- implicit assumptions;
- stale docs;
- memory.

ADRs prevent that by recording:

- what decision was made;
- why it was made;
- what alternatives were considered;
- what consequences follow;
- when the decision may need review.

## ADR Naming

ADR files use this format:

```text
ADR-NNNN-short-kebab-case-title.md
````

Examples:

```text
ADR-0001-use-rust-for-core-runtime.md
ADR-0002-use-monad-as-unified-product-name.md
```

The ADR number is permanent once assigned.

Do not renumber ADRs after they are committed.

## ADR Statuses

ADR frontmatter uses the standard documentation statuses:

```text
stub
draft
review
accepted
superseded
archived
```

In the ADR body, also include a visible `Status` section.

Preferred ADR lifecycle:

```text
draft → review → accepted
accepted → superseded
draft → archived
```

## ADR Index

| ADR      | Title                                            | Status   | Summary                                                                       |
| -------- | ------------------------------------------------ | -------- | ----------------------------------------------------------------------------- |
| ADR-0000 | Template                                         | accepted | Defines the standard structure for Monad ADRs.                                |
| ADR-0001 | Use Rust for Core Runtime                        | accepted | Monad’s durable local runtime and CLI foundation will be implemented in Rust. |
| ADR-0002 | Use Monad as Unified Product Name                | accepted | Monad is the canonical umbrella/product name for the consolidated system.     |
| ADR-0003 | Use Repo-Native Context as Source of Truth       | stub     | Planned ADR for repository-owned context and handoff artifacts.               |
| ADR-0004 | Use Work Packets as Primary Delivery Unit        | stub     | Planned ADR for work packet-centered delivery.                                |
| ADR-0005 | Use Multi-Crate Rust Workspace                   | stub     | Planned ADR for initial Rust workspace structure.                             |
| ADR-0006 | Keep CLI Thin and Core Durable                   | stub     | Planned ADR for crate/module responsibility boundaries.                       |
| ADR-0007 | Use Supervised Autonomy for Agent Workflows      | stub     | Planned ADR for human-in-command agent workflows.                             |
| ADR-0008 | Coordinate Native Tools Rather Than Replace Them | stub     | Planned ADR for Monad’s native-tool coordination strategy.                    |

## When to Write an ADR

Write an ADR when a decision affects:

* core technology choices;
* crate boundaries;
* module boundaries;
* command architecture;
* file operation safety;
* command execution safety;
* AI/provider architecture;
* context model;
* verification model;
* evolution workflow;
* project workflow;
* public product identity;
* long-term maintainability.

Do not write an ADR for every small implementation detail.

## ADR Review Questions

Before accepting an ADR, ask:

* Is the decision stated clearly?
* Is the context accurate?
* Are the alternatives represented fairly?
* Are consequences explicit?
* Does the decision align with Monad’s product vision?
* Does the decision align with architecture principles?
* Does the decision protect safety and verification?
* Is the decision reversible or difficult to reverse?
* Does implementation need to change because of this ADR?
* Do related docs need to be updated?

## ADR Change Rule

Accepted ADRs should not be casually edited to change the decision.

Minor editorial fixes are acceptable.

Material changes should usually be handled by:

1. writing a new ADR;
2. marking the old ADR as `superseded`;
3. linking the old and new ADRs.

## Current Status

This ADR index is a draft. It is sufficient to guide initial Monad work and should be updated whenever ADRs are added, accepted, or superseded.
