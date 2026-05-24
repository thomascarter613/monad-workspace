---
title: "T-WP-E1-001-002 — Add Minimal Core Runtime Identity"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-001"
task: "T-WP-E1-001-002"
tags:

* task
* rust
* monad-core

---

# T-WP-E1-001-002 — Add Minimal Core Runtime Identity

## Product Area

Core Runtime

## Objective

Add a minimal `monad-core` runtime identity API that can be called by the CLI and verified with tests.

## Parent Work Packet

WP-E1-001 — Establish Rust Workspace Runtime Foundation

## Expected Result

`monad-core` exposes `runtime_identity()` and tests verify the product name, runtime crate, execution model, and banner.

## Verification

Run:

```bash
cargo test -p monad-core
tools/scripts/verify.sh
```

## Status

Complete

## Priority

High

## Size

S
