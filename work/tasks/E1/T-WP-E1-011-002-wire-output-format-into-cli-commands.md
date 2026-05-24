---
title: "T-WP-E1-011-002 — Wire Output Format into CLI Commands"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-011"
task: "T-WP-E1-011-002"
tags:

* task
* rust
* cli
* output

---

# T-WP-E1-011-002 — Wire Output Format into CLI Commands

## Product Area

CLI Experience

## Objective

Pass parsed `OutputFormat` values into `monad info` and `monad check`.

## Parent Work Packet

WP-E1-011 — Establish CLI Output Format Argument Foundation

## Expected Result

`monad info --format text` and `monad check --format=text` continue to render text output through `monad-core`.

## Verification

Run:

```bash id="nzwxwm"
cargo run --quiet -p monad-cli -- info --format text
cargo run --quiet -p monad-cli -- check --format=text
tools/scripts/verify.sh
```

## Status

Complete

## Priority

High

## Size

S
