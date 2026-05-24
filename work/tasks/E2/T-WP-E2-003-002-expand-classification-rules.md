---
title: "T-WP-E2-003-002 — Expand Classification Rules"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-003"
task: "T-WP-E2-003-002"
---

# T-WP-E2-003-002 — Expand Classification Rules

## Product Area

Repository intelligence and deterministic inspection.

## Objective

Teach the shallow inspector to recognize common top-level files and directories using conservative name-based classification.

## Parent Work Packet

WP-E2-003 — Enrich Repository Inspection Classification.

## Expected Result

The shallow inspector recognizes important repository artifacts such as `LICENSE`, `.gitignore`, `.editorconfig`, `Cargo.lock`, `rust-toolchain.toml`, `package.json`, `tools/`, `infra/`, `contracts/`, `db/`, `.github/`, and `.devcontainer/`.

## Verification

Run:

- `cargo test repository_inspection`

Expected result:

- classification tests pass;
- inspection remains deterministic and non-recursive.

## Status

Complete.

## Priority

High.

## Size

Small.
