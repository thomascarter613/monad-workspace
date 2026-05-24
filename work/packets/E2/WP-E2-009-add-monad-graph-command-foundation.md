---
title: "WP-E2-009 — Add Monad Graph Command Foundation"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-009"
---

# WP-E2-009 — Add Monad Graph Command Foundation

## Product Area

Repository intelligence, graph rendering, CLI command surface, and verification.

## Objective

Expose the repository graph renderer through a thin `monad graph` CLI command.

## Scope

This work packet adds:

- `monad graph`;
- `monad graph --format=json`;
- `monad graph --format=mermaid`;
- `monad graph --format=dot`;
- graph command parsing tests;
- graph command smoke tests in `tools/scripts/verify.sh`.

Graph construction and graph rendering remain in `monad-core`.

## Expected Result After Verification

After verification:

- `cargo test` passes;
- `monad graph` prints text graph output;
- `monad graph --format=json` prints JSON graph output;
- `monad graph --format=mermaid` prints Mermaid graph output;
- `monad graph --format=dot` prints DOT graph output;
- `tools/scripts/verify.sh` finishes with `Verification baseline passed.`.

## Priority

High.

## Size

Medium.
