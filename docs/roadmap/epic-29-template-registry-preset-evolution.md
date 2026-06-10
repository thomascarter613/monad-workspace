# E29 — Template Registry and Preset Evolution Foundation

## Product Area

Template Registry and Preset Evolution Foundation

## Objective

Add Monad's local template registry evolution contract, template metadata schema,
preset metadata schema, compatibility validation, preset upgrade planning, and
registry smoke tests.

## Work Packets

- WP-E29-001 — Define template registry evolution contract
- WP-E29-002 — Add template metadata schema
- WP-E29-003 — Add preset metadata schema
- WP-E29-004 — Add template compatibility validation
- WP-E29-005 — Add preset upgrade planning
- WP-E29-006 — Add template registry fixtures and tests

## Delivered Behavior

- `crates/monad-core/src/template_registry.rs`
- `monad template-registry --dry-run`
- `monad template-registry --dry-run --format=json`
- `monad template-registry --yes`
- `monad templates --dry-run`
- `monad presets --dry-run`
- `tools/scripts/verify-template-registry.sh`
- `tools/scripts/verify-e29.sh`

## Safety

E29 is planning-only. Monad discovers local metadata and writes generated registry evidence, but it does not fetch, render, apply, or upgrade templates or presets.
