---
title: "WP-E2-003 — Enrich Repository Inspection Classification"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-003"
---

# WP-E2-003 — Enrich Repository Inspection Classification

## Product Area

Repository intelligence, repository structure classification, traversal safety, and inspection readiness.

## Objective

Improve Monad's shallow repository inspection model so it can classify more top-level repository files and directories without making the CLI smarter or introducing recursive traversal too early.

## Scope

This work packet enriches `RepositoryEntryRole` with classifications for:

- Rust runtime files;
- developer-experience files;
- JavaScript package configuration;
- tooling configuration;
- infrastructure configuration;
- AI context files;
- tooling roots;
- infrastructure roots;
- contract roots;
- data roots;
- governance roots;
- asset roots;
- test roots;
- CI roots;
- development environment roots;
- generated or external directories.

This work packet also expands traversal policy coverage so future recursive inspection can avoid dependency caches, build outputs, virtual environments, and VCS internals by default.

## Expected Result After Verification

After verification:

- `cargo test` passes;
- repository inspection tests confirm richer role classification;
- `monad inspect` output reflects richer roles through existing role labels and role counts;
- generated or external directories remain marked with `skip_generated_or_external`;
- known safe top-level roots are marked `safe_for_future_traversal`;
- `tools/scripts/verify.sh` finishes with `Verification baseline passed.`.

## Priority

High.

## Size

Small.
