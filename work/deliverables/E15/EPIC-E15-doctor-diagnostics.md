---
title: Epic E15 Deliverable Record
epic: E15
status: complete
---

# Epic E15 Deliverable Record

## Epic

E15 — Doctor and Environment Diagnostics Foundation.

## Completed work packets

- WP-E15-001 — Define `monad doctor` diagnostic contract
- WP-E15-002 — Add local tool detection foundation
- WP-E15-003 — Add Rust, Git, and repository readiness diagnostics
- WP-E15-004 — Add ecosystem diagnostics for Node/Bun/Python/Go/Java
- WP-E15-005 — Add Monad context and repo contract diagnostics
- WP-E15-006 — Add doctor report output and smoke tests

## Implementation files

```text
crates/monad-core/src/doctor.rs
crates/monad-core/src/lib.rs
crates/monad-cli/src/main.rs
```

## Documentation files

```text
docs/commands/DOCTOR.md
docs/architecture/DIAGNOSTICS-MODEL.md
docs/workflows/DOCTOR-WORKFLOW.md
docs/verification/DOCTOR-SMOKE-TESTS.md
docs/verification/E15-CLOSEOUT.md
```

## Verification files

```text
tools/scripts/verify-doctor.sh
tools/scripts/verify-e15.sh
```

## Verification command

```bash
tools/scripts/verify-e15.sh
```

## Next epic

E16.
