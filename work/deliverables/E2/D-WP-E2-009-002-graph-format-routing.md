---
title: "D-WP-E2-009-002 — Graph Format Routing"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-009"
deliverable: "D-WP-E2-009-002"
---

# D-WP-E2-009-002 — Graph Format Routing

## Product Area

CLI output formats and graph rendering.

## Objective

Support graph-specific render formats from the CLI.

## Source Work Packet

WP-E2-009 — Add Monad Graph Command Foundation.

## Deliverable Type

CLI behavior.

## Artifact Path

`crates/monad-cli/src/main.rs`

## Expected Result After Verification

`monad graph` supports text, JSON, Mermaid, and DOT formats.

## Verification

Run `cargo run --quiet -p monad-cli -- graph --format=mermaid`.

## Status

Complete.
