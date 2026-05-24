---
title: "WP-E1-006 — Establish Manifest Loading Foundation"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-006"
tags:
  - work-packet
  - rust
  - runtime
  - manifest
  - loading
---

# WP-E1-006 — Establish Manifest Loading Foundation

## Product Area

Core Runtime

## Objective

Add TOML parsing and file loading for Monad's root `monad.toml` manifest.

## Rationale

WP-E1-005 established the manifest data model. The next runtime step is to load that model from repository state.

Manifest loading lets future commands inspect repository intent from `monad.toml` instead of relying only on hardcoded defaults.

## Scope

This work packet covers:

- adding `serde` and `toml` dependencies to `monad-core`;
- parsing manifest TOML text;
- loading a manifest from a file path;
- loading a manifest from `WorkspaceContext`;
- validating loaded manifests;
- tests for valid TOML, invalid TOML, unsupported schema, missing files, and workspace-based loading;
- E1 planning, context, and verification updates.

## Deliverables

Expected deliverables include:

- updated `crates/monad-core/Cargo.toml`;
- updated `crates/monad-core/src/manifest.rs`;
- updated `crates/monad-core/src/lib.rs`;
- updated `Cargo.lock`;
- E1 task records;
- E1 deliverable records;
- updated E1 work packet index;
- updated context handoff files;
- updated verification baseline.

## Expected Result After Verification

`monad-core` can parse and validate `monad.toml`, load it from a path, load it from a workspace context, and the full verification baseline passes.

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
