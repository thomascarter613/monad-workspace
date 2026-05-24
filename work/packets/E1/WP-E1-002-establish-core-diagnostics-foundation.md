---
title: "WP-E1-002 — Establish Core Diagnostics Foundation"
document_type: "work-packet"
status: "in-progress"
version: "0.1.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-002"
tags:
  - work-packet
  - rust
  - runtime
  - diagnostics
---

# WP-E1-002 — Establish Core Diagnostics Foundation

## Product Area

Core Runtime

## Objective

Create the first reusable diagnostics model in `monad-core` so future Monad commands can report information, warnings, and errors consistently.

## Rationale

Monad will inspect repositories, validate manifests, enforce policies, and report findings. Those operations need structured diagnostics instead of ad hoc strings.

Adding diagnostics early keeps the CLI thin and gives future runtime modules a shared reporting vocabulary.

## Scope

This work packet covers:

- `Severity`;
- `Diagnostic`;
- `DiagnosticReport`;
- diagnostic rendering;
- diagnostic tests;
- runtime identity integration through `startup_diagnostic`;
- E1 planning, context, and verification updates.

## Deliverables

Expected deliverables include:

- `crates/monad-core/src/diagnostics.rs`;
- updated `crates/monad-core/src/lib.rs`;
- E1 task records;
- E1 deliverable records;
- updated E1 work packet index;
- updated context handoff files;
- updated verification baseline.

## Expected Result After Verification

`monad-core` exposes a tested diagnostics model, `RuntimeIdentity` can produce a startup diagnostic, and the full verification baseline passes.

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

In Progress

## Priority

High

## Size

M
