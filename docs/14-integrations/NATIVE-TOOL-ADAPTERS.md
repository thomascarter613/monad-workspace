---
title: "Native Tool Adapters"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - integrations
  - native-tools
  - adapters
  - architecture
related:
  - docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
  - docs/05-architecture/MODULE-BOUNDARIES.md
  - docs/12-verification/CHECK-REGISTRY-STANDARD.md
  - docs/11-security/COMMAND-EXECUTION-SAFETY.md
---

# Native Tool Adapters

## Purpose

This document defines Monad’s native tool adapter concept.

Monad coordinates native ecosystem tools. It should not unnecessarily replace them.

## Core Rule

Monad should understand and coordinate native tools while preserving each ecosystem’s source of truth.

Examples:

- Rust uses Cargo.
- JavaScript uses Bun, npm, pnpm, or yarn.
- Go uses Go tooling.
- Python uses uv, pip, Poetry, pytest, ruff, mypy, or similar tools.
- Java uses Maven or Gradle.

Monad should inspect, select, run, and report around native tools.

## Why Coordinate Instead of Replace

Native tools already know their ecosystems.

Replacing them would create unnecessary complexity and reduce trust.

Monad’s value is in:

- discovery;
- orchestration;
- context;
- verification;
- evidence;
- safe evolution;
- reporting;
- AI-readable summaries.

## Adapter Responsibilities

A native tool adapter may eventually provide:

- detection;
- metadata extraction;
- recommended checks;
- command construction;
- graph contribution;
- manifest reading;
- capability reporting;
- evidence formatting;
- safety classification.

## Adapter Non-Responsibilities

Adapters should not:

- replace the native tool;
- hide native tool failures;
- invent unsupported behavior;
- silently install dependencies;
- run destructive commands by default;
- bypass Monad approval gates.

## Initial Adapter Targets

### Rust / Cargo

MVP priority.

Detect:

```text
Cargo.toml
Cargo.lock
crates/
```

Potential checks:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

### JavaScript / TypeScript

Early detection target.

Detect:

```text
package.json
bun.lock
bun.lockb
pnpm-lock.yaml
package-lock.json
yarn.lock
```

Potential package manager preference:

```text
Bun preferred where practical.
```

Monad should detect what a repository actually uses rather than forcing Bun.

### Go

Future adapter.

Detect:

```text
go.mod
go.sum
```

Potential checks:

```bash
go test ./...
go vet ./...
```

### Python

Future adapter.

Detect:

```text
pyproject.toml
requirements.txt
uv.lock
poetry.lock
```

Potential checks depend on project configuration.

### Java

Future adapter.

Detect:

```text
pom.xml
build.gradle
settings.gradle
```

## Adapter Detection Output

Detection should be explicit.

Example:

```text
Detected Rust/Cargo workspace because Cargo.toml exists at repository root.
Detected Bun because bun.lock exists.
Detected npm because package-lock.json exists.
```

If detection is uncertain, say so.

## Adapter Check Selection

Adapters may recommend checks.

Example:

```text
Rust adapter detected Cargo.toml.
Recommended checks:
- rust.fmt
- rust.test
- rust.clippy
```

The check registry decides how checks are represented and reported.

## Adapter Safety

Adapters must respect command execution safety.

Adapter-generated commands should be structured:

```text
program: cargo
args: ["test"]
working_directory: <workspace-root>
```

Avoid shell strings unless explicitly needed.

## Adapter Boundaries

Adapters belong in `monad-core` conceptually, likely under a future module such as:

```text
repo_intelligence/
adapters/
checks/adapters/
```

The exact implementation path may evolve.

## MVP Adapter Rule

The MVP should not attempt to support every ecosystem.

Start with:

```text
Rust/Cargo
basic JavaScript package manager marker detection
```

Then expand when the architecture is proven.

## Current Status

This native tool adapter document is a draft. It is authoritative enough to guide E2 repo intelligence and E4 adapter-specific check design.
