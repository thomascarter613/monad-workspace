---
title: "T-WP-E2-005-001 — Add Traversal Planning Types"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-005"
task: "T-WP-E2-005-001"
---

# T-WP-E2-005-001 — Add Traversal Planning Types

## Product Area

Repository intelligence and traversal planning.

## Objective

Add typed traversal planning concepts before implementing recursive traversal.

## Parent Work Packet

WP-E2-005 — Add Recursive Traversal Plan and Guardrails.

## Expected Result

`monad-core` exposes stable traversal planning types for future bounded recursive traversal work.

## Verification

Run:

- `cargo test traversal_planning_types_are_exported_from_core_root`

Expected result:

- traversal mode, decision, and guardrail types are exported and tested.

## Status

Complete.

## Priority

High.

## Size

Small.
