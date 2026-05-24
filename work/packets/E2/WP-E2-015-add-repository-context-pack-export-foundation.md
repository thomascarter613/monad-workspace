---
title: "WP-E2-015 — Add Repository Context Pack Export Foundation"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-015"
---

# WP-E2-015 — Add Repository Context Pack Export Foundation

## Product Area

Repository intelligence, AI-readable context packs, generated context artifacts, and deterministic file export.

## Objective

Add a core export foundation for writing repository context packs to deterministic generated files.

## Scope

This work packet adds:

- deterministic default export directory calculation;
- Markdown context-pack export;
- JSON context-pack export;
- exported file metadata;
- export result metadata;
- a root-level helper for exporting context packs from a workspace.

This work packet does not add CLI write flags.

## Expected Result After Verification

After verification:

- `cargo test` passes;
- context packs export to `.monad/context/generated/`;
- Markdown export writes `repository-context-pack.md`;
- JSON export writes `repository-context-pack.json`;
- exported file metadata includes format, path, and byte count;
- `tools/scripts/verify.sh` finishes with `Verification baseline passed.`.

## Priority

High.

## Size

Medium.
