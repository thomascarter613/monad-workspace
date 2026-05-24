---
title: "T-WP-E2-009-003 — Add Graph Smoke Verification"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-009"
task: "T-WP-E2-009-003"
---

# T-WP-E2-009-003 — Add Graph Smoke Verification

## Product Area

Verification and CLI reliability.

## Objective

Add graph command smoke tests to the root verification script.

## Parent Work Packet

WP-E2-009 — Add Monad Graph Command Foundation.

## Expected Result

`tools/scripts/verify.sh` verifies text, JSON, Mermaid, and DOT graph output paths.

## Verification

Run:

- `tools/scripts/verify.sh`

Expected result:

- graph smoke tests run;
- verification finishes with `Verification baseline passed.`.

## Status

Complete.

## Priority

High.

## Size

Small.
