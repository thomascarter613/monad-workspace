---
title: "Current State"
document_type: "context-artifact"
artifact_type: "current-state"
status: "current"
generated: true
reviewed: false
epic: "E2"
source_files:
  - "crates/monad-core/src/lib.rs"
  - "monad.toml"
  - "work/epics/"
---

# Current State

## Project

Monad is AI-native, repo-native, local-first Software Foundry OS for understanding, verifying, and safely evolving software repositories.

## Completed Epics

E0 — Project Foundation is complete.
E1 — Runtime Foundation is complete.

## Current Epic

E2 — Repository Intelligence Foundation

## Active Focus

The current focus is E2 — Repository Intelligence Foundation.

## Runtime Capabilities

Public modules in `monad-core`:

- `checks`
- `context`
- `dependency_detection`
- `diagnostics`
- `error`
- `manifest`
- `output`
- `repo_contract`
- `repository_context_pack`
- `repository_graph`
- `repository_inspection`
- `repository_policy`
- `toolchain_detection`
- `workspace`

## Verification

Run:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```
