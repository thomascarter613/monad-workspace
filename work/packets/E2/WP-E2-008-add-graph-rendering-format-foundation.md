---
title: "WP-E2-008 — Add Graph Rendering Format Foundation"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-008"
---

# WP-E2-008 — Add Graph Rendering Format Foundation

## Product Area

Repository intelligence, graph rendering, machine-readable output, and future visualization.

## Objective

Add deterministic graph rendering formats to `monad-core` while keeping graph construction separate from graph rendering.

## Scope

This work packet adds:

- `RepositoryGraphRenderFormat`;
- text graph rendering;
- JSON graph rendering;
- Mermaid graph rendering;
- DOT / Graphviz graph rendering;
- parsing for graph render format names;
- tests for deterministic graph rendering.

This work packet does not add a dedicated `monad graph` command yet.

## Expected Result After Verification

After verification:

- `cargo test` passes;
- graph render format parsing works;
- text graph rendering includes nodes and edges;
- JSON graph rendering includes nodes and edges;
- Mermaid graph rendering starts with `flowchart TD`;
- DOT graph rendering starts with `digraph repository`;
- rendering is deterministic;
- `tools/scripts/verify.sh` finishes with `Verification baseline passed.`.

## Priority

High.

## Size

Medium.
