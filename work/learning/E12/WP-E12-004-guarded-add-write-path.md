---
title: "Learning Note — WP-E12-004 Guarded Add Write Path"
document_type: "learning-note"
status: active
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-004
tags:
  - learning
  - rust
  - write-path
  - safety
  - monad
---

# Learning Note — WP-E12-004 Guarded Add Write Path

## What You Are Building

You are adding:

```bash
monad add app web --yes
```

This is the first write path for `monad add`.

## Important Mental Model

The write path does not invent a second workflow.

It reuses the dry-run plan:

```text
build plan
  -> evaluate plan
  -> if conflicts exist, stop
  -> if no conflicts exist, write files
```

That keeps the write path consistent with the preview path.

## Main Rust Functions

Read:

```text
apply_add_plan
render_add_apply_result
render_add_dry_run
```

in:

```text
crates/monad-core/src/component_add.rs
```

## Why This Is Safer

The write path checks the exact same file-operation plan that dry-run shows.

That means the user can preview before writing.

The same target paths are used for both modes.

## What `apply_add_plan` Does

It:

1. Builds the add plan.
2. Evaluates it against the filesystem.
3. Refuses to continue if conflicts exist.
4. Creates parent directories for approved targets.
5. Writes embedded template contents.
6. Returns a result listing created files.

## What It Does Not Do

It does not:

- run Git;
- commit;
- push;
- install packages;
- modify package manager lockfiles;
- call remote services.

## CLI Change

The CLI now supports two modes:

```bash
monad add app web --dry-run
monad add app web --yes
```

It rejects:

```bash
monad add app web
monad add app web --dry-run --yes
```

## What to Inspect

```bash
git diff -- crates/monad-core/src/component_add.rs
git diff -- crates/monad-cli/src/main.rs
```

Look for:

```text
AddApplyResult
apply_add_plan
render_add_apply_result
require_add_mode
```

## Verification

Test `--yes` in a temporary directory:

```bash
tmpdir="$(mktemp -d)"
(
  cd "$tmpdir"
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- add app web --yes
  test -f apps/web/README.md
  test -f apps/web/.gitkeep
)
rm -rf "$tmpdir"
```


## Fix Note — Parser Guard and Match Arm Drift

The guarded-write patch had two integration mistakes:

1. It guessed a parser variable named `arguments`, but that variable was not in scope.
2. It added `yes` to `CliCommand::Add`, but one match arm still destructured the old shape.

The compiler errors were useful:

```text
cannot find value `arguments` in this scope
pattern does not mention field `yes`
this function takes 4 arguments but 3 arguments were supplied
```

The fix is to align all layers:

```text
parser guard
CliCommand::Add variant
run match arm
render_add function signature
```

Also note: `std::fs` does not require adding an external `fs` crate to Cargo.toml.


## Fix Note — Add Requires a Monad Workspace

A plain `mktemp -d` directory is not automatically a Monad workspace.

The command:

```bash
cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- add app web --yes
```

fails from an uninitialized temp directory because `monad add` tries to discover the Monad workspace root from `.`.

That is correct for the current command model: `add` adds a component to an existing Monad workspace.

The corrected temporary verification flow is:

```bash
tmpdir="$(mktemp -d)"
(
  cd "$tmpdir"
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- init --yes
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- add app web --yes
  test -f apps/web/README.md
  test -f apps/web/.gitkeep
)
rm -rf "$tmpdir"
```

The important lesson is that command verification should match the command's preconditions.
