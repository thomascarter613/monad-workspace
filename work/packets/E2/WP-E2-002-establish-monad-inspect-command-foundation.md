---
title: "WP-E2-002 — Establish monad inspect Command Foundation"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-002"
---

# WP-E2-002 — Establish `monad inspect` Command Foundation

## Product Area

Repository intelligence, CLI user experience, reusable output formatting, and verification.

## Objective

Add the first user-facing `monad inspect` command that calls the repository inspection model from `monad-core` and renders a top-level repository summary using the existing text and JSON output infrastructure.

## Scope

This work packet adds:

- reusable repository inspection summary rendering in `monad-core`;
- a thin `monad inspect` command in `monad-cli`;
- text output for human users;
- JSON output for scripts, CI, and future automation;
- Rust tests for the new inspection output path;
- verification smoke tests for `monad inspect`.

## Expected Result After Verification

After verification:

- `cargo test` passes;
- `cargo run --quiet -p monad-cli -- inspect` prints a human-readable repository inspection summary;
- `cargo run --quiet -p monad-cli -- inspect --format=json` prints machine-readable JSON;
- `tools/scripts/verify.sh` runs the new inspect smoke tests and finishes with `Verification baseline passed.`;
- the CLI remains thin and durable inspection logic remains in `monad-core`.

## Priority

High.

## Size

Small.
