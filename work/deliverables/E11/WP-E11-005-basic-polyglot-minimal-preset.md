---
title: "WP-E11-005 Basic and Polyglot-Minimal Preset Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E11
work_packet: WP-E11-005
tags:
  - monad
  - e11
  - init
  - preset
  - scaffold
related:
  - crates/monad-core/src/init.rs
  - crates/monad-cli/src/main.rs
  - docs/commands/INIT.md
  - docs/commands/INIT-PRESETS.md
---

# WP-E11-005 Basic and Polyglot-Minimal Preset Deliverable

## Work Packet

WP-E11-005 — Add basic/polyglot-minimal preset.

## Outcome

Implemented.

## Summary

This work packet hardens the initial `monad init` preset UX.

The implemented preset surface is:

```text
basic
minimal
polyglot-minimal
```

`basic` is the recommended user-facing name for the minimal scaffold.

`minimal` remains supported as an equivalent alias for the same scaffold.

`polyglot-minimal` remains the first monorepo-shaped scaffold preset.

## Deliverables

- `crates/monad-core/src/init.rs`
- `crates/monad-cli/src/main.rs`
- `docs/commands/INIT.md`
- `docs/commands/INIT-PRESETS.md`
- `docs/project/MVP-COMMAND-REFERENCE.md`
- `README.md`
- `work/deliverables/E11/WP-E11-005-basic-polyglot-minimal-preset.md`

## Safety Boundary

This work packet adds no new destructive behavior.

The existing guarded write model remains:

- `--dry-run` previews;
- `--yes` applies only after conflict checks;
- existing target files block writes;
- no Git commands are run;
- no packages are installed;
- no remote services are called.

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- init --preset=basic --dry-run
cargo run -p monad-cli -- init --preset=minimal --dry-run
cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run
cargo run -p monad-cli -- --help | grep "init --preset=basic --dry-run"
tools/scripts/verify.sh
git status --short
```

For write-path verification, use an empty temporary directory.

## Recommended Commit

```bash
git commit -m "feat(init): add basic preset alias"
```

## Closeout Note

WP-E11-005 is complete once the preset alias/docs are committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E11-006 — Add init smoke tests and verification evidence
```
