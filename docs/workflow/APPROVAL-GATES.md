---
title: Approval Gates
description: Required approval gates for Monad supervised evolution and agent workflows.
status: draft
version: 0.1.0
created: 2026-05-28
updated: 2026-05-28
---

# Approval Gates

## 1. Purpose

Approval gates define when Monad must stop and require explicit user authorization.

They protect the user from accidental repository changes, unsafe command execution, destructive operations, and uncontrolled agent behavior.

## 2. Approval principle

No risky action without explicit approval.

Approval must be:

- intentional
- specific
- scoped
- reviewable
- auditable where practical

## 3. Gate levels

### Gate 0: Read-only explanation

Examples:

- explain code
- summarize docs
- inspect file names
- interpret compiler errors

Approval requirement:

- no explicit approval required beyond the user request

Restrictions:

- no writes
- no command execution unless separately allowed

### Gate 1: Read-only command execution

Examples:

- `git status --porcelain=v1 --branch`
- `cargo fmt --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`

Approval requirement:

- may be allowed by user request or workflow policy

Restrictions:

- no mutation
- output must be shown or summarized accurately

### Gate 2: Planning and dry-run

Examples:

- create file operation plan
- evaluate dry-run
- render proposed operations
- generate reviewable plan

Approval requirement:

- no write approval required because no writes occur

Restrictions:

- dry-run must not write files
- conflicts must be visible
- plan must not be represented as already applied

### Gate 3: Draft generation

Examples:

- draft file contents
- draft patches
- draft planned operations
- prepare sandboxed changes

Approval requirement:

- user approval required before drafts become repository writes

Restrictions:

- drafts must remain reviewable
- existing files must not be overwritten silently

### Gate 4: Local file writes

Examples:

- create a file
- update a file
- format files
- run a generator that writes files

Approval requirement:

- explicit approval required

Restrictions:

- must use safe file operation model when possible
- must respect worktree safety
- must show planned operations first

### Gate 5: Destructive local operations

Examples:

- delete files
- overwrite existing files
- reset files
- clean working tree
- remove generated directories

Approval requirement:

- elevated explicit approval required

Restrictions:

- must explain destructive impact
- must identify affected files
- must provide recovery guidance where practical

### Gate 6: Git history mutation

Examples:

- `git add`
- `git commit`
- `git commit --amend`
- `git rebase`
- `git reset`
- branch creation
- worktree creation

Approval requirement:

- explicit approval required

Restrictions:

- commit must be atomic
- commit message must be shown
- dirty state must be understood

### Gate 7: Remote side effects

Examples:

- `git push`
- creating pull requests
- publishing packages
- deployment
- cloud changes

Approval requirement:

- explicit high-confidence approval required

Restrictions:

- out of scope for early Monad supervised agent workflows
- must not be performed by autonomous agents

## 4. Approval examples

Good approval:

```text
Approved: create docs/verification/README.md exactly as shown in the dry-run plan.
````

Good approval:

```text
Approved: run cargo fmt to format the Rust files changed in this work packet.
```

Not sufficient:

```text
Do whatever you think is best.
```

Not sufficient:

```text
Fix it all and push.
```

## 5. Approval invalidation

Approval becomes invalid when:

* the plan changes materially
* new files are added to scope
* verification reveals new failures
* conflicts are detected
* Git state changes
* a command has broader effects than expected
* destructive behavior is introduced

When approval is invalidated, Monad must return to planning or review.

## 6. Relationship to E4 verification

Verification gates ensure changes are tested before completion.

A commit should not be recommended as complete unless verification has passed or the user explicitly accepts a documented exception.

## 7. Relationship to E5 evolution

Evolution commands must use approval gates before moving from dry-run to apply.

Current E5 commands are dry-run only. Future apply commands must enforce approval gates.

## 8. Relationship to E6 agents

Agents must not self-approve.

Agent recommendations are advisory. The user or an explicitly authorized human review step is the approval authority.

## 9. Minimum approval record

Future audit logs should record:

* action approved
* approving actor
* timestamp if available
* affected files or commands
* plan/evidence reference
* verification expectation
* whether approval was used

WP-E6-001 documents this expectation but does not implement audit logging.
