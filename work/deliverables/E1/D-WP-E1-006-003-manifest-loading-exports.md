---
title: "D-WP-E1-006-003 — Manifest Loading Exports"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-006"
deliverable: "D-WP-E1-006-003"
tags:

* deliverable
* rust
* monad-core

---

# D-WP-E1-006-003 — Manifest Loading Exports

## Product Area

Core Runtime

## Objective

Expose manifest loading convenience behavior from `monad-core`.

## Source Work Packet

WP-E1-006 — Establish Manifest Loading Foundation

## Deliverable Type

Source Code

## Artifact Path

`crates/monad-core/src/lib.rs`

## Expected Result After Verification

`monad-core` exposes `load_manifest_from_workspace`.

## Verification

Run:

```bash
cargo test -p monad-core
tools/scripts/verify.sh
```

## Status

Complete
