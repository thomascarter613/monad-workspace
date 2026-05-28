---
title: Worktree and Branch Safety Strategy
description: Conservative safety rules for Monad repository evolution workflows.
status: draft
version: 0.1.0
created: 2026-05-28
updated: 2026-05-28
---

# Worktree and Branch Safety Strategy

## 1. Purpose

Monad's evolution engine exists to improve repositories safely.

Repository evolution is trust-critical because it can create, update, or eventually delete files. Monad must therefore avoid surprising users, damaging an active working tree, overwriting local work, or hiding important filesystem changes.

This strategy defines the initial rules for branch and worktree safety.

## 2. Scope

This strategy applies to Monad workflows that propose or prepare repository changes, including:

- `monad evolve verify-baseline`
- `monad evolve context-baseline`
- future template-driven baseline commands
- future safe apply commands
- future supervised agent-assisted repository evolution

This strategy does not define remote Git behavior, automatic pushing, automatic PR creation, deployment, merge conflict resolution, or multi-agent branch orchestration.

## 3. Core safety principles

Monad repository evolution must follow these principles:

1. **Dry-run first.** Evolution commands should support dry-run preview before writes.
2. **Human review first.** Planned file operations must be inspectable before apply.
3. **No silent overwrite.** Existing files must not be overwritten without explicit approval.
4. **No surprise deletion.** Delete operations require explicit approval and should be rare.
5. **No hidden Git mutation.** Monad must not create branches, worktrees, commits, pushes, or pull requests without explicit user intent.
6. **Conservative failure.** If Monad cannot determine whether a repository state is safe, it should warn or refuse to apply.
7. **Local state protection.** Uncommitted user changes must be protected.
8. **Deterministic plans.** The same repository state and inputs should produce the same plan.

## 4. Working tree cleanliness

Before applying repository evolution changes, Monad should inspect Git state when the repository is inside a Git work tree.

A working tree is considered **clean for direct apply** only when:

- Git can be executed successfully.
- The repository is inside a Git work tree.
- `git status --porcelain=v1 --branch` reports no tracked or untracked file changes.
- The current HEAD state is understandable.
- The command is not attempting destructive operations without explicit approval.

A working tree is considered **not clean** when any of the following are true:

- modified files exist
- staged changes exist
- deleted files exist
- renamed files exist
- copied files exist
- unmerged files exist
- untracked files exist
- Git status cannot be read
- the repository is not a Git work tree and the command requires Git safety guarantees

## 5. Direct apply rules

Monad may eventually allow direct apply only when all of the following are true:

- the user explicitly requests apply behavior
- the plan has no conflicts
- the plan has no unapproved destructive operations
- the working tree is clean or the command explicitly allows non-Git operation
- the files to be changed are represented in the dry-run plan
- the user has seen or can inspect the plan
- verification commands can be run after apply where feasible

WP-E5-006 does not implement apply behavior. These are future rules.

## 6. Branch and worktree isolation rules

For non-trivial evolution commands, Monad should prefer branch or worktree isolation.

Monad should recommend branch/worktree isolation when:

- the operation affects more than one file
- the operation touches configuration, CI, dependency, or source files
- the operation may require follow-up verification
- the repository has uncommitted changes
- the user is on a protected or important branch
- the operation is generated from templates or future AI-assisted workflows

Monad should require branch/worktree isolation when:

- future operations include destructive changes
- future operations include broad repository rewrites
- future operations involve agent-assisted edits
- future operations may affect many files
- future operations modify files outside clearly bounded baseline paths

## 7. Unsafe states

Monad should refuse or strongly warn before apply when it detects:

- dirty working tree
- untracked files that would overlap with planned targets
- detached HEAD
- merge in progress
- rebase in progress
- cherry-pick in progress
- unresolved conflicts
- no Git repository when Git safety is required
- plan conflicts
- delete operations without explicit approval

## 8. Initial Git status prototype

The first prototype should support a conservative read-only Git status check.

It may run:

```bash
git status --porcelain=v1 --branch
````

This command is read-only and provides enough information to detect:

* branch line
* detached HEAD
* tracked changes
* untracked paths
* ahead/behind summary when present

The prototype must not run:

* `git checkout`
* `git switch`
* `git branch`
* `git worktree add`
* `git add`
* `git commit`
* `git push`
* `git reset`
* `git clean`

## 9. Recommended safety behavior by mode

### Dry-run mode

Dry-run mode may run read-only Git status checks, but it must not write files or mutate Git state.

Dry-run output should include safety notes when useful.

### Apply mode

Apply mode is not implemented in WP-E5-006.

Future apply mode should require explicit user intent and should refuse unsafe states by default.

### Worktree mode

Worktree mode is not implemented in WP-E5-006.

Future worktree mode should create isolated worktrees only after explicit user request and should clearly display:

* worktree path
* branch name
* base branch or commit
* planned operations
* cleanup instructions

## 10. Branch naming strategy

Future generated branches should use deterministic, reviewable names such as:

```text
monad/evolve/<workflow>/<timestamp-or-short-id>
```

Examples:

```text
monad/evolve/verify-baseline/20260528-001
monad/evolve/context-baseline/20260528-001
```

Branch naming remains future work.

## 11. Definition of safe enough for current E5

For the current E5 foundation, Monad is safe enough when:

* file operations are planned before writes
* dry-run previews exist
* conflicts are represented clearly
* template source material is known and local
* evolution commands are dry-run only
* worktree/branch safety strategy is documented
* basic Git status detection exists as a read-only prototype

## 12. Future ADR recommendation

Before Monad adds apply behavior, branch creation, or worktree creation, this strategy should be promoted or refined into an ADR covering:

* direct apply policy
* branch creation policy
* worktree creation policy
* dirty working tree refusal rules
* protected branch behavior
* rollback expectations
* post-apply verification requirements
