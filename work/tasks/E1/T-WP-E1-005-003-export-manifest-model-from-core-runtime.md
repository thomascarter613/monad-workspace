---
title: "T-WP-E1-005-003 — Export Manifest Model from Core Runtime"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-005"
task: "T-WP-E1-005-003"
tags:

* task
* rust
* monad-core

---

# T-WP-E1-005-003 — Export Manifest Model from Core Runtime

## Product Area

Core Runtime

## Objective

Expose manifest model types from the `monad-core` library root.

## Parent Work Packet

WP-E1-005 — Establish Manifest Model Foundation

## Expected Result

Other crates can import manifest model types directly from `monad_core`.

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
