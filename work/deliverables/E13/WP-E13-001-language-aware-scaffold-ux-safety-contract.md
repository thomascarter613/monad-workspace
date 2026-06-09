---
title: WP-E13-001 Deliverable Record
epic: E13
work_packet: WP-E13-001
status: complete
---

# Deliverable Record: WP-E13-001

## Work packet

WP-E13-001 — Define language-aware scaffold UX and safety contract.

## Purpose

Create the documentation contract for E13 before changing Rust source code.

## Files created

```text
docs/commands/ADD-LANGUAGE.md
docs/workflows/LANGUAGE-AWARE-SCAFFOLDS.md
work/learning/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md
work/deliverables/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md
```

## Scope completed

This packet defines:

- supported initial language IDs;
- target CLI shape;
- safety boundaries;
- unsupported behavior;
- dry-run behavior;
- `--yes` write behavior;
- planned scaffold file sets;
- deferred root manifest and lockfile behavior;
- recommended E13 implementation order.

## Supported initial language IDs

```text
rust
typescript
python
go
```

## Explicitly deferred

The following are deferred beyond WP-E13-001:

- Rust implementation;
- language option parsing;
- language-aware dry-run plans;
- language-specific file writes;
- root workspace membership mutation;
- package installs;
- native toolchain execution;
- lockfile generation;
- template registries;
- network calls.

## Verification

Recommended verification after this packet:

```bash
test -f docs/commands/ADD-LANGUAGE.md
test -f docs/workflows/LANGUAGE-AWARE-SCAFFOLDS.md
test -f work/learning/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md
test -f work/deliverables/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md
grep -R "rust" docs/commands/ADD-LANGUAGE.md
grep -R "typescript" docs/commands/ADD-LANGUAGE.md
grep -R "python" docs/commands/ADD-LANGUAGE.md
grep -R "go" docs/commands/ADD-LANGUAGE.md
git diff --check
```

If E12 verification has not been run recently, also run:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-add.sh
tools/scripts/verify-e12.sh
tools/scripts/verify.sh
git status --short
```

## Next packet

WP-E13-002 — Add language option parsing and dry-run model.
