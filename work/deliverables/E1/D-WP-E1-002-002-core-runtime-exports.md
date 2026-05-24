---
title: "D-WP-E1-002-002 — Core Runtime Exports"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-002"
deliverable: "D-WP-E1-002-002"
tags:

* deliverable
* rust
* monad-core

---

# D-WP-E1-002-002 — Core Runtime Exports

## Product Area

Core Runtime

## Objective

Expose diagnostics from the `monad-core` library root.

## Source Work Packet

WP-E1-002 — Establish Core Diagnostics Foundation

## Deliverable Type

Source Code

## Artifact Path

`crates/monad-core/src/lib.rs`

## Expected Result After Verification

`monad-core` exports diagnostic types and runtime identity can create a startup diagnostic.

## Verification

Run:

```bash
cargo test -p monad-core
tools/scripts/verify.sh
```

## Status

Complete
