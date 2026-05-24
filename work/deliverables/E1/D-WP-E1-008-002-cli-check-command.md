---
title: "D-WP-E1-008-002 — CLI Check Command"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-008"
deliverable: "D-WP-E1-008-002"
tags:

* deliverable
* cli
* rust

---

# D-WP-E1-008-002 — CLI Check Command

## Product Area

CLI Experience

## Objective

Add `monad check` to the CLI.

## Source Work Packet

WP-E1-008 — Establish CLI Check Command Foundation

## Deliverable Type

Source Code

## Artifact Path

`crates/monad-cli/src/main.rs`

## Expected Result After Verification

`monad check` runs workspace checks and prints structured diagnostics.

## Verification

Run:

```bash
cargo run --quiet -p monad-cli -- check
tools/scripts/verify.sh
```

## Status

Complete
