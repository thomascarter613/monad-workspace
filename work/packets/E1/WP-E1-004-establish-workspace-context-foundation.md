---
title: "WP-E1-004 — Establish Workspace Context Foundation"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-004"
tags:
  - work-packet
  - rust
  - runtime
  - workspace
---

# WP-E1-004 — Establish Workspace Context Foundation

## Product Area

Core Runtime

## Objective

Create Monad's shared workspace context model so future runtime modules and CLI commands can reason about repository roots and canonical Monad paths consistently.

## Rationale

Monad operates on repositories. Future features such as manifest loading, repository inspection, context packs, verification, and safe file operations all need a reliable way to locate and describe the workspace root.

A shared `WorkspaceContext` prevents path logic from being duplicated across modules.

## Scope

This work packet covers:

- `WorkspaceContext`;
- workspace root discovery;
- canonical path helpers for `docs/`, `work/`, `.monad/`, `.monad/context/`, `monad.toml`, and `Cargo.toml`;
- workspace context exports from `monad-core`;
- tests;
- E1 planning, context, and verification updates.

## Deliverables

Expected deliverables include:

- `crates/monad-core/src/workspace.rs`;
- updated `crates/monad-core/src/lib.rs`;
- E1 task records;
- E1 deliverable records;
- updated E1 work packet index;
- updated context handoff files;
- updated verification baseline.

## Expected Result After Verification

`monad-core` exposes a tested workspace context model, workspace discovery works from nested paths, canonical Monad paths are generated consistently, and the full verification baseline passes.

## Verification

Run:

```bash
cargo fmt --check
cargo test
tools/scripts/verify.sh
````

Expected output includes:

```text
test result: ok
Verification baseline passed.
```

## Status

Complete

## Priority

High

## Size

M
