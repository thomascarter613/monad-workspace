---
title: "Product Charter"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - project
  - charter
  - product
related:
  - docs/01-project/00-vision/PRODUCT-VISION.md
  - docs/01-project/00-vision/HOLY-GRAIL-VISION.md
  - docs/02-product/MVP-SCOPE.md
  - docs/02-product/NON-GOALS.md
  - docs/06-adrs/ADR-0001-use-rust-for-core-runtime.md
  - docs/06-adrs/ADR-0002-use-monad-as-unified-product-name.md
---

# Product Charter

## Purpose

This charter defines Monad’s initial product identity, mission, scope, constraints, and operating commitments.

It is the first canonical project charter for the unified Monad product.

## Product Name

The canonical product name is:

```text
Monad
```

Monad absorbs the former concepts discussed as AionX, Foundry, Charon, and related named subsystems.

Those concepts may survive as internal capability inspirations, but they are not separate products for this project.

## Product Definition

Monad is an AI-native, repo-native, local-first Software Foundry OS for understanding, verifying, and safely evolving software repositories.

Monad begins as a Rust-based developer tool and should grow into a broader system that supports repo intelligence, context preservation, verification, safe evolution, and supervised AI-assisted development.

## Mission

Monad’s mission is to help developers and teams understand, verify, and safely evolve software repositories without losing context, trust, or control.

## Problem Statement

Software projects become difficult to change because knowledge is scattered across code, docs, issues, chats, CI logs, local conventions, and individual memory.

AI tools can generate code, but generated code is often difficult to trust when the tool lacks project context, verification evidence, architectural awareness, or safe execution boundaries.

Monad exists to close that gap.

## Product Goals

Monad should help users:

1. understand unfamiliar repositories quickly;
2. preserve context across sessions;
3. coordinate native tools;
4. verify work with evidence;
5. plan safe repository improvements;
6. make repo changes reviewable;
7. keep architecture, docs, checks, and implementation aligned;
8. use AI assistance without surrendering control;
9. reduce repeated setup and repo-hardening work;
10. leave repositories better than they were found.

## Initial Product Areas

Monad is organized around these product areas:

| Product Area | Purpose |
|---|---|
| Core Runtime | Durable local Rust foundation and shared logic. |
| CLI | User-facing command-line interface. |
| Repo Intelligence | Repository inspection, tool detection, workspace discovery, and project graphing. |
| Context Bridge | Repo-native current state, handoff, bootstrap, and context pack artifacts. |
| Verification | Checks, evidence packets, report output, and quality gates. |
| Evolution Engine | Safe file operations, templates, dry-run planning, and baseline improvements. |
| Agent Supervision | Human-in-command AI planning, drafting, approval, and audit workflows. |
| Policy / Governance | Rules, approvals, safety boundaries, and later enterprise governance. |
| Integrations | GitHub, MCP, model providers, native tools, and future external systems. |
| Documentation | Canonical project knowledge and AI-readable repo memory. |
| Business / Monetization | Open-core strategy, repo audits, pricing, distribution, and validation. |

## Initial Technical Direction

Monad should be built with:

- Rust for the durable local core and CLI;
- a multi-crate Rust workspace;
- small, teachable implementation slices;
- strong tests and verification;
- repo-native Markdown documentation;
- GitHub Issues and Projects for execution tracking;
- AppFlowy or similar private workspace only for non-canonical planning;
- provider-agnostic AI architecture;
- native tool coordination rather than native tool replacement.

## Core Architectural Commitments

### Rust core

Monad’s durable local runtime should be implemented in Rust.

Rationale:

- local-first developer tooling benefits from fast startup and reliable binaries;
- file operations and command execution are trust-critical;
- Rust supports safe, explicit, high-confidence systems programming;
- the project should grow around a durable core rather than a later rewrite.

### Thin CLI

The CLI should be a user interface over `monad-core`.

Durable logic belongs in core modules, not buried in command handlers.

### Repo-native context

Monad should treat repository files as the canonical source of durable project truth.

Context files, ADRs, work packets, docs, manifests, and reports should be reviewable and versioned.

### Native tool coordination

Monad should coordinate native ecosystem tools rather than attempting to replace every tool’s internal implementation.

### Supervised autonomy

AI-assisted work should remain human-in-command.

Planning, drafting, verification, and review may be assisted, but risky actions require explicit approval.

## MVP Scope

The MVP should prove Monad can:

- run as a Rust CLI;
- maintain a clean core/CLI boundary;
- inspect a repository;
- detect basic toolchains and manifests;
- generate context artifacts;
- run local checks;
- produce evidence;
- plan safe file operations;
- support dry-run evolution workflows;
- document and preserve current project state.

## Explicit Non-Goals for Early Work

Monad should not initially attempt to be:

- a full IDE;
- a full CI/CD platform;
- a fully autonomous coding agent;
- a project-management SaaS;
- a hosted-only tool;
- a package manager replacement;
- a cloud agent platform;
- an enterprise compliance product;
- a plugin marketplace;
- a billing platform;
- a replacement for all native developer tools.

## Operating Model

Monad work should proceed through:

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
```

The primary delivery unit is the work packet.

Each meaningful work packet should include:

- product area;
- objective;
- user value;
- scope;
- tasks;
- deliverables;
- verification commands;
- expected result after verification;
- Definition of Done;
- recommended Conventional Commit;
- priority;
- size.

## Rust Apprenticeship Mode

Because the initial maintainer is learning Rust while building Monad, Rust implementation must follow Rust Apprenticeship Mode.

This means:

- small slices;
- complete file contents;
- clear comments;
- tests with each slice;
- beginner-readable code;
- explanation of introduced Rust concepts;
- verification commands;
- expected results;
- atomic commits.

Production quality and teachability are both requirements.

## Source of Truth

The repository is the canonical source of truth.

External systems may support work:

- GitHub Issues for epics, work packets, tasks, bugs, research, and ADR candidates;
- GitHub Projects for status and planning views;
- AppFlowy for private notes and research;
- future tools only when they improve flow without creating source-of-truth confusion.

Binding decisions must be promoted into repository docs or ADRs.

## Success Criteria

Monad is succeeding when:

- a new session can understand the project from repo files;
- a new contributor can understand the repo structure;
- the CLI can inspect and explain repositories;
- verification output is clear and evidence-backed;
- context handoff works without hidden chat memory;
- work is sliced into reviewable commits;
- architecture decisions are recorded;
- users can trust Monad because it shows its work.

## Current Risks

- Scope expansion could slow MVP delivery.
- Documentation could become too heavy before implementation.
- Rust learning curve could slow progress.
- AI-agent ambition could distract from local core value.
- Tooling integrations could create complexity before the core is proven.
- The project must avoid becoming a loose collection of ideas instead of a coherent product.

## Current Status

This charter is a draft. It is authoritative enough to guide E0 and E1 work, but it should be reviewed and refined after the initial Rust core foundation is implemented.
