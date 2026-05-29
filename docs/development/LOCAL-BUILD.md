---
title: Local Build Guide
description: Local build and run guide for Monad internal MVP candidate preparation.
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

# Local Build Guide

## 1. Purpose

This guide explains how to build and run Monad locally as an internal MVP candidate.

It does not describe public installation, package publishing, binary installers, hosted deployment, or cloud distribution.

## 2. Current release posture

Monad is currently an **internal MVP candidate**, not a public release.

Use this guide for local development and verification only.

## 3. Supported local assumptions

The current MVP candidate assumes:

- a local Git checkout of the repository
- a working Rust toolchain
- Cargo available on `PATH`
- shell access from the repository root
- no installer
- no published package
- no hosted service
- no autonomous agent runtime

## 4. Repository root

Run all commands from the repository root:

```bash
pwd
git status --short
```

The repository root should contain:

```text
Cargo.toml
crates/
docs/
tools/
monad.toml
```

## 5. Build the CLI

Build the CLI package:

```bash
cargo build -p monad-cli
```

Build all workspace packages:

```bash
cargo build
```

## 6. Run the CLI through Cargo

The CLI package is:

```text
monad-cli
```

The compiled binary is:

```text
monad
```

During local development, run Monad through Cargo:

```bash
cargo run -p monad-cli -- <command>
```

Examples:

```bash
cargo run -p monad-cli -- --help
cargo run -p monad-cli -- version
cargo run -p monad-cli -- info
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- check
cargo run -p monad-cli -- graph
cargo run -p monad-cli -- context
cargo run -p monad-cli -- plan "explain this repository"
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run
```

## 7. Run the compiled binary directly

After building, the development binary is usually available at:

```text
target/debug/monad
```

Example:

```bash
target/debug/monad version
```

Release builds are outside the current MVP candidate cut unless a later work packet explicitly prepares them.

## 8. Current command boundaries

The internal MVP candidate supports:

* workspace summary
* repository inspection
* workspace checks
* repository graph rendering
* repo-native context rendering and generation
* supervised no-write planning
* dry-run evolution previews

It does not support:

* apply/write evolution
* autonomous agent execution
* real model-provider execution by default
* MCP server release
* package publishing
* installer generation
* hosted service launch

## 9. Related documents

* `docs/development/LOCAL-VERIFY.md`
* `docs/project/MVP-SCOPE-FREEZE.md`
* `docs/project/MVP-COMMAND-REFERENCE.md`
* `docs/release/VERSIONING.md`
