---
title: "Documentation Map"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - documentation
  - meta
  - map
related:
  - docs/00-meta/DOCUMENTATION-STANDARD.md
  - docs/00-meta/FRONTMATTER-STANDARD.md
  - docs/07-workflow/OPERATING-MODEL.md
---

# Documentation Map

## Purpose

This document defines the canonical map of Monad’s documentation system.

Monad is intended to be a repo-native, AI-readable, verification-oriented software foundry. Its documentation must therefore be organized enough for humans, maintainers, contributors, and AI assistants to understand the project without relying on scattered chat history.

The documentation tree is not decorative. It is part of the product architecture.

## Documentation Principles

Monad documentation follows these principles:

1. **The repository is the source of truth.**
2. **Important decisions belong in versioned Markdown files.**
3. **Architecture must be documented before it is enforced.**
4. **Workflow must be explicit enough to survive handoff.**
5. **AI-readable context is a first-class project artifact.**
6. **Stubs are acceptable when they mark a clear future home for knowledge.**
7. **Accepted decisions must be promoted into ADRs or canonical docs.**
8. **Generated context must be distinguishable from human-authored doctrine.**
9. **Docs should help implementation, not delay it indefinitely.**
10. **Every important document should be reviewable in Git.**

## Canonical Documentation Areas

### `docs/00-meta/`

Defines the documentation system itself.

This area answers:

- How are docs organized?
- What frontmatter is required?
- What document statuses exist?
- What naming conventions do we use?
- How should future docs be added?

### `docs/01-project/`

Defines the project-level identity of Monad.

This area contains:

- vision;
- charter;
- thesis;
- strategy;
- roadmap;
- project glossary.

### `docs/02-product/`

Defines Monad as a product.

This area contains:

- problem statement;
- target users;
- personas;
- value proposition;
- use cases;
- MVP scope;
- non-goals;
- success metrics;
- competitive landscape.

### `docs/03-requirements/`

Defines what Monad must do and how success is evaluated.

This area contains:

- functional requirements;
- nonfunctional requirements;
- system qualities;
- acceptance criteria standards;
- requirements traceability;
- MVP requirements;
- future requirements.

### `docs/04-domain/`

Defines Monad’s core domain language and conceptual model.

This area contains:

- domain model;
- bounded contexts;
- ubiquitous language;
- domain events;
- invariants;
- conceptual model.

### `docs/05-architecture/`

Defines Monad’s technical architecture.

This area contains:

- system overview;
- architecture principles;
- module boundaries;
- runtime architecture;
- data flow;
- control flow;
- workspace model;
- project graph model;
- context bridge architecture;
- verification architecture;
- evolution engine architecture;
- agent supervision architecture;
- worktree safety strategy;
- MCP integration strategy.

### `docs/06-adrs/`

Contains Architecture Decision Records.

This area records consequential decisions that affect:

- technology choices;
- module boundaries;
- workflow commitments;
- safety rules;
- architecture direction;
- long-term maintainability.

ADRs are not casual notes. They are canonical decision records.

### `docs/07-workflow/`

Defines how Monad work is planned, executed, verified, reviewed, and committed.

This area contains:

- operating model;
- work hierarchy;
- epic standard;
- work packet standard;
- task standard;
- deliverable standard;
- Definition of Ready;
- Definition of Done;
- verification standard;
- commit standard;
- branching standard;
- review standard;
- release standard;
- context update standard.

### `docs/08-context/`

Defines Monad’s context bridge.

This area contains standards for:

- current-state files;
- handoffs;
- session chronicles;
- context packs;
- rehydration;
- decision logs;
- generated context.

This area is the documentation side of the former Charon concept, now absorbed into Monad.

### `docs/09-ai/`

Defines how humans and AI assistants collaborate on Monad.

This area contains:

- bootstrap prompt;
- current state;
- fresh-chat handoff;
- AI collaboration rules;
- agent runbook;
- agent safety rules;
- prompting standard;
- model provider standard;
- MCP tooling standard.

### `docs/10-engineering/`

Defines engineering standards for Monad implementation.

This area contains:

- coding standards;
- Rust coding standards;
- Rust learning notes;
- Rust verification;
- error handling;
- diagnostics;
- testing;
- fixtures;
- output formats;
- CLI UX;
- dependency standards.

### `docs/11-security/`

Defines Monad’s security and safety model.

This area contains:

- security model;
- threat model;
- secret handling;
- sandboxing principles;
- command execution safety;
- file operation safety;
- agent safety model;
- MCP safety boundaries;
- supply-chain security;
- responsible disclosure.

### `docs/12-verification/`

Defines Monad’s verification model.

This area contains:

- verification model;
- check registry standard;
- evidence packet standard;
- reporting standard;
- exit-code standard;
- test matrix;
- quality gates.

### `docs/13-operations/`

Defines how Monad is set up, maintained, released, and supported.

This area contains:

- local development;
- repository setup;
- toolchain setup;
- release process;
- versioning policy;
- support model;
- maintenance model;
- backups and exports.

### `docs/14-integrations/`

Defines Monad’s integration strategy.

This area contains:

- GitHub integration;
- GitHub Projects workflow;
- GitHub Issues workflow;
- MCP integration;
- model-provider integrations;
- native tool adapters.

### `docs/15-business/`

Defines Monad’s commercial and validation thinking.

This area contains:

- business thesis;
- repo audit offer;
- pricing hypotheses;
- customer segments;
- distribution strategy;
- validation plan;
- risks.

### `docs/16-reference/`

Contains reference material.

This area contains:

- command catalog;
- configuration reference;
- `monad.toml` reference;
- terminology;
- FAQ;
- resources.

## Documentation Maturity Model

Monad documents move through the following maturity levels:

| Status | Meaning |
|---|---|
| `stub` | File exists as a planned knowledge location, but content is incomplete. |
| `draft` | Content is meaningfully written but still expected to change. |
| `review` | Content is ready for focused review before acceptance. |
| `accepted` | Content is canonical unless later superseded. |
| `superseded` | Content has been replaced by a newer document or ADR. |
| `archived` | Content is retained for history but no longer active guidance. |

## Canonical Source Rule

The repository owns canonical project truth.

External systems such as GitHub Projects, GitHub Issues, AppFlowy, Linear, or other planning tools may support the workflow, but binding product, architecture, workflow, implementation, and context decisions must be promoted into the repository.

## Documentation Before Implementation

Monad should be documentation-first but not documentation-paralyzed.

That means:

- create the map before filling every detail;
- write foundation docs before dependent implementation;
- create stubs for future areas;
- promote important decisions into ADRs;
- update docs when implementation teaches us something important;
- avoid pretending unresolved design is final.

## Required Practice

When adding or updating documentation:

1. Use YAML frontmatter.
2. Put the document in the correct numbered docs area.
3. Prefer precise, actionable wording.
4. Link related docs when known.
5. Update status as the document matures.
6. Keep docs useful for both humans and AI assistants.
7. Promote durable decisions into ADRs.
8. Avoid making canonical claims without evidence or decision history.

## Current Status

This map is a draft. It is authoritative enough to guide the initial repository setup, but it may be refined as Monad implementation begins.
