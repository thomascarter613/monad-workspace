---
title: "monad init Presets"
document_type: "command-reference"
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
  - command
  - init
  - presets
  - scaffold
related:
  - docs/commands/INIT.md
  - crates/monad-core/src/init.rs
  - crates/monad-core/src/templates/registry.rs
---

# `monad init` Presets

## Status

Accepted.

## Work Packet

WP-E11-005 — Add basic/polyglot-minimal preset.

## Purpose

This document records the initial `monad init` preset model.

The preset model is intentionally small and conservative.

It gives Monad a useful initialization experience without becoming a full application/framework generator.

## Current Presets

| Preset | Status | Meaning |
| --- | --- | --- |
| `basic` | Accepted | Friendly alias for the smallest useful Monad-aware repository scaffold. |
| `minimal` | Accepted | Backward-compatible name for the same scaffold as `basic`. |
| `polyglot-minimal` | Accepted | Minimal polyglot monorepo directory scaffold layered on top of the basic scaffold. |

## Recommended Default

The recommended user-facing default name is:

```text
basic
```

The `minimal` name remains supported because it was introduced first and is still accurate.

## `basic`

Preview:

```bash
monad init --preset=basic --dry-run
```

Apply after review:

```bash
monad init --preset=basic --yes
```

The `basic` preset creates:

```text
monad.toml
README.md
docs/README.md
work/README.md
.monad/.gitignore
```

## `minimal`

Preview:

```bash
monad init --preset=minimal --dry-run
```

Apply after review:

```bash
monad init --preset=minimal --yes
```

`minimal` is equivalent to `basic`.

It creates the same file set:

```text
monad.toml
README.md
docs/README.md
work/README.md
.monad/.gitignore
```

## `polyglot-minimal`

Preview:

```bash
monad init --preset=polyglot-minimal --dry-run
```

Apply after review:

```bash
monad init --preset=polyglot-minimal --yes
```

The `polyglot-minimal` preset includes the basic scaffold plus:

```text
apps/.gitkeep
packages/.gitkeep
services/.gitkeep
tools/.gitkeep
```

It does not add Bazel, Pants, Buck2, or Nx.

It does not install package managers.

It does not create language-specific manifests.

Those behaviors require separate future command contracts.

## Safety Boundary

All presets follow the same safety model:

- preview with `--dry-run`;
- apply only with `--yes`;
- refuse to overwrite existing files;
- abort when conflicts are detected;
- run no Git commands;
- call no remote services;
- install no dependencies;
- publish nothing.

## Relationship to Future Presets

Future presets may include:

```text
rust-library
rust-cli
typescript-app
python-service
go-service
java-service
monorepo-polyglot
```

Those are intentionally out of scope for E11 until the preset/schema/template architecture is more mature.

## Verification

```bash
cargo run -p monad-cli -- init --preset=basic --dry-run
cargo run -p monad-cli -- init --preset=minimal --dry-run
cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run
```

For write-path verification, run inside an empty temporary directory only:

```bash
cargo run --manifest-path /path/to/monad-workspace/Cargo.toml -p monad-cli -- init --preset=basic --yes
cargo run --manifest-path /path/to/monad-workspace/Cargo.toml -p monad-cli -- init --preset=polyglot-minimal --yes
```

## Outcome

Accepted.

`basic`, `minimal`, and `polyglot-minimal` are the initial `monad init` preset surface.
