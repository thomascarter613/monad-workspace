---
title: "T-WP-E1-007-002 — Add CLI Info Rendering"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-007"
task: "T-WP-E1-007-002"
tags:

* task
* rust
* cli
* manifest

---

# T-WP-E1-007-002 — Add CLI Info Rendering

## Product Area

CLI Experience

## Objective

Render workspace and manifest information through `monad info`.

## Parent Work Packet

WP-E1-007 — Establish CLI Info Command Foundation

## Expected Result

`monad info` discovers the workspace, loads `monad.toml`, and prints project/runtime information.

## Verification

Run:

```bash
cargo run --quiet -p monad-cli -- info
tools/scripts/verify.sh
```

## Status

Complete

## Priority

High

## Size

S
