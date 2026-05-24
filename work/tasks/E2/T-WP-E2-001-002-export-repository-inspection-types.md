---
title: "T-WP-E2-001-002 — Export Repository Inspection Types"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E2"
work_packet: "WP-E2-001"
task: "T-WP-E2-001-002"
tags:

* task
* rust
* monad-core

---

# T-WP-E2-001-002 — Export Repository Inspection Types

## Product Area

Core Runtime

## Objective

Export repository inspection types from the `monad-core` library root.

## Parent Work Packet

WP-E2-001 — Establish Repository Inspection Foundation

## Expected Result

Other crates can import repository inspection primitives directly from `monad_core`.

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
