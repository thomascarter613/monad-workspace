---
title: "Decision Log"
document_type: "context-decision-log"
status: "current"
version: "1.9.0"
created: "2026-05-23"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-010"
tags:

* decisions
* e1
* output-formatting

---

# Decision Log

## Accepted Decisions

1. Monad is the unified product name.
2. Rust is the durable local core runtime language.
3. Initial Rust workspace separates `monad-cli` and `monad-core`.
4. CLI stays thin.
5. Durable product logic belongs in `monad-core`.
6. Repository is the source of truth.
7. Work packets are the primary delivery unit.
8. Native tools are coordinated, not unnecessarily replaced.
9. Monad remains local-first and provider-agnostic.
10. Agent workflows are supervised and human-in-command.
11. Bazel, Pants, Buck2, and Nx are not default Monad dependencies.
12. E0 — Project Foundation is complete.
13. WP-E1-001 through WP-E1-009 are complete.
14. WP-E1-010 establishes shared Output Formatting in `monad-core`.

## Current Epic

E1 — Runtime Foundation

## Current Work Packet

WP-E1-010 — Establish Runtime Output Formatting Foundation
