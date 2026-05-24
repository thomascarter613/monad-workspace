---
title: "D-WP-E1-001-001 — Rust Workspace Manifest"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-001"
deliverable: "D-WP-E1-001-001"
tags:
  - deliverable
  - rust
  - workspace
---

# D-WP-E1-001-001 — Rust Workspace Manifest

## Product Area

Core Runtime

## Objective

Define the initial Rust workspace membership for Monad.

## Source Work Packet

WP-E1-001 — Establish Rust Workspace Runtime Foundation

## Deliverable Type

Configuration

## Artifact Path

`Cargo.toml`

## Expected Result After Verification

The root Rust workspace includes `crates/monad-cli` and `crates/monad-core`.

## Verification

Run:

```bash
tools/scripts/verify.sh
````

## Status

Complete
