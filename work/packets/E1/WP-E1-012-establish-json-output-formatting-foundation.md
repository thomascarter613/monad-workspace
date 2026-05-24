---
title: "WP-E1-012 — Establish JSON Output Formatting Foundation"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-012"
tags:
  - work-packet
  - rust
  - cli
  - json
  - output
---

# WP-E1-012 — Establish JSON Output Formatting Foundation

## Product Area

Core Runtime

## Objective

Add JSON output support for `monad info` and `monad check`.

## Rationale

WP-E1-010 created shared output formatting, and WP-E1-011 added CLI output-format arguments. The next step is to make that format argument useful for machine-readable automation.

JSON output lets future scripts, CI jobs, editor integrations, and supervised execution workflows consume Monad output without scraping text.

## Scope

This work packet covers:

- adding `serde_json` to `monad-core`;
- extending `OutputFormat` with `Json`;
- JSON rendering for workspace summaries;
- JSON rendering for diagnostic reports;
- CLI tests for JSON output;
- JSON smoke checks in `tools/scripts/verify.sh`;
- E1 planning, context, and verification updates.

## Deliverables

Expected deliverables include:

- updated `crates/monad-core/Cargo.toml`;
- updated `crates/monad-core/src/output.rs`;
- updated `crates/monad-cli/src/main.rs`;
- updated `tools/scripts/verify.sh`;
- updated E1 task records;
- updated E1 deliverable records;
- updated E1 work packet index;
- updated context handoff files;
- updated verification baseline.

## Expected Result After Verification

`monad info --format json` and `monad check --format=json` produce machine-readable JSON output, text output remains the default, and the full verification baseline passes.

## Verification

Run:

```bash
cargo fmt --check
cargo test
cargo run --quiet -p monad-cli -- info --format json
cargo run --quiet -p monad-cli -- check --format=json
tools/scripts/verify.sh
````

Expected output includes:

```text
"format": "json"
"kind": "workspace_summary"
"kind": "diagnostic_report"
Verification baseline passed.
```

## Status

Complete

## Priority

High

## Size

M
