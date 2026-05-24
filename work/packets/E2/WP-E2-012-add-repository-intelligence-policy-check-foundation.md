---
title: "WP-E2-012 — Add Repository Intelligence Policy Check Foundation"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-012"
---

# WP-E2-012 — Add Repository Intelligence Policy Check Foundation

## Product Area

Repository intelligence, advisory policy checks, bounded traversal, dependency signals, and inspect output.

## Objective

Add advisory repository-intelligence policy diagnostics to `monad-core`.

## Scope

This work packet adds:

- `RepositoryPolicySeverity`;
- `RepositoryPolicyDiagnostic`;
- `RepositoryPolicyReport`;
- `evaluate_repository_intelligence_policy`;
- advisory diagnostics for missing README and license signals;
- warning diagnostics for dependency manifest signals without lockfile signals;
- informational diagnostics for traversal safety outcomes;
- policy metrics in `monad inspect`.

This work packet does not make policy checks blocking.

## Expected Result After Verification

After verification:

- `cargo test` passes;
- policy diagnostics are generated from repository intelligence inputs;
- `monad inspect` text output includes `policy:`;
- `monad inspect --format=json` includes `policy`;
- `tools/scripts/verify.sh` finishes with `Verification baseline passed.`.

## Priority

High.

## Size

Medium.
