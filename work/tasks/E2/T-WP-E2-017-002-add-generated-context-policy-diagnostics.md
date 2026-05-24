---
title: "T-WP-E2-017-002 — Add Generated Context Policy Diagnostics"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-017"
task: "T-WP-E2-017-002"
---

# T-WP-E2-017-002 — Add Generated Context Policy Diagnostics

## Product Area

Repository intelligence and advisory policy diagnostics.

## Objective

Add diagnostics describing generated context artifact policy status.

## Parent Work Packet

WP-E2-017 — Add Generated Context Artifact Policy Foundation.

## Expected Result

Repository policy checks report whether generated context artifacts are ignored and whether generated context artifacts exist.

## Verification

Run:

- `cargo test policy_advises_generated_context_artifact_ignore_rule_when_missing`
- `cargo test policy_reports_generated_context_artifact_ignore_rule_when_present`
- `cargo test policy_reports_generated_context_artifact_directory_when_present`

Expected result:

- generated context artifact policy diagnostics are produced as expected.

## Status

Complete.

## Priority

High.

## Size

Medium.
