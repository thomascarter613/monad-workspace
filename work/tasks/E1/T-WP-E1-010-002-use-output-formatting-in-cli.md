---
title: "T-WP-E1-010-002 — Use Output Formatting in CLI"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-010"
task: "T-WP-E1-010-002"
tags:

* task
* rust
* cli
* output

---

# T-WP-E1-010-002 — Use Output Formatting in CLI

## Product Area

CLI Experience

## Objective

Update `monad-cli` so `monad info` and `monad check` use output rendering from `monad-core`.

## Parent Work Packet

WP-E1-010 — Establish Runtime Output Formatting Foundation

## Expected Result

The CLI still renders the same information, but shared output formatting is owned by `monad-core`.

## Verification

Run:

```bash
cargo run --quiet -p monad-cli -- info
cargo run --quiet -p monad-cli -- check
tools/scripts/verify.sh
```

## Status

Complete

## Priority

High

## Size

S
