---
title: "T-WP-E2-002-003 — Add Inspect Smoke Verification"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-002"
task: "T-WP-E2-002-003"
---

# T-WP-E2-002-003 — Add Inspect Smoke Verification

## Product Area

Verification and release confidence.

## Objective

Add `monad inspect` smoke tests to the repository verification baseline.

## Parent Work Packet

WP-E2-002 — Establish `monad inspect` Command Foundation.

## Expected Result

`tools/scripts/verify.sh` runs both text and JSON inspect smoke checks.

## Verification

Run:

- `tools/scripts/verify.sh`

Expected result:

- verification includes `Running CLI inspect smoke test`;
- verification includes `Running CLI inspect JSON smoke test`;
- verification finishes with `Verification baseline passed.`.

## Status

Complete.

## Priority

High.

## Size

Small.
