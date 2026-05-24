---
title: "T-WP-E1-008-001 — Add Core Workspace Checks"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-008"
task: "T-WP-E1-008-001"
tags:
  - task
  - rust
  - checks
---

# T-WP-E1-008-001 — Add Core Workspace Checks

## Product Area

Core Runtime

## Objective

Add `monad-core` workspace checks that produce structured diagnostics.

## Parent Work Packet

WP-E1-008 — Establish CLI Check Command Foundation

## Expected Result

`monad-core` exposes `run_workspace_checks`, and tests verify successful and failing workspace reports.

## Verification

Run:

```bash
cargo test -p monad-core
tools/scripts/verify.sh
````

## Status

Complete

## Priority

High

## Size

S
