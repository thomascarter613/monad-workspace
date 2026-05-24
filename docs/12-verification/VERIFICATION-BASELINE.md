---
title: "Verification Baseline"
document_type: "verification-standard"
status: "current"
version: "2.0.0"
created: "2026-05-23"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E2"
work_packet: "WP-E2-001"
tags:
  - verification
  - quality
  - repository-contract
  - workflow
  - rust
  - cli
  - output
  - json
  - diagnostics
  - work-packets
  - tasks
  - deliverables
  - epics
  - adrs
  - context
---

# Verification Baseline

## 1. Purpose

This document defines Monad's repository verification baseline.

The baseline exists so foundational repository work and Rust runtime work can be checked from durable repo-resident scripts.

## 2. Current Baseline Command

Run from the repository root:

```bash
tools/scripts/verify.sh
````

## 3. Current Checks

The baseline currently verifies:

1. Git diff whitespace validity.
2. Required foundation and runtime paths.
3. YAML frontmatter presence for Markdown files under:

   * `docs/`
   * `work/`
   * `.monad/`
4. Work packet record structure.
5. Task record structure.
6. Deliverable record structure.
7. Epic record structure.
8. ADR record structure.
9. E1 closure and E2 handoff context records.
10. Rust formatting with `cargo fmt --check`.
11. Rust tests with `cargo test`.
12. CLI info smoke test with `cargo run --quiet -p monad-cli -- info`.
13. CLI check smoke test with `cargo run --quiet -p monad-cli -- check`.
14. CLI info JSON smoke test.
15. CLI check JSON smoke test.
16. Current working tree status.

## 4. Runtime Checks

The runtime baseline verifies:

* `monad-cli` compiles;
* `monad-core` compiles;
* unit tests pass;
* `monad info` works;
* `monad check` works;
* text output works;
* JSON Output works;
* formatting is stable.

## 5. Expected Successful Result

A successful run should include:

```text
All required foundation and runtime paths exist.
All docs/work/.monad Markdown files have YAML frontmatter.
All work packet records satisfy the required structure.
All task records satisfy the required baseline structure.
All deliverable records satisfy the required baseline structure.
All epic records satisfy the required baseline structure.
All ADR records satisfy the required baseline structure.
All context records satisfy the E1 closure and E2 handoff baseline.
Monad workspace
[INFO] MONAD4000
[INFO] MONAD4500
Verification baseline passed.
```

## 6. Failure Meaning

A failure means at least one foundational repository, runtime, or context-handoff expectation is not satisfied.

Common causes include:

* a required file was not created;
* a file was created at the wrong path;
* a Markdown file is missing YAML frontmatter;
* a work packet, task, deliverable, epic, or ADR record is missing required structure;
* context files do not identify E2, WP-E2-001, and Repository Intelligence;
* Rust code is not formatted;
* Rust tests fail;
* `monad info` fails;
* `monad check` fails;
* JSON output fails.

## 7. Maintenance Rules

This document must be updated when verification behavior changes.
