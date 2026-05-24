---
title: "T-WP-E2-009-001 — Add Graph CLI Command"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-009"
task: "T-WP-E2-009-001"
---

# T-WP-E2-009-001 — Add Graph CLI Command

## Product Area

CLI command surface and repository intelligence.

## Objective

Add a thin `monad graph` command that delegates graph construction and rendering to `monad-core`.

## Parent Work Packet

WP-E2-009 — Add Monad Graph Command Foundation.

## Expected Result

`monad graph` renders the repository graph using the default text format.

## Verification

Run:

- `cargo run --quiet -p monad-cli -- graph`

Expected result:

- output begins with `Monad repository graph`.

## Status

Complete.

## Priority

High.

## Size

Small.
