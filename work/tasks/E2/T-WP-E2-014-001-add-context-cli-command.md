---
title: "T-WP-E2-014-001 — Add Context CLI Command"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-014"
task: "T-WP-E2-014-001"
---

# T-WP-E2-014-001 — Add Context CLI Command

## Product Area

CLI command surface and AI-readable repository intelligence.

## Objective

Add a thin `monad context` command that delegates context-pack construction and rendering to `monad-core`.

## Parent Work Packet

WP-E2-014 — Add Monad Context Command Foundation.

## Expected Result

`monad context` renders a Markdown repository context pack.

## Verification

Run:

- `cargo run --quiet -p monad-cli -- context`

Expected result:

- output begins with `# Monad Repository Context Pack`.

## Status

Complete.

## Priority

High.

## Size

Small.
