---
title: E22 — Repo Contract Schema and Validation Foundation
status: implemented
last_reviewed: 2026-06-09
---

# E22 — Repo Contract Schema and Validation Foundation

## Scope

E22 implements the foundation for repository contract schema validation and generated state.

## Work packets

- WP-E22-001 — Define repository contract schema boundary
- WP-E22-002 — Add `monad.toml` schema validation
- WP-E22-003 — Add `monad.lock` / generated state model
- WP-E22-004 — Add schema migration planning model
- WP-E22-005 — Add contract validation fixtures
- WP-E22-006 — Add contract validation reports and smoke tests

## Delivered behavior

- Added `monad contract --dry-run` for deterministic human-readable validation plans.
- Added `monad contract --dry-run --format=json` for deterministic machine-readable plans.
- Added `monad contract --yes` for generated contract evidence/state only.
- Added a core `contract_schema` module with schema boundary, lock/generated-state model, migration planning, reports, and tests.
- Added local verification scripts for contract schema behavior and E22 closeout.

## Safety notes

E22 remains local-first and supervised. It does not mutate user-owned source, does not rewrite `monad.toml`, does not run destructive migrations, does not fetch remote schema definitions, and does not call AI providers.

## Verification

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-contract-schema.sh
tools/scripts/verify-e22.sh
```
