---
title: "Naming Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - documentation
  - naming
  - standard
related:
  - docs/00-meta/DOCUMENTATION-STANDARD.md
  - docs/00-meta/DOCUMENTATION-MAP.md
  - docs/07-workflow/COMMIT-STANDARD.md
---

# Naming Standard

## Purpose

This document defines naming conventions for Monad documentation, directories, identifiers, epics, work packets, tasks, ADRs, commits, and future implementation artifacts.

Naming matters because Monad is intended to be repo-native, AI-readable, and verification-oriented. Consistent names make the project easier to search, automate, review, and continue across sessions.

## Product Name

The canonical product/project name is:

```text
Monad
```

Use **Monad** as the unified project name.

Former concept names such as AionX, Foundry, and Charon are absorbed into Monad as internal capability concepts. They should not be used as competing product names.

## Repository Name

Preferred repository name:

```text
monad
```

If a namespace or organization requires disambiguation, use:

```text
monad-devtool
monad-foundry
monad-runtime
```

But the preferred name remains `monad`.

## Documentation Directory Names

Use numbered top-level documentation directories:

```text
docs/00-meta/
docs/01-project/
docs/02-product/
docs/03-requirements/
docs/04-domain/
docs/05-architecture/
docs/06-adrs/
docs/07-workflow/
docs/08-context/
docs/09-ai/
docs/10-engineering/
docs/11-security/
docs/12-verification/
docs/13-operations/
docs/14-integrations/
docs/15-business/
docs/16-reference/
```

The numbering preserves reading order and makes AI/context bootstrapping easier.

## Documentation File Names

Use uppercase kebab-style topic names for canonical documents:

```text
PRODUCT-VISION.md
MVP-SCOPE.md
SYSTEM-OVERVIEW.md
WORK-PACKET-STANDARD.md
```

Use `README.md` for directory overviews.

Avoid vague names such as:

```text
notes.md
misc.md
stuff.md
todo.md
ideas.md
```

If a document is temporary, place it in the appropriate workflow or context area and name it clearly.

## Markdown Headings

The first heading should match the document title.

Example:

```markdown
---
title: "Product Vision"
---

# Product Vision
```

Use sentence-style headings unless a proper noun or identifier is required.

## ADR Names

Architecture Decision Records use this format:

```text
ADR-NNNN-short-kebab-case-title.md
```

Examples:

```text
ADR-0001-use-rust-for-core-runtime.md
ADR-0002-use-monad-as-unified-product-name.md
ADR-0003-use-repo-native-context-as-source-of-truth.md
```

ADR titles should state the decision, not merely the topic.

Good:

```text
Use Rust for Core Runtime
```

Weak:

```text
Rust Discussion
```

## Epic IDs

Epics use this format:

```text
E0
E1
E2
```

Epic issue titles use:

```text
[Epic]: E1 — Rust Core Foundation
```

Use an em dash between the ID and title.

## Work Packet IDs

Work packets use this format:

```text
WP-E<epic-number>-NNN
```

Examples:

```text
WP-E0-001
WP-E1-003
WP-E4-006
```

Work packet issue titles use:

```text
[Work Packet]: WP-E1-003 — Add core error and diagnostic model
```

## Task IDs

Tasks may be lightweight checklist items inside work packets.

If task IDs are needed, use:

```text
TASK-WP-E1-003-001
TASK-WP-E1-003-002
```

Do not create task IDs unless they provide real tracking value.

## Product Areas

Use these canonical product area names:

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

Use the exact names in GitHub Projects where possible.

## GitHub Labels

Use colon-delimited labels:

```text
type:epic
type:work-packet
type:task
type:bug
type:research
type:adr
type:risk

area:core
area:cli
area:repo-intelligence
area:context-bridge
area:verification
area:evolution
area:agents
area:docs
area:workflow
area:business

priority:p0
priority:p1
priority:p2
priority:p3
```

Labels classify issues. Project fields track status and planning metadata.

## Status Names

Use these project statuses:

```text
Inbox
Ready
Active
Blocked
Review
Done
Deferred
```

Do not invent extra statuses unless the workflow standard is updated.

## Size Names

Use these size values:

```text
XS
S
M
L
XL
Unknown
```

Size describes effort/complexity, not importance.

## Commit Names

Use Conventional Commits.

Examples:

```text
docs(project): scaffold monad documentation tree
docs(meta): define documentation map and standard
chore(repo): establish monad repository foundation
feat(cli): add initial command shell
feat(core): add workspace context resolver
```

## Rust Crate Names

Use lowercase kebab-case for package names in `Cargo.toml`.

Preferred crate/package names:

```text
monad-cli
monad-core
monad-mcp
```

Use Rust module names in snake_case:

```text
workspace_context
diagnostic_report
file_ops
```

## CLI Command Names

Use lowercase kebab-case for multi-word commands and subcommands where needed.

Examples:

```text
monad inspect
monad check
monad graph
monad context generate
monad evolve verify-baseline
monad evolve context-baseline
```

Prefer clear words over abbreviations.

## Generated Files

Generated files should be named clearly and should usually live under `.monad/` unless they are intended to become canonical docs.

Examples:

```text
.monad/context/current-state.md
.monad/context/latest-handoff.md
.monad/context/latest-context-pack.md
.monad/reports/latest-verification.md
```

Generated files should identify themselves as generated or reviewable where appropriate.

## Avoided Names

Avoid using these as competing top-level product names:

```text
AionX
Foundry
Charon
Mnemosyne
Argos
Anamnesis
Themis
Hephaestus
Athena
Clio
Daedalus
```

These may appear as internal concepts, historical references, or capability inspirations, but **Monad** is the unified product name.

## Current Status

This naming standard is a draft. It is authoritative enough for the initial repository and documentation foundation.
