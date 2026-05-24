---
title: "WP-E2-006 — Implement Bounded Repository Traversal Foundation"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-006"
---

# WP-E2-006 — Implement Bounded Repository Traversal Foundation

## Product Area

Repository intelligence, bounded traversal, traversal safety, and inspect output.

## Objective

Implement the first bounded repository traversal foundation using the traversal plan and guardrails introduced in WP-E2-005.

## Scope

This work packet adds:

- `RepositoryTraversalEntry`;
- `RepositoryBoundedTraversal`;
- `traverse_workspace_bounded`;
- bounded recursive walking with max depth;
- deterministic child ordering;
- generated/external skip behavior;
- simple root `.gitignore` directory and exact-name pattern support;
- bounded traversal metrics in `monad inspect`.

The CLI remains unchanged.

## Expected Result After Verification

After verification:

- `cargo test` passes;
- bounded traversal walks safe directories;
- bounded traversal respects max depth;
- generated/external directories are recorded but not descended into;
- simple root `.gitignore` patterns are respected;
- `monad inspect` text output includes `bounded_traversal:`;
- `monad inspect --format=json` includes `bounded_traversal`;
- `tools/scripts/verify.sh` finishes with `Verification baseline passed.`.

## Priority

High.

## Size

Medium.
