---
title: "Command Catalog"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - reference
  - commands
  - cli
related:
  - docs/02-product/MVP-SCOPE.md
  - docs/05-architecture/SYSTEM-OVERVIEW.md
  - docs/12-verification/EXIT-CODE-STANDARD.md
  - docs/13-operations/LOCAL-DEVELOPMENT.md
---

# Command Catalog

## Purpose

This document catalogs planned Monad CLI commands.

This catalog is a planning reference, not a guarantee that every command already exists.

## Command Status Values

Command status values:

```text
planned
stub
implemented
experimental
deferred
removed
```

## MVP Command Catalog

| Command | Status | Purpose |
|---|---|---|
| `monad --help` | planned | Show CLI help. |
| `monad --version` | planned | Show CLI version. |
| `monad info` | planned | Show basic Monad/runtime information. |
| `monad inspect` | planned | Inspect repository structure and detected tooling. |
| `monad inspect --format json` | planned | Output repository inspection as JSON. |
| `monad graph` | planned | Produce project graph output. |
| `monad graph --format json` | planned | Output graph as JSON. |
| `monad graph --format mermaid` | planned | Output graph as Mermaid. |
| `monad graph --format dot` | planned | Output graph as DOT. |
| `monad context generate` | planned | Generate context bridge artifacts. |
| `monad context verify` | planned | Verify context bridge artifacts. |
| `monad check` | planned | Run repository verification checks. |
| `monad check --format json` | planned | Output verification results as JSON. |
| `monad evolve verify-baseline --dry-run` | planned | Preview verification baseline changes. |
| `monad evolve context-baseline --dry-run` | planned | Preview context baseline changes. |
| `monad plan "<intent>"` | planned | Produce a supervised plan from user intent. |

## Command: `monad --help`

Purpose:

```text
Show available commands and options.
```

Expected early verification:

```bash
cargo run -p monad-cli -- --help
```

## Command: `monad info`

Purpose:

```text
Show basic runtime, version, and workspace information.
```

Possible output:

- Monad version;
- current directory;
- detected workspace root;
- enabled features;
- configuration status.

## Command: `monad inspect`

Purpose:

```text
Inspect the repository and report detected structure, manifests, languages, package managers, and toolchains.
```

MVP output may include:

- repository root;
- detected Cargo workspace;
- detected package manager markers;
- docs directory presence;
- context file presence.

## Command: `monad graph`

Purpose:

```text
Build and render a basic project graph.
```

Planned formats:

```text
text
json
mermaid
dot
```

## Command: `monad context generate`

Purpose:

```text
Generate repo-native context artifacts.
```

Possible generated files:

```text
.monad/context/current-state.md
.monad/context/latest-handoff.md
.monad/context/latest-context-pack.md
```

## Command: `monad context verify`

Purpose:

```text
Verify context bridge artifacts exist and are coherent.
```

## Command: `monad check`

Purpose:

```text
Run verification checks selected for the repository.
```

Possible checks:

- Rust formatting;
- Rust tests;
- Rust Clippy;
- documentation frontmatter;
- context artifacts;
- CLI smoke tests.

## Command: `monad evolve verify-baseline --dry-run`

Purpose:

```text
Preview changes that would establish or improve verification baseline files.
```

Must not write files in dry-run mode.

## Command: `monad evolve context-baseline --dry-run`

Purpose:

```text
Preview changes that would establish or improve context bridge baseline files.
```

Must not write files in dry-run mode.

## Command: `monad plan "<intent>"`

Purpose:

```text
Create a structured, reviewable plan from user intent.
```

Early implementation may use deterministic local behavior or a mock provider.

It must not write files.

## Post-MVP Command Candidates

Potential future commands:

```text
monad init
monad add
monad doctor
monad recommend
monad draft
monad review
monad apply
monad repair
monad release
monad upgrade
monad policy check
monad audit
monad mcp serve
```

These are not required for MVP unless promoted by future work packets.

## Current Status

This command catalog is a draft. It should be updated whenever CLI commands are added, renamed, removed, or promoted from planned to implemented.
