# E25 — Dependency Graph and Impact Analysis Foundation

## Product Area

Dependency Graph and Impact Analysis Foundation

## Objective

Add Monad's first local-only model for dependency graph planning, changed-file
impact analysis, and impacted verification recommendations.

## Work Packets

- WP-E25-001 — Define dependency and impact graph model
- WP-E25-002 — Add component dependency edge detection
- WP-E25-003 — Add task-to-component graph linkage
- WP-E25-004 — Add changed-file impact analysis
- WP-E25-005 — Add impacted verification recommendation output
- WP-E25-006 — Add graph/impact fixtures and smoke tests

## Delivered Behavior

- `crates/monad-core/src/dependency_impact.rs`
- `monad impact --dry-run`
- `monad impact --dry-run --format=json`
- `monad impact --yes`
- `monad dependency-impact --dry-run`
- `tools/scripts/verify-dependency-impact.sh`
- `tools/scripts/verify-e25.sh`

## Safety

E25 remains local-first, deterministic, and supervised. Monad does not execute Git
or native tooling to discover changed files. Changed-file impact analysis reads an
optional local `.monad/changed-files.txt` file. Generated evidence writes remain
approval-gated and limited to `.monad/reports`.
