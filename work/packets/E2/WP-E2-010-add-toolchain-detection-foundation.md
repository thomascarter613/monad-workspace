---
title: "WP-E2-010 — Add Toolchain Detection Foundation"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-010"
---

# WP-E2-010 — Add Toolchain Detection Foundation

## Product Area

Repository intelligence, toolchain detection, bounded traversal, and inspect output.

## Objective

Add conservative file-pattern-based toolchain detection to `monad-core`.

## Scope

This work packet adds:

- `RepositoryToolchainKind`;
- `RepositoryToolchainSignalKind`;
- `RepositoryToolchainSignal`;
- `RepositoryToolchainDetection`;
- `detect_repository_toolchains`;
- detection for Rust, JavaScript, TypeScript, Python, Go, Java, PHP, and Ruby;
- toolchain metrics in `monad inspect`.

This work packet does not invoke external tools.

## Expected Result After Verification

After verification:

- `cargo test` passes;
- common toolchains are detected from bounded traversal signals;
- `monad inspect` text output includes `toolchains:`;
- `monad inspect --format=json` includes `toolchains`;
- `tools/scripts/verify.sh` finishes with `Verification baseline passed.`.

## Priority

High.

## Size

Medium.
