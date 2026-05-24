---
title: "MVP Scope"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - product
  - mvp
  - scope
related:
  - docs/01-project/00-vision/PRODUCT-VISION.md
  - docs/01-project/01-charter/PRODUCT-CHARTER.md
  - docs/02-product/PROBLEM-STATEMENT.md
  - docs/02-product/NON-GOALS.md
  - docs/03-requirements/MVP-REQUIREMENTS.md
  - docs/01-project/03-roadmap/MVP-ROADMAP.md
---

# MVP Scope

## Purpose

This document defines the initial MVP scope for Monad.

The MVP must prove the core thesis without attempting to build the entire long-term Software Foundry OS.

## MVP Thesis

The Monad MVP should prove that a local-first Rust CLI can:

1. understand a repository;
2. preserve useful context;
3. run meaningful checks;
4. produce reviewable evidence;
5. prepare safe, dry-run repository improvements;
6. support human-in-command AI workflows at a foundational level.

## MVP Outcome

The MVP is successful when a user can run Monad in a real repository and receive useful, trustworthy output.

The user should be able to answer:

- What kind of repository is this?
- What tooling does it use?
- What structure did Monad detect?
- What context files exist?
- What checks can run?
- What evidence was produced?
- What baseline improvements could Monad safely prepare?
- What should happen next?

## MVP Product Areas

The MVP includes these product areas:

```text
Core Runtime
CLI
Repo Intelligence
Context Bridge
Verification
Evolution Engine
Agent Supervision foundation
Documentation
Workflow
````

## MVP Epics

The initial MVP roadmap is organized as:

```text
E0 — Project Foundation
E1 — Rust Core Foundation
E2 — Repo Intelligence
E3 — Context Bridge
E4 — Verification Engine
E5 — Evolution Engine
E6 — Agent Supervision
```

## MVP Capabilities

### E0 — Project Foundation

MVP includes:

* repository foundation;
* documentation architecture;
* context bridge foundation;
* workflow standards;
* product canon;
* initial ADRs;
* GitHub issue/work packet structure.

### E1 — Rust Core Foundation

MVP includes:

* Rust workspace;
* `monad-cli`;
* `monad-core`;
* initial CLI shell;
* error and diagnostic model;
* workspace context resolver;
* `monad.toml` manifest foundation;
* Rust verification and learning baseline.

### E2 — Repo Intelligence

MVP includes:

* toolchain detection model;
* JavaScript package manager marker detection;
* Rust Cargo workspace detection;
* `monad inspect`;
* basic project graph model;
* graph output foundation.

### E3 — Context Bridge

MVP includes:

* context artifact schemas;
* current-state generator;
* handoff generator;
* context pack assembler;
* bootstrap prompt generator;
* context verification checks.

### E4 — Verification Engine

MVP includes:

* check registry model;
* command runner;
* `monad check`;
* evidence packet report;
* adapter-specific checks;
* JSON verification output.

### E5 — Evolution Engine

MVP includes:

* safe file operation model;
* dry-run planner;
* template registry foundation;
* `monad evolve verify-baseline --dry-run`;
* `monad evolve context-baseline --dry-run`;
* worktree and branch safety strategy.

### E6 — Agent Supervision

MVP includes:

* supervised agent workflow documentation;
* model-provider abstraction;
* `monad plan`;
* draft sandbox workflow foundation;
* approval gate and audit log model;
* MCP integration foundation.

## MVP Commands

The MVP should aim toward these commands:

```text
monad --help
monad info
monad inspect
monad graph
monad context generate
monad context verify
monad check
monad evolve verify-baseline --dry-run
monad evolve context-baseline --dry-run
monad plan
```

Not all commands need to be fully mature in the first pass. They should be coherent, useful, and extensible.

## MVP Quality Bar

MVP does not mean low quality.

MVP means limited scope.

The MVP should still be:

* locally runnable;
* documented;
* tested;
* formatted;
* linted;
* reviewable;
* understandable;
* safe by default;
* clear about limitations.

## MVP Verification Bar

MVP code should pass:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

MVP docs should have:

* YAML frontmatter;
* clear status;
* correct location;
* useful content;
* related references where known.

## MVP Non-Requirements

The MVP does not require:

* cloud control plane;
* hosted SaaS;
* billing;
* plugin marketplace;
* full autonomous agents;
* remote execution;
* enterprise SSO;
* full MCP server;
* all language ecosystems;
* production installer;
* perfect project graph;
* full static analysis;
* every possible repo layout.

## MVP Demo Goal

The target MVP demo should be:

```text
git clone <repo>
cd <repo>
monad inspect
monad context generate
monad check
monad evolve context-baseline --dry-run
monad evolve verify-baseline --dry-run
```

The output should demonstrate that Monad can:

* understand the repo at a basic level;
* generate context;
* run checks;
* produce evidence;
* prepare safe baseline improvements;
* keep changes reviewable.

## MVP Success Criteria

The MVP is successful if:

* Monad can run locally as a Rust CLI.
* The project has clear docs and workflow.
* A new session can resume from repo context.
* Monad can inspect its own repo.
* Monad can detect Rust/Cargo structure.
* Monad can detect basic JavaScript package manager markers in fixtures.
* Monad can generate context artifacts.
* Monad can run at least basic checks.
* Monad can produce human-readable evidence.
* Monad can dry-run baseline evolution.
* Monad avoids unapproved destructive changes.
* The maintainer understands the Rust code written so far.

## MVP Exit Criteria

Monad can exit MVP when:

* E0 through E6 MVP work packets are complete;
* verification commands pass;
* the CLI demonstrates the core flow;
* the docs describe the product accurately;
* context handoff works;
* at least one non-trivial repo can be inspected;
* at least one dry-run evolution flow works;
* there is enough value to share with early users.

## Current Status

This MVP scope is a draft. It is intentionally ambitious but bounded. It should be refined after E1 and E2 produce the first working runtime and repo inspection flows.
