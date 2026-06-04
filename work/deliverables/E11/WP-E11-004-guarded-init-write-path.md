---
title: "WP-E11-004 Guarded Init Write Path Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E11
work_packet: WP-E11-004
tags:
  - monad
  - e11
  - init
  - guarded-write
  - safety
related:
  - crates/monad-core/src/init.rs
  - crates/monad-core/src/lib.rs
  - crates/monad-cli/src/main.rs
  - docs/commands/INIT.md
---

# WP-E11-004 Guarded Init Write Path Deliverable

## Work Packet

WP-E11-004 — Add guarded init write path.

## Outcome

Implemented.

## Summary

This work packet adds the first guarded write path for `monad init`.

The command now supports:

```bash
cargo run -p monad-cli -- init --dry-run
cargo run -p monad-cli -- init --yes
```

The write path is conservative and aborts if the planned operation set has conflicts.

## Deliverables

- `crates/monad-core/src/init.rs`
- `crates/monad-core/src/lib.rs`
- `crates/monad-cli/src/main.rs`
- `docs/commands/INIT.md`
- `docs/project/MVP-COMMAND-REFERENCE.md`
- `README.md`
- `work/deliverables/E11/WP-E11-004-guarded-init-write-path.md`

## Safety Boundary

`monad init --yes`:

- writes only the selected embedded scaffold templates;
- refuses to overwrite existing target files;
- creates parent directories only for approved scaffold targets;
- runs no Git commands;
- does not commit;
- does not push;
- does not install packages;
- does not call remote services.

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings

tmpdir="$(mktemp -d)"
(
  cd "$tmpdir"
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- init --dry-run
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- init --yes
  test -f monad.toml
  test -f README.md
  test -f docs/README.md
  test -f work/README.md
  test -f .monad/.gitignore
)
rm -rf "$tmpdir"

tools/scripts/verify.sh
git status --short
```

Adjust `/data/monad-workspace/Cargo.toml` if your repository path differs.

## Recommended Commit

```bash
git commit -m "feat(init): add guarded init write path"
```

## Closeout Note

WP-E11-004 is complete once the guarded init write path is committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E11-005 — Add basic/polyglot-minimal preset
```
