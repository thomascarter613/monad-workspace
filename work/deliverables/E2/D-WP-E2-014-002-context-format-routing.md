---
title: "D-WP-E2-014-002 — Context Format Routing"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-014"
deliverable: "D-WP-E2-014-002"
---

# D-WP-E2-014-002 — Context Format Routing

## Product Area

CLI output formats and context-pack rendering.

## Objective

Support context-specific render formats from the CLI.

## Source Work Packet

WP-E2-014 — Add Monad Context Command Foundation.

## Deliverable Type

CLI behavior.

## Artifact Path

`crates/monad-cli/src/main.rs`

## Expected Result After Verification

`monad context` supports Markdown, JSON, `md`, and `text` formats.

## Verification

Run `cargo run --quiet -p monad-cli -- context --format=json`.

## Status

Complete.
