---
title: "File Operation Safety"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - security
  - file-operations
  - safety
related:
  - docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
  - docs/05-architecture/MODULE-BOUNDARIES.md
  - docs/11-security/COMMAND-EXECUTION-SAFETY.md
  - docs/07-workflow/DEFINITION-OF-DONE.md
---

# File Operation Safety

## Purpose

This document defines Monad’s file operation safety principles.

Monad will eventually create, update, and possibly delete repository files. These operations are trust-critical.

## Core Rule

Monad must plan before it writes.

A user should be able to understand proposed file changes before they are applied.

## Default Safety Position

File operations should be:

```text
planned
reviewable
dry-run capable
non-destructive by default
conflict-aware
verifiable
```

## Planned Operation Model

Before writing files, Monad should represent proposed operations such as:

- create file;
- update file;
- skip existing file;
- report conflict;
- no-op;
- delete file only with explicit approval.

## Dry-Run First

Evolution workflows should support dry-run before apply.

Example:

```bash
monad evolve context-baseline --dry-run
monad evolve verify-baseline --dry-run
```

Dry-run output should show what would happen without changing files.

## No Silent Overwrites

Monad must not silently overwrite existing user-authored files.

When a target file exists, Monad should usually:

- skip it;
- report a conflict;
- show a diff;
- require explicit approval;
- write to an alternate path only if clearly indicated.

## Delete Operations

Delete operations are high risk.

During MVP, delete behavior should be avoided unless explicitly scoped and approved.

If delete operations are introduced later, they should require:

- explicit intent;
- preview;
- approval;
- verification;
- audit record where practical.

## Generated Files

Generated files should be clearly identified when appropriate.

Generated files may live under:

```text
.monad/
```

Canonical human-authored docs should live under:

```text
docs/
```

Generated context should not be confused with accepted documentation.

## Conflict Handling

A conflict exists when Monad’s planned operation cannot be applied safely.

Examples:

- target file already exists;
- expected old content does not match;
- path points outside repository root;
- file type is unsupported;
- operation would overwrite user work;
- operation would delete non-generated content.

Conflicts should be reported clearly.

## Repository Boundary

Monad should not write outside the repository root unless explicitly configured and approved.

Path handling must prevent accidental traversal outside the workspace.

## Binary Files

Binary file operations are higher risk and should be avoided during early MVP work unless explicitly needed.

## MVP File Operation Requirements

The MVP file operation model should support:

- create operation;
- update operation if safe;
- skip/no-op operation;
- conflict representation;
- dry-run plan;
- human-readable summary;
- tests using temporary directories.

Actual apply behavior can come after planning and dry-run behavior are proven.

## Verification

File operation work should verify:

- planned operations are represented correctly;
- dry-run does not write files;
- existing files produce skip or conflict behavior;
- paths remain inside expected workspace;
- tests pass.

Example verification:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Agent Rule

AI or agent workflows must not bypass file operation safety.

An agent must not directly write files without going through planned, reviewable, and approved file operation workflows.

## Current Status

This file operation safety document is a draft. It is authoritative enough to guide E5 Evolution Engine design and initial file operation implementation.
