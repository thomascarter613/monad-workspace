---
title: "WP-E11-002 Init Dry-Run Plan Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E11
work_packet: WP-E11-002
tags:
  - monad
  - e11
  - init
  - dry-run
  - rust
related:
  - crates/monad-core/src/init.rs
  - crates/monad-core/src/lib.rs
  - crates/monad-cli/src/main.rs
  - docs/commands/INIT.md
---

# WP-E11-002 Init Dry-Run Plan Deliverable

## Work Packet

WP-E11-002 — Add init dry-run plan.

## Outcome

Implemented.

## Summary

This work packet adds the first implemented `monad init` behavior.

The command is intentionally dry-run only:

```bash
cargo run -p monad-cli -- init --dry-run
```

It supports:

```bash
cargo run -p monad-cli -- init --dry-run
cargo run -p monad-cli -- init --preset=minimal --dry-run
cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run
cargo run -p monad-cli -- init --name=my-project --dry-run
```

It rejects write approval for now:

```bash
cargo run -p monad-cli -- init --dry-run --yes
```

## Deliverables

- `crates/monad-core/src/init.rs`
- `crates/monad-core/src/lib.rs`
- `crates/monad-cli/src/main.rs`
- `docs/commands/INIT.md`
- `docs/project/MVP-COMMAND-REFERENCE.md`
- `README.md`
- `work/deliverables/E11/WP-E11-002-init-dry-run-plan.md`

## Safety Boundary

WP-E11-002 adds no write behavior.

The command:

- writes no files;
- does not create directories;
- does not initialize Git;
- does not commit;
- does not apply templates;
- rejects `--yes`;
- uses the existing file-operation dry-run evaluator.

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- init --dry-run
cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run
cargo run -p monad-cli -- init --name=my-project --dry-run
cargo run -p monad-cli -- init --dry-run --yes
tools/scripts/verify.sh
git status --short
```

The `--yes` command is expected to fail with an actionable dry-run-only error.

## Recommended Commit

```bash
git commit -m "feat(init): add init dry-run plan"
```

## Closeout Note

WP-E11-002 is complete once the init dry-run plan is committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E11-003 — Add minimal embedded scaffold templates
```
