---
title: "Command Execution Safety"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - security
  - command-execution
  - safety
related:
  - docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
  - docs/10-engineering/RUST-CODING-STANDARD.md
  - docs/12-verification/VERIFICATION-MODEL.md
  - docs/09-ai/AI-COLLABORATION-RULES.md
---

# Command Execution Safety

## Purpose

This document defines Monad’s command execution safety principles.

Monad will eventually run native tools such as formatters, test runners, linters, package managers, and project-specific scripts. Command execution is powerful and must be handled carefully.

## Core Rule

Monad must not hide command execution.

When Monad runs a command, the user should be able to understand:

- what command ran;
- which arguments were used;
- where it ran;
- why it ran;
- what exit code occurred;
- what output matters;
- whether it passed or failed.

## Default Safety Position

Command execution should be:

```text
explicit
bounded
observable
non-destructive by default
reported
verifiable
```

## Avoid Shell by Default

Monad should avoid shell execution by default.

Prefer direct command execution:

```text
program: cargo
args: ["test"]
```

Avoid:

```text
sh -c "cargo test"
```

Shell execution increases risk because shell strings can introduce quoting bugs, injection risks, and hidden behavior.

## Working Directory

Every command should have an explicit working directory.

The command result should record the working directory where practical.

## Output Capture

Command execution should capture or summarize:

- stdout;
- stderr;
- exit code;
- duration where practical.

Monad should not hide failures.

## Exit Codes

Exit codes must be meaningful.

A failed required check should produce a failing Monad command when appropriate.

Skipped checks should be distinguishable from passed checks.

## No Unapproved Agent Execution

AI or agent workflows must not run commands without approval unless the command is clearly safe and the workflow explicitly allows it.

Riskier commands include:

- package installation;
- file deletion;
- migrations;
- deployment;
- publishing;
- credential-related commands;
- destructive Git operations;
- shell scripts from unknown sources.

## Verification Commands

Common safe verification commands include:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Even safe commands should still be visible to the user.

## Dangerous Command Categories

Monad should treat these as high risk:

- `rm -rf`;
- force pushes;
- destructive Git resets;
- deployment commands;
- publishing commands;
- migration apply commands;
- package install scripts from untrusted repos;
- commands that expose secrets;
- commands that modify system-level files;
- commands that require elevated privileges.

## MVP Command Runner Requirements

The MVP command runner should:

- run local commands explicitly;
- avoid shell by default;
- capture exit status;
- capture stdout/stderr;
- preserve working directory;
- report failures;
- be testable;
- not include remote execution;
- not include background execution;
- not include privileged execution.

## Future Policy Layer

Later Monad may introduce a policy layer that classifies commands as:

```text
safe
review-required
approval-required
forbidden
```

MVP should keep the model simple but avoid decisions that prevent future policy enforcement.

## Secrets Rule

Command output may contain secrets.

Monad should avoid storing full command output in generated context unless necessary.

Future evidence packets may need redaction rules.

## Current Status

This command execution safety document is a draft. It is authoritative enough to guide MVP command runner and verification engine work.
