---
title: "T-WP-E2-014-002 — Add Context Format Routing"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-014"
task: "T-WP-E2-014-002"
---

# T-WP-E2-014-002 — Add Context Format Routing

## Product Area

CLI output formats and context-pack rendering.

## Objective

Route `monad context --format=<format>` to the context-pack render formats supported by `monad-core`.

## Parent Work Packet

WP-E2-014 — Add Monad Context Command Foundation.

## Expected Result

`monad context` supports Markdown and JSON output, with `md` and `text` aliases for Markdown.

## Verification

Run:

- `cargo run --quiet -p monad-cli -- context --format=markdown`
- `cargo run --quiet -p monad-cli -- context --format=json`
- `cargo run --quiet -p monad-cli -- context --format=md`
- `cargo run --quiet -p monad-cli -- context --format=text`

Expected result:

- Markdown output starts with `# Monad Repository Context Pack`;
- JSON output includes `repository_context_pack`.

## Status

Complete.

## Priority

High.

## Size

Small.
