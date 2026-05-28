---
title: Draft Sandbox Workflow
description: Safe draft workflow for supervised AI-assisted repository changes.
status: draft
version: 0.1.0
created: 2026-05-28
updated: 2026-05-28
---

# Draft Sandbox Workflow

## 1. Purpose

The draft sandbox workflow defines how Monad prepares AI-assisted repository changes without immediately mutating the user's primary working tree.

A draft is a reviewable, non-authoritative proposal. It may contain intended file operations, explanations, verification expectations, and approval requirements, but it must not itself bypass dry-run planning, user review, or file-operation safety.

## 2. Core principle

Drafts are proposals, not applied changes.

A draft may describe what should happen. A draft may contain planned file operations. A draft may be inspected, verified conceptually, accepted, rejected, or superseded. A draft must not silently write files, mutate Git state, push, open pull requests, deploy, or approve itself.

## 3. Relationship to supervised agent workflow

The draft sandbox sits between plan and apply:

1. User intent is captured.
2. Monad creates a supervised plan.
3. Monad creates a draft from the approved planning direction.
4. Draft operations are represented using the safe file operation model.
5. Draft operations are evaluated through dry-run.
6. The user reviews the draft and dry-run output.
7. The user approves, rejects, or revises.
8. Future apply behavior may write files only through explicit approval gates.

## 4. Draft lifecycle

A draft may move through these states:

- **Proposed** — draft exists as a candidate proposal.
- **Reviewed** — draft has been inspected by a user or review process.
- **Approved** — user approved the draft for a specific next action.
- **Rejected** — user declined the draft.
- **Superseded** — a newer draft replaced this draft.
- **Applied** — future state for drafts that have been applied through safe operations.

WP-E6-004 defines the model but does not implement file application.

## 5. Draft sandbox kinds

Monad may eventually support several draft storage strategies:

- **In-memory draft** — exists only during a command or session.
- **File-backed draft** — stored under `.monad/drafts`.
- **Branch-backed draft** — represented on a Git branch.
- **Worktree-backed draft** — represented in an isolated Git worktree.

WP-E6-004 models the concept but does not create branches, create worktrees, or write draft files.

## 6. Relationship to safe file operations

Drafts must connect to E5 safe file operation planning.

A draft operation should describe one intended file operation:

- create
- update
- delete
- skip
- conflict
- no-op

Drafts should preserve:

- operation kind
- target path
- explanation
- whether approval is required

A draft must not directly overwrite existing files. Existing-file behavior must still be resolved by dry-run evaluation and later explicit approval.

## 7. Relationship to worktree safety

Future applied drafts must respect the worktree and branch safety strategy.

If a draft is non-trivial, Monad should recommend branch or worktree isolation before apply. If Git state is dirty, detached, conflicted, or otherwise unsafe, Monad should refuse direct apply by default.

## 8. Relationship to verification

A draft should include expected verification commands or verification notes where practical.

A draft is not verified simply because a model generated it. Verification evidence must come from actual approved checks.

## 9. Relationship to approval gates

Drafts require approval before any mutation.

Approval must be specific:

- which draft is approved
- which operations are approved
- whether destructive operations are included
- which verification should follow
- whether apply is direct, branch-backed, or worktree-backed

Agents must not self-approve drafts.

## 10. Forbidden draft behavior

A draft workflow must not:

- write files without approval
- overwrite files silently
- delete files without elevated approval
- mutate Git state
- create branches automatically
- create worktrees automatically
- push to remotes
- open pull requests
- deploy
- hide conflicts
- treat model output as verified truth

## 11. Minimum draft record

A useful draft record should contain:

- draft ID
- title
- summary
- state
- sandbox kind
- file operations
- approval requirement
- verification notes
- known risks

## 12. WP-E6-004 implementation target

The first implementation should define:

- draft ID
- draft state
- draft sandbox kind
- draft file operation relationship
- draft object
- conversion from a file operation plan into a draft
- tests proving draft behavior is non-mutating and reviewable

No apply behavior is implemented in this packet.
