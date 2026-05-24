---
title: "WP-E2-014 — Add Monad Context Command Foundation"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-014"
---

# WP-E2-014 — Add Monad Context Command Foundation

## Product Area

Repository intelligence, AI-readable context packs, CLI command surface, and verification.

## Objective

Expose repository context pack rendering through a thin `monad context` CLI command.

## Scope

This work packet adds:

- `monad context`;
- `monad context --format=markdown`;
- `monad context --format=json`;
- `monad context --format=md`;
- `monad context --format=text`;
- context command parsing tests;
- context command smoke tests in `tools/scripts/verify.sh`.

Context pack construction and rendering remain in `monad-core`.

## Expected Result After Verification

After verification:

- `cargo test` passes;
- `monad context` prints Markdown context-pack output;
- `monad context --format=json` prints JSON context-pack output;
- context format aliases work;
- `tools/scripts/verify.sh` finishes with `Verification baseline passed.`.

## Priority

High.

## Size

Medium.
