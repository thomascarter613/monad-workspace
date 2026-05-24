---
title: "Decision Log"
document_type: "context-decision-log"
status: "current"
version: "1.2.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-003"
tags:

* decisions
* e1
* core-error

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
16. WP-E1-003 establishes Core Error in `monad-core`.

## Current Epic

E1 — Runtime Foundation

## Current Work Packet

WP-E1-003 — Establish Core Error Foundation

## Prior E1 Work Packets

* WP-E1-001 — Establish Rust Workspace Runtime Foundation
* WP-E1-002 — Establish Core Diagnostics Foundation
  MD

cat > .monad/context/work-packet-handoffs/WP-E1-003.md <<'MD'

title: "WP-E1-003 Handoff"
document_type: "work-packet-handoff"
status: "current"
version: "0.1.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-003"
tags:

* handoff
* rust
* runtime-foundation
* core-error

---

# WP-E1-003 Handoff

## Work Packet

WP-E1-003 — Establish Core Error Foundation

## Current Epic

E1 — Runtime Foundation

## Prior Work

E0 — Project Foundation is complete.

WP-E1-001 — Establish Rust Workspace Runtime Foundation is complete.

WP-E1-002 — Establish Core Diagnostics Foundation is complete.

## Objective

Create a durable core error foundation in `monad-core`.

## Core Error

This slice adds:

* `MonadError`;
* `MonadResult<T>`;
* stable error codes;
* conversion from errors to diagnostics;
* `Display` implementation;
* standard Rust error trait integration;
* unit tests.

## Rules

* Use Rust Apprenticeship Mode.
* Provide complete file contents.
* Add beginner-readable comments.
* Keep CLI thin.
* Put durable runtime logic in `monad-core`.
* Add tests.
* Run verification.
* Use `python3`, not `python`.
* Do not introduce Bazel, Pants, Buck2, or Nx.

## Expected Verification

```bash
cargo fmt --check
cargo test
tools/scripts/verify.sh
```

