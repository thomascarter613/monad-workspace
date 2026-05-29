---
title: "E1 — Runtime Foundation"
document_type: "epic"
status: complete
version: "1.0.0"
created: "2026-05-23"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
tags:
  - epic
  - runtime
  - rust
  - foundation
  - complete
---

# E1 — Runtime Foundation

## Product Area

Core Runtime

## Objective

Establish Monad's Rust runtime foundation by creating the initial multi-crate workspace, separating CLI entrypoint concerns from durable core runtime logic, and preparing the first executable verification path.

## Rationale

E0 established Monad's repository, workflow, verification, ADR, and context foundations. E1 turned those foundations into a working Rust runtime structure.

Monad's accepted architecture requires Rust as the durable local core runtime, a thin CLI, and durable product logic in `monad-core`.

## Scope

E1 completed:

- initial Rust workspace structure;
- `monad-cli` crate foundation;
- `monad-core` crate foundation;
- workspace-level Rust verification;
- beginner-readable Rust comments;
- initial tests;
- diagnostics foundation;
- core error foundation;
- workspace context foundation;
- manifest model foundation;
- manifest loading foundation;
- CLI info command foundation;
- CLI check command foundation;
- repository contract check foundation;
- runtime output formatting foundation;
- CLI output-format argument foundation;
- JSON output formatting foundation;
- E1 closure and E2 handoff.

## Out of Scope

E1 did not include:

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
| WP-E1-004 | Establish workspace context foundation | Complete |
| WP-E1-005 | Establish manifest model foundation | Complete |
| WP-E1-006 | Establish manifest loading foundation | Complete |
| WP-E1-007 | Establish CLI info command foundation | Complete |
| WP-E1-008 | Establish CLI check command foundation | Complete |
| WP-E1-009 | Establish repository contract check foundation | Complete |
| WP-E1-010 | Establish runtime output formatting foundation | Complete |
| WP-E1-011 | Establish CLI output format argument foundation | Complete |
| WP-E1-012 | Establish JSON output formatting foundation | Complete |
| WP-E1-013 | Close E1 and prepare E2 handoff | Complete |

## Expected Result After Verification

The repository has a working Rust runtime foundation, reusable diagnostics, a shared error model, workspace context, manifest loading, `monad info`, `monad check`, repository contract checks, shared output formatting, text output, and JSON output.

## Verification

Run:

```bash
tools/scripts/verify.sh
````

Expected result:

```text
Verification baseline passed.
```

## Priority

High

## Size

XL
