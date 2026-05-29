---
title: MCP Integration Strategy
description: Strategy for exposing selected Monad capabilities through Model Context Protocol-compatible surfaces.
status: draft
version: 0.1.0
created: 2026-05-28
updated: 2026-05-28
---

# MCP Integration Strategy

## 1. Purpose

Monad should integrate with the broader AI tooling ecosystem without surrendering its local-first, repo-native, human-in-command safety model.

Model Context Protocol compatibility is a future integration surface for exposing selected Monad capabilities to compatible AI clients through tool, resource, and prompt concepts.

WP-E6-006 establishes the strategy and a minimal crate boundary. It does not implement a full MCP server.

## 2. Strategic position

MCP is an integration boundary, not Monad's core identity.

Monad's core value remains:

- local CLI workflows
- repo-native context
- safe file operation planning
- dry-run previews
- verification evidence
- approval gates
- auditability
- provider-agnostic agent supervision

MCP should expose selected Monad capabilities only when they can preserve those safety properties.

## 3. Candidate MCP resources

Future MCP resources may expose read-only, reviewable repository information.

Candidate resources:

- repository summary
- workspace manifest summary
- `monad.toml` interpretation
- current context state
- work packet handoff summaries
- ADR index summaries
- verification report summaries
- dry-run plan summaries
- audit log summaries
- approval gate summaries

Resource exposure should default to read-only.

## 4. Candidate MCP tools

Future MCP tools may expose safe Monad commands to AI clients.

Candidate low-risk tools:

- inspect repository
- list workspace packages
- render repository graph
- validate manifest
- render current context
- run read-only context verification
- produce supervised plan
- produce dry-run evolution preview
- summarize approval requirements
- summarize verification evidence

Candidate higher-risk tools requiring explicit approval gates before implementation:

- draft file operation plan
- prepare draft sandbox
- run approved verification commands
- apply approved file operations
- create branch or worktree
- stage or commit changes

WP-E6-006 does not expose mutating tools.

## 5. Candidate MCP prompts

Future MCP prompts may standardize safe workflows.

Candidate prompts:

- Explain this repository.
- Plan a safe change.
- Review this proposed file operation plan.
- Summarize verification evidence.
- Prepare a context handoff.
- Identify approval gates for this action.
- Explain why this action is unsafe.

Prompts must clearly distinguish planning from execution.

## 6. Capability classification

Monad capabilities should be classified before MCP exposure.

### Read-only

Examples:

- inspect repo
- read context
- summarize docs
- render graph
- render plan

Default MCP exposure may be acceptable.

### Dry-run only

Examples:

- preview verify baseline evolution
- preview context baseline evolution
- preview planned file operations

MCP exposure may be acceptable if output states that no files were written.

### Approval-gated mutating

Examples:

- write files
- apply drafts
- format files
- create branches
- create worktrees
- commit changes

Do not expose until approval gates, audit events, and worktree safety are enforceable.

### Forbidden for early MCP

Examples:

- push to remote
- deploy
- publish packages
- modify secrets
- rewrite history
- run arbitrary shell commands

Do not expose in early MCP integration.

## 7. Server boundary

A future `monad-mcp` crate may eventually provide MCP-compatible server behavior.

The crate should depend on stable `monad-core` abstractions rather than duplicating business logic.

The crate should not own:

- workspace inspection rules
- file operation safety
- approval gate policy
- audit model
- verification model
- provider abstraction
- context model

Those belong in `monad-core`.

## 8. Local-first operation

The first MCP implementation should be local-first.

Preferred early shape:

- user starts MCP server explicitly
- server runs against one repository root
- server exposes read-only and dry-run capabilities first
- server never writes files without explicit approval support
- server never performs remote side effects

## 9. Relationship to provider abstraction

MCP integration must remain provider-agnostic.

Monad should not assume:

- one model provider
- one paid subscription
- one hosted service
- one AI client

MCP is a protocol integration surface. It is not a model provider.

## 10. Relationship to approval gates

MCP tools must respect approval gates.

Any future tool that can mutate state must:

- declare its gate kind
- produce a proposed action
- require explicit approval
- emit audit events
- support dry-run where possible
- refuse unsafe working tree states when applicable

## 11. Relationship to audit logs

MCP-exposed consequential actions should be auditable.

Future audit events should record:

- tool invoked
- actor or client identity if available
- repository root or subject
- proposed action
- approval decision
- execution result
- verification evidence reference

WP-E6-006 does not implement persisted audit storage.

## 12. Non-goals

This strategy does not implement:

- full MCP server protocol
- authentication
- authorization
- streaming transport
- hosted services
- remote control plane
- marketplace distribution
- provider-specific agent integrations
- write/apply MCP tools
- arbitrary shell execution

## 13. Initial crate placeholder

The `monad-mcp` crate is introduced as a placeholder boundary only.

It should compile and document the intended integration categories, but it should not expose operational MCP server behavior yet.
