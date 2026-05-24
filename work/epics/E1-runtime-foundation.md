---
title: "E1 — Runtime Foundation"
document_type: "epic"
status: "in-progress"
version: "0.5.0"
created: "2026-05-23"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
tags:
  - epic
  - runtime
  - rust
  - foundation
---

# E1 — Runtime Foundation

## Product Area

Core Runtime

## Objective

Establish Monad's Rust runtime foundation by creating the initial multi-crate workspace, separating CLI entrypoint concerns from durable core runtime logic, and preparing the first executable verification path.

## Rationale

E0 established Monad's repository, workflow, verification, ADR, and context foundations. E1 begins implementation by turning those foundations into a working Rust runtime structure.

Monad's accepted architecture requires Rust as the durable local core runtime, a thin CLI, and durable product logic in `monad-core`.

## Scope

E1 includes:

- initial Rust workspace structure;
- `monad-cli` crate foundation;
- `monad-core` crate foundation;
- workspace-level Rust verification;
- beginner-readable Rust comments;
- initial tests;
- initial CLI command wiring;
- diagnostics foundation;
- core error foundation;
- workspace context foundation;
- manifest/runtime foundation slices as later E1 work packets.

## Out of Scope

E1 does not include:

- full production CLI behavior;
- provider integrations;
- AI provider execution;
- marketplace/plugin systems;
- release packaging;
- advanced repository graphing;
- full policy engine behavior.

## Work Packets

| Work Packet | Title | Status |
|---|---|---|
| WP-E1-001 | Establish Rust workspace runtime foundation | Complete |
| WP-E1-002 | Establish core diagnostics foundation | Complete |
| WP-E1-003 | Establish core error foundation | Complete |
| WP-E1-004 | Establish workspace context foundation | In Progress |

## Expected Result After Verification

The repository has a working Rust workspace foundation, reusable diagnostics, a shared core error model, and a workspace context model.

## Verification

Run:

```bash
tools/scripts/verify.sh
````

Expected result:

```text
All required foundation and runtime paths exist.
cargo fmt --check
cargo test
Verification baseline passed.
```

## Priority

High

## Size

XL
