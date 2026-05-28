---
title: "Bootstrap Prompt"
document_type: "bootstrap-prompt"
artifact_type: "bootstrap-prompt"
status: "current"
generated: true
reviewed: false
project: "Monad"
source: "repository"
source_files:
  - "monad.toml"
  - "README.md"
  - "docs/01-project/01-charter/PRODUCT-CHARTER.md"
  - "docs/05-architecture/SYSTEM-OVERVIEW.md"
  - "docs/07-workflow/OPERATING-MODEL.md"
  - "docs/08-context/CONTEXT-BRIDGE.md"
  - ".monad/context/current-state.md"
  - ".monad/context/latest-handoff.md"
  - ".monad/context/latest-context-pack.md"
  - "work/epics/"
  - "work/packets/"
---

# Bootstrap Prompt — Monad

## Project Identity

**Project:** Monad
**Description:** Monad is an AI-native, repo-native, local-first Software Foundry OS for understanding, verifying, and safely evolving software repositories.
**Progress:** 2 of 3 epics completed.

## Source of Truth

The repository is the canonical source of truth. Use repo files — not chat history, not memory, not assumptions — as the authoritative reference for project state, decisions, and work.

Do not rely on prior conversation context. Read the files listed below.

## Required Reading Order

Read these files in order before beginning any work:

1. `.monad/context/current-state.md`
2. `.monad/context/latest-handoff.md`
3. `.monad/context/latest-context-pack.md`
4. `docs/01-project/01-charter/PRODUCT-CHARTER.md`
5. `docs/05-architecture/SYSTEM-OVERVIEW.md`
6. `docs/07-workflow/OPERATING-MODEL.md`
7. `docs/08-context/CONTEXT-BRIDGE.md`
8. `README.md`

## Current Work

**Active Epic:** E2 — Repository Intelligence Foundation
**Active Work Packet:** WP-E2-001 — Establish Repository Inspection Foundation

Read the work packet file for scope, tasks, deliverables, and verification commands.

## Workflow Rules

- Read current-state and handoff before starting work.
- Follow the active work packet scope; do not expand beyond it.
- Use conventional commits (feat, fix, docs, refactor, test, chore).
- Run verification before committing: cargo fmt, cargo test, cargo clippy.
- Update context artifacts after completing a work packet.
- Do not introduce new dependencies without an ADR or explicit approval.
- Do not modify files outside the work packet scope without justification.
- Prefer small, reviewable changes over large rewrites.
- Treat docs/ and work/ files as canonical; do not contradict them.
- If uncertain, ask rather than assume.

## Response Expectations

- Be concise and precise.
- Reference file paths when discussing code or docs.
- Propose changes as diffs or file edits, not vague descriptions.
- Verify your changes compile and pass tests before presenting them.
- If something is ambiguous, state the ambiguity and ask for clarification.
- Do not invent requirements that are not in the work packet.

## Continuation Protocol

When resuming work in a new session:

1. Read the bootstrap prompt (this file).
2. Read `.monad/context/current-state.md` for project status.
3. Read `.monad/context/latest-handoff.md` for session continuity.
4. Read the active work packet file for task details.
5. Run `cargo test` and `cargo clippy` to confirm the baseline is clean.
6. Continue from where the handoff left off.

Do not start over. Do not re-derive decisions that are already accepted.
Do not contradict docs/ or ADRs without proposing a new ADR.
