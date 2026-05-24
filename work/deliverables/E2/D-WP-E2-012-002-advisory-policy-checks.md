---
title: "D-WP-E2-012-002 — Advisory Policy Checks"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-012"
deliverable: "D-WP-E2-012-002"
---

# D-WP-E2-012-002 — Advisory Policy Checks

## Product Area

Repository intelligence and advisory policy checks.

## Objective

Evaluate non-blocking policy diagnostics from repository intelligence inputs.

## Source Work Packet

WP-E2-012 — Add Repository Intelligence Policy Check Foundation.

## Deliverable Type

Runtime behavior.

## Artifact Path

`crates/monad-core/src/repository_policy.rs`

## Expected Result After Verification

Advisory diagnostics are generated for missing repository signals, dependency lockfile gaps, and traversal safety findings.

## Verification

Run `cargo test policy_`.

## Status

Complete.
