---
title: MVP Command Reference
description: Current Monad MVP hardening command reference aligned with implemented CLI behavior.
status: draft
version: 0.1.0
created: 2026-05-28
updated: 2026-05-28
owner: Thomas Carter
project: Monad
phase: MVP Hardening
epic: E7
work_packet: WP-E7-004
---

# MVP Command Reference

## 1. Purpose

This document records the current implemented Monad command surface during E7 MVP hardening.

It is intentionally conservative. It should describe commands that exist now, not commands planned for future phases.

## 2. Cargo package and binary

The CLI package is:

```bash
monad-cli
```

The binary name is:

```bash
monad
```

During local development, run commands through Cargo:

```bash
cargo run -p monad-cli -- <command>
```

## 3. Current core commands

### Help

```bash
cargo run -p monad-cli -- --help
cargo run -p monad-cli -- help
```

Expected behavior:

* prints current command help
* lists core commands
* lists context commands
* lists dry-run evolution commands
* includes safety notes

### Version

```bash
cargo run -p monad-cli -- version
cargo run -p monad-cli -- --version
```

Expected behavior:

* prints Monad runtime identity/version information

### Info

```bash
cargo run -p monad-cli -- info
cargo run -p monad-cli -- info --format=json
```

Expected behavior:

* prints workspace summary
* supports text and JSON output

### Inspect

```bash
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- inspect --format=json
```

Expected behavior:

* inspects repository structure
* supports text and JSON output

### Check

```bash
cargo run -p monad-cli -- check
cargo run -p monad-cli -- check --format=json
```

Expected behavior:

* runs workspace checks
* text output writes check evidence to `.monad/reports/latest-check-evidence.md`
* JSON output prints check report JSON

## 4. Graph command

```bash
cargo run -p monad-cli -- graph
cargo run -p monad-cli -- graph --format=json
cargo run -p monad-cli -- graph --format=mermaid
cargo run -p monad-cli -- graph --format=dot
```

Expected behavior:

* renders repository graph information
* supports text, JSON, Mermaid, and DOT formats

## 5. Context commands

### Render context

```bash
cargo run -p monad-cli -- context
cargo run -p monad-cli -- context --format=json
```

Expected behavior:

* renders an AI-readable repository context pack
* supports Markdown/text aliases and JSON

### Export context

```bash
cargo run -p monad-cli -- context --write
```

Expected behavior:

* writes repository context pack artifacts
* `--write` is only supported for the `context` command

### Generate current-state artifact

```bash
cargo run -p monad-cli -- context generate current-state
```

Expected behavior:

* writes `.monad/context/current-state.md`

### Generate handoff artifact

```bash
cargo run -p monad-cli -- context generate handoff
```

Expected behavior:

* writes `.monad/context/latest-handoff.md`

### Generate bootstrap prompt

```bash
cargo run -p monad-cli -- context generate bootstrap
```

Expected behavior:

* writes `docs/ai/BOOTSTRAP-PROMPT.md`

### Assemble context pack

```bash
cargo run -p monad-cli -- context pack
```

Expected behavior:

* writes `.monad/context/latest-context-pack.md`

### Verify context

```bash
cargo run -p monad-cli -- context verify
```

Expected behavior:

* verifies required context files and structural expectations

## 6. Agent planning command

```bash
cargo run -p monad-cli -- plan "explain this repository"
```

Expected behavior:

* produces a supervised no-write plan
* does not create, update, or delete files
* does not run shell commands
* does not mutate Git state
* does not call a real model provider or external AI API

Known limitation:

```bash
cargo run -p monad-cli -- plan
```

returns an actionable missing-intent error.

Current limitation:

```bash
cargo run -p monad-cli -- plan "explain this repository" --format=json
```

returns an unsupported-format error because plan output does not support `--format` yet.

## 7. Evolution dry-run commands

### Verification baseline dry-run

```bash
cargo run -p monad-cli -- evolve verify-baseline --dry-run
```

Expected behavior:

* previews verification baseline file operations
* does not write files

Without `--dry-run`, the command fails intentionally:

```bash
cargo run -p monad-cli -- evolve verify-baseline
```

### Context baseline dry-run

```bash
cargo run -p monad-cli -- evolve context-baseline --dry-run
```

Expected behavior:

* previews context baseline file operations
* does not write files

Without `--dry-run`, the command fails intentionally:

```bash
cargo run -p monad-cli -- evolve context-baseline
```

## 8. Safety boundaries

Current MVP hardening safety boundaries:

* planning is no-write
* evolution commands are dry-run only
* `--write` is limited to `context`
* no apply command exists yet
* no autonomous agent execution exists yet
* no remote Git operation exists yet
* no MCP server exists yet
* no deployment command exists yet

## 9. Verification commands

Use this command set to verify the current MVP command surface:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings

cargo run -p monad-cli -- --help
cargo run -p monad-cli -- version
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- check
cargo run -p monad-cli -- plan "explain this repository"
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run

tools/scripts/verify.sh
```

