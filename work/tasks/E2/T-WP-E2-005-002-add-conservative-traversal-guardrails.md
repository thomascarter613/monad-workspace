---
title: "T-WP-E2-005-002 — Add Conservative Traversal Guardrails"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-005"
task: "T-WP-E2-005-002"
---

# T-WP-E2-005-002 — Add Conservative Traversal Guardrails

## Product Area

Traversal safety and repository intelligence guardrails.

## Objective

Define conservative defaults for future recursive traversal.

## Parent Work Packet

WP-E2-005 — Add Recursive Traversal Plan and Guardrails.

## Expected Result

Future recursive traversal is constrained by default depth, symlink, generated/external, ignore-file, and deterministic ordering guardrails.

## Verification

Run:

- `cargo test traversal_guardrails_are_conservative_by_default`

Expected result:

- max depth is bounded;
- symlinks are not followed;
- generated/external entries are excluded by default;
- ignore files are respected;
- deterministic ordering is required.

## Status

Complete.

## Priority

High.

## Size

Small.
