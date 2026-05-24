---
title: "T-WP-E1-011-001 — Add CLI Output Format Parsing"
document_type: "task"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-011"
task: "T-WP-E1-011-001"
tags:
  - task
  - rust
  - cli
  - output
---

# T-WP-E1-011-001 — Add CLI Output Format Parsing

## Product Area

CLI Experience

## Objective

Add `CliInvocation` and support for `--format text` and `--format=text`.

## Parent Work Packet

WP-E1-011 — Establish CLI Output Format Argument Foundation

## Expected Result

The CLI can parse output-format options before or after command names.

## Verification

Run:

```bash
cargo test -p monad-cli
tools/scripts/verify.sh
````

## Status

Complete

## Priority

High

## Size

S
