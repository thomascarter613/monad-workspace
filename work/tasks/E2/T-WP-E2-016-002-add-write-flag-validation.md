---
title: "T-WP-E2-016-002 — Add Write Flag Validation"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-016"
task: "T-WP-E2-016-002"
---

# T-WP-E2-016-002 — Add Write Flag Validation

## Product Area

CLI parsing and command safety.

## Objective

Ensure `--write` is accepted only for the context command.

## Parent Work Packet

WP-E2-016 — Add Monad Context Write Foundation.

## Expected Result

Non-context commands reject `--write` with a clear error.

## Verification

Run:

- `cargo test write_flag_is_rejected_for_non_context_commands`

Expected result:

- non-context write attempts return `--write is only supported for the context command`.

## Status

Complete.

## Priority

High.

## Size

Small.
