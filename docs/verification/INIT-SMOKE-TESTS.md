---
title: "Init Smoke Tests"
document_type: "verification-evidence"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E11
work_packet: WP-E11-006
tags:
  - monad
  - init
  - smoke-tests
  - verification
related:
  - tools/scripts/verify-init.sh
  - docs/commands/INIT.md
  - docs/commands/INIT-PRESETS.md
  - crates/monad-core/src/init.rs
  - crates/monad-cli/src/main.rs
---

# Init Smoke Tests

## Status

Accepted.

## Work Packet

WP-E11-006 — Add init smoke tests and verification evidence.

## Purpose

This document records the smoke-test evidence path for `monad init`.

The goal is to verify that the E11 init foundation behaves safely and predictably before closing the epic.

## Verification Script

The reusable smoke-test script is:

```text
tools/scripts/verify-init.sh
```

Run it from the repository root:

```bash
tools/scripts/verify-init.sh
```

## Coverage

The script verifies:

| Area | Coverage |
| --- | --- |
| Dry-run command | `monad init --preset=basic --dry-run` |
| Minimal alias | `monad init --preset=minimal --dry-run` |
| Polyglot dry-run | `monad init --preset=polyglot-minimal --dry-run` |
| Basic guarded write | `monad init --preset=basic --yes` in a temporary directory |
| Polyglot guarded write | `monad init --preset=polyglot-minimal --yes` in a temporary directory |
| Conflict refusal | Existing `README.md` blocks `init --yes` |
| Mode conflict | `init --dry-run --yes` fails |
| Help text | Help output mentions init, basic preset, and `--yes` |

## Safety Properties Verified

The smoke test verifies that:

- dry-run writes no files;
- guarded write creates the expected scaffold in an empty temp directory;
- guarded write refuses to overwrite existing target files;
- conflicting mode flags fail;
- polyglot-minimal creates expected monorepo placeholder directories;
- no verification command requires writing into the Monad repository itself.

## Expected Basic Scaffold

The `basic` preset creates:

```text
monad.toml
README.md
docs/README.md
work/README.md
.monad/.gitignore
```

## Expected Polyglot-Minimal Scaffold

The `polyglot-minimal` preset creates the basic scaffold plus:

```text
apps/.gitkeep
packages/.gitkeep
services/.gitkeep
tools/.gitkeep
```

## Full E11 Verification Command Set

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-init.sh
tools/scripts/verify.sh
git status --short
```

## Acceptance Criteria

WP-E11-006 is complete when:

- `tools/scripts/verify-init.sh` exists;
- the script verifies dry-run, guarded write, presets, conflict refusal, and help text;
- this verification evidence document exists;
- the E11 deliverable record exists;
- all normal repository verification commands pass.

## Outcome

Accepted.

Monad now has a reusable init smoke verification script and E11 verification evidence.
