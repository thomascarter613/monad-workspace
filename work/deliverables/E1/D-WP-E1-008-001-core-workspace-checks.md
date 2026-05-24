---
title: "D-WP-E1-008-001 — Core Workspace Checks"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-008"
deliverable: "D-WP-E1-008-001"
tags:
  - deliverable
  - rust
  - checks
---

# D-WP-E1-008-001 — Core Workspace Checks

## Product Area

Core Runtime

## Objective

Add the first reusable workspace check primitive to `monad-core`.

## Source Work Packet

WP-E1-008 — Establish CLI Check Command Foundation

## Deliverable Type

Source Code

## Artifact Path

`crates/monad-core/src/checks.rs`

## Expected Result After Verification

`run_workspace_checks` returns structured diagnostics for initial workspace and manifest checks.

## Verification

Run:

```bash
cargo test -p monad-core
tools/scripts/verify.sh
````

## Status

Complete
