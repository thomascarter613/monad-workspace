---
title: "Decision Log"
document_type: "context-decision-log"
status: "current"
version: "1.6.0"
created: "2026-05-23"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-007"
tags:

* decisions
* e1
* cli-info

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
13. E1 — Runtime Foundation began with WP-E1-001.
14. WP-E1-001 established the Rust workspace foundation.
15. WP-E1-002 established Core Diagnostics in `monad-core`.
16. WP-E1-003 established Core Error in `monad-core`.
17. WP-E1-004 established Workspace Context in `monad-core`.
18. WP-E1-005 established Manifest Model in `monad-core`.
19. WP-E1-006 established Manifest Loading in `monad-core`.
20. WP-E1-007 establishes CLI Info in `monad-cli`.

## Current Epic

E1 — Runtime Foundation

## Current Work Packet

WP-E1-007 — Establish CLI Info Command Foundation
