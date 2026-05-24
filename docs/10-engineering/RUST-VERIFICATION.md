---
title: "Rust Verification"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - engineering
  - rust
  - verification
related:
  - docs/10-engineering/RUST-CODING-STANDARD.md
  - docs/12-verification/VERIFICATION-MODEL.md
  - docs/07-workflow/DEFINITION-OF-DONE.md
---

# Rust Verification

## Purpose

This document defines the initial Rust verification standard for Monad.

Rust work is not complete until formatting, tests, and linting pass unless an exception is documented.

## Minimum Rust Verification

Every Rust implementation slice should run:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Formatting

Check formatting:

```bash
cargo fmt --check
```

Apply formatting:

```bash
cargo fmt
```

Formatting failures should be fixed before commit.

## Tests

Run tests:

```bash
cargo test
```

Tests should be added for core behavior where practical.

Good test targets:

- diagnostics;
- errors;
- workspace resolution;
- manifest parsing;
- toolchain detection;
- graph models;
- check results;
- command runner;
- file operation planning;
- context generation.

## Clippy

Run Clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Warnings are treated as errors.

This protects early code quality and prevents warnings from becoming normal.

## CLI Smoke Tests

When CLI behavior changes, run the relevant command.

Examples:

```bash
cargo run -p monad-cli -- --help
cargo run -p monad-cli -- info
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- check
```

A CLI command is not verified until it has been manually run or covered by tests.

## Expected Result Format

Every work packet should state expected verification results.

Example:

```text
Formatting passes, tests pass, Clippy passes with warnings denied, and `cargo run -p monad-cli -- --help` prints help output.
```

## Test Naming

Test names should describe behavior.

Good:

```rust
creates_warning_diagnostic
returns_error_when_workspace_root_missing
detects_cargo_workspace_manifest
```

Weak:

```rust
test_one
works
basic
```

## When Verification Fails

If verification fails:

1. Read the error.
2. Identify whether the failure is caused by the current work.
3. Fix the failure if it is in scope.
4. If unrelated, document it clearly.
5. Do not mark the work packet Done until the exception is understood.

## Verification Evidence

Verification evidence may include:

- command output;
- generated reports;
- passing test results;
- manual smoke test output;
- screenshots only when relevant;
- linked CI run later.

During early local development, command output and clean exit codes are enough.

## Future Verification Enhancements

Later Monad may add:

- `tools/scripts/verify.sh`;
- GitHub Actions;
- fixture-based integration tests;
- snapshot tests;
- JSON output validation;
- documentation frontmatter checks;
- repo contract checks;
- generated evidence packets;
- release checks.

## Current Status

This Rust verification document is a draft. It is authoritative enough for E1 Rust implementation and should be refined when a root verification script exists.
