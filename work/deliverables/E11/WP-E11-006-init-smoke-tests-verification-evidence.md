---
title: "WP-E11-006 Init Smoke Tests and Verification Evidence Deliverable"
document_type: "deliverable-record"
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
  - e11
  - init
  - smoke-tests
  - verification
related:
  - tools/scripts/verify-init.sh
  - docs/verification/INIT-SMOKE-TESTS.md
  - docs/commands/INIT.md
  - docs/commands/INIT-PRESETS.md
---

# WP-E11-006 Init Smoke Tests and Verification Evidence Deliverable

## Work Packet

WP-E11-006 — Add init smoke tests and verification evidence.

## Outcome

Implemented.

## Summary

This work packet adds a reusable smoke-test script for the `monad init` foundation and records verification evidence.

The script verifies:

- dry-run behavior;
- basic preset behavior;
- minimal preset behavior;
- polyglot-minimal preset behavior;
- guarded write behavior in temporary directories;
- conflict refusal;
- conflicting mode flag rejection;
- help text coverage.

## Deliverables

- `tools/scripts/verify-init.sh`
- `docs/verification/INIT-SMOKE-TESTS.md`
- `work/deliverables/E11/WP-E11-006-init-smoke-tests-verification-evidence.md`

## Verification

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

## Recommended Commit

```bash
git commit -m "test(init): add init smoke verification"
```

## Closeout Note

WP-E11-006 is complete once the init smoke verification script and evidence docs are committed and the corresponding GitHub work-packet issue or tracking item is closed.

After WP-E11-006 is closed, E11 may be closed.

## Next Epic

```text
E12 — Component Add and Polyglot Scaffold Foundation
```
