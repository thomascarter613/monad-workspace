---
title: "Decision Log"
document_type: "context-decision-log"
status: "current"
version: "2.1.0"
created: "2026-05-23"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E2"
work_packet: "WP-E2-001"
tags:

* decisions
* e2
* repository-intelligence
* repository-inspection

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
13. E1 — Runtime Foundation is complete.
14. E2 — Repository Intelligence Foundation begins with WP-E2-001.
15. WP-E2-001 establishes Repository Inspection in `monad-core`.

## Current Epic

E2 — Repository Intelligence Foundation

## Current Work Packet

WP-E2-001 — Establish Repository Inspection Foundation
