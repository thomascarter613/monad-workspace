---
title: WP-E8-004 Local Build and Verification Evidence
description: Evidence record for adding Monad local build and verification documentation.
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

# WP-E8-004 Local Build and Verification Evidence

## 1. Purpose

This document records the evidence for WP-E8-004.

The goal is to document how to build, run, and verify Monad locally as an internal MVP candidate.

## 2. Files added or updated

```text
README.md
docs/development/README.md
docs/development/LOCAL-BUILD.md
docs/development/LOCAL-VERIFY.md
```

## 3. Scope satisfied

WP-E8-004 required:

* local build guide
* local run guide
* verification guide
* supported development assumptions
* command examples using `monad-cli` package and `monad` binary

## 4. Boundaries preserved

This work does not add:

* installer documentation
* public distribution documentation
* hosted deployment documentation
* cloud environment documentation
* package publishing instructions

## 5. Verification commands

```bash
grep -R "cargo run -p monad-cli" README.md docs || true
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

## 6. Expected result

* Local build documentation exists.
* Local verification documentation exists.
* README links to local development documentation.
* Command examples use `cargo run -p monad-cli --`.
* Verification passes or blockers are recorded honestly.
