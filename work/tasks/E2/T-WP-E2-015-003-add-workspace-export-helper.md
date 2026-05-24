---
title: "T-WP-E2-015-003 — Add Workspace Export Helper"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-015"
task: "T-WP-E2-015-003"
---

# T-WP-E2-015-003 — Add Workspace Export Helper

## Product Area

Runtime API ergonomics and generated context artifacts.

## Objective

Add a root-level helper that builds and exports a repository context pack from a workspace context.

## Parent Work Packet

WP-E2-015 — Add Repository Context Pack Export Foundation.

## Expected Result

`export_repository_context_pack_from_workspace` is available from `monad-core`.

## Verification

Run:

- `cargo test repository_context_pack_export_types_are_exported_from_core_root`

Expected result:

- export helper dependencies compile and remain accessible from the root crate.

## Status

Complete.

## Priority

High.

## Size

Small.
