---
title: "Local Development"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - operations
  - local-development
  - developer-experience
related:
  - docs/13-operations/TOOLCHAIN-SETUP.md
  - docs/10-engineering/RUST-VERIFICATION.md
  - docs/07-workflow/OPERATING-MODEL.md
  - docs/05-architecture/SYSTEM-OVERVIEW.md
---

# Local Development

## Purpose

This document defines the initial local development workflow for Monad.

Monad is local-first. The repository should be easy to clone, inspect, verify, and continue without depending on a hosted service.

## Local Development Goals

Local development should be:

- simple;
- reproducible;
- documented;
- verification-oriented;
- beginner-friendly for Rust;
- friendly to AI-assisted handoff;
- safe around file and command execution.

## Initial Assumption

Monad begins as a Rust workspace.

Initial crates:

```text
crates/monad-cli/
crates/monad-core/
```

The root workspace should eventually include:

```text
Cargo.toml
rust-toolchain.toml
README.md
docs/
work/
.monad/
```

## Required Local Tools

Initial required tools:

```text
git
rustup
cargo
rustfmt
clippy
python3
GitHub CLI
```

GitHub CLI is needed for issue/project automation, not for compiling Monad.

## Recommended Local Tools

Recommended tools:

```text
gh
ripgrep
fd
tree
jq
taplo
```

These are useful but should not be required for the first Rust build unless explicitly added.

## Initial Clone Workflow

Expected workflow:

```bash
git clone <repo-url>
cd monad
git status --short
```

## Initial Verification Workflow

Once Rust crates exist, run:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Before Rust crates exist, documentation verification should include:

```bash
find docs -type f | sort
```

and frontmatter checks.

## Running the CLI

Once `monad-cli` exists:

```bash
cargo run -p monad-cli -- --help
```

Later examples:

```bash
cargo run -p monad-cli -- info
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- check
```

## Development Loop

Recommended development loop:

```text
Select active work packet
  → read relevant docs
  → edit bounded files
  → run verification
  → inspect git diff
  → commit atomically
  → update context if needed
```

## Git Workflow

Use small atomic commits.

Before committing:

```bash
git status --short
git diff
```

Commit with Conventional Commits.

Example:

```bash
git commit -m "feat(core): add diagnostic model"
```

## Documentation Workflow

For documentation-only slices:

```bash
find docs -type f | sort
python3 - <<'PY'
from pathlib import Path

missing = []
for path in sorted(Path("docs").rglob("*.md")):
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        missing.append(str(path))

if missing:
    print("Files missing frontmatter:")
    for item in missing:
        print(f"  {item}")
    raise SystemExit(1)

print("All docs Markdown files have YAML frontmatter.")
PY
```

## Rust Apprenticeship Workflow

For Rust slices:

1. Read the active work packet.
2. Identify target crate and module.
3. Write simple code first.
4. Add comments for new Rust concepts.
5. Add tests.
6. Run formatting.
7. Run tests.
8. Run Clippy.
9. Run CLI smoke test if applicable.
10. Commit atomically.

## Avoided Defaults

Do not introduce these as default Monad dependencies:

```text
Bazel
Pants
Buck2
Nx
```

These may be studied, but Monad should not depend on them by default.

## Local-First Rule

Core Monad development should not require:

- a hosted account;
- a cloud control plane;
- an AI subscription;
- remote execution;
- enterprise services.

Some optional integrations may require credentials later, but local core value must remain available without them.

## Current Status

This local development document is a draft. It is authoritative enough for initial repository setup and E1 Rust implementation.
