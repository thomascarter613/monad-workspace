---
title: "Exit Code Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - verification
  - cli
  - exit-codes
related:
  - docs/12-verification/VERIFICATION-MODEL.md
  - docs/12-verification/CHECK-REGISTRY-STANDARD.md
  - docs/10-engineering/RUST-VERIFICATION.md
  - docs/05-architecture/MODULE-BOUNDARIES.md
---

# Exit Code Standard

## Purpose

This document defines Monad’s initial exit code standard.

Exit codes are part of Monad’s contract with users, scripts, CI systems, and future automation.

## Core Rule

Monad exit codes must be predictable.

A command should return success only when the requested operation completed successfully according to its defined scope.

## Initial Exit Code Values

Recommended initial exit codes:

| Code | Meaning |
|---:|---|
| 0 | Success. |
| 1 | General failure. |
| 2 | Invalid usage or invalid arguments. |
| 3 | Verification failed. |
| 4 | Configuration or manifest error. |
| 5 | Workspace/repository resolution error. |
| 6 | Unsafe operation blocked. |
| 7 | External command failed. |

The MVP may begin with `0` and `1`, then refine as command behavior matures.

## Exit Code 0: Success

Use `0` when the command completed successfully.

Examples:

- `monad --help` displayed help.
- `monad inspect` completed inspection.
- `monad check` ran required checks and they passed.
- `monad evolve context-baseline --dry-run` produced a valid dry-run plan.

## Exit Code 1: General Failure

Use `1` for failures that do not yet have a more specific code.

This is acceptable during early MVP, but more specific codes should be introduced for stable command behavior.

## Exit Code 2: Invalid Usage

Use `2` when the user invoked Monad incorrectly.

Examples:

- unknown command;
- missing required argument;
- invalid option value;
- invalid output format.

CLI parsing libraries may already use this convention.

## Exit Code 3: Verification Failed

Use `3` when checks ran but one or more required checks failed.

Examples:

- tests failed;
- formatting check failed;
- Clippy failed;
- required docs check failed.

## Exit Code 4: Configuration or Manifest Error

Use `4` when Monad configuration is invalid.

Examples:

- invalid `monad.toml`;
- unsupported manifest version;
- malformed configuration value.

## Exit Code 5: Workspace Resolution Error

Use `5` when Monad cannot determine the repository or workspace context.

Examples:

- command requires repo root but none found;
- workspace root is ambiguous;
- required root marker is missing.

## Exit Code 6: Unsafe Operation Blocked

Use `6` when Monad refuses to perform an unsafe operation.

Examples:

- attempted write outside repository root;
- destructive operation requires approval;
- existing file conflict blocks apply;
- agent attempted unapproved action.

## Exit Code 7: External Command Failed

Use `7` when a native tool or external command failed in a way that is not specifically a verification failure.

Examples:

- command not found;
- process failed to spawn;
- permission denied;
- native command crashed unexpectedly.

## Verification Command Behavior

For `monad check`:

| Situation | Exit Code |
|---|---:|
| All required checks pass | 0 |
| Required check fails | 3 |
| Optional check skipped | 0 unless policy says otherwise |
| Native command cannot run | 7 or 3 depending on check requirement |
| Invalid check configuration | 4 |

## Dry-Run Command Behavior

For dry-run evolution commands:

| Situation | Exit Code |
|---|---:|
| Plan generated successfully | 0 |
| Conflict detected but represented as planned conflict | 0 or 6 depending on command contract |
| Unsafe path detected | 6 |
| Invalid arguments | 2 |
| Workspace cannot be resolved | 5 |

The command should distinguish between “a plan contains conflicts” and “Monad itself failed.”

## CLI and Core Boundary

`monad-core` should return structured errors and statuses.

`monad-cli` should map those structured outcomes to process exit codes.

This keeps exit-code behavior centralized and consistent.

## Error Display

When returning a non-zero exit code, Monad should explain:

- what failed;
- why it failed if known;
- what the user can do next;
- where evidence or logs can be found if applicable.

## Current Status

This exit code standard is a draft. It is authoritative enough to guide early CLI behavior and should be refined when `monad-cli` command handling becomes real.
