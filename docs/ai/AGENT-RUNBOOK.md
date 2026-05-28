---
title: Agent Runbook
description: Operator runbook for supervised Monad agent-assisted workflows.
status: draft
version: 0.1.0
created: 2026-05-28
updated: 2026-05-28
---

# Agent Runbook

## 1. Purpose

This runbook explains how a user or maintainer should operate future Monad agent-assisted workflows.

WP-E6-001 is documentation-only. No agent execution behavior is implemented by this packet.

## 2. Operating rule

Do not ask an agent to directly change the repository.

Ask the agent to:

1. explain
2. plan
3. draft
4. dry-run
5. verify
6. review
7. request approval
8. apply only through explicit controlled commands when implemented

## 3. Standard supervised session flow

### Step 1: State intent

Good intent statement:

```text
Plan a safe verification baseline improvement for this repository. Do not write files. Show the planned operations and verification commands.
````

Avoid:

```text
Fix everything and commit it.
```

### Step 2: Load repo context

The agent should inspect durable repo files before making claims.

Preferred context sources:

* README files
* docs architecture files
* ADRs
* work packets
* `.monad/context` files
* current-state handoffs
* verification reports

### Step 3: Ask for a plan

The plan should include:

* objective
* files likely affected
* planned operations
* risks
* approval gates
* verification commands
* expected result

### Step 4: Convert to file operations

The agent should represent changes as safe file operations:

* create
* update
* delete
* skip
* conflict
* no-op

### Step 5: Dry-run

Before writing files, run or request dry-run output.

Dry-run output should show:

* files that would be created
* files that would be updated
* files that would be skipped
* conflicts
* whether the plan appears safe to apply

### Step 6: Verify

After approved changes are eventually applied, run verification.

Typical commands:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

### Step 7: Review evidence

Review:

* dry-run output
* diffs
* verification results
* generated evidence packets
* risks and open questions

### Step 8: Approve or revise

Approve only specific actions.

Examples:

```text
Approved: apply the planned creation of docs/verification/README.md only.
```

```text
Not approved: revise the plan because it tries to overwrite existing files.
```

## 4. Agent modes in practice

### Explain

Use when learning the repository or diagnosing output.

Example:

```text
Explain why this Clippy error is happening and identify the smallest safe fix.
```

### Plan

Use before implementation.

Example:

```text
Plan WP-E6-002. Do not write code yet. Include files, risks, and verification.
```

### Draft

Use to create proposed file contents or planned operations.

Example:

```text
Draft the files for this work packet as planned operations. Do not apply them.
```

### Verify

Use to run or interpret verification.

Example:

```text
Interpret this cargo test failure and recommend the smallest repair.
```

### Repair

Use only after a failed verification result.

Example:

```text
Propose a repair for this compiler error. Keep the patch minimal.
```

### Review

Use before approval.

Example:

```text
Review this diff against the work packet scope and tell me if it is safe to commit.
```

### Apply

Apply mode is future work. It must remain approval-gated.

## 5. Required operator checks

Before approving agent-assisted changes, confirm:

* The work packet scope is clear.
* The plan is bounded.
* Existing files are not silently overwritten.
* Destructive operations are absent or explicitly approved.
* Verification commands are listed.
* Git status is understood.
* The final commit is atomic.
* The commit message follows Conventional Commits.

## 6. Stop conditions

Stop and revise when:

* the agent expands scope
* the plan touches unrelated files
* the plan includes unapproved deletion
* verification fails
* Git state is dirty unexpectedly
* generated output conflicts with ADRs
* the agent cannot explain why a change is needed
* the agent suggests pushing, deploying, or opening a PR without approval

## 7. Evidence expectations

A good supervised agent run leaves behind:

* a plan
* a dry-run preview
* a diff or file operation list
* verification output
* a commit message
* known limitations
* next recommended work packet

## 8. Current implementation status

As of WP-E6-001:

* supervised workflow is documented
* agent safety rules are documented
* approval gates are documented
* no model calls are implemented
* no `monad plan` command is implemented
* no agent execution is implemented
* no MCP integration is implemented
