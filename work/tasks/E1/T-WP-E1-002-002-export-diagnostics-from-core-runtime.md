---
title: "T-WP-E1-002-002 — Export Diagnostics from Core Runtime"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-002"
task: "T-WP-E1-002-002"
tags:

* task
* rust
* monad-core

---

# T-WP-E1-002-002 — Export Diagnostics from Core Runtime

## Product Area

Core Runtime

## Objective

Expose diagnostics through `monad-core` and connect runtime identity to a startup diagnostic.

## Parent Work Packet

WP-E1-002 — Establish Core Diagnostics Foundation

## Expected Result

`monad-core` re-exports `Diagnostic`, `DiagnosticReport`, and `Severity`, and `RuntimeIdentity` can produce a startup diagnostic.

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
