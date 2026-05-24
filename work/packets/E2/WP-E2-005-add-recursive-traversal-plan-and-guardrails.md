---
title: "WP-E2-005 — Add Recursive Traversal Plan and Guardrails"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-005"
---

# WP-E2-005 — Add Recursive Traversal Plan and Guardrails

## Product Area

Repository intelligence, traversal safety, future recursive inspection, and machine-readable planning.

## Objective

Define the safe API shape, traversal plan model, and guardrails required before Monad implements any recursive repository traversal.

## Scope

This work packet adds:

- `RepositoryTraversalMode`;
- `RepositoryTraversalDecision`;
- `RepositoryTraversalGuardrails`;
- `RepositoryTraversalPlanEntry`;
- `RepositoryTraversalPlan`;
- `build_traversal_plan`;
- conservative defaults for future bounded traversal;
- traversal decision output in `monad inspect`;
- future traversal guardrail output in text and JSON summaries.

This work packet intentionally does not implement deep traversal.

## Expected Result After Verification

After verification:

- `cargo test` passes;
- `monad inspect` text output includes `future_traversal_guardrails:`;
- `monad inspect --format=json` includes `future_traversal`;
- future traversal defaults are conservative;
- generated/external entries are planned as `skip_by_default`;
- safe roots are planned as `candidate_for_future_traversal`;
- `tools/scripts/verify.sh` finishes with `Verification baseline passed.`.

## Priority

High.

## Size

Small.
