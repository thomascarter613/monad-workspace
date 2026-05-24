---
title: "D-WP-E2-001-002 — Repository Inspection Exports"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E2"
work_packet: "WP-E2-001"
deliverable: "D-WP-E2-001-002"
tags:

* deliverable
* rust
* monad-core

---

# D-WP-E2-001-002 — Repository Inspection Exports

## Product Area

Core Runtime

## Objective

Export repository inspection primitives from `monad-core`.

## Source Work Packet

WP-E2-001 — Establish Repository Inspection Foundation

## Deliverable Type

Source Code

## Artifact Path

`crates/monad-core/src/lib.rs`

## Expected Result After Verification

Repository inspection types are available from the `monad_core` crate root.

## Verification

Run:

```bash
cargo test -p monad-core
tools/scripts/verify.sh
```

## Status

Complete
