---
title: MCP Safety Boundaries
description: Safety boundaries for future Monad Model Context Protocol integration.
status: draft
version: 0.1.0
created: 2026-05-28
updated: 2026-05-28
---

# MCP Safety Boundaries

## 1. Purpose

This document defines safety boundaries for future Monad MCP-compatible tools, resources, and prompts.

MCP integration must not create a back door around Monad's supervised workflow, safe file operation model, approval gates, audit model, or worktree safety strategy.

## 2. Primary safety rule

MCP clients must not receive more authority than the local Monad CLI would grant.

If an action requires approval in Monad, it also requires approval when requested through MCP.

If an action is forbidden in Monad's early supervised workflow, it is forbidden through MCP.

## 3. Read-only first

Initial MCP exposure should be read-only or dry-run only.

Allowed early categories:

- repository inspection
- context reading
- manifest interpretation
- graph rendering
- verification summary reading
- supervised planning
- dry-run previews
- approval requirement summaries

Not allowed early:

- file writes
- file deletion
- arbitrary command execution
- Git mutation
- remote side effects
- deployment
- credential changes

## 4. No unsafe write tools

Do not expose tools that can directly:

- overwrite files
- delete files
- mutate Git state
- stage files
- commit files
- push branches
- open pull requests
- deploy services
- publish packages
- modify secrets

These may only be considered later with explicit approval gates, audit logging, worktree safety, and narrow scope.

## 5. No arbitrary shell tool

Monad should not expose a general-purpose shell execution MCP tool in early versions.

A general shell tool is too broad because it can bypass:

- file operation planning
- approval gates
- audit expectations
- command classification
- worktree safety
- user review

Future command execution must be narrow, classified, and approval-gated.

## 6. Context exposure safety

MCP resources may expose repo context, but context can contain sensitive data.

Future MCP resource design should classify context as:

- public
- internal
- sensitive
- secret-bearing

Early implementation should avoid exposing secrets and should not assume all repo files are safe to send to AI clients.

## 7. Prompt injection risk

Repository text may contain malicious or misleading instructions.

MCP clients and Monad integration layers must treat repository content as data, not authority.

Repo content must not be allowed to override:

- Monad safety policy
- user approval requirements
- tool restrictions
- system instructions
- provider configuration

## 8. Approval gate preservation

Future mutating MCP tools must produce or reference approval gates before execution.

Required information:

- proposed action
- affected files or targets
- gate kind
- reason for approval
- whether elevated approval is required
- dry-run output if applicable
- verification expectation

MCP tools must not self-approve.

## 9. Audit event preservation

Future consequential MCP actions should emit audit events.

Audit events should include:

- MCP capability invoked
- actor/client identity if available
- proposed action ID
- approval gate ID
- decision ID if applicable
- execution result
- verification evidence reference if applicable

WP-E6-006 does not implement audit persistence.

## 10. Worktree safety preservation

MCP integration must not bypass worktree and branch safety rules.

Future write/apply tools should refuse:

- dirty working tree
- detached HEAD
- merge/rebase conflict state
- untracked overlap with planned targets
- unknown Git state
- protected branch operations without policy

## 11. Provider-agnostic boundary

MCP integration must not assume a specific AI provider.

MCP is a client/tool protocol surface, not a provider requirement.

Monad must remain compatible with:

- local models
- hosted providers
- self-hosted services
- user-supplied providers
- no-provider local deterministic workflows

## 12. Early allowed surface

The first safe MCP surface should be limited to descriptors and read-only planning concepts:

- capability descriptors
- read-only resource descriptors
- dry-run tool descriptors
- prompt descriptors
- safety classification metadata

## 13. Explicit non-goals

Do not implement the following in WP-E6-006:

- full MCP server
- streaming protocol
- authentication
- authorization
- hosted service
- provider integration
- write tools
- apply tools
- remote Git operations
- deployment tools
- marketplace packaging
