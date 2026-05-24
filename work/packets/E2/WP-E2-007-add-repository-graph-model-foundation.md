---
title: "WP-E2-007 — Add Repository Graph Model Foundation"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-007"
---

# WP-E2-007 — Add Repository Graph Model Foundation

## Product Area

Repository intelligence, graph model, bounded traversal, and inspect output.

## Objective

Add the first internal repository graph model derived from bounded traversal so future Monad graph commands and renderers have a stable core model to build on.

## Scope

This work packet adds:

- `RepositoryGraphNodeKind`;
- `RepositoryGraphEdgeKind`;
- `RepositoryGraphNode`;
- `RepositoryGraphEdge`;
- `RepositoryGraph`;
- `build_repository_graph`;
- graph node, edge, depth, category, and traversal-decision metrics;
- graph metrics in `monad inspect` text and JSON output.

This work packet does not add Mermaid, DOT, or dedicated graph rendering yet.

## Expected Result After Verification

After verification:

- `cargo test` passes;
- repository graph tests pass;
- `monad inspect` text output includes `graph:`;
- `monad inspect --format=json` includes `graph`;
- graph output is deterministic;
- graph nodes include the workspace root and bounded traversal entries;
- graph edges represent parent-child containment;
- `tools/scripts/verify.sh` finishes with `Verification baseline passed.`.

## Priority

High.

## Size

Medium.
