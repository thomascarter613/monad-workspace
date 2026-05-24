---
title: "Monad Documentation"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - documentation
related:
  - docs/00-meta/DOCUMENTATION-MAP.md
  - docs/00-meta/DOCUMENTATION-STANDARD.md
  - docs/09-ai/BOOTSTRAP-PROMPT.md
---

# Monad Documentation

## Purpose

This directory contains Monad’s canonical project documentation.

Monad is a repo-native project. Durable product, architecture, workflow, requirements, context, implementation, security, and verification knowledge belongs in the repository.

## Documentation Areas

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

## Start Here

Recommended reading order for new sessions:

```text
docs/09-ai/BOOTSTRAP-PROMPT.md
docs/09-ai/FRESH-CHAT-HANDOFF.md
docs/01-project/01-charter/PRODUCT-CHARTER.md
docs/01-project/00-vision/PRODUCT-VISION.md
docs/02-product/MVP-SCOPE.md
docs/05-architecture/SYSTEM-OVERVIEW.md
docs/06-adrs/README.md
docs/07-workflow/OPERATING-MODEL.md
```

## Documentation Rule

Every canonical Markdown document under `docs/` should have YAML frontmatter.

See:

```text
docs/00-meta/FRONTMATTER-STANDARD.md
```

## Current Status

The documentation tree is established and the critical pre-implementation foundation docs are being drafted during E0.
