---
title: "Toolchain Setup"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - operations
  - toolchain
  - rust
related:
  - docs/13-operations/LOCAL-DEVELOPMENT.md
  - docs/10-engineering/RUST-VERIFICATION.md
  - docs/06-adrs/ADR-0001-use-rust-for-core-runtime.md
  - docs/05-architecture/SYSTEM-OVERVIEW.md
---

# Toolchain Setup

## Purpose

This document defines the initial toolchain setup expectations for Monad development.

Monad is a Rust-first local developer tool. Its initial development environment should be straightforward and reproducible.

## Required Toolchain

Initial required tools:

```text
git
rustup
cargo
rustfmt
clippy
python3
```

GitHub issue automation also requires:

```text
gh
```

## Rust Toolchain

Monad should use stable Rust.

The repository should eventually include:

```text
rust-toolchain.toml
```

Initial policy:

```text
Use the stable channel during early development.
Use rustfmt and clippy.
Revisit fixed MSRV before public v1.0.
```

## Install Rust

Install Rust with rustup.

After installation, verify:

```bash
rustc --version
cargo --version
rustup --version
```

## Install Rust Components

Install formatting and linting components:

```bash
rustup component add rustfmt clippy
```

Verify:

```bash
cargo fmt --version
cargo clippy --version
```

## Python

Python is used for lightweight repository scripts during early setup.

Use:

```bash
python3 --version
```

Do not assume `python` points to Python 3.

## GitHub CLI

GitHub CLI is used for issue and project automation.

Verify:

```bash
gh --version
gh auth status
```

If not authenticated:

```bash
gh auth login
```

For GitHub Projects automation, project scope may be needed:

```bash
gh auth refresh -s project
```

## Optional Tools

Optional but useful tools:

```text
ripgrep
fd
tree
jq
taplo
```

These should not be required for the initial Rust build unless future docs or scripts explicitly require them.

## JavaScript Tooling

Monad may later include JavaScript/TypeScript surfaces or templates.

The user preference is:

```text
Prefer Bun over pnpm where practical.
```

However, Monad itself should not require JavaScript tooling for the initial Rust core.

## Avoided Default Toolchains

Do not make these default dependencies:

```text
Bazel
Pants
Buck2
Nx
```

Monad may study them or interoperate with repositories that use them, but they are not default dependencies for Monad.

## Verification Commands

Once Rust crates exist, verify toolchain health with:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Before Rust crates exist, verify docs with:

```bash
find docs -type f | sort
```

and the documentation frontmatter check.

## Troubleshooting

### `cargo fmt` is missing

Run:

```bash
rustup component add rustfmt
```

### `cargo clippy` is missing

Run:

```bash
rustup component add clippy
```

### `gh project` fails

Refresh GitHub CLI auth scope:

```bash
gh auth refresh -s project
```

### `python` does not work

Use:

```bash
python3
```

Monad setup commands should prefer `python3`.

## Current Status

This toolchain setup document is a draft. It is authoritative enough for initial local development and should be refined after the root repository files and Rust workspace are created.
