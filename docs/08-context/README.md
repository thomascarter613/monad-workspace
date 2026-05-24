---
title: "Context Bridge"
status: approved
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:

* monad
* context
* handoff
  related:
* docs/08-context/CONTEXT-BRIDGE.md
* docs/08-context/HANDOFF-STANDARD.md
* docs/08-context/CONTEXT-PACK-STANDARD.md
* docs/09-ai/BOOTSTRAP-PROMPT.md

---

# Context Bridge

## Purpose

This directory defines Monad’s Context Bridge standards.

The Context Bridge preserves current state, handoffs, context packs, session chronicles, and AI-readable continuity in repository artifacts.

## Belongs Here

* Context bridge concept.
* Context artifact schemas.
* Current-state standard.
* Handoff standard.
* Session chronicle standard.
* Context pack standard.
* Rehydration standard.
* Decision log standard.
* Generated context standard.

## Does Not Belong Here

* Generated context files themselves unless intentionally canonical.
* AI prompting rules that belong in `docs/09-ai/`.
* General product strategy.

## Start Here

```text
CONTEXT-BRIDGE.md
HANDOFF-STANDARD.md
CONTEXT-PACK-STANDARD.md
```