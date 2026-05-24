---
title: "Domain Model"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - domain
  - model
  - ddd
related:
  - docs/04-domain/BOUNDED-CONTEXTS.md
  - docs/04-domain/UBIQUITOUS-LANGUAGE.md
  - docs/05-architecture/SYSTEM-OVERVIEW.md
  - docs/07-workflow/WORK-HIERARCHY.md
---

# Domain Model

## Purpose

This document defines Monad’s initial domain model.

The domain model identifies the core concepts Monad must understand in order to inspect, verify, preserve context for, and safely evolve software repositories.

## Domain Summary

Monad’s domain is:

```text
Repository understanding, verification, context preservation, safe evolution, and supervised AI-assisted software development.
```

Monad is not merely a CLI or generator. It is a system for helping a repository become understandable, verifiable, and safely changeable.

## Core Domain Objects

```text
Repository
Workspace
Manifest
Toolchain
Native Tool
Adapter
Project Graph
Context Artifact
Work Packet
Check
Check Result
Evidence Packet
File Operation Plan
Template
Evolution Plan
Agent Plan
Approval Gate
Audit Event
```

## Repository

A repository is the root project Monad inspects and operates on.

A repository may contain:

- source code;
- docs;
- manifests;
- tests;
- CI configuration;
- generated artifacts;
- context files;
- native tool configuration.

Important properties:

- root path;
- Git status where available;
- detected workspace markers;
- docs presence;
- context readiness;
- verification readiness.

## Workspace

A workspace is Monad’s resolved view of the active project area.

A workspace may be the repository root or a sub-workspace.

Important properties:

- root path;
- current directory;
- root markers;
- manifests;
- workspace type;
- resolution diagnostics.

## Manifest

A manifest describes project or tool intent.

Examples:

```text
Cargo.toml
package.json
pyproject.toml
go.mod
monad.toml
```

Monad’s own manifest is:

```text
monad.toml
```

Native ecosystem manifests remain authoritative for their ecosystems.

## Toolchain

A toolchain is a language or ecosystem tool family.

Examples:

```text
Rust/Cargo
JavaScript/Bun
JavaScript/npm
Go
Python
Java/Maven
Java/Gradle
```

Monad detects toolchains to understand what checks, commands, and project structures may apply.

## Native Tool

A native tool is an ecosystem tool Monad coordinates.

Examples:

```text
cargo
bun
npm
go
pytest
maven
gradle
```

Monad does not replace native tools by default.

## Adapter

An adapter represents Monad’s knowledge about an ecosystem or native tool.

An adapter may provide:

- detection;
- metadata extraction;
- recommended checks;
- command construction;
- graph contribution;
- manifest interpretation.

## Project Graph

A project graph represents repository components and relationships.

Nodes may include:

- crates;
- packages;
- apps;
- services;
- docs;
- configuration files;
- generated artifacts.

Edges may represent:

- dependency;
- ownership;
- generated-from;
- verifies;
- documents;
- belongs-to.

## Context Artifact

A context artifact preserves project state or handoff information.

Examples:

```text
docs/09-ai/BOOTSTRAP-PROMPT.md
docs/09-ai/FRESH-CHAT-HANDOFF.md
.monad/context/current-state.md
.monad/context/latest-handoff.md
.monad/context/latest-context-pack.md
```

Context artifacts must distinguish accepted truth from generated or draft summaries.

## Work Packet

A work packet is the primary delivery unit.

Important properties:

- ID;
- parent epic;
- title;
- product area;
- objective;
- scope;
- tasks;
- deliverables;
- verification;
- expected result;
- priority;
- size.

## Check

A check is a defined verification action.

Examples:

```text
rust.fmt
rust.test
rust.clippy
docs.frontmatter
context.bootstrap_prompt
```

Important properties:

- ID;
- name;
- category;
- severity;
- required flag;
- applies-when logic;
- execution kind;
- expected success.

## Check Result

A check result records what happened when a check ran.

Possible statuses:

```text
passed
failed
warning
skipped
not_applicable
error
```

Important properties:

- check ID;
- status;
- message;
- command result if applicable;
- duration if available;
- evidence reference if available.

## Evidence Packet

An evidence packet records verification activity.

Important properties:

- scope;
- checks run;
- command results;
- failures;
- skipped checks;
- warnings;
- expected result;
- actual result;
- conclusion;
- limitations.

## File Operation Plan

A file operation plan describes proposed file changes before they occur.

Possible operation kinds:

```text
create
update
skip
conflict
no_op
delete
```

Delete operations are high risk and should be avoided or gated.

## Template

A template is reusable source material for generating or updating files.

Important properties:

- template ID;
- name;
- description;
- version;
- output path;
- content source;
- required variables.

Templates should not directly write files. They should produce planned operations.

## Evolution Plan

An evolution plan describes a proposed repository improvement.

Examples:

- add context baseline;
- add verification baseline;
- add docs baseline;
- add a new crate;
- add a new package.

Important properties:

- objective;
- planned file operations;
- conflicts;
- dry-run status;
- required approvals;
- recommended verification.

## Agent Plan

An agent plan is a structured proposal produced from user intent.

Important properties:

- user intent;
- assumptions;
- proposed steps;
- files likely affected;
- required approvals;
- verification plan;
- risks;
- non-actions.

An agent plan must not be treated as verified truth.

## Approval Gate

An approval gate represents a required human decision before a consequential action.

Examples:

- approve file writes;
- approve command execution;
- approve destructive operation;
- approve commit;
- approve push.

## Audit Event

An audit event records a consequential action or decision.

Examples:

- plan created;
- dry-run generated;
- approval granted;
- file write applied;
- check run;
- evidence packet generated.

## Domain Relationships

```text
Repository contains Workspace
Workspace has Manifests
Manifests indicate Toolchains
Toolchains use Native Tools
Adapters understand Native Tools
Adapters contribute Checks
Checks produce Check Results
Check Results become Evidence Packets
Repository has Context Artifacts
Work Packets define expected Deliverables
Evolution Plans produce File Operation Plans
File Operation Plans may require Approval Gates
Agent Plans may create Evolution Plans
Audit Events record consequential actions
```

## Initial Bounded Contexts

Initial bounded contexts are:

```text
Core Runtime
Repository Intelligence
Context Bridge
Verification
Evolution
Agent Supervision
Policy / Governance
Documentation / Workflow
```

These contexts may become modules, crates, or integration boundaries as the implementation matures.

## Domain Invariants

Monad should preserve these invariants:

1. The repository is the durable source of truth.
2. CLI is an interface; core owns reusable logic.
3. File writes are planned before application.
4. Dry-run does not write files.
5. AI output is proposed, not verified.
6. Accepted ADRs guide implementation.
7. Work packets must define verification.
8. Native tools are coordinated, not unnecessarily replaced.
9. Generated context must identify itself.
10. Required failed checks must be visible.

## Domain Events

Potential future domain events:

```text
RepositoryInspected
WorkspaceResolved
ManifestParsed
ToolchainDetected
ContextArtifactGenerated
CheckRegistered
CheckRunStarted
CheckRunCompleted
EvidencePacketGenerated
FileOperationPlanned
EvolutionPlanCreated
ApprovalRequested
ApprovalGranted
ApprovalDenied
AgentPlanCreated
AuditEventRecorded
```

These events are conceptual for now. They should not be implemented before there is a concrete need.

## Current Status

This domain model is a draft. It is authoritative enough to guide MVP module design and should be refined as implementation creates concrete Rust types.
