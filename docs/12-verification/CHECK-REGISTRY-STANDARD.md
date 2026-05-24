---
title: "Check Registry Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - verification
  - checks
  - registry
related:
  - docs/12-verification/VERIFICATION-MODEL.md
  - docs/12-verification/EVIDENCE-PACKET-STANDARD.md
  - docs/12-verification/EXIT-CODE-STANDARD.md
  - docs/10-engineering/RUST-VERIFICATION.md
---

# Check Registry Standard

## Purpose

This document defines the standard for Monad’s check registry.

The check registry is the catalog of checks Monad knows how to describe, select, run, skip, and report.

## Core Rule

A check must be explicit.

Users should be able to understand:

- what the check is;
- why it exists;
- when it applies;
- how it runs;
- what success means;
- what failure means;
- whether failure blocks completion.

## Check Registry Definition

A check registry is a structured collection of check definitions.

Each check definition should describe a verification action, not merely a command string.

Example check concepts:

```text
rust.fmt
rust.test
rust.clippy
docs.frontmatter
context.bootstrap_exists
evolution.dry_run_no_writes
```

## Required Check Fields

A check definition should eventually include:

```text
id
name
description
category
severity
required
applies_when
execution_kind
command
expected_success
failure_meaning
```

## Field: `id`

The check ID is stable and machine-readable.

Examples:

```text
rust.fmt
rust.test
rust.clippy
docs.frontmatter
context.current_state
```

Use lowercase dot-separated IDs.

## Field: `name`

The name is human-readable.

Examples:

```text
Rust formatting
Rust test suite
Rust Clippy linting
Documentation frontmatter
Current-state context artifact
```

## Field: `description`

The description explains what the check verifies.

Example:

```text
Verifies that Rust source files are formatted according to rustfmt.
```

## Field: `category`

Recommended categories:

```text
documentation
rust
cli
repo-intelligence
context
verification
evolution
agent
security
operations
```

## Field: `severity`

Recommended severities:

```text
info
warning
error
critical
```

Severity describes how serious failure is.

## Field: `required`

A required check blocks success if it fails.

Optional checks may warn without failing the whole command.

## Field: `applies_when`

This describes when the check applies.

Examples:

```text
Cargo.toml exists
docs directory exists
monad context bridge is enabled
package.json exists
```

## Field: `execution_kind`

Recommended execution kinds:

```text
command
built_in
manual
generated
not_applicable
```

### `command`

Runs a native command.

Example:

```bash
cargo test
```

### `built_in`

Runs Monad internal logic.

Example:

```text
Check all docs Markdown files start with YAML frontmatter.
```

### `manual`

Requires manual review.

Example:

```text
Review generated architecture summary for accuracy.
```

### `generated`

Produced as part of another generation/reporting workflow.

### `not_applicable`

Check does not apply to the current repository.

## Field: `command`

For command checks, define:

- program;
- args;
- working directory;
- expected exit code.

Prefer structured command representation over shell strings.

Good:

```text
program: cargo
args: ["test"]
```

Avoid:

```text
sh -c "cargo test"
```

## Field: `expected_success`

Describe what success means.

Example:

```text
Exit code 0 and no formatting differences detected.
```

## Field: `failure_meaning`

Describe what failure likely means.

Example:

```text
Rust source files are not formatted according to rustfmt.
```

## Check Result Statuses

Recommended statuses:

```text
passed
failed
warning
skipped
not_applicable
error
```

## Required MVP Checks

Monad’s own MVP should support these checks first:

```text
docs.frontmatter
rust.fmt
rust.test
rust.clippy
cli.help
```

Later MVP checks may include:

```text
context.bootstrap_prompt
context.fresh_chat_handoff
verification.evidence_packet
evolution.dry_run_no_writes
```

## Rust Check Examples

### `rust.fmt`

Command:

```bash
cargo fmt --check
```

Success:

```text
All Rust files are correctly formatted.
```

Failure:

```text
Rust formatting changes are needed.
```

### `rust.test`

Command:

```bash
cargo test
```

Success:

```text
All Rust tests pass.
```

Failure:

```text
One or more Rust tests failed or the project did not compile.
```

### `rust.clippy`

Command:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Success:

```text
Clippy reports no warnings or errors.
```

Failure:

```text
Rust lint warning or error must be fixed.
```

## Documentation Check Examples

### `docs.frontmatter`

Execution kind:

```text
built_in
```

Success:

```text
Every Markdown file under docs starts with YAML frontmatter.
```

Failure:

```text
One or more docs files are missing frontmatter.
```

## Context Check Examples

### `context.bootstrap_prompt`

Success:

```text
docs/09-ai/BOOTSTRAP-PROMPT.md exists and has frontmatter.
```

### `context.fresh_chat_handoff`

Success:

```text
docs/09-ai/FRESH-CHAT-HANDOFF.md exists and has frontmatter.
```

## Check Selection

Monad should select checks based on:

- repository structure;
- detected manifests;
- detected toolchains;
- enabled Monad capabilities;
- user command;
- explicit manifest configuration.

Example:

```text
If Cargo.toml exists, select Rust checks.
If docs/ exists, select docs checks.
If context bridge is enabled, select context checks.
```

## Check Registry MVP Rule

The first check registry should be simple.

Do not build a full policy engine during the first implementation.

Start with clear types and a few checks.

## Current Status

This check registry standard is a draft. It is authoritative enough to guide E4 check registry implementation.
