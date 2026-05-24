---
title: "WP-E2-011 — Add Dependency Signal Detection Foundation"
document_type: "work-packet"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Thomas Carter"
epic: "E2"
work_packet: "WP-E2-011"
---

# WP-E2-011 — Add Dependency Signal Detection Foundation

## Product Area

Repository intelligence, dependency signals, bounded traversal, and inspect output.

## Objective

Add conservative dependency signal detection to `monad-core`.

## Scope

This work packet adds:

- `RepositoryDependencySignalKind`;
- `RepositoryDependencySignal`;
- `RepositoryDependencyDetection`;
- `detect_repository_dependency_signals`;
- detection for dependency manifests, lockfiles, package manager config files, and build files;
- dependency metrics in `monad inspect`.

This work packet does not parse dependency contents and does not invoke external package managers.

## Expected Result After Verification

After verification:

- `cargo test` passes;
- dependency signals are detected from bounded traversal output;
- `monad inspect` text output includes `dependencies:`;
- `monad inspect --format=json` includes `dependencies`;
- `tools/scripts/verify.sh` finishes with `Verification baseline passed.`.

## Priority

High.

## Size

Medium.
