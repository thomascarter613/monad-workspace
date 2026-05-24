---
title: "WP-E1-009 — Establish Repository Contract Check Foundation"
document_type: "work-packet"
status: "in-progress"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-009"
tags:
  - work-packet
  - rust
  - repository-contract
  - checks
  - diagnostics
---

# WP-E1-009 — Establish Repository Contract Check Foundation

## Product Area

Core Runtime

## Objective

Add the first repository-contract check foundation so Monad can verify canonical workspace paths as structured diagnostics.

## Rationale

Monad's purpose includes making repositories understandable, verifiable, and safe to evolve. That requires a durable repository contract: a machine-checkable statement of what important files and directories are expected to exist.

This slice begins with the canonical paths established by E0 and early E1.

## Scope

This work packet covers:

- `RequiredPathKind`;
- `RequiredPath`;
- `RepositoryContract`;
- `check_repository_contract`;
- integration into `run_workspace_checks`;
- test workspace updates;
- E1 planning, context, and verification updates.

## Deliverables

Expected deliverables include:

- `crates/monad-core/src/repo_contract.rs`;
- updated `crates/monad-core/src/checks.rs`;
- updated `crates/monad-core/src/lib.rs`;
- updated `crates/monad-cli/src/main.rs`;
- updated E1 task records;
- updated E1 deliverable records;
- updated E1 work packet index;
- updated context handoff files;
- updated verification baseline.

## Expected Result After Verification

`monad check` includes repository-contract diagnostics, missing canonical paths are reported as structured errors, valid Monad workspaces pass the repository-contract foundation, and the full verification baseline passes.

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
[INFO] MONAD4500
```

## Status

In Progress

## Priority

High

## Size

M
