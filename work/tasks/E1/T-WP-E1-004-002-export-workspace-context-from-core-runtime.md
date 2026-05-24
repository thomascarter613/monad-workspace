---
title: "T-WP-E1-004-002 — Export Workspace Context from Core Runtime"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-004"
task: "T-WP-E1-004-002"
tags:

* task
* rust
* monad-core

---

# T-WP-E1-004-002 — Export Workspace Context from Core Runtime

## Product Area

Core Runtime

## Objective

Expose `WorkspaceContext`, `discover_workspace_root`, and `is_workspace_root` from the `monad-core` library root.

## Parent Work Packet

WP-E1-004 — Establish Workspace Context Foundation

## Expected Result

Other crates can import workspace context types and helpers directly from `monad_core`.

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
