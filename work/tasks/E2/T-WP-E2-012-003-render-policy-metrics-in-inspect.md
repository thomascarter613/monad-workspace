---
title: "T-WP-E2-012-003 — Render Policy Metrics in Inspect"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-012"
task: "T-WP-E2-012-003"
---

# T-WP-E2-012-003 — Render Policy Metrics in Inspect

## Product Area

Inspect output and machine-readable repository summaries.

## Objective

Expose repository policy diagnostics through existing `monad inspect` output.

## Parent Work Packet

WP-E2-012 — Add Repository Intelligence Policy Check Foundation.

## Expected Result

`monad inspect` includes policy diagnostics in text and JSON output.

## Verification

Run:

- `cargo run --quiet -p monad-cli -- inspect`
- `cargo run --quiet -p monad-cli -- inspect --format=json`

Expected result:

- text output includes `policy:`;
- JSON output includes `policy`.

## Status

Complete.

## Priority

High.

## Size

Small.
