---
title: "T-WP-E2-004-003 — Render Summary Metrics"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-004"
task: "T-WP-E2-004-003"
---

# T-WP-E2-004-003 — Render Summary Metrics

## Product Area

Text output, JSON output, and CLI user experience.

## Objective

Render repository summary metrics in both human-readable and machine-readable inspect output.

## Parent Work Packet

WP-E2-004 — Add Repository Inspection Summary Metrics.

## Expected Result

`monad inspect` includes `metrics:` and `categories:` in text output, while JSON output includes `metrics` and `category_counts`.

## Verification

Run:

- `cargo run --quiet -p monad-cli -- inspect`
- `cargo run --quiet -p monad-cli -- inspect --format=json`

Expected result:

- text output includes metrics and categories;
- JSON output includes metrics and category counts.

## Status

Complete.

## Priority

High.

## Size

Small.
