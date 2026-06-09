# E27 — Build Cache and Incremental Execution Foundation

## Product Area

Build Cache and Incremental Execution Foundation

## Objective

Add Monad's first local-only model for cache contracts, deterministic task
fingerprints, local execution metadata, cache-aware dry-run planning, and
incremental execution recommendations.

## Work Packets

- WP-E27-001 — Define cache and incremental execution contract
- WP-E27-002 — Add task fingerprint model
- WP-E27-003 — Add local execution metadata store
- WP-E27-004 — Add cache-aware dry-run planning
- WP-E27-005 — Add incremental execution proof of concept
- WP-E27-006 — Add cache evidence and invalidation tests

## Delivered Behavior

- `crates/monad-core/src/build_cache.rs`
- `monad cache-plan --dry-run`
- `monad cache-plan --dry-run --format=json`
- `monad cache-plan --yes`
- `monad build-cache --dry-run`
- `monad incremental-plan --dry-run`
- `tools/scripts/verify-build-cache.sh`
- `tools/scripts/verify-e27.sh`

## Safety

E27 remains supervised and planning-only. Monad computes fingerprints and
decisions, but does not execute tasks or restore artifacts. Generated evidence
writes remain approval-gated and limited to `.monad/reports` and `.monad/cache`.
