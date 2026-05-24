---
title: "WP-E2-001 — Establish Repository Inspection Foundation"
document_type: "work-packet"
status: "in-progress"
version: "0.2.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E2"
work_packet: "WP-E2-001"
tags:

* work-packet
* repository-intelligence
* inspection
* rust

---

# WP-E2-001 — Establish Repository Inspection Foundation

## Product Area

Repository Intelligence

## Objective

Add the first repository inspection model and runtime behavior so Monad can summarize top-level workspace structure.

## Rationale

E1 established runtime primitives and initial checks. E2 begins the repository intelligence layer by creating a typed inspection model that can later support richer analysis, graphing, language detection, policy checks, and AI-readable repository context.

## Scope

This work packet covers:

* a new `repository_inspection` module in `monad-core`;
* typed repository inspection results;
* top-level file and directory inventory;
* important entry role classification;
* generated/external traversal safeguards;
* deterministic sorting;
* tests using temporary repositories;
* integration with `monad check` through a summary diagnostic;
* context and verification updates.

## Deliverables

Expected deliverables include:

* `crates/monad-core/src/repository_inspection.rs`;
* updated `crates/monad-core/src/lib.rs`;
* updated `crates/monad-core/src/checks.rs`;
* E2 task records;
* E2 deliverable records;
* updated runtime context;
* updated verification baseline.

## Expected Result After Verification

Monad has a typed repository inspection foundation that can list and classify initial workspace structure, `monad check` emits a repository-inspection diagnostic, and the verification baseline passes.

## Verification

Run:

```bash
cargo fmt --check
cargo test
cargo run --quiet -p monad-cli -- check
tools/scripts/verify.sh
```

Expected output includes:

```text
[INFO] MONAD4600
Verification baseline passed.
```

## Status

In Progress

## Priority

High

## Size

M
