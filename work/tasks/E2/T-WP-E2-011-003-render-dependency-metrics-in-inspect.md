---
title: "T-WP-E2-011-003 — Render Dependency Metrics in Inspect"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-011"
task: "T-WP-E2-011-003"
---

# T-WP-E2-011-003 — Render Dependency Metrics in Inspect

## Product Area

Inspect output and machine-readable repository summaries.

## Objective

Expose dependency signal metrics through existing `monad inspect` output.

## Parent Work Packet

WP-E2-011 — Add Dependency Signal Detection Foundation.

## Expected Result

`monad inspect` includes dependency signal counts in text and JSON output.

## Verification

Run:

- `cargo run --quiet -p monad-cli -- inspect`
- `cargo run --quiet -p monad-cli -- inspect --format=json`

Expected result:

- text output includes `dependencies:`;
- JSON output includes `dependencies`.

## Status

Complete.

## Priority

High.

## Size

Small.
