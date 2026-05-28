---
title: Agent Safety Model
description: Security and governance boundaries for Monad supervised agent workflows.
status: draft
version: 0.1.0
created: 2026-05-28
updated: 2026-05-28
---

# Agent Safety Model

## 1. Purpose

This document defines safety boundaries for Monad agent-assisted workflows.

Monad must not become an uncontrolled coding agent. It must remain a supervised repository evolution system where the user controls plans, file changes, command execution, verification, and Git operations.

## 2. Safety goals

Monad agent features must protect:

- user source code
- local working tree state
- secrets and credentials
- Git history
- verification integrity
- project architecture
- user authority
- auditability

## 3. Trust model

AI model output is untrusted until reviewed or verified.

Model output may be useful, but it may also be incomplete, stale, fabricated, unsafe, or inconsistent with repository policy.

Monad must therefore separate:

- generated suggestions
- planned operations
- approved operations
- executed operations
- verified results

## 4. Authority model

The user has final authority.

Monad may recommend. Monad may not silently decide to perform risky actions.

The system should require explicit approval for:

- file writes
- destructive operations
- Git mutations
- networked side effects
- dependency installation
- deployment-related changes
- policy changes
- credential or secret handling

## 5. Command execution safety

Commands should be classified before execution.

### Read-only commands

Examples:

- `git status --porcelain=v1 --branch`
- `cargo fmt --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `find ... -type f`
- `sed -n ...`

Read-only commands may still need approval depending on future policy, but they are lower risk.

### Mutating local commands

Examples:

- `cargo fmt`
- file writes
- code generators
- dependency installation
- `git add`
- `git commit`
- `git reset`
- `git clean`

These require explicit approval.

### Remote or external side-effect commands

Examples:

- `git push`
- creating pull requests
- publishing packages
- deployment commands
- cloud provider commands

These require explicit approval and should remain out of early local-agent scope.

## 6. File safety

Agents must not directly overwrite files.

All proposed file changes must be represented through safe file operations and evaluated through dry-run planning before future apply behavior.

Existing files must produce conflict or update behavior according to policy. Silent overwrite is forbidden.

Delete operations must be treated as destructive and require elevated approval.

## 7. Git safety

Agent workflows must respect worktree and branch safety rules.

Future apply behavior should refuse unsafe Git states by default, including:

- dirty working tree
- detached HEAD
- unresolved merge
- rebase in progress
- untracked overlap with planned target
- protected branch concerns
- unknown Git state

WP-E6-001 does not implement Git mutation.

## 8. Verification safety

Agents must not claim success without verification.

Verification failures must be shown clearly. Repair suggestions must be planned and reviewed before changes are made.

A model-generated explanation of a failure is not itself a verified fix.

## 9. Secret safety

Agents must not request, expose, copy, summarize, or commit secrets.

If a secret appears in logs or files, Monad should avoid repeating it and should recommend remediation.

Future provider integrations must avoid sending secrets to model providers unless the user explicitly configures such behavior and understands the risk.

## 10. Context safety

Repo-native context should be treated as project memory, but not all context is public.

Future context workflows should distinguish:

- public project context
- internal developer context
- sensitive operational context
- secrets, credentials, and private data

Agents must not publish or export sensitive context without approval.

## 11. Approval gates

Approval gates must be explicit, scoped, and auditable.

A valid approval should answer:

- What action is approved?
- Which files or commands are involved?
- Is the action read-only or mutating?
- What risks are known?
- What verification will run afterward?
- Is approval for one action or a series?

## 12. Prohibited early behaviors

Until explicitly implemented and governed, Monad agents must not:

- run long-lived autonomous loops
- self-approve changes
- push commits
- open pull requests
- deploy applications
- install tools globally
- modify credentials
- rewrite Git history
- perform broad repository rewrites
- operate without repo-visible plans
- hide failed verification

## 13. Safety posture

The correct default is conservative refusal.

If Monad cannot determine whether an agent action is safe, it should stop, explain the uncertainty, and require human decision.
