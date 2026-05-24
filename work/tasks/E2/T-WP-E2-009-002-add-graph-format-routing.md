---
title: "T-WP-E2-009-002 — Add Graph Format Routing"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-009"
task: "T-WP-E2-009-002"
---

# T-WP-E2-009-002 — Add Graph Format Routing

## Product Area

CLI output formats and graph rendering.

## Objective

Route `monad graph --format=<format>` to the graph render formats supported by `monad-core`.

## Parent Work Packet

WP-E2-009 — Add Monad Graph Command Foundation.

## Expected Result

`monad graph` supports text, JSON, Mermaid, and DOT formats.

## Verification

Run:

- `cargo run --quiet -p monad-cli -- graph --format=json`
- `cargo run --quiet -p monad-cli -- graph --format=mermaid`
- `cargo run --quiet -p monad-cli -- graph --format=dot`

Expected result:

- JSON output includes `repository_graph`;
- Mermaid output starts with `flowchart TD`;
- DOT output starts with `digraph repository`.

## Status

Complete.

## Priority

High.

## Size

Small.
