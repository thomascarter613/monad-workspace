---
title: "WP-E1-005 — Establish Manifest Model Foundation"
document_type: "work-packet"
status: "in-progress"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-005"
tags:
  - work-packet
  - rust
  - runtime
  - manifest
---

# WP-E1-005 — Establish Manifest Model Foundation

## Product Area

Core Runtime

## Objective

Create Monad's initial manifest model so `monad.toml` has a durable in-memory representation before file loading and TOML parsing are introduced.

## Rationale

Monad needs a canonical repo-native intent file. The runtime should first define the manifest shape and validation rules before adding parsing dependencies or file I/O behavior.

This keeps the model stable, testable, and independent from parser implementation details.

## Scope

This work packet covers:

- root `monad.toml`;
- `ManifestSchemaVersion`;
- `ManifestProject`;
- `ManifestWorkspace`;
- `ManifestRuntime`;
- `MonadManifest`;
- manifest diagnostics;
- manifest validation;
- runtime identity integration;
- tests;
- E1 planning, context, and verification updates.

## Deliverables

Expected deliverables include:

- `monad.toml`;
- `crates/monad-core/src/manifest.rs`;
- updated `crates/monad-core/src/lib.rs`;
- E1 task records;
- E1 deliverable records;
- updated E1 work packet index;
- updated context handoff files;
- updated verification baseline.

## Expected Result After Verification

`monad-core` exposes a tested manifest model, root `monad.toml` exists, manifest validation works without TOML parsing dependencies, and the full verification baseline passes.

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
