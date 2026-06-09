# E26 — Test Intelligence and Verification Planning Foundation

## Product Area

Test Intelligence and Verification Planning Foundation

## Objective

Add Monad's first local-only model for test intelligence, manifest-based test
command discovery, component mapping, targeted verification planning, and
verification confidence evidence.

## Work Packets

- WP-E26-001 — Define test intelligence model
- WP-E26-002 — Discover test commands from manifests
- WP-E26-003 — Map tests to components/packages
- WP-E26-004 — Generate targeted verification plans
- WP-E26-005 — Add verification confidence/evidence model
- WP-E26-006 — Add verification planner smoke tests

## Delivered Behavior

- `crates/monad-core/src/test_intelligence.rs`
- `monad verify-plan --dry-run`
- `monad verify-plan --dry-run --format=json`
- `monad verify-plan --yes`
- `monad test-intelligence --dry-run`
- `monad verification-plan --dry-run`
- `tools/scripts/verify-test-intelligence.sh`
- `tools/scripts/verify-e26.sh`

## Safety

E26 remains local-first, deterministic, and supervised. Monad discovers likely
commands by inspecting local manifests, but it does not execute those commands.
Generated evidence writes remain approval-gated and limited to `.monad/reports`.
