---
title: Build Metadata
description: Reviewable build and version metadata record for Monad internal MVP candidate preparation.
status: draft
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
phase: MVP Candidate Cut
epic: E8
work_packet: WP-E8-003
---

# Build Metadata

## 1. Purpose

This document records the version and build metadata expectations for the Monad internal MVP candidate.

## 2. Package identity

| Item | Value |
|---|---|
| Product | Monad |
| CLI package | `monad-cli` |
| CLI binary | `monad` |
| Core package | `monad-core` |
| MCP placeholder package | `monad-mcp` |
| Current version line | `0.1.0` |
| Internal candidate identifier | `0.1.0-internal-mvp-candidate` |

## 3. Expected local version command

```bash
cargo run -p monad-cli -- version
```
Expected result:

* command succeeds
* output identifies Monad
* output reports the compiled version
* output does not claim public release status

## 4. Build metadata boundaries

The internal MVP candidate does not include:

* binary signing
* installer metadata
* package registry metadata hardening
* release artifact checksums
* SBOM generation
* public publishing automation

Those may be added in later release-preparation work.

## 5. Verification commands

```bash
cargo run -p monad-cli -- version
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

