---
title: "D-WP-E2-012-003 — Policy Inspect Output"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-012"
deliverable: "D-WP-E2-012-003"
---

# D-WP-E2-012-003 — Policy Inspect Output

## Product Area

Inspect output and machine-readable repository summaries.

## Objective

Expose repository policy diagnostics through `monad inspect`.

## Source Work Packet

WP-E2-012 — Add Repository Intelligence Policy Check Foundation.

## Deliverable Type

Output rendering behavior.

## Artifact Path

`crates/monad-core/src/output.rs`

## Expected Result After Verification

`monad inspect` text output includes `policy:`, and JSON output includes `policy`.

## Verification

Run `cargo run --quiet -p monad-cli -- inspect --format=json`.

## Status

Complete.
