---
title: "T-WP-E2-016-001 — Add Context Write CLI Flag"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-016"
task: "T-WP-E2-016-001"
---

# T-WP-E2-016-001 — Add Context Write CLI Flag

## Product Area

CLI command surface and generated context artifacts.

## Objective

Add `--write` support to the `monad context` command.

## Parent Work Packet

WP-E2-016 — Add Monad Context Write Foundation.

## Expected Result

`monad context --write` delegates context-pack export to `monad-core`.

## Verification

Run:

- `cargo run --quiet -p monad-cli -- context --write`

Expected result:

- output begins with `Monad repository context pack export`.

## Status

Complete.

## Priority

High.

## Size

Small.
