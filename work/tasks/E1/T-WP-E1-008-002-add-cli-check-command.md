---
title: "T-WP-E1-008-002 — Add CLI Check Command"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-008"
task: "T-WP-E1-008-002"
tags:

* task
* rust
* cli
* checks

---

# T-WP-E1-008-002 — Add CLI Check Command

## Product Area

CLI Experience

## Objective

Add `monad check` to the CLI.

## Parent Work Packet

WP-E1-008 — Establish CLI Check Command Foundation

## Expected Result

`monad check` runs workspace checks, prints structured diagnostics, and exits successfully when no error diagnostics exist.

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
