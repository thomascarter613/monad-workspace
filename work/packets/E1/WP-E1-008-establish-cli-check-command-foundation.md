---
title: "WP-E1-008 — Establish CLI Check Command Foundation"
document_type: "work-packet"
status: "in-progress"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-008"
tags:
  - work-packet
  - rust
  - cli
  - checks
  - diagnostics
---

# WP-E1-008 — Establish CLI Check Command Foundation

## Product Area

CLI Experience

## Objective

Add the first diagnostics-producing Monad CLI command, `monad check`, backed by `monad-core` workspace checks.

## Rationale

Monad now has diagnostics, core errors, workspace context, manifest loading, and `monad info`. The next step is a command that checks repository state and reports structured diagnostics.

This creates the foundation for future repository contract checks, manifest validation checks, architecture boundary checks, and policy checks.

## Scope

This work packet covers:

- `monad-core` workspace check runtime primitive;
- `monad check`;
- CLI outcome handling for diagnostics with failure states;
- CLI check unit tests;
- CLI check smoke test in `tools/scripts/verify.sh`;
- E1 planning, context, and verification updates.

## Deliverables

Expected deliverables include:

- `crates/monad-core/src/checks.rs`;
- updated `crates/monad-core/src/lib.rs`;
- updated `crates/monad-cli/src/main.rs`;
- updated `tools/scripts/verify.sh`;
- updated E1 task records;
- updated E1 deliverable records;
- updated E1 work packet index;
- updated context handoff files;
- updated verification baseline.

## Expected Result After Verification

`monad check` discovers the current workspace, validates initial workspace and manifest expectations, prints structured diagnostics, exits successfully when no error diagnostics exist, and the full verification baseline passes.

## Verification

Run:

```bash
cargo fmt --check
cargo test
cargo run --quiet -p monad-cli -- check
tools/scripts/verify.sh
````

Expected CLI output includes:

```text
[INFO] MONAD4000
[INFO] MONAD4001
[INFO] MONAD4002
[INFO] MONAD4003
[INFO] MONAD4004
```

## Status

In Progress

## Priority

High

## Size

M
