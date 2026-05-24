---
title: "MVP Roadmap"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - roadmap
  - mvp
  - planning
related:
  - docs/02-product/MVP-SCOPE.md
  - docs/03-requirements/MVP-REQUIREMENTS.md
  - docs/07-workflow/WORK-HIERARCHY.md
  - docs/16-reference/COMMAND-CATALOG.md
---

# MVP Roadmap

## Purpose

This document defines Monad’s MVP roadmap.

The MVP roadmap turns Monad’s product vision into a sequence of epics and work packets that can be implemented, verified, reviewed, and committed in small slices.

## MVP Thesis

The MVP should prove that Monad can operate as a local-first Rust CLI that helps a repository become understandable, context-preserving, verifiable, and safely evolvable.

The MVP does not need to build the full long-term Software Foundry OS.

It must prove the core foundation.

## MVP Success Statement

Monad reaches MVP when a user can run the CLI in a repository and use it to:

- inspect the repository;
- detect basic tooling and workspace structure;
- generate or verify context artifacts;
- run meaningful checks;
- produce evidence;
- preview safe baseline improvements;
- produce a structured plan from user intent;
- preserve a human-in-command workflow.

## MVP Epic Sequence

```text
E0 — Project Foundation
E1 — Rust Core Foundation
E2 — Repo Intelligence
E3 — Context Bridge
E4 — Verification Engine
E5 — Evolution Engine
E6 — Agent Supervision
```

## E0 — Project Foundation

### Goal

Establish the repo, documentation architecture, workflow model, context foundation, product canon, and planning system before implementation begins.

### Expected Outcomes

- Monad has a complete documentation tree.
- Core foundation docs are drafted.
- GitHub Issues and Projects workflow is defined.
- Work packets are seeded.
- ADR foundation exists.
- Context bridge and AI handoff standards exist.
- Repository is ready for WP-E0-001 and implementation.

### Representative Work Packets

```text
WP-E0-001 — Establish repository foundation
WP-E0-002 — Establish documentation architecture
WP-E0-003 — Establish context bridge foundation
WP-E0-004 — Establish workflow standards
WP-E0-005 — Establish initial product canon
```

### Exit Criteria

- Critical foundation docs exist.
- Docs have YAML frontmatter.
- Initial accepted ADRs exist.
- Workflow standards exist.
- GitHub planning structure exists.
- Next implementation work is clear.

## E1 — Rust Core Foundation

### Goal

Create Monad’s Rust workspace, CLI shell, core module boundaries, diagnostic/error foundation, workspace context resolver, and manifest foundation.

### Expected Outcomes

- Root Rust workspace exists.
- `monad-cli` exists.
- `monad-core` exists.
- CLI help/version behavior works.
- Core error and diagnostic model exists.
- Workspace root/context resolution exists.
- `monad.toml` foundation exists.
- Rust verification baseline passes.

### Representative Work Packets

```text
WP-E1-001 — Create Rust workspace crates
WP-E1-002 — Add CLI shell
WP-E1-003 — Add core error and diagnostic model
WP-E1-004 — Add workspace context resolver
WP-E1-005 — Add monad.toml manifest foundation
WP-E1-006 — Add Rust verification and learning baseline
```

### Exit Criteria

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- --help
```

Expected result:

- All Rust checks pass.
- CLI help output works.
- Core/CLI boundary is clean.
- Rust learning notes are updated where needed.

## E2 — Repo Intelligence

### Goal

Teach Monad to inspect repositories and detect basic structure, manifests, toolchains, and project graph information.

### Expected Outcomes

- Toolchain detection model exists.
- Rust/Cargo detection works.
- Basic JavaScript package manager marker detection works.
- `monad inspect` works.
- Inspection output can be human-readable and JSON.
- Basic project graph model exists.
- Graph output can be deterministic.

### Representative Work Packets

```text
WP-E2-001 — Add toolchain detection model
WP-E2-002 — Detect Node and JavaScript package managers
WP-E2-003 — Detect Rust Cargo workspaces
WP-E2-004 — Add inspect command report
WP-E2-005 — Add basic project graph model
WP-E2-006 — Add graph output formats
```

### Exit Criteria

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- inspect --format json
```

Expected result:

- Monad can inspect its own repository.
- Monad can detect Rust/Cargo structure.
- Monad can detect basic JavaScript package manager markers in fixtures.
- Inspection output is understandable.

## E3 — Context Bridge

### Goal

Implement Monad’s initial repo-native context bridge: current-state generation, handoff generation, context packs, bootstrap prompts, and context verification.

### Expected Outcomes

- Context artifact schemas exist.
- `monad context generate` exists.
- Current-state artifact can be generated.
- Handoff artifact can be generated.
- Context pack can be assembled.
- Bootstrap prompt can be generated or maintained.
- Context verification checks exist.

### Representative Work Packets

```text
WP-E3-001 — Define context artifact schemas
WP-E3-002 — Implement current-state generator
WP-E3-003 — Implement handoff generator
WP-E3-004 — Implement context pack assembler
WP-E3-005 — Implement bootstrap prompt generator
WP-E3-006 — Add context verification checks
```

### Exit Criteria

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- context generate
cargo run -p monad-cli -- context verify
```

Expected result:

- Context artifacts can be generated.
- Context files identify source/trust status.
- New sessions can orient from repo files.
- Generated context does not pretend to be accepted doctrine.

## E4 — Verification Engine

### Goal

Build Monad’s verification foundation: check registry, command runner, `monad check`, evidence packets, adapter-specific checks, and JSON output.

### Expected Outcomes

- Check registry model exists.
- Command runner exists.
- `monad check` works.
- Verification results are structured.
- Evidence packet can be produced.
- JSON verification output exists.
- Adapter-specific checks can be selected.

### Representative Work Packets

```text
WP-E4-001 — Define check registry and result model
WP-E4-002 — Add command runner
WP-E4-003 — Add monad check command
WP-E4-004 — Add evidence packet report
WP-E4-005 — Add adapter-specific checks
WP-E4-006 — Add JSON verification output
```

### Exit Criteria

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- check
cargo run -p monad-cli -- check --format json
```

Expected result:

- Verification checks run.
- Results show pass/fail/skipped status.
- Native tool failures are visible.
- Evidence output is reviewable.
- JSON output is valid.

## E5 — Evolution Engine

### Goal

Build Monad’s safe repository evolution foundation: planned file operations, dry-run behavior, template registry, baseline evolution commands, and worktree safety.

### Expected Outcomes

- Safe file operation model exists.
- Dry-run planner exists.
- Template registry foundation exists.
- `monad evolve verify-baseline --dry-run` exists.
- `monad evolve context-baseline --dry-run` exists.
- Worktree/branch safety strategy is documented.
- File changes are planned before writes.

### Representative Work Packets

```text
WP-E5-001 — Define safe file operation model
WP-E5-002 — Add dry-run and diff planner
WP-E5-003 — Add template registry foundation
WP-E5-004 — Add evolve verify-baseline command
WP-E5-005 — Add evolve context-baseline command
WP-E5-006 — Add worktree and branch safety strategy
```

### Exit Criteria

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run
```

Expected result:

- Dry-run commands do not write files.
- Planned file operations are visible.
- Conflicts are represented.
- Existing files are not silently overwritten.

## E6 — Agent Supervision

### Goal

Create Monad’s initial supervised AI workflow foundation: provider abstraction, plan command, draft workflow, approval gates, audit model, and MCP strategy.

### Expected Outcomes

- Supervised agent workflow is documented.
- Model provider abstraction exists.
- `monad plan` exists.
- Draft sandbox workflow is defined.
- Approval gate and audit log model exists.
- MCP integration foundation exists.
- Agent workflows remain human-in-command.

### Representative Work Packets

```text
WP-E6-001 — Define supervised agent workflow
WP-E6-002 — Add model provider abstraction
WP-E6-003 — Add plan command
WP-E6-004 — Add draft sandbox workflow
WP-E6-005 — Add approval gates and audit log
WP-E6-006 — Add MCP integration foundation
```

### Exit Criteria

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- plan "explain this repository"
```

Expected result:

- Plan command produces structured output.
- No files are modified by planning.
- Provider abstraction does not require one vendor.
- Approval/audit concepts are represented.

## MVP Command Targets

The MVP roadmap should produce these command targets:

```text
monad --help
monad --version
monad info
monad inspect
monad inspect --format json
monad graph
monad context generate
monad context verify
monad check
monad check --format json
monad evolve verify-baseline --dry-run
monad evolve context-baseline --dry-run
monad plan "<intent>"
```

## MVP Quality Gates

MVP work must preserve:

- Rust formatting;
- tests;
- Clippy with warnings denied;
- docs frontmatter;
- work packet traceability;
- context handoff;
- atomic commits;
- safe file operation principles;
- human-in-command agent principles.

## MVP Completion Criteria

The MVP is complete when:

- E0 through E6 MVP work packets are complete or intentionally descoped;
- Monad can run locally as a Rust CLI;
- Monad can inspect its own repository;
- Monad can generate or verify context artifacts;
- Monad can run checks and produce evidence;
- Monad can dry-run baseline evolution;
- Monad can produce a supervised plan from intent;
- documentation accurately reflects implementation;
- context handoff works from repository files;
- local verification passes.

## Current Status

This MVP roadmap is a draft. It is authoritative enough to sequence early Monad work and should be updated after E1 produces the first working Rust runtime.
