---
title: "T-WP-E1-009-002 — Integrate Contract Checks with Monad Check"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-009"
task: "T-WP-E1-009-002"
tags:

* task
* rust
* checks
* cli

---

# T-WP-E1-009-002 — Integrate Contract Checks with Monad Check

## Product Area

CLI Experience

## Objective

Wire repository-contract checks into `run_workspace_checks` so `monad check` reports contract diagnostics.

## Parent Work Packet

WP-E1-009 — Establish Repository Contract Check Foundation

## Expected Result

`monad check` emits repository-contract diagnostics, including `MONAD4500` for satisfied contract paths.

## Verification

Run:

```bash
cargo run --quiet -p monad-cli -- check
tools/scripts/verify.sh
```

## Status

Complete

## Priority

High

## Size

S
