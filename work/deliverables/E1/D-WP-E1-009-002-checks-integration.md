---
title: "D-WP-E1-009-002 — Checks Integration"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-009"
deliverable: "D-WP-E1-009-002"
tags:

* deliverable
* rust
* checks

---

# D-WP-E1-009-002 — Checks Integration

## Product Area

Core Runtime

## Objective

Integrate repository-contract diagnostics into `run_workspace_checks`.

## Source Work Packet

WP-E1-009 — Establish Repository Contract Check Foundation

## Deliverable Type

Source Code

## Artifact Path

`crates/monad-core/src/checks.rs`

## Expected Result After Verification

`run_workspace_checks` includes repository-contract diagnostics.

## Verification

Run:

```bash
cargo test -p monad-core
cargo run --quiet -p monad-cli -- check
tools/scripts/verify.sh
```

## Status

Complete
