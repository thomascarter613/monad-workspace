---
title: "D-WP-E2-014-001 — Context CLI Command"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-014"
deliverable: "D-WP-E2-014-001"
---

# D-WP-E2-014-001 — Context CLI Command

## Product Area

CLI command surface and AI-readable repository intelligence.

## Objective

Expose repository context-pack rendering through `monad context`.

## Source Work Packet

WP-E2-014 — Add Monad Context Command Foundation.

## Deliverable Type

CLI behavior.

## Artifact Path

`crates/monad-cli/src/main.rs`

## Expected Result After Verification

`monad context` renders a Markdown repository context pack.

## Verification

Run `cargo run --quiet -p monad-cli -- context`.

## Status

Complete.
