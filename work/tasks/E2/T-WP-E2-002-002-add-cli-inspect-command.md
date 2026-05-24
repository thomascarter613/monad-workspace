---
title: "T-WP-E2-002-002 — Add CLI Inspect Command"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-002"
task: "T-WP-E2-002-002"
---

# T-WP-E2-002-002 — Add CLI Inspect Command

## Product Area

CLI user experience and repository intelligence.

## Objective

Add `monad inspect` to the thin CLI entrypoint and route it through `monad-core`.

## Parent Work Packet

WP-E2-002 — Establish `monad inspect` Command Foundation.

## Expected Result

The CLI accepts:

- `monad inspect`
- `monad inspect --format text`
- `monad inspect --format json`
- `monad --format json inspect`

## Verification

Run:

- `cargo run --quiet -p monad-cli -- inspect`
- `cargo run --quiet -p monad-cli -- inspect --format=json`

Expected result:

- text output starts with `Monad repository inspection`;
- JSON output contains `"kind": "repository_inspection_summary"`.

## Status

Complete.

## Priority

High.

## Size

Small.
