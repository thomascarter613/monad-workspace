---
title: WP-E7-003 Command Smoke Test Evidence
description: Evidence record for CLI smoke test hardening during E7 MVP hardening.
status: draft
version: 0.1.0
created: 2026-05-28
updated: 2026-05-28
owner: Thomas Carter
project: Monad
phase: MVP Hardening
epic: E7
work_packet: WP-E7-003
---

# WP-E7-003 Command Smoke Test Evidence

## 1. Purpose

This document records the command smoke test hardening performed for WP-E7-003.

The goal is to catch regressions in Monad's current MVP command surface before release-readiness work.

## 2. Smoke coverage added

This packet adds integration-style CLI smoke tests for the compiled `monad` binary.

Covered success paths:

- `monad --help`
- `monad help`
- `monad version`
- `monad --version`
- `monad inspect`
- `monad check`
- `monad plan "explain this repository"`
- `monad evolve verify-baseline --dry-run`
- `monad evolve context-baseline --dry-run`

Covered failure paths:

- `monad plan`
- `monad plan "..." --format=json`
- `monad evolve verify-baseline`
- `monad evolve context-baseline`
- `monad inspect --wat`
- `monad inspect --write`

## 3. Intentional boundaries

This packet does not add:

- full end-to-end test framework
- real model-provider tests
- MCP server tests
- remote Git tests
- deployment tests
- arbitrary shell execution tests

## 4. Verification commands

```bash
cargo fmt --check
cargo test -p monad-cli
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

5. Expected result
CLI smoke tests pass.
Full test suite passes.
Clippy passes with warnings denied.
Root verification script either passes or reports a remaining blocker for a later E7 packet.
