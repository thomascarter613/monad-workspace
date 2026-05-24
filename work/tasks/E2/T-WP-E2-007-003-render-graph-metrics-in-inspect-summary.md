---
title: "T-WP-E2-007-003 — Render Graph Metrics in Inspect Summary"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-007"
task: "T-WP-E2-007-003"
---

# T-WP-E2-007-003 — Render Graph Metrics in Inspect Summary

## Product Area

Inspect output and machine-readable repository summaries.

## Objective

Expose graph node, edge, category, and traversal-decision metrics through existing `monad inspect` output.

## Parent Work Packet

WP-E2-007 — Add Repository Graph Model Foundation.

## Expected Result

`monad inspect` includes graph metrics in text and JSON output.

## Verification

Run:

- `cargo run --quiet -p monad-cli -- inspect`
- `cargo run --quiet -p monad-cli -- inspect --format=json`

Expected result:

- text output includes `graph:`;
- JSON output includes `graph`.

## Status

Complete.

## Priority

High.

## Size

Small.
