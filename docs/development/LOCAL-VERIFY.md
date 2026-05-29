---
title: Local Verification Guide
description: Local verification guide for Monad internal MVP candidate preparation.
status: draft
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
phase: MVP Candidate Cut
epic: E8
work_packet: WP-E8-004
---

# Local Verification Guide

## 1. Purpose

This guide explains how to verify Monad locally during internal MVP candidate preparation.

Verification is required before treating Monad as an internal MVP candidate.

## 2. Baseline verification command set

Run from the repository root:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

## 3. Formatting

Check formatting:

```bash
cargo fmt --check
```

Apply formatting when needed:

```bash
cargo fmt
```

After formatting, rerun:

```bash
cargo fmt --check
```

## 4. Tests

Run all tests:

```bash
cargo test
```

Run CLI smoke tests directly:

```bash
cargo test -p monad-cli --test cli_smoke -- --nocapture
```

Run CLI binary tests directly:

```bash
cargo test -p monad-cli --bin monad -- --nocapture
```

## 5. Clippy

Run strict Clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Warnings should be treated as blockers unless a work packet explicitly records a justified exception.

## 6. Root verifier

Run the root verification script:

```bash
tools/scripts/verify.sh
```

This is the preferred single verification entry point for release-readiness work.

## 7. No-write verification

For current planning and dry-run commands:

```bash
tools/scripts/verify-no-write-commands.sh
```

This verifies that current no-write commands do not change Git status.

## 8. Command smoke verification

Manually sample the current MVP command surface:

```bash
cargo run -p monad-cli -- --help
cargo run -p monad-cli -- version
cargo run -p monad-cli -- info
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- check --format=json
cargo run -p monad-cli -- graph --format=mermaid
cargo run -p monad-cli -- context
cargo run -p monad-cli -- plan "explain this repository"
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run
```

## 9. Working tree check

Before and after verification, check:

```bash
git status --short
```

Expected result:

* no unrelated changes
* generated artifacts intentionally committed or intentionally ignored
* no accidental report churn

## 10. Verification failure rule

Do not hide verification failures.

When a failure occurs:

1. Copy the failing command.
2. Copy the relevant error output.
3. Identify whether it is a code, docs, generated-artifact, or environment issue.
4. Fix only within the active work packet scope.
5. Record blockers honestly when not fixed.

## 11. Internal MVP candidate verification checklist

Before closing an E8 work packet:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
git status --short
```

## 12. Related documents

* `docs/development/LOCAL-BUILD.md`
* `docs/project/MVP-SCOPE-FREEZE.md`
* `docs/project/MVP-COMMAND-REFERENCE.md`
* `docs/release/VERSIONING.md`
