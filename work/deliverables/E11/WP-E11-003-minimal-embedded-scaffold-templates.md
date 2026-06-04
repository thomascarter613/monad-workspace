---
title: "WP-E11-003 Minimal Embedded Scaffold Templates Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E11
work_packet: WP-E11-003
tags:
  - monad
  - e11
  - init
  - templates
  - scaffold
related:
  - crates/monad-core/src/templates/registry.rs
  - crates/monad-core/src/init.rs
  - docs/commands/INIT.md
---

# WP-E11-003 Minimal Embedded Scaffold Templates Deliverable

## Work Packet

WP-E11-003 — Add minimal embedded scaffold templates.

## Outcome

Implemented.

## Summary

This work packet extends the existing embedded template registry with init scaffold templates and updates the `monad init --dry-run` plan to source planned file operations from those templates.

The implementation remains dry-run only.

## Deliverables

- `crates/monad-core/src/templates/registry.rs`
- `crates/monad-core/src/init.rs`
- `docs/commands/INIT.md`
- `docs/project/MVP-COMMAND-REFERENCE.md`
- `work/deliverables/E11/WP-E11-003-minimal-embedded-scaffold-templates.md`

## Embedded Init Templates

Minimal preset templates:

```text
init.minimal.monad-toml
init.minimal.readme
init.minimal.docs-readme
init.minimal.work-readme
init.minimal.monad-gitignore
```

Polyglot-minimal additional templates:

```text
init.polyglot.apps-gitkeep
init.polyglot.packages-gitkeep
init.polyglot.services-gitkeep
init.polyglot.tools-gitkeep
```

## Safety Boundary

WP-E11-003 adds template source material and planning integration only.

It does not:

- write files;
- create directories;
- apply templates;
- initialize Git;
- commit changes;
- enable `--yes`.

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- init --dry-run
cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run
cargo run -p monad-cli -- init --dry-run | grep "Template source"
cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run | grep "apps/.gitkeep"
tools/scripts/verify.sh
git status --short
```

## Recommended Commit

```bash
git commit -m "feat(init): add embedded scaffold templates"
```

## Closeout Note

WP-E11-003 is complete once the embedded scaffold templates are committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E11-004 — Add guarded init write path
```
