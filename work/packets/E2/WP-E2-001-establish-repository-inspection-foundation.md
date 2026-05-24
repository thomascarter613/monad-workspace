---
title: "WP-E2-001 — Establish Repository Inspection Foundation"
document_type: "work-packet"
status: "ready"
version: "0.1.0"
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

This work packet should cover:

* a `repository` or `inspection` module in `monad-core`;
* typed repository inspection results;
* top-level file and directory inventory;
* ignored/generated-directory safeguards;
* tests using temporary repositories;
* integration path for future `monad inspect`;
* context and verification updates.

## Deliverables

Expected deliverables include:

* a new `monad-core` repository inspection module;
* exported inspection types;
* tests;
* updated runtime context;
* updated E2 task and deliverable records;
* verification baseline updates if needed.

## Expected Result After Verification

Monad has a typed repository inspection foundation that can list and classify initial workspace structure, and the verification baseline passes.

## Verification

Run:

```bash
cargo fmt --check
cargo test
tools/scripts/verify.sh
```

Expected output includes:

```text
Verification baseline passed.
```

## Status

Ready

## Priority

High

## Size

M
