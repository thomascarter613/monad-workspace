---
title: "Rust Coding Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - engineering
  - rust
  - coding-standard
related:
  - docs/06-adrs/ADR-0001-use-rust-for-core-runtime.md
  - docs/05-architecture/MODULE-BOUNDARIES.md
  - docs/10-engineering/RUST-LEARNING-NOTES.md
  - docs/10-engineering/RUST-VERIFICATION.md
---

# Rust Coding Standard

## Purpose

This document defines the initial Rust coding standard for Monad.

Monad is implemented in Rust for its durable local core runtime. The code should be reliable, testable, readable, and teachable.

## Core Rule

Write Rust that is clear before it is clever.

Monad’s Rust code should be understandable to a maintainer who is learning Rust while building a serious developer tool.

## Rust Apprenticeship Mode

Monad Rust implementation must use Rust Apprenticeship Mode.

That means implementation guidance should include:

- small slices;
- complete file contents;
- clear comments;
- tests;
- verification commands;
- expected results;
- explanations of new Rust concepts;
- atomic commits.

## Workspace Structure

The initial Rust workspace should use:

```text
crates/
  monad-cli/
  monad-core/
```

Expected responsibilities:

| Crate | Responsibility |
|---|---|
| `monad-cli` | CLI parsing, command routing, terminal rendering, exit behavior. |
| `monad-core` | Durable reusable product logic. |

## CLI Boundary

The CLI must remain thin.

`monad-cli` should not own durable business logic.

Good:

```text
CLI parses arguments → calls monad-core → renders result
```

Bad:

```text
CLI parses arguments → scans repo → runs checks → writes files directly
```

## Core Boundary

`monad-core` should own:

- errors;
- diagnostics;
- workspace resolution;
- manifests;
- repo intelligence;
- graph model;
- context bridge logic;
- verification model;
- command execution abstraction;
- file operation planning;
- template registry;
- evolution planning;
- policy models;
- provider abstractions.

## Naming

Use clear Rust names.

Crates:

```text
monad-cli
monad-core
```

Modules:

```text
workspace
manifest
diagnostics
file_ops
repo_intelligence
```

Types:

```text
WorkspaceContext
Diagnostic
DiagnosticSeverity
MonadError
FileOperationPlan
```

Functions should use snake_case:

```text
resolve_workspace_root
generate_context_pack
run_check
```

## Error Handling

Prefer explicit error handling.

Use `Result<T, E>` for fallible operations.

Avoid `unwrap()` and `expect()` in production code unless there is a documented reason.

Tests may use `expect()` when it improves clarity.

Good:

```rust
let contents = std::fs::read_to_string(path)?;
```

Bad:

```rust
let contents = std::fs::read_to_string(path).unwrap();
```

## Comments

Comments should explain why something exists, not merely repeat what the code says.

Good:

```rust
// Keep workspace resolution in core so future CLI, MCP, and daemon surfaces can reuse it.
```

Weak:

```rust
// This calls the function.
```

Because the maintainer is learning Rust, comments are encouraged when introducing:

- ownership;
- borrowing;
- lifetimes;
- traits;
- generics;
- error conversions;
- module exports;
- filesystem behavior;
- command execution.

## Tests

Core behavior should have tests where practical.

Tests should be used to explain intended behavior.

Preferred test names:

```rust
detects_workspace_root_from_current_directory
returns_error_when_manifest_is_missing
creates_warning_diagnostic
```

Avoid vague test names:

```rust
test1
works
stuff
```

## Module Exports

`lib.rs` should expose intentional module boundaries.

Do not expose internal details unless needed.

Prefer:

```rust
pub mod diagnostics;
pub mod workspace;
```

over a large unstructured public surface.

## Dependency Discipline

Add dependencies deliberately.

Before adding a dependency, ask:

- Is this needed now?
- Is the crate maintained?
- Is it appropriate for a local developer tool?
- Does it increase security or supply-chain risk?
- Can the standard library handle this slice?
- Does this dependency belong in core or CLI?

## Formatting

Rust formatting is enforced with:

```bash
cargo fmt --check
```

Use `cargo fmt` to apply formatting.

## Linting

Rust linting is enforced with:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Warnings should be fixed, not ignored, unless there is a documented reason.

## Code Organization

Prefer small focused modules.

Avoid dumping unrelated types into one file.

Initial likely module areas:

```text
error
diagnostics
workspace
manifest
repo_intelligence
graph
context
checks
exec
file_ops
templates
evolution
agents
policy
```

Do not create all modules with full implementation before needed. Create boundaries when work packets require them.

## Filesystem Safety

Filesystem code must be conservative.

When writing file operation logic:

- plan before writing;
- prefer dry-run first;
- detect existing files;
- avoid destructive behavior;
- report conflicts;
- avoid hidden overwrites.

## Command Execution Safety

Command execution must be explicit.

When running native tools:

- avoid shell execution by default;
- capture exit status;
- capture stdout/stderr where useful;
- preserve working directory;
- report failures clearly;
- do not hide native tool output.

## Output

Core should return structured results.

CLI should render those results.

Do not embed terminal styling inside core domain models unless there is a deliberate output boundary.

## Current Status

This Rust coding standard is a draft. It is authoritative enough to guide E1 Rust foundation work and should be refined as the first modules are implemented.
