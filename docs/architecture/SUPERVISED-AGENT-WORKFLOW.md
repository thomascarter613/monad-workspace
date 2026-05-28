---
title: Supervised Agent Workflow
description: Human-in-command workflow standard for Monad AI-assisted repository work.
status: draft
version: 0.1.0
created: 2026-05-28
updated: 2026-05-28
---

# Supervised Agent Workflow

## 1. Purpose

Monad agents assist the user. They do not silently take control of the repository.

The supervised agent workflow defines how Monad may use AI assistance safely: plan first, draft separately, verify, review, require approval, and apply only through explicit user-controlled steps.

This document is the architectural workflow standard for future E6 implementation work.

## 2. Core principle

The human remains in command.

An agent may propose, explain, analyze, draft, and help verify. An agent must not independently decide to mutate repository state, run risky commands, commit, push, open pull requests, deploy, or bypass approval gates.

## 3. Lifecycle

The standard supervised lifecycle is:

1. **Intent capture**
   - The user states a goal.
   - Monad records the requested outcome and constraints.
   - Ambiguity is resolved through explicit assumptions or user confirmation.

2. **Context loading**
   - Monad reads repo-native context.
   - Monad uses durable project files as the source of truth.
   - Chat memory and model output are not treated as canonical truth.

3. **Plan**
   - Monad produces a structured plan.
   - The plan identifies files, operations, risks, verification commands, and approval gates.
   - The plan must be reviewable before any writes occur.

4. **Draft**
   - Proposed changes are represented as planned file operations or sandboxed drafts.
   - Drafts must remain separate from uncontrolled direct writes.
   - Drafts should be reproducible from known inputs where practical.

5. **Dry-run**
   - Monad evaluates proposed file operations.
   - Conflicts, skips, no-ops, creates, updates, and deletes are visible.
   - No repository files are written in dry-run mode.

6. **Verify**
   - Monad identifies relevant verification commands.
   - Verification output should be captured as evidence.
   - Failed verification must block automatic progression.

7. **Review**
   - The user reviews the plan, diff, dry-run output, risks, and verification evidence.
   - Monad may summarize but must not hide material risks.

8. **Approval**
   - Explicit approval is required before any write, destructive operation, commit, push, pull request, or deployment.
   - Approval must be scoped to the specific plan or action.

9. **Apply**
   - Apply behavior must use the safe file operation model.
   - Future apply behavior should respect worktree and branch safety rules.
   - Destructive operations require elevated approval.

10. **Evidence and audit**
    - Actions, decisions, verification results, and approvals should be recorded.
    - The audit trail should be local-first and repo-compatible.

## 4. Agent modes

Monad may eventually support the following supervised agent modes.

### Explain mode

Explains repository structure, code, errors, logs, design decisions, or workflows.

Allowed:
- read files
- summarize repository state
- explain commands
- explain verification failures

Not allowed:
- write files
- run commands without user approval
- modify Git state

### Plan mode

Produces structured implementation or remediation plans.

Allowed:
- propose steps
- identify files likely to change
- identify verification commands
- identify risks and approval gates

Not allowed:
- apply changes
- commit changes
- present unverified assumptions as fact

### Draft mode

Creates proposed file operations or draft patches.

Allowed:
- produce planned file operations
- produce reviewable patch-like output
- use known templates
- stage drafts in a controlled representation

Not allowed:
- overwrite files directly
- delete files directly
- bypass dry-run planning

### Verify mode

Runs or recommends verification.

Allowed:
- run approved safe verification commands
- capture evidence packets
- parse check results
- summarize failures

Not allowed:
- repair failures without approval
- hide failed checks
- mark unverified output as verified

### Repair mode

Proposes fixes for failed verification.

Allowed:
- inspect failure output
- propose repair plan
- draft repair operations
- request approval for changes

Not allowed:
- automatically patch without approval
- repeatedly mutate files without review
- ignore new failures introduced by repair

### Review mode

Reviews proposed changes, plans, or diffs.

Allowed:
- check consistency with architecture
- identify risks
- compare against ADRs and policies
- recommend approval, revision, or rejection

Not allowed:
- approve its own work as final authority
- suppress concerns to make a plan look complete

### Apply mode

Applies approved changes.

Allowed only when future implementation provides:
- explicit user approval
- safe file operation plan
- worktree/branch safety checks
- conflict detection
- audit entry

Not allowed:
- implicit apply
- destructive apply without elevated approval
- direct modification of dirty working trees unless policy explicitly permits it

## 5. Relationship to safe file operations

All file mutations must flow through the safe file operation model.

Agent-drafted changes should become one or more planned operations:

- create
- update
- delete
- skip
- conflict
- no-op

The dry-run planner must evaluate these operations before any future apply path writes files.

## 6. Relationship to verification

Agent workflows must treat verification as a gate, not a decoration.

A supervised agent workflow should identify:

- formatting commands
- tests
- Clippy or linting checks
- repo contract checks
- context validation checks
- command-specific verification

Verification evidence should be captured when possible.

## 7. Relationship to evidence packets

Evidence packets provide the review trail for agent-assisted work.

Agent workflows should eventually attach or reference:

- plan summary
- dry-run output
- selected checks
- command summaries
- verification results
- known failures
- unresolved risks

Evidence must distinguish between model claims and verified facts.

## 8. Relationship to repo-native context

Monad agents should use repo-native context as the durable source of project truth.

Relevant context sources may include:

- README files
- docs architecture files
- ADRs
- work packets
- `.monad/context` files
- handoff packets
- current-state files
- verification reports

The agent must not treat transient chat history as more authoritative than committed project files.

## 9. Approval requirements

Approval is required before:

- writing files
- deleting files
- modifying Git state
- creating branches
- creating worktrees
- staging files
- committing
- pushing
- opening pull requests
- running risky commands
- installing dependencies
- changing secrets or configuration
- changing CI/CD behavior
- modifying production or deployment settings

Approval must be specific to the action.

## 10. Forbidden actions without explicit approval

An agent must not do any of the following without explicit user approval:

- write repository files
- overwrite existing files
- delete files
- run destructive shell commands
- install dependencies
- run networked commands that alter remote state
- create commits
- amend commits
- rebase
- reset
- clean the working tree
- push to remotes
- create pull requests
- deploy services
- change secrets
- change credentials
- bypass failed verification
- mark unreviewed changes as complete

## 11. Completion standard

A supervised agent task is complete only when:

- the plan was reviewable
- planned file operations were visible
- conflicts were handled
- verification was run or explicitly deferred
- evidence was captured or summarized
- remaining risks were stated
- the user retained final authority
