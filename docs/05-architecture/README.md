---
title: "Architecture"
status: approved
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:

* monad
* architecture
  related:
* docs/05-architecture/SYSTEM-OVERVIEW.md
* docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
* docs/05-architecture/MODULE-BOUNDARIES.md
* docs/06-adrs/README.md
---

# Architecture

## Purpose

This directory defines Monad’s architecture.

It explains system boundaries, runtime shape, module responsibilities, safety strategies, and major architectural models.

## Belongs Here

* System overview.
* Architecture principles.
* Module boundaries.
* Runtime architecture.
* Data flow.
* Control flow.
* Workspace model.
* Project graph model.
* Context bridge architecture.
* Verification architecture.
* Evolution engine architecture.
* Agent supervision architecture.
* Worktree safety strategy.
* MCP integration strategy.

## Does Not Belong Here

* ADR index details.
* Product personas.
* Work packet issue bodies.
* Generated context packs.

## Start Here

```text
SYSTEM-OVERVIEW.md
ARCHITECTURE-PRINCIPLES.md
MODULE-BOUNDARIES.md
```

## Decision Rule

Consequential architecture decisions should be recorded in `docs/06-adrs/`.