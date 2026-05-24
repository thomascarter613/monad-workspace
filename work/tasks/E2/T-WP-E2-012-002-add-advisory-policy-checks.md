---
title: "T-WP-E2-012-002 — Add Advisory Policy Checks"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-012"
task: "T-WP-E2-012-002"
---

# T-WP-E2-012-002 — Add Advisory Policy Checks

## Product Area

Repository intelligence and advisory policy checks.

## Objective

Evaluate advisory diagnostics from inspection, bounded traversal, and dependency signal inputs.

## Parent Work Packet

WP-E2-012 — Add Repository Intelligence Policy Check Foundation.

## Expected Result

Monad reports advisory and warning diagnostics for repository-intelligence policy findings.

## Verification

Run:

- `cargo test policy_reports_missing_readme_and_license`
- `cargo test policy_warns_when_manifest_has_no_lockfile`
- `cargo test policy_reports_traversal_safety_information`

Expected result:

- policy diagnostics are generated and counted by severity.

## Status

Complete.

## Priority

High.

## Size

Medium.
