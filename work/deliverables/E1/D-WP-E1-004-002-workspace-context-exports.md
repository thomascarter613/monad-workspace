---
title: "D-WP-E1-004-002 — Workspace Context Exports"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-004"
deliverable: "D-WP-E1-004-002"
tags:

* deliverable
* rust
* monad-core

---

# D-WP-E1-004-002 — Workspace Context Exports

## Product Area

Core Runtime

## Objective

Expose workspace context types and helpers from the `monad-core` library root.

## Source Work Packet

WP-E1-004 — Establish Workspace Context Foundation

## Deliverable Type

Source Code

## Artifact Path

`crates/monad-core/src/lib.rs`

## Expected Result After Verification

`monad-core` exports the workspace context model and tests pass.

## Verification

Run:

```bash
cargo test -p monad-core
tools/scripts/verify.sh
```

## Status

Complete
