---
title: "WP-E1-001 — Establish Rust Workspace Runtime Foundation"
document_type: "work-packet"
status: "complete"
version: "0.2.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-001"
tags:

* work-packet
* rust
* runtime
* workspace

---

# WP-E1-001 — Establish Rust Workspace Runtime Foundation

## Product Area

Core Runtime

## Objective

Create or normalize the initial Rust workspace foundation for Monad, separating the CLI entrypoint crate from durable core runtime logic.

## Rationale

Monad's accepted architecture places durable runtime behavior in Rust, keeps the CLI thin, and separates `monad-cli` from `monad-core`.

This work packet begins implementation while preserving the project discipline established in E0.

## Scope

This work packet covers:

* root `Cargo.toml` workspace membership;
* `crates/monad-cli/` scaffold;
* `crates/monad-core/` scaffold;
* minimal CLI entrypoint;
* minimal core library;
* initial Rust tests;
* formatting and test verification;
* beginner-readable comments where Rust concepts first appear.

## Deliverables

Expected deliverables include:

* root `Cargo.toml`;
* `crates/monad-cli/Cargo.toml`;
* `crates/monad-cli/src/main.rs`;
* `crates/monad-core/Cargo.toml`;
* `crates/monad-core/src/lib.rs`;
* initial Rust unit tests;
* updated verification command guidance.

## Expected Result After Verification

The Rust workspace builds and tests successfully, with `monad-cli` depending on `monad-core` and durable runtime logic beginning in `monad-core`.

## Verification

Run:

```bash
cargo fmt --check
cargo test
cargo run -p monad-cli
tools/scripts/verify.sh
```

Expected output from the CLI includes:

```text
Monad runtime foundation ready (crate: monad-core, model: local-first)
```

## Status

Complete

## Priority

High

## Size

M
