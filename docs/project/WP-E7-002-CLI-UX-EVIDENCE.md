---
title: WP-E7-002 CLI UX Evidence
description: Evidence record for normalizing Monad CLI help and command UX during MVP hardening.
status: draft
version: 0.1.0
created: 2026-05-28
updated: 2026-05-28
owner: Thomas Carter
project: Monad
phase: MVP Hardening
epic: E7
work_packet: WP-E7-002
---

# WP-E7-002 CLI UX Evidence

## 1. Purpose

This document records the CLI UX hardening performed for WP-E7-002.

The goal of this packet is to make Monad's command help, missing-argument errors, unsupported flag behavior, and dry-run messaging better match the current implemented MVP command surface.

## 2. Problem addressed

The E7 foundation closure audit identified root verification failure as a hardening blocker.

A likely contributor was that the CLI command surface had moved faster than the help text and user-facing command contract.

Specifically, the CLI supported planning and dry-run evolution behavior, but help output did not clearly list the current MVP planning and evolution commands.

## 3. Changes made

WP-E7-002 updates the CLI UX contract by:

- listing `plan "<intent>"` in help output
- listing `evolve verify-baseline --dry-run` in help output
- listing `evolve context-baseline --dry-run` in help output
- adding explicit examples for current MVP commands
- adding safety notes explaining that `plan` is no-write
- adding safety notes explaining that `evolve` commands are dry-run only
- improving dry-run-required errors so each evolve command names itself correctly
- adding tests for plan parsing
- adding tests for missing plan intent
- adding tests for dry-run-required errors
- adding tests that help text mentions planning and dry-run evolution commands

## 4. Non-goals

This packet does not add:

- new major commands
- autonomous agent execution
- write/apply evolution behavior
- external provider integrations
- full CLI redesign

## 5. Verification commands

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings

cargo run -p monad-cli -- --help
cargo run -p monad-cli -- plan
cargo run -p monad-cli -- plan "explain this repository"
cargo run -p monad-cli -- evolve verify-baseline
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline
cargo run -p monad-cli -- evolve context-baseline --dry-run

tools/scripts/verify.sh
```

## 6. Expected result

* Formatting passes.
* Tests pass.
* Clippy passes.
* `monad --help` lists current MVP commands.
* `monad plan` returns an actionable missing-intent error.
* `monad plan "explain this repository"` succeeds.
* `monad evolve verify-baseline` returns a dry-run-required error.
* `monad evolve context-baseline` returns a command-specific dry-run-required error.
* Dry-run evolution commands succeed or report legitimate repository-state issues.
