---
title: "T-WP-E2-003-003 — Harden Traversal Policy"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-003"
task: "T-WP-E2-003-003"
---

# T-WP-E2-003-003 — Harden Traversal Policy

## Product Area

Traversal safety and repository intelligence guardrails.

## Objective

Expand generated and external directory safeguards before recursive inspection exists.

## Parent Work Packet

WP-E2-003 — Enrich Repository Inspection Classification.

## Expected Result

Dependency caches, build outputs, virtual environments, coverage outputs, temporary directories, and VCS internals are marked `skip_generated_or_external`.

## Verification

Run:

- `cargo test generated_or_external_directories_are_marked_for_skip`
- `cargo run --quiet -p monad-cli -- inspect`

Expected result:

- generated/external directories are classified correctly;
- inspect output remains successful.

## Status

Complete.

## Priority

High.

## Size

Small.
