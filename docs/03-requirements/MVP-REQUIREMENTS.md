---
title: "MVP Requirements"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - requirements
  - mvp
related:
  - docs/02-product/MVP-SCOPE.md
  - docs/01-project/03-roadmap/MVP-ROADMAP.md
  - docs/03-requirements/FUNCTIONAL-REQUIREMENTS.md
  - docs/03-requirements/NONFUNCTIONAL-REQUIREMENTS.md
---

# MVP Requirements

## Purpose

This document defines the MVP requirements for Monad.

The MVP requirements translate Monad’s roadmap into testable product expectations.

## Requirement Status Values

Requirements may use these statuses:

```text
planned
in-progress
implemented
verified
deferred
removed
```

## MVP Requirement Groups

```text
MVP-RG-001 — Project foundation
MVP-RG-002 — Rust runtime foundation
MVP-RG-003 — Repository intelligence
MVP-RG-004 — Context bridge
MVP-RG-005 — Verification engine
MVP-RG-006 — Evolution engine
MVP-RG-007 — Agent supervision foundation
```

## MVP-RG-001 — Project Foundation

### MVP-REQ-001

Monad shall have a canonical documentation tree under `docs/`.

Status: planned

Verification:

```bash
find docs -type f | sort
```

### MVP-REQ-002

Every canonical Markdown document under `docs/` shall begin with YAML frontmatter.

Status: planned

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

### MVP-REQ-003

Monad shall have an ADR index and initial accepted ADRs for Rust core runtime and unified product naming.

Status: planned

Verification:

```bash
test -f docs/06-adrs/README.md
test -f docs/06-adrs/ADR-0001-use-rust-for-core-runtime.md
test -f docs/06-adrs/ADR-0002-use-monad-as-unified-product-name.md
```

### MVP-REQ-004

Monad shall define a work packet standard that includes Product Area before Objective, Expected Result After Verification, and Priority and Size at the end.

Status: planned

Verification:

```bash
grep -n "Product Area" docs/07-workflow/WORK-PACKET-STANDARD.md
grep -n "Expected Result After Verification" docs/07-workflow/WORK-PACKET-STANDARD.md
grep -n "Priority" docs/07-workflow/WORK-PACKET-STANDARD.md
grep -n "Size" docs/07-workflow/WORK-PACKET-STANDARD.md
```

## MVP-RG-002 — Rust Runtime Foundation

### MVP-REQ-005

Monad shall use a Rust workspace.

Status: planned

Verification:

```bash
test -f Cargo.toml
cargo metadata --no-deps
```

### MVP-REQ-006

Monad shall provide a `monad-cli` crate.

Status: planned

Verification:

```bash
test -f crates/monad-cli/Cargo.toml
```

### MVP-REQ-007

Monad shall provide a `monad-core` crate.

Status: planned

Verification:

```bash
test -f crates/monad-core/Cargo.toml
```

### MVP-REQ-008

The CLI shall provide help output.

Status: planned

Verification:

```bash
cargo run -p monad-cli -- --help
```

### MVP-REQ-009

Rust code shall pass formatting, tests, and Clippy.

Status: planned

Verification:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## MVP-RG-003 — Repository Intelligence

### MVP-REQ-010

Monad shall detect the repository/workspace root for supported commands.

Status: planned

Verification:

```bash
cargo test
```

### MVP-REQ-011

Monad shall detect Rust/Cargo workspace markers.

Status: planned

Verification:

```bash
cargo run -p monad-cli -- inspect
```

### MVP-REQ-012

Monad shall detect basic JavaScript package manager markers where present.

Status: planned

Verification:

```bash
cargo test
```

### MVP-REQ-013

Monad shall provide an `inspect` command.

Status: planned

Verification:

```bash
cargo run -p monad-cli -- inspect
```

### MVP-REQ-014

Monad shall support machine-readable inspection output.

Status: planned

Verification:

```bash
cargo run -p monad-cli -- inspect --format json
```

## MVP-RG-004 — Context Bridge

### MVP-REQ-015

Monad shall define context artifact standards.

Status: planned

Verification:

```bash
test -f docs/08-context/CONTEXT-BRIDGE.md
test -f docs/08-context/HANDOFF-STANDARD.md
test -f docs/08-context/CONTEXT-PACK-STANDARD.md
```

### MVP-REQ-016

Monad shall provide a bootstrap prompt for new AI sessions.

Status: planned

Verification:

```bash
test -f docs/09-ai/BOOTSTRAP-PROMPT.md
```

### MVP-REQ-017

Monad shall provide a fresh-chat handoff file.

Status: planned

Verification:

```bash
test -f docs/09-ai/FRESH-CHAT-HANDOFF.md
```

### MVP-REQ-018

Monad shall generate or maintain current-state context artifacts.

Status: planned

Verification:

```bash
cargo run -p monad-cli -- context generate
```

### MVP-REQ-019

Monad shall verify required context artifacts.

Status: planned

Verification:

```bash
cargo run -p monad-cli -- context verify
```

## MVP-RG-005 — Verification Engine

### MVP-REQ-020

Monad shall define a check registry model.

Status: planned

Verification:

```bash
cargo test
```

### MVP-REQ-021

Monad shall provide a command runner for local native commands.

Status: planned

Verification:

```bash
cargo test
```

### MVP-REQ-022

Monad shall provide a `check` command.

Status: planned

Verification:

```bash
cargo run -p monad-cli -- check
```

### MVP-REQ-023

Monad shall produce human-readable verification output.

Status: planned

Verification:

```bash
cargo run -p monad-cli -- check
```

### MVP-REQ-024

Monad shall support JSON verification output.

Status: planned

Verification:

```bash
cargo run -p monad-cli -- check --format json
```

### MVP-REQ-025

Monad shall support evidence packet generation or rendering.

Status: planned

Verification:

```bash
cargo run -p monad-cli -- check
```

## MVP-RG-006 — Evolution Engine

### MVP-REQ-026

Monad shall define a safe file operation model.

Status: planned

Verification:

```bash
cargo test
```

### MVP-REQ-027

Monad shall support dry-run planning before file writes.

Status: planned

Verification:

```bash
cargo test
```

### MVP-REQ-028

Monad shall provide a template registry foundation.

Status: planned

Verification:

```bash
cargo test
```

### MVP-REQ-029

Monad shall provide a dry-run verification baseline evolution command.

Status: planned

Verification:

```bash
cargo run -p monad-cli -- evolve verify-baseline --dry-run
```

### MVP-REQ-030

Monad shall provide a dry-run context baseline evolution command.

Status: planned

Verification:

```bash
cargo run -p monad-cli -- evolve context-baseline --dry-run
```

## MVP-RG-007 — Agent Supervision Foundation

### MVP-REQ-031

Monad shall document supervised agent workflow rules.

Status: planned

Verification:

```bash
test -f docs/09-ai/AI-COLLABORATION-RULES.md
```

### MVP-REQ-032

Monad shall define a provider-agnostic model provider abstraction.

Status: planned

Verification:

```bash
cargo test
```

### MVP-REQ-033

Monad shall provide a `plan` command that does not modify files.

Status: planned

Verification:

```bash
cargo run -p monad-cli -- plan "explain this repository"
git status --short
```

### MVP-REQ-034

Monad shall define approval and audit models for consequential agent actions.

Status: planned

Verification:

```bash
cargo test
```

### MVP-REQ-035

Monad shall define MCP integration boundaries.

Status: planned

Verification:

```bash
test -f docs/05-architecture/MCP-INTEGRATION-STRATEGY.md
```

## MVP Requirement Completion Rule

An MVP requirement is not verified until:

- the relevant command or check passes;
- the expected result is documented;
- the work packet is complete;
- the change is committed;
- docs/context are updated if needed.

## Current Status

These MVP requirements are a draft. They are authoritative enough to guide E1 through E6 implementation and should be updated as requirements are implemented or descoped.
