---
title: "WP-E1-007 — Establish CLI Info Command Foundation"
document_type: "work-packet"
status: "in-progress"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-007"
tags:
  - work-packet
  - rust
  - cli
  - manifest
  - workspace
---

# WP-E1-007 — Establish CLI Info Command Foundation

## Product Area

CLI Experience

## Objective

Add the first useful Monad CLI command, `monad info`, backed by workspace discovery and loaded `monad.toml` manifest state.

## Rationale

Monad now has workspace context and manifest loading in `monad-core`. The CLI should begin using those runtime primitives rather than only printing a static startup banner.

This creates a thin CLI path that proves the intended architecture:

- CLI parses a command;
- CLI delegates durable behavior to `monad-core`;
- core discovers workspace state;
- core loads repository intent from `monad.toml`;
- CLI renders the result.

## Scope

This work packet covers:

- manual early command parsing;
- `monad help`;
- `monad info`;
- workspace discovery from the current directory;
- manifest loading through `WorkspaceContext`;
- CLI tests;
- CLI smoke test in `tools/scripts/verify.sh`;
- E1 planning, context, and verification updates.

## Deliverables

Expected deliverables include:

- updated `crates/monad-cli/src/main.rs`;
- updated `tools/scripts/verify.sh`;
- updated E1 task records;
- updated E1 deliverable records;
- updated E1 work packet index;
- updated context handoff files;
- updated verification baseline.

## Expected Result After Verification

`monad info` discovers the current workspace, loads `monad.toml`, prints project/runtime information, and the full verification baseline passes.

## Verification

Run:

```bash
cargo fmt --check
cargo test
cargo run --quiet -p monad-cli -- info
tools/scripts/verify.sh
````

Expected CLI output includes:

```text
Monad workspace
  project: Monad (monad)
  schema_version: 1
  core_crate: monad-core
  cli_crate: monad-cli
  execution_model: local-first
```

## Status

In Progress

## Priority

High

## Size

M
