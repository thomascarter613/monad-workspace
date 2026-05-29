---
title: Context Freshness Policy
description: Policy for keeping Monad repo-native context, handoff, decision, and release-state artifacts accurate after milestone changes.
status: draft
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
phase: Post-MVP Candidate Stabilization
epic: E9
work_packet: WP-E9-003
---

# Context Freshness Policy

## 1. Purpose

Monad uses repository-native context records so future development sessions can orient from the repository itself rather than relying on chat history.

This policy defines how those context records stay fresh after epics, work packets, release candidates, audits, and tags change.

## 2. Context record categories

Monad context records fall into four categories.

| Category | Purpose | Examples |
|---|---|---|
| Current state | Describes the present repository state. | `.monad/context/current-state.md`, `docs/09-ai/CURRENT-STATE.md` |
| Handoff | Helps a future session resume work. | `.monad/context/latest-handoff.md`, `docs/09-ai/FRESH-CHAT-HANDOFF.md` |
| Context pack | Bundles project state for AI/developer rehydration. | `.monad/context/latest-context-pack.md` |
| Durable history | Preserves milestone and decision continuity. | `.monad/context/decision-log.md`, `.monad/context/work-packet-handoffs/` |

## 3. Freshness rules

Context records should be refreshed when any of the following occur:

- an epic is opened or closed
- a work packet is opened, completed, or materially changed
- a release audit is completed
- a release candidate tag is created
- a public-readiness blocker is discovered or cleared
- a verifier policy changes
- a context-generation or handoff format changes
- a major architectural decision is recorded

## 4. Required refresh commands

The standard context refresh sequence is:

```bash
cargo run -p monad-cli -- context generate current-state
cargo run -p monad-cli -- context generate handoff
cargo run -p monad-cli -- context pack
```

Optional bootstrap refresh:

```bash
cargo run -p monad-cli -- context generate bootstrap
```

## 5. Release metadata expectations

After an internal candidate, release audit, or tag event, context should make the following discoverable somewhere in the repository context corpus:

- current epic
- current work packet
- latest internal candidate tag, if any
- release posture
- public-readiness blockers
- verification/audit status
- next recommended work packet

## 6. Historical continuity

Context verification should preserve historical continuity across the context corpus.

Historical terms do not need to appear in every current-state file. It is sufficient that durable handoff records preserve important prior milestones.

Examples of durable historical continuity include:

- E0 foundation records
- E1 runtime foundation records
- E2 repository intelligence records
- prior work packet handoff records
- prior decisions that still constrain the architecture

## 7. Anti-drift rule

Context records must not pretend future work is complete.

A context file may describe intended future work only if it clearly labels that work as planned, proposed, deferred, or not yet implemented.

## 8. Verification rule

Context verification should check:

- required context files exist
- required context files have YAML frontmatter
- durable historical terms remain discoverable
- generated context scratch output is ignored when appropriate
- current release state is discoverable somewhere in first-party release/context records

Context verification should not require every current context artifact to repeat every active epic, work packet, or release term.

## 9. Public-readiness rule

Before any public pre-release, context records must clearly state:

- whether the release is public or internal
- whether packages are published
- whether installers exist
- whether autonomous agent execution is implemented
- whether apply/write evolution is implemented
- whether known high-severity security blockers remain
