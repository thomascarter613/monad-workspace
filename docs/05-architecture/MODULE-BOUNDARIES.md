---
title: "Module Boundaries"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - architecture
  - modules
  - boundaries
  - rust
related:
  - docs/05-architecture/SYSTEM-OVERVIEW.md
  - docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
  - docs/10-engineering/RUST-CODING-STANDARD.md
  - docs/06-adrs/ADR-0005-use-multi-crate-rust-workspace.md
  - docs/06-adrs/ADR-0006-keep-cli-thin-and-core-durable.md
---

# Module Boundaries

## Purpose

This document defines Monad’s initial module and crate boundaries.

Its purpose is to prevent core product logic from becoming tangled with CLI rendering, provider-specific integrations, generated file handling, or future cloud concerns.

## Boundary Rule

Monad should separate:

```text
Interface concerns
Domain/runtime concerns
External tool concerns
Generated artifact concerns
Policy/safety concerns
```

The first implementation should be simple, but the boundary direction should be clear from the beginning.

## Initial Crate Boundary

The MVP begins with:

```text
crates/
  monad-cli/
  monad-core/
```

## `monad-cli`

### Responsibility

`monad-cli` is the command-line user interface.

It owns:

- argument parsing;
- command dispatch;
- help output;
- terminal-friendly rendering;
- process exit codes;
- mapping CLI options into core requests.

### It may depend on

```text
monad-core
clap or another CLI parser
terminal output libraries if needed
```

### It must not own

- workspace discovery logic;
- repository inspection logic;
- verification domain logic;
- context generation logic;
- file operation planning;
- provider abstractions;
- architecture policy;
- business/domain decisions.

### Boundary rule

If logic should be reusable by a future desktop app, daemon, MCP server, or web interface, it belongs in `monad-core`, not `monad-cli`.

## `monad-core`

### Responsibility

`monad-core` is the durable local runtime and domain engine.

It owns:

- errors;
- diagnostics;
- workspace context;
- manifests;
- repository intelligence;
- graph model;
- context bridge;
- verification;
- command execution abstraction;
- file operation planning;
- template registry;
- evolution planning;
- policy and approval models;
- agent/provider abstractions.

### It may depend on

- standard Rust libraries;
- carefully chosen crates for parsing, serialization, filesystem traversal, diagnostics, and testing;
- native tools indirectly through explicit command execution boundaries.

### It must not depend on

- `monad-cli`;
- terminal-only rendering assumptions;
- one AI provider;
- GitHub-only workflow assumptions;
- a hosted Monad service;
- hidden chat memory.

### Boundary rule

`monad-core` should return structured results. Interfaces decide how to render those results.

## Planned `monad-core` Module Areas

The exact file tree may evolve, but the conceptual modules are:

```text
crates/monad-core/src/
  lib.rs
  error.rs
  diagnostics.rs
  workspace/
  manifest/
  repo_intelligence/
  graph/
  context/
  checks/
  exec/
  file_ops/
  templates/
  evolution/
  agents/
  policy/
  output/
```

## Module Responsibilities

### `error`

Owns Monad’s core error types.

Should answer:

- What went wrong?
- Can the error be displayed?
- Can it be converted from lower-level errors?
- Is the error useful for callers?

Should avoid:

- terminal styling;
- CLI-specific exit behavior;
- hiding source errors without context.

### `diagnostics`

Owns structured diagnostic messages.

Should answer:

- What should the user know?
- Is this info, warning, or error?
- Is there a suggested next step?
- Can this diagnostic become part of a report?

Should avoid:

- direct printing;
- terminal styling in the core model;
- mixing diagnostics with arbitrary logs.

### `workspace`

Owns workspace and repository context.

Should answer:

- Where is the repository root?
- What is the current working directory?
- What files define the workspace?
- What root markers were detected?
- What assumptions were made?

Should avoid:

- tool-specific detection beyond root context;
- running external checks;
- generating reports.

### `manifest`

Owns Monad manifest parsing and validation.

Should answer:

- Does `monad.toml` exist?
- What version is it?
- What intent does it define?
- Is it valid?

Should avoid:

- replacing native manifests;
- embedding CLI behavior;
- overcomplicated schema evolution too early.

### `repo_intelligence`

Owns repository inspection and tool detection.

Should answer:

- What languages were detected?
- What package managers were detected?
- What native manifests exist?
- What workspace structure is present?
- What commands may be available?

Should avoid:

- executing checks directly;
- making unsafe assumptions;
- claiming full certainty when detection is heuristic.

### `graph`

Owns project graph representation and rendering data.

Should answer:

- What nodes exist?
- What relationships exist?
- What output formats are supported?
- Is output deterministic?

Should avoid:

- graph rendering mixed with discovery logic;
- nondeterministic ordering;
- premature complex graph algorithms.

### `context`

Owns context bridge generation and verification.

Should answer:

- What context artifacts should exist?
- How is current state generated?
- How is a handoff generated?
- How is a context pack assembled?
- Are required context artifacts present?

Should avoid:

- treating generated context as accepted truth;
- provider-specific prompt assumptions;
- secret leakage.

### `checks`

Owns verification models and check orchestration.

Should answer:

- What checks exist?
- What checks should run?
- What passed?
- What failed?
- What was skipped?
- What evidence was produced?

Should avoid:

- hiding native command output;
- running unsafe commands by default;
- conflating check definitions with terminal rendering.

### `exec`

Owns external command execution abstraction.

Should answer:

- What command was run?
- Which arguments were used?
- What working directory was used?
- What exit status occurred?
- What stdout/stderr was captured?

Should avoid:

- unrestricted shell execution by default;
- secret exposure;
- hidden background execution;
- policy decisions that belong in `policy`.

### `file_ops`

Owns planned file operations.

Should answer:

- What file would be created?
- What file would be updated?
- What file would be skipped?
- What conflict exists?
- What operation is unsafe?

Should avoid:

- direct unplanned writes;
- destructive behavior without policy/approval;
- template registry concerns.

### `templates`

Owns reusable templates and template metadata.

Should answer:

- What templates exist?
- What is the template ID?
- What files can a template produce?
- What version or source does a template represent?

Should avoid:

- direct writes;
- remote marketplace behavior during MVP;
- policy decisions.

### `evolution`

Owns safe repository evolution workflows.

Should answer:

- What improvement is being planned?
- Which file operations are required?
- Is this a dry run?
- What verification should follow?
- What approval is needed?

Should avoid:

- bypassing `file_ops`;
- bypassing `policy`;
- directly hiding changes from the user.

### `agents`

Owns supervised AI/agent abstractions.

Should answer:

- What provider abstraction exists?
- What is the user intent?
- What plan was proposed?
- What draft is being prepared?
- What approvals are required?

Should avoid:

- direct unapproved writes;
- direct unapproved command execution;
- provider lock-in;
- treating model output as verified truth.

### `policy`

Owns safety and governance models.

Should answer:

- Is this operation allowed?
- Does this require approval?
- What audit event should be recorded?
- What safety rule applies?

Should avoid:

- knowing about terminal output;
- owning low-level file operations;
- becoming enterprise RBAC too early.

### `output`

Owns structured output models where useful.

Should answer:

- How can results be represented as text, JSON, Markdown, or future formats?
- What output is stable enough for automation?
- What output is intended only for humans?

Should avoid:

- mixing domain logic with rendering concerns;
- ANSI/terminal styling inside domain types.

## Dependency Direction

Preferred dependency direction:

```text
monad-cli
  → monad-core

monad-core modules
  → shared core models
  → no dependency on monad-cli
```

Within `monad-core`, dependency direction should generally be:

```text
workspace / manifest / diagnostics / error
  → repo_intelligence
  → graph
  → context
  → checks / exec
  → file_ops / templates
  → evolution
  → agents / policy
```

This is conceptual, not a strict import graph yet.

## Boundary Examples

### Good boundary

The CLI parses:

```text
monad inspect --format json
```

Then calls a core function that returns an inspection result.

The CLI renders JSON.

### Bad boundary

The CLI manually scans the repository, detects package managers, runs checks, and writes context files directly.

### Good boundary

`file_ops` creates a planned operation:

```text
Create docs/ai/BOOTSTRAP-PROMPT.md
```

`evolution` decides why that operation is needed.

The CLI renders the plan.

### Bad boundary

A template directly writes files to disk without planning, dry-run, conflict detection, or approval.

### Good boundary

`agents` produces a proposed plan.

`policy` determines whether execution requires approval.

`evolution` converts approved work into planned file operations.

`checks` verifies afterward.

### Bad boundary

An agent directly edits files, runs shell commands, and commits changes without structured approval.

## Future Crate Extraction

Some modules may eventually become separate crates.

Possible future crates:

```text
monad-core
monad-cli
monad-mcp
monad-protocol
monad-templates
monad-policy
monad-sdk
```

Extraction should happen only when boundaries are proven by implementation.

Do not split crates prematurely.

## MVP Boundary Priorities

During MVP, the most important boundaries are:

1. Keep CLI thin.
2. Keep core reusable.
3. Keep file operations planned.
4. Keep command execution explicit.
5. Keep AI provider abstraction provider-agnostic.
6. Keep generated context distinguishable from accepted docs.
7. Keep verification results structured.

## Boundary Review Checklist

Before adding a new module or feature, ask:

- Does this belong in CLI or core?
- Is this domain logic or rendering logic?
- Does this run native tools?
- Does this write files?
- Does this need approval?
- Does this produce evidence?
- Does this depend on one provider?
- Could a future interface reuse this logic?
- Is there a testable core model?
- Is the boundary simple enough for the current MVP?

## Current Status

This module boundary document is a draft. It should guide early Rust crate setup and be revised after E1 establishes the first real workspace and module structure.
