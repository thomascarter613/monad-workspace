---
title: MVP Scope Freeze
description: Formal scope-freeze artifact for Monad internal MVP candidate preparation.
status: draft
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
phase: MVP Candidate Cut
epic: E8
work_packet: WP-E8-001
---

# MVP Scope Freeze

## 1. Purpose

This document freezes the scope of the Monad internal MVP candidate.

It exists to prevent scope drift during E8 release preparation. The MVP candidate must be evaluated against implemented, verified behavior, not future product ambition.

## 2. Scope-freeze decision

Monad may proceed toward an **internal MVP candidate cut** only under the scope defined in this document.

This scope freeze does not authorize:

- public release
- package publication
- installer distribution
- hosted service launch
- marketing launch
- autonomous agent execution
- write/apply evolution behavior
- MCP server release
- enterprise SaaS claims

## 3. MVP candidate cut line

The frozen MVP candidate cut line is:

> Monad is an internal, local-first, Rust-based monorepo runtime and governance-grade developer-experience CLI that can inspect a repository, run checks, render graphs, generate repo-native context artifacts, produce supervised no-write plans, and preview safe baseline evolution operations in dry-run mode.

Any capability outside that statement is deferred unless explicitly reclassified by a later ADR or release-preparation work packet.

## 4. Included MVP candidate capabilities

The following capabilities are inside the internal MVP candidate boundary.

### 4.1 Workspace summary

Included commands:

```bash
cargo run -p monad-cli -- info
cargo run -p monad-cli -- info --format=json
```

Included behavior:

* discover Monad workspace
* load manifest
* render workspace summary
* support text and JSON output

### 4.2 Repository inspection

Included commands:

```bash
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- inspect --format=json
```

Included behavior:

* inspect repository structure
* produce a reviewable repository summary
* support text and JSON output

### 4.3 Workspace checks

Included commands:

```bash
cargo run -p monad-cli -- check
cargo run -p monad-cli -- check --format=json
```

Included behavior:

* run current workspace checks
* report pass/fail/warning/skipped counts
* write check evidence in text mode
* support JSON report output

### 4.4 Repository graph rendering

Included commands:

```bash
cargo run -p monad-cli -- graph
cargo run -p monad-cli -- graph --format=json
cargo run -p monad-cli -- graph --format=mermaid
cargo run -p monad-cli -- graph --format=dot
```

Included behavior:

* build repository graph from bounded traversal
* render graph in text, JSON, Mermaid, and DOT formats

### 4.5 Repo-native context

Included commands:

```bash
cargo run -p monad-cli -- context
cargo run -p monad-cli -- context --format=json
cargo run -p monad-cli -- context --write
cargo run -p monad-cli -- context generate current-state
cargo run -p monad-cli -- context generate handoff
cargo run -p monad-cli -- context generate bootstrap
cargo run -p monad-cli -- context pack
cargo run -p monad-cli -- context verify
```

Included behavior:

* render AI-readable repository context
* write context pack artifacts
* generate current-state artifact
* generate handoff artifact
* generate bootstrap prompt artifact
* assemble latest context pack
* verify required context files

### 4.6 Supervised no-write planning

Included command:

```bash
cargo run -p monad-cli -- plan "explain this repository"
```

Included behavior:

* produce a supervised plan from a user intent
* use current local/mock provider behavior
* do not write files
* do not run shell commands
* do not mutate Git state
* do not call a real external AI provider

### 4.7 Dry-run evolution previews

Included commands:

```bash
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run
```

Included behavior:

* preview verification baseline file operations
* preview context baseline file operations
* render dry-run plan output
* state that no files were written
* require `--dry-run`

### 4.8 Local verification

Included verification baseline:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

Included behavior:

* local verification can be run by a developer
* verification outcome is reviewable
* failures must be recorded honestly

## 5. Deferred capabilities

The following capabilities are explicitly deferred.

### 5.1 Apply/write evolution

Deferred:

* applying dry-run plans
* writing evolution-generated files
* patch application
* conflict resolution workflow
* rollback workflow
* branch/worktree creation

### 5.2 Autonomous agent execution

Deferred:

* autonomous code modification
* unsupervised shell execution
* multi-agent orchestration
* agent lifecycle supervision
* background task execution
* tool-call autonomy

### 5.3 Real model-provider execution

Deferred:

* hosted LLM calls
* local model execution
* provider credential management
* provider routing
* provider billing/account handling
* provider health checks

### 5.4 MCP runtime/server behavior

Deferred:

* MCP server execution
* MCP client runtime
* MCP tool invocation
* MCP permissions flow
* remote MCP integrations
* MCP audit persistence

### 5.5 Public distribution

Deferred:

* crates.io publishing
* installer generation
* package signing
* public release notes
* public binary distribution
* release announcement

### 5.6 Hosted/enterprise platform

Deferred:

* hosted control plane
* multi-user RBAC
* SSO
* SaaS onboarding
* marketplace/plugin billing
* enterprise audit dashboard

## 6. Prohibited release claims

Until a later milestone explicitly changes this scope, Monad must not be described as:

* production-ready
* publicly released
* enterprise-ready
* autonomous
* self-evolving without supervision
* capable of applying repository changes
* capable of mutating Git state
* capable of calling real model providers by default
* a hosted SaaS product
* a package published for public installation

Approved phrasing:

* internal MVP candidate
* local-first CLI
* no-write planning prototype
* dry-run evolution preview
* repo-native context foundation
* governance-grade developer-experience foundation

## 7. Required verification before moving past scope freeze

Before E8 proceeds beyond WP-E8-001, run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
git status --short
```

## 8. Scope-change rule

Any change that expands the MVP candidate beyond this document requires one of the following:

1. A new E8 work packet explicitly scoped for the change.
2. A new ADR if the change affects architecture, safety, release policy, provider behavior, Git mutation, or execution model.
3. A later milestone after E8.

No implicit expansion is allowed.

## 9. Relationship to MVP readiness report

This scope-freeze document is downstream of:

```text
docs/project/MVP-READINESS-REPORT.md
```

The readiness report explains whether Monad may be treated as an internal MVP candidate.

This document freezes what that candidate includes and excludes.
