# E28 — Local Artifact and Report Store Foundation

## Product Area

Local Artifact and Report Store Foundation

## Objective

Add Monad's local `.monad/reports` and `.monad/artifacts` store contract,
metadata schemas, retention policy, deterministic index foundation, and smoke
tests.

## Work Packets

- WP-E28-001 — Define `.monad/reports` and `.monad/artifacts` contract
- WP-E28-002 — Add report metadata schema
- WP-E28-003 — Add artifact metadata schema
- WP-E28-004 — Add report writing and retention policy
- WP-E28-005 — Add report index and lookup command foundation
- WP-E28-006 — Add artifact/report store smoke tests

## Delivered Behavior

- `crates/monad-core/src/report_store.rs`
- `monad report-store --dry-run`
- `monad report-store --dry-run --format=json`
- `monad report-store --yes`
- `monad reports --dry-run`
- `monad artifacts --dry-run`
- `tools/scripts/verify-report-store.sh`
- `tools/scripts/verify-e28.sh`

## Safety

E28 indexes local store metadata and writes generated index evidence only. It
does not upload, delete, sync, or rewrite existing report/artifact content.
