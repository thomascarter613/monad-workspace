---
title: "Monad Decision Log"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
generated: false
reviewed: true
source: "manual-bootstrap"
tags:
  - monad
  - context
  - decisions
related:
  - docs/06-adrs/README.md
  - docs/08-context/DECISION-LOG-STANDARD.md
---

# Monad Decision Log

## Purpose

This file records useful project decisions that may not yet require a full ADR or that summarize already accepted ADRs for handoff purposes.

Consequential architecture decisions should still be promoted into `docs/06-adrs/`.

## Decision Status Values

```text
noted
candidate
accepted
superseded
rejected
```

## Decisions

### DEC-0001 — Monad is the unified product name

Status: accepted

Source:

```text
docs/06-adrs/ADR-0002-use-monad-as-unified-product-name.md
```

Summary:

Monad is the canonical product name. Prior concepts such as AionX, Foundry, Charon, Context Bridge, repo-native memory, and supervised execution are absorbed into Monad.

### DEC-0002 — Rust is the core runtime language

Status: accepted

Source:

```text
docs/06-adrs/ADR-0001-use-rust-for-core-runtime.md
```

Summary:

Monad’s durable local runtime will be implemented in Rust.

### DEC-0003 — Repository is source of truth

Status: accepted

Source:

```text
docs/08-context/CONTEXT-BRIDGE.md
docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
```

Summary:

Durable project truth belongs in repository files. Chat history is not canonical.

### DEC-0004 — Work packets are primary delivery units

Status: accepted

Source:

```text
docs/07-workflow/WORK-PACKET-STANDARD.md
```

Summary:

Work packets are the primary unit for planned, verifiable, commit-ready work.

### DEC-0005 — Work packet field order

Status: accepted

Source:

```text
docs/07-workflow/WORK-PACKET-STANDARD.md
```

Summary:

Work packets include Product Area before Objective, Expected Result After Verification, and Priority and Size at the end.

### DEC-0006 — Prefer python3 in commands

Status: accepted

Source:

```text
docs/13-operations/TOOLCHAIN-SETUP.md
```

Summary:

Use `python3` rather than `python` in project commands and walkthroughs.

### DEC-0007 — Do not use Bazel, Pants, Buck2, or Nx as default dependencies

Status: accepted

Source:

```text
docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
docs/10-engineering/RUST-CODING-STANDARD.md
```

Summary:

Monad may learn from or inspect repositories using these tools, but they are not default Monad dependencies.

### DEC-0008 — Coordinate native tools rather than replacing them unnecessarily

Status: accepted

Source:

```text
docs/14-integrations/NATIVE-TOOL-ADAPTERS.md
docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
```

Summary:

Monad should coordinate native ecosystem tools such as Cargo, Bun, Go tooling, and Python tooling rather than reimplementing them prematurely.

### DEC-0009 — AI output is proposed, not verified

Status: accepted

Source:

```text
docs/09-ai/AI-COLLABORATION-RULES.md
docs/12-verification/VERIFICATION-MODEL.md
```

Summary:

AI may propose, draft, explain, and summarize, but repository acceptance and verification determine truth.

## Promotion Rule

If a decision affects architecture, safety, product identity, runtime strategy, workflow, or long-term maintainability, promote it into an ADR.
