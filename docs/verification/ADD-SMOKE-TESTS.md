---
title: "Add Command Smoke Tests"
document_type: "verification-evidence"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-005
tags:
  - monad
  - add
  - smoke-tests
  - verification
related:
  - tools/scripts/verify-add.sh
  - docs/commands/ADD.md
  - crates/monad-core/src/component_add.rs
  - crates/monad-cli/src/main.rs
---

# Add Command Smoke Tests

## Status

Accepted.

## Work Packet

WP-E12-005 — Add add-command smoke verification.

## Purpose

This document records the smoke verification path for `monad add`.

The purpose is to prove that the `add` command is safe enough to continue building on.

## Verification Script

The reusable smoke-test script is:

```text
tools/scripts/verify-add.sh
```

Run it from the repository root:

```bash
tools/scripts/verify-add.sh
```

## Coverage

The script verifies:

| Area | Coverage |
| --- | --- |
| Dry-run preview | `monad add app web --dry-run` |
| Workspace precondition | `monad add app web --yes` fails in an uninitialized temp directory |
| Guarded write | `monad init --yes`, then `monad add app web --yes` in a temp workspace |
| Dry-run non-write | `monad add service api --dry-run` creates no files |
| Conflict refusal | Existing `apps/api/README.md` blocks `monad add app api --yes` |
| Mode conflict | `monad add app web --dry-run --yes` fails |

## Expected Add Write Output

A successful guarded add should report:

```text
Monad add applied
```

It should also report:

```text
No Git commands were run.
```

## Expected Files

For:

```bash
monad add app web --yes
```

inside an initialized Monad workspace, expected files are:

```text
apps/web/README.md
apps/web/.gitkeep
```

## Safety Properties Verified

The smoke test verifies that:

- `add` previews before writing;
- `add --yes` works only inside a Monad workspace;
- `add --dry-run` writes nothing;
- `add --yes` refuses conflicts;
- `add --dry-run --yes` is rejected;
- verification writes only into temp directories.

## Full Verification Command Set

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-add.sh
tools/scripts/verify.sh
git status --short
```

## Acceptance Criteria

WP-E12-005 is complete when:

- `tools/scripts/verify-add.sh` exists;
- this verification evidence document exists;
- the deliverable record exists;
- add dry-run is smoke-tested;
- add guarded write is smoke-tested;
- add conflict refusal is smoke-tested;
- all normal verification commands pass.

## Outcome

Accepted.

Monad now has a reusable add-command smoke verification script and evidence record.
