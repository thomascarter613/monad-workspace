---
title: E24 — LSP and Static Analysis Foundation
status: implemented
owner: Monad
last_reviewed: 2026-06-09
---

# E24 — LSP and Static Analysis Foundation

## Scope

E24 implements the foundation for static-analysis and LSP-aware repository understanding.

Implemented work packets:

- WP-E24-001 — Define static-analysis and symbol model
- WP-E24-002 — Add parser abstraction foundation
- WP-E24-003 — Add LSP discovery and capability model
- WP-E24-004 — Add symbol extraction proof of concept
- WP-E24-005 — Add source map and ownership metadata
- WP-E24-006 — Add static-analysis report tests

## Deliverables

- `crates/monad-core/src/static_analysis.rs`
- `monad analysis --dry-run`
- `monad analysis --dry-run --format=json`
- `monad analysis --yes`
- static-analysis documentation
- verification scripts

## Safety posture

The implementation is local-first and non-executing. It does not launch language servers, invoke package managers, call AI providers, fetch remote analyzers, or mutate source files.
