---
title: "Monad Documentation"
status: approved
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - documentation
  - index
related:
  - docs/00-meta/DOCUMENTATION-MAP.md
  - docs/00-meta/DOCUMENTATION-STANDARD.md
  - docs/09-ai/BOOTSTRAP-PROMPT.md
  - docs/09-ai/FRESH-CHAT-HANDOFF.md
---

# Monad Documentation

## Purpose

This directory contains Monad’s canonical project documentation.

Monad is a repo-native project. Durable product, architecture, workflow, requirements, context, implementation, security, verification, and operational knowledge belongs in the repository.

## Documentation Architecture

Monad documentation is organized into numbered areas so humans and AI assistants can read the project in a stable order.

```text
00-meta/          Documentation standards and metadata rules
01-project/       Vision, charter, strategy, roadmap, and project glossary
02-product/       Product problem, users, value, scope, and positioning
03-requirements/  Functional, nonfunctional, and MVP requirements
04-domain/        Domain model, bounded contexts, invariants, and language
05-architecture/  System design, module boundaries, and architecture models
06-adrs/          Architecture Decision Records
07-workflow/      Operating model, work hierarchy, and delivery standards
08-context/       Context bridge standards and handoff models
09-ai/            AI collaboration, bootstrap, and handoff rules
10-engineering/   Coding, Rust, diagnostics, testing, and CLI standards
11-security/      Safety, threat model, command execution, and file operations
12-verification/  Checks, evidence, reports, exit codes, and quality gates
13-operations/    Local setup, toolchains, releases, and maintenance
14-integrations/  GitHub, MCP, model providers, and native tool adapters
15-business/      Business thesis, repo audit offer, and validation strategy
16-reference/     Command catalog, configuration reference, terminology, FAQ
```

## Recommended Reading Order

For a new session, read:

```text
docs/09-ai/BOOTSTRAP-PROMPT.md
docs/09-ai/FRESH-CHAT-HANDOFF.md
docs/01-project/01-charter/PRODUCT-CHARTER.md
docs/01-project/00-vision/PRODUCT-VISION.md
docs/02-product/MVP-SCOPE.md
docs/05-architecture/SYSTEM-OVERVIEW.md
docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
docs/05-architecture/MODULE-BOUNDARIES.md
docs/06-adrs/README.md
docs/07-workflow/OPERATING-MODEL.md
docs/07-workflow/WORK-PACKET-STANDARD.md
docs/08-context/CONTEXT-BRIDGE.md
```

## Canonical Rules

* Every Markdown document in `docs/` should have YAML frontmatter.
* Durable decisions should be promoted into accepted docs or ADRs.
* Generated context must be distinguishable from accepted doctrine.
* Documentation should guide implementation, not prevent it.
* When implementation changes behavior, related docs should be updated.

## Documentation Rule

Every canonical Markdown document under `docs/` should have YAML frontmatter.

See:

```text
docs/00-meta/FRONTMATTER-STANDARD.md
```

## Current Status

The documentation tree is established and the critical pre-implementation foundation docs are being drafted during E0.
