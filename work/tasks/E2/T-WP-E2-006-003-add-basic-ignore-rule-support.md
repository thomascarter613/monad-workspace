---
title: "T-WP-E2-006-003 — Add Basic Ignore Rule Support"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-006"
task: "T-WP-E2-006-003"
---

# T-WP-E2-006-003 — Add Basic Ignore Rule Support

## Product Area

Traversal safety and repository ignore handling.

## Objective

Respect simple root `.gitignore` patterns during bounded traversal.

## Parent Work Packet

WP-E2-006 — Implement Bounded Repository Traversal Foundation.

## Expected Result

Simple exact-name and directory-only root `.gitignore` patterns prevent ignored directories from being descended into.

## Verification

Run:

- `cargo test bounded_traversal_respects_simple_root_gitignore_patterns`

Expected result:

- ignored directories are recorded as skipped;
- ignored directory contents are not traversed.

## Status

Complete.

## Priority

High.

## Size

Small.
