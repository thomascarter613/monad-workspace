---
title: "Verification Model"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - verification
  - quality
  - evidence
related:
  - docs/07-workflow/DEFINITION-OF-DONE.md
  - docs/10-engineering/RUST-VERIFICATION.md
  - docs/12-verification/CHECK-REGISTRY-STANDARD.md
  - docs/12-verification/EVIDENCE-PACKET-STANDARD.md
  - docs/12-verification/EXIT-CODE-STANDARD.md
---

# Verification Model

## Purpose

This document defines Monad’s verification model.

Monad should not ask users to trust generated changes, AI output, repository analysis, or tool coordination without evidence. Verification is the mechanism by which Monad earns trust.

## Core Rule

Monad work is not complete until the expected result has been verified or an exception has been explicitly documented.

Verification must answer:

```text
What was checked?
What passed?
What failed?
What was skipped?
What evidence exists?
What remains uncertain?
```

## Verification Philosophy

Monad uses the principle:

```text
Verification over vibes.
```

This means:

- do not rely on plausible output alone;
- do not treat AI output as truth;
- do not hide native tool failures;
- do not mark work complete without evidence;
- do not confuse a passing check with proof of everything;
- always state what the verification does and does not prove.

## Verification Layers

Monad verification can occur at several layers.

### 1. Documentation Verification

Checks whether documentation is present, structured, and machine-readable.

Examples:

- every Markdown file has YAML frontmatter;
- required docs exist;
- ADR index is current;
- work packet records include required fields;
- context files exist.

### 2. Rust Verification

Checks whether Rust code is formatted, compiled, tested, and linted.

Minimum Rust verification:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

### 3. CLI Verification

Checks whether user-facing commands work.

Examples:

```bash
cargo run -p monad-cli -- --help
cargo run -p monad-cli -- info
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- check
```

### 4. Repository Intelligence Verification

Checks whether Monad correctly detects repository structure, manifests, tools, and project graph information.

Examples:

- Cargo workspace detection;
- package manager detection;
- manifest detection;
- deterministic graph output;
- fixture-based inspection tests.

### 5. Context Verification

Checks whether context bridge artifacts exist and are coherent.

Examples:

- current-state file exists;
- handoff file exists;
- bootstrap prompt exists;
- context pack is generated;
- generated context identifies source files;
- generated context does not claim unreviewed truth.

### 6. Evolution Verification

Checks whether proposed file changes are safe and reviewable.

Examples:

- dry-run does not write files;
- conflicts are detected;
- existing files are not silently overwritten;
- planned operations are visible;
- apply behavior matches dry-run plan if apply exists.

### 7. Agent Verification

Checks whether agent-assisted output remains supervised and bounded.

Examples:

- plan does not modify files;
- provider output is marked as proposed;
- approval gates are represented;
- audit events are recorded;
- no unapproved command execution occurs.

## Verification Objects

Monad should eventually model verification using structured objects.

### Check Definition

A check definition describes what can be checked.

It should include:

- check ID;
- name;
- description;
- category;
- command or built-in behavior;
- required or optional status;
- severity;
- applicable conditions.

### Check Run

A check run is one execution of a check.

It should include:

- check ID;
- start time if needed;
- end time or duration if needed;
- working directory;
- command executed if applicable;
- status;
- stdout/stderr summary if applicable.

### Check Result

A check result states the outcome.

Possible statuses:

```text
passed
failed
warning
skipped
not_applicable
error
```

### Evidence Packet

An evidence packet records verification activity in reviewable form.

It should include:

- checks run;
- results;
- command summaries;
- failures;
- skipped checks;
- generated reports;
- known limitations.

## Human-Readable and Machine-Readable Output

Monad should support both:

```text
human-readable terminal output
machine-readable JSON output
```

Human-readable output helps developers.

Machine-readable output enables:

- CI;
- dashboards;
- future UIs;
- MCP tools;
- AI workflows;
- audit and evidence pipelines.

## Verification and Exit Codes

Exit codes must be meaningful.

A required failed check should normally cause a non-zero exit code.

Skipped optional checks should not necessarily fail the command.

See:

```text
docs/12-verification/EXIT-CODE-STANDARD.md
```

## Verification and Work Packets

Every work packet should define:

- verification commands;
- expected result after verification;
- what the evidence proves;
- what remains out of scope.

A work packet is not Done unless verification passes or an exception is documented.

## Verification and AI

AI output is not verification.

AI may help:

- propose tests;
- explain errors;
- suggest fixes;
- summarize command output;
- draft evidence packets.

But AI output does not replace actual checks.

## Verification and Native Tools

Monad coordinates native tools rather than replacing them.

Examples:

- Rust verification uses Cargo.
- JavaScript verification may use Bun, npm, pnpm, or yarn scripts.
- Go verification may use Go tooling.
- Python verification may use uv, pytest, ruff, mypy, or other native tools where configured.

Monad should report what native tool was used and what happened.

## MVP Verification Scope

The MVP verification scope should include:

- documentation frontmatter checks;
- Rust formatting/tests/Clippy;
- CLI smoke tests;
- basic command runner;
- check registry model;
- `monad check`;
- human-readable check report;
- evidence packet foundation;
- JSON output foundation.

## Verification Limits

Verification does not prove everything.

A passing test suite may not prove:

- product correctness;
- security correctness;
- business validity;
- complete architecture compliance;
- absence of bugs;
- absence of hallucinated assumptions.

Monad should make verification useful without overstating what it proves.

## Current Status

This verification model is a draft. It is authoritative enough to guide E0 documentation, E1 Rust verification, and E4 Verification Engine implementation.
