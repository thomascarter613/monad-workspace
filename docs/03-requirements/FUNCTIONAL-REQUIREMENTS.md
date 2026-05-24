---
title: "Functional Requirements"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - requirements
  - functional
related:
  - docs/03-requirements/MVP-REQUIREMENTS.md
  - docs/03-requirements/NONFUNCTIONAL-REQUIREMENTS.md
  - docs/02-product/MVP-SCOPE.md
  - docs/16-reference/COMMAND-CATALOG.md
---

# Functional Requirements

## Purpose

This document defines Monad’s functional requirements.

Functional requirements describe what Monad must do.

## Requirement Format

Functional requirements use this format:

```text
FR-XXX — Requirement title
```

Each requirement should be testable or traceable to future tests.

## FR-001 — CLI Help

Monad shall provide command-line help output.

Verification:

```bash
cargo run -p monad-cli -- --help
```

## FR-002 — CLI Version

Monad shall provide version output.

Verification:

```bash
cargo run -p monad-cli -- --version
```

## FR-003 — Thin CLI Boundary

Monad CLI commands shall delegate durable product behavior to `monad-core`.

Verification:

- code review;
- module boundary review;
- tests where practical.

## FR-004 — Core Runtime Crate

Monad shall provide a reusable `monad-core` crate for durable product logic.

Verification:

```bash
test -f crates/monad-core/Cargo.toml
```

## FR-005 — Workspace Context Resolution

Monad shall resolve the active repository/workspace context for commands that require repository awareness.

Verification:

```bash
cargo test
```

## FR-006 — Monad Manifest Foundation

Monad shall support a `monad.toml` manifest foundation for Monad-level repository intent.

Verification:

```bash
cargo test
```

## FR-007 — Repository Inspection

Monad shall inspect a repository and report detected structure.

Verification:

```bash
cargo run -p monad-cli -- inspect
```

## FR-008 — Toolchain Detection

Monad shall detect supported toolchain indicators such as Cargo manifests and JavaScript package manager lockfiles.

Verification:

```bash
cargo test
cargo run -p monad-cli -- inspect
```

## FR-009 — Inspection JSON Output

Monad shall support JSON output for repository inspection.

Verification:

```bash
cargo run -p monad-cli -- inspect --format json
```

## FR-010 — Project Graph Model

Monad shall represent basic repository nodes and relationships as a project graph.

Verification:

```bash
cargo test
```

## FR-011 — Project Graph Output

Monad shall output the project graph in at least one human-readable format and at least one machine-readable format.

Verification:

```bash
cargo run -p monad-cli -- graph
cargo run -p monad-cli -- graph --format json
```

## FR-012 — Context Generation

Monad shall generate or maintain repo-native context artifacts.

Verification:

```bash
cargo run -p monad-cli -- context generate
```

## FR-013 — Context Verification

Monad shall verify required context artifacts.

Verification:

```bash
cargo run -p monad-cli -- context verify
```

## FR-014 — Bootstrap Prompt

Monad shall maintain a bootstrap prompt for AI-assisted sessions.

Verification:

```bash
test -f docs/09-ai/BOOTSTRAP-PROMPT.md
```

## FR-015 — Fresh Chat Handoff

Monad shall maintain a fresh-chat handoff document.

Verification:

```bash
test -f docs/09-ai/FRESH-CHAT-HANDOFF.md
```

## FR-016 — Check Registry

Monad shall define and use a registry of verification checks.

Verification:

```bash
cargo test
```

## FR-017 — Command Runner

Monad shall run native commands through an explicit command execution abstraction.

Verification:

```bash
cargo test
```

## FR-018 — Check Command

Monad shall provide a `check` command.

Verification:

```bash
cargo run -p monad-cli -- check
```

## FR-019 — Verification JSON Output

Monad shall support JSON output for verification results.

Verification:

```bash
cargo run -p monad-cli -- check --format json
```

## FR-020 — Evidence Packet

Monad shall produce or render verification evidence.

Verification:

```bash
cargo run -p monad-cli -- check
```

## FR-021 — Safe File Operation Model

Monad shall represent planned file operations before writes occur.

Verification:

```bash
cargo test
```

## FR-022 — Dry-Run Planning

Monad shall support dry-run behavior for evolution workflows.

Verification:

```bash
cargo run -p monad-cli -- evolve context-baseline --dry-run
```

## FR-023 — Template Registry

Monad shall define a local template registry foundation.

Verification:

```bash
cargo test
```

## FR-024 — Verification Baseline Evolution

Monad shall preview verification baseline changes.

Verification:

```bash
cargo run -p monad-cli -- evolve verify-baseline --dry-run
```

## FR-025 — Context Baseline Evolution

Monad shall preview context baseline changes.

Verification:

```bash
cargo run -p monad-cli -- evolve context-baseline --dry-run
```

## FR-026 — Model Provider Abstraction

Monad shall define a provider-agnostic model provider abstraction.

Verification:

```bash
cargo test
```

## FR-027 — Plan Command

Monad shall provide a `plan` command that converts user intent into a structured plan without modifying files.

Verification:

```bash
cargo run -p monad-cli -- plan "explain this repository"
git status --short
```

## FR-028 — Approval Gate Model

Monad shall define an approval gate model for consequential actions.

Verification:

```bash
cargo test
```

## FR-029 — Audit Event Model

Monad shall define an audit event model for consequential actions.

Verification:

```bash
cargo test
```

## FR-030 — Documentation Frontmatter

Monad documentation shall use YAML frontmatter for canonical Markdown files.

Verification:

```bash
python3 - <<'PY'
from pathlib import Path

missing = []
for path in sorted(Path("docs").rglob("*.md")):
    if not path.read_text(encoding="utf-8").startswith("---\n"):
        missing.append(str(path))

if missing:
    print("Files missing frontmatter:")
    for item in missing:
        print(f"  {item}")
    raise SystemExit(1)

print("All docs Markdown files have YAML frontmatter.")
PY
```

## Current Status

These functional requirements are a draft. They should be refined and traced to work packets as implementation proceeds.
