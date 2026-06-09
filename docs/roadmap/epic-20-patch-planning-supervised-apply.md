# E20 — Patch Planning and Supervised Apply Foundation

## Product area

Patch Planning and Supervised Apply Foundation

## Objective

Advance Monad from safe planning/evidence into a supervised local patch foundation while preserving governance-grade safety boundaries.

## Implemented work packets

- WP-E20-001 — Define patch planning and supervised apply contract.
- WP-E20-002 — Add change-set and patch diff model.
- WP-E20-003 — Add patch dry-run plan output.
- WP-E20-004 — Add patch validation and conflict checks.
- WP-E20-005 — Add supervised apply under approval gates.
- WP-E20-006 — Add patch evidence reports and smoke tests.

## Command surface

```bash
monad patch --dry-run
monad patch --dry-run --format=json
monad patch --yes
```

## Safety posture

E20 keeps patch behavior local-first and supervised:

- no autonomous patch application;
- no silent overwrites;
- no destructive file deletion;
- no remote patch fetching;
- no arbitrary script execution;
- no AI-provider patch execution;
- no user-owned source mutation in this foundation slice;
- generated writes require explicit `--yes` and E19 approval gates.

## Evidence outputs

`monad patch --yes` writes generated local artifacts only:

- `.monad/patches/e20-supervised-apply-foundation.md`
- `.monad/reports/patch-plan.md`
- `.monad/reports/patch-plan.json`

Existing files with different content are treated as conflicts and are not overwritten.

## Verification

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-patch.sh
tools/scripts/verify-e20.sh
```
