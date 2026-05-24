---
title: "D-WP-E2-009-001 — Graph CLI Command"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-009"
deliverable: "D-WP-E2-009-001"
---

# D-WP-E2-009-001 — Graph CLI Command

## Product Area

CLI command surface and repository intelligence.

## Objective

Expose repository graph rendering through `monad graph`.

## Source Work Packet

WP-E2-009 — Add Monad Graph Command Foundation.

## Deliverable Type

CLI behavior.

## Artifact Path

`crates/monad-cli/src/main.rs`

## Expected Result After Verification

`monad graph` renders a repository graph.

## Verification

Run `cargo run --quiet -p monad-cli -- graph`.

## Status

Complete.
