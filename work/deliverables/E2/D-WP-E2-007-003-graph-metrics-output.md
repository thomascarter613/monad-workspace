---
title: "D-WP-E2-007-003 — Graph Metrics Output"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-007"
deliverable: "D-WP-E2-007-003"
---

# D-WP-E2-007-003 — Graph Metrics Output

## Product Area

Inspect output and machine-readable repository summaries.

## Objective

Expose graph metrics through `monad inspect`.

## Source Work Packet

WP-E2-007 — Add Repository Graph Model Foundation.

## Deliverable Type

Output rendering behavior.

## Artifact Path

`crates/monad-core/src/output.rs`

## Expected Result After Verification

`monad inspect` text output includes `graph:`, and JSON output includes `graph`.

## Verification

Run `cargo run --quiet -p monad-cli -- inspect --format=json`.

## Status

Complete.
