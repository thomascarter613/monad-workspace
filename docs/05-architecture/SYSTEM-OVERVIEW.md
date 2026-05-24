---
title: "System Overview"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - architecture
  - system-overview
  - runtime
related:
  - docs/01-project/00-vision/PRODUCT-VISION.md
  - docs/01-project/01-charter/PRODUCT-CHARTER.md
  - docs/02-product/MVP-SCOPE.md
  - docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
  - docs/05-architecture/MODULE-BOUNDARIES.md
  - docs/06-adrs/ADR-0001-use-rust-for-core-runtime.md
  - docs/06-adrs/ADR-0005-use-multi-crate-rust-workspace.md
---

# System Overview

## Purpose

This document defines the initial system overview for Monad.

Monad is an AI-native, repo-native, local-first Software Foundry OS for understanding, verifying, and safely evolving software repositories.

This overview explains the major system areas, their responsibilities, and how they should fit together during MVP development.

## System Summary

Monad begins as a Rust-based local developer tool with a command-line interface and a durable core runtime.

The initial architecture is intentionally simple:

```text
User
  → monad CLI
  → monad-core
  → repository filesystem
  → native tools
  → generated reports/context/evidence
```

The CLI accepts user intent. The core runtime performs durable work. Native tools remain responsible for their own ecosystems. Monad coordinates, inspects, verifies, reports, and safely evolves.

## Primary Architectural Goal

The primary architectural goal is to create a trustworthy local foundation for repository intelligence, context preservation, verification, and safe evolution.

The system should be:

- local-first;
- repo-native;
- deterministic where practical;
- explicit about uncertainty;
- safe around file operations;
- clear about command execution;
- provider-agnostic for AI;
- easy to test;
- easy to explain;
- easy to extend without architectural drift.

## MVP Runtime Shape

The MVP runtime should start with two Rust crates:

```text
crates/
  monad-cli/
  monad-core/
```

### `monad-cli`

The CLI crate owns:

- argument parsing;
- help output;
- user-facing command routing;
- terminal output coordination;
- process exit behavior.

The CLI should remain thin.

Durable product logic should not be buried in CLI command handlers.

### `monad-core`

The core crate owns:

- domain models;
- workspace resolution;
- manifest parsing;
- repository inspection;
- diagnostics;
- errors;
- command execution abstraction;
- verification model;
- file operation planning;
- context artifact generation;
- evolution planning;
- policy and approval models;
- provider abstractions.

The core should be usable by future interfaces such as a desktop app, web control plane, MCP server, or daemon.

## Future System Shape

The long-term architecture may include:

```text
monad CLI
monad core
monad local daemon
monad MCP server
monad desktop app
monad web control plane
monad provider plugins
monad team/cloud service
```

But MVP development should not require those future surfaces.

The core principle is:

> Build the durable local core first. Add additional surfaces later.

## Major System Areas

### 1. CLI Interface

The CLI is the first user-facing surface.

Initial commands may include:

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

The CLI should be clear, predictable, and conservative.

### 2. Core Runtime

The core runtime contains reusable product logic.

It should not know about terminal styling, shell-specific UX, or GitHub-specific project management details unless those are expressed through explicit integration boundaries.

### 3. Workspace Context

Workspace context answers:

- Where is the repository root?
- What files and directories matter?
- What manifest files exist?
- What execution context is active?
- What should commands consider the current workspace?

This is foundational for almost every other feature.

### 4. Manifest System

Monad should eventually use `monad.toml` as a repo-level intent manifest.

The manifest should describe Monad-level project intent without replacing ecosystem-native manifests.

Examples of native manifests:

```text
Cargo.toml
package.json
pyproject.toml
go.mod
pom.xml
build.gradle
```

Monad coordinates around these. It does not erase them.

### 5. Repo Intelligence

Repo intelligence inspects the repository and detects:

- languages;
- package managers;
- toolchains;
- workspace layout;
- manifests;
- scripts;
- basic project graph relationships;
- verification opportunities;
- context readiness.

The first version should be conservative and explain what was detected.

### 6. Context Bridge

The context bridge creates and verifies repo-native AI/human handoff artifacts.

It should support:

- current-state files;
- fresh-chat handoffs;
- bootstrap prompts;
- context packs;
- session chronicles;
- decision logs;
- generated context verification.

The context bridge replaces hidden chat memory with reviewable repository artifacts.

### 7. Verification Engine

The verification engine coordinates checks and produces evidence.

It should support:

- check definitions;
- check registry;
- command runner;
- check results;
- human-readable reports;
- JSON reports;
- evidence packets;
- exit-code standards.

Verification should produce evidence, not vague confidence.

### 8. Evolution Engine

The evolution engine safely prepares repository changes.

It should support:

- planned file operations;
- dry-run mode;
- conflict detection;
- diff or preview summaries;
- template registry;
- baseline evolution commands;
- worktree and branch safety rules.

File writes are trust-critical and should remain conservative.

### 9. Agent Supervision

Agent supervision defines how AI assistance is used safely.

It should support:

- supervised workflows;
- provider abstraction;
- plan generation;
- draft sandboxing;
- approval gates;
- audit logs;
- MCP integration foundation.

The human remains in command.

### 10. Policy and Governance

Policy and governance define safety rules, approvals, and future enforcement models.

Initial policy concerns include:

- command execution safety;
- file operation safety;
- approval gates;
- audit events;
- context trust levels;
- MCP safety boundaries.

### 11. Integrations

Integrations connect Monad to external tools and ecosystems.

Initial integrations are conceptual or local:

- Git;
- GitHub Issues and Projects;
- native package managers;
- native language tools;
- MCP;
- model providers.

Integrations must not become the source of truth unless explicitly designed that way.

## Source of Truth Model

Monad uses a layered source-of-truth model.

### Canonical repository truth

Canonical project truth lives in the repository:

```text
docs/
work/
.monad/
Cargo.toml
monad.toml
native manifests
```

### External planning support

External systems may support the workflow:

```text
GitHub Issues
GitHub Projects
AppFlowy
```

But external systems should not become the only place where durable architecture, product, or workflow decisions live.

### Generated state

Generated state may live under:

```text
.monad/
```

Generated state must be clearly distinguishable from accepted human-authored project doctrine.

## Data Flow

A typical command flow should look like this:

```text
User command
  → CLI parses arguments
  → CLI creates request
  → monad-core resolves workspace context
  → monad-core performs domain operation
  → monad-core returns structured result
  → CLI renders result
  → optional report/context/evidence file is written
```

The CLI should not directly perform complex domain work.

## File Write Model

File writes must be planned before they are applied.

The preferred evolution flow is:

```text
Intent
  → Plan
  → Dry run
  → Preview
  → Approval
  → Apply
  → Verify
  → Evidence
  → Context update
```

MVP work should prioritize dry-run behavior before actual apply behavior.

## Command Execution Model

Monad may run native tools, but command execution must be explicit and inspectable.

The system should record:

- command name;
- arguments;
- working directory;
- exit code;
- stdout summary or captured output;
- stderr summary or captured output;
- duration where practical;
- check result.

Monad should not hide native tool failures.

## AI Provider Model

Monad must be provider-agnostic.

The architecture should allow future support for:

- hosted model providers;
- local models;
- self-hosted endpoints;
- MCP-compatible tools;
- future Monad-managed services.

The MVP should not depend on a real provider to prove core local value.

## Safety Model

Monad should be safe by default.

The architecture should prevent or discourage:

- unapproved destructive file changes;
- hidden command execution;
- unrestricted shell access;
- hidden model-provider calls;
- treating model output as verified truth;
- replacing native tools without clear reason;
- storing secrets in generated context;
- confusing generated files with accepted docs.

## Quality Attributes

Monad should optimize for:

| Quality Attribute | Meaning |
|---|---|
| Understandability | A contributor can understand the system from docs and code. |
| Safety | Risky operations are planned, previewed, and approved. |
| Verifiability | Work produces evidence through tests/checks/reports. |
| Extensibility | New ecosystems and surfaces can be added without rewriting the core. |
| Local-first usefulness | Core value works without a hosted service. |
| Provider agnosticism | AI workflows do not require one vendor. |
| Determinism | Outputs are stable where possible. |
| Reviewability | Changes are visible, diffable, and auditable. |
| Teachability | Rust implementation remains understandable to the maintainer. |

## MVP Architecture Boundaries

During MVP, the architecture should avoid:

- web app implementation;
- desktop app implementation;
- cloud service implementation;
- plugin marketplace;
- billing system;
- enterprise SSO;
- production MCP server;
- full autonomous agents;
- remote execution;
- complex distributed state.

These may be revisited after local value is proven.

## Current Status

This system overview is a draft. It is authoritative enough to guide E0 and E1 implementation and should be updated as the Rust workspace and core modules become real.
