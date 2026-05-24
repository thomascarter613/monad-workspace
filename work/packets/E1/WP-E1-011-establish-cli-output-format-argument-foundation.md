---
title: "WP-E1-011 — Establish CLI Output Format Argument Foundation"
document_type: "work-packet"
status: "in-progress"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-011"
tags:
  - work-packet
  - rust
  - cli
  - output
  - formatting
---

# WP-E1-011 — Establish CLI Output Format Argument Foundation

## Product Area

CLI Experience

## Objective

Add CLI output-format argument parsing so commands can accept `--format text` and `--format=text`.

## Rationale

WP-E1-010 established reusable output formatting in `monad-core`. The CLI now needs an argument path into that runtime model so future output formats can be added without redesigning command parsing.

Only `text` is supported in this slice. JSON and other formats remain future work.

## Scope

This work packet covers:

- `CliInvocation`;
- `--format text`;
- `--format=text`;
- output format parsing before or after command names;
- invalid format handling;
- missing format value handling;
- multiple command detection;
- CLI tests;
- E1 planning, context, and verification updates.

## Deliverables

Expected deliverables include:

- updated `crates/monad-cli/src/main.rs`;
- updated E1 task records;
- updated E1 deliverable records;
- updated E1 work packet index;
- updated context handoff files;
- updated verification baseline.

## Expected Result After Verification

`monad info --format text`, `monad --format text info`, and `monad check --format=text` work, unsupported formats fail with structured errors, and the full verification baseline passes.

## Verification

Run:

```bash
cargo fmt --check
cargo test
cargo run --quiet -p monad-cli -- info --format text
cargo run --quiet -p monad-cli -- check --format=text
tools/scripts/verify.sh
````

Expected output includes:

```text id="702ccu"
Monad workspace
[INFO] MONAD4000
[INFO] MONAD4500
Verification baseline passed.
```

## Status

In Progress

## Priority

High

## Size

M
