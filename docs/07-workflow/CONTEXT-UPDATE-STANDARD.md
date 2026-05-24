---
title: "Context Update Standard"
document_type: "workflow-standard"
status: "draft"
version: "0.1.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E0"
work_packet: "WP-E0-004"
tags:

* workflow
* context
* handoff
* ai

---

# Context Update Standard

## 1. Purpose

This document defines when and how Monad context artifacts are updated.

Monad is repo-native. The repository is the source of truth for project state.

Context files exist so humans, future AI sessions, and tools can resume work from durable repo state rather than fragile chat memory.

## 2. Canonical Context Areas

Monad context currently lives in:

| Path                                    | Purpose                        |
| --------------------------------------- | ------------------------------ |
| `docs/09-ai/CURRENT-STATE.md`           | Human-readable current state   |
| `docs/09-ai/FRESH-CHAT-HANDOFF.md`      | Handoff for a new AI session   |
| `.monad/context/current-state.md`       | Operational current state      |
| `.monad/context/latest-handoff.md`      | Latest handoff summary         |
| `.monad/context/latest-context-pack.md` | Compact context pack           |
| `.monad/context/decision-log.md`        | Decision log                   |
| `.monad/context/session-chronicles/`    | Session records                |
| `.monad/context/work-packet-handoffs/`  | Work-packet-specific handoffs  |
| `.monad/context/decision-records/`      | Context-level decision records |

## 3. When Context MUST Be Updated

Context MUST be updated when:

* a work packet is completed;
* the next work packet changes;
* an ADR is accepted;
* a major product decision changes;
* a new blocker appears;
* verification status changes materially;
* the repo structure changes in a way future sessions must know;
* a handoff is needed before ending a long session.

## 4. When Context SHOULD Be Updated

Context SHOULD be updated when:

* important assumptions are made;
* non-obvious trade-offs are accepted;
* follow-up work is identified;
* a work packet is split or superseded;
* a new workflow standard is added;
* generated artifacts are introduced;
* toolchain expectations change.

## 5. Context Update Requirements

A context update SHOULD include:

* current epic;
* current work packet;
* completed work;
* changed files;
* accepted decisions;
* verification commands run;
* expected verification result;
* known blockers;
* next recommended action;
* commit status if known.

## 6. Handoff Quality Bar

A handoff is sufficient when a new session can answer:

* What project is this?
* What is the source of truth?
* What has been completed?
* What is currently active?
* What should happen next?
* Which files should be read first?
* What decisions are locked?
* What commands verify the current state?
* What should not be changed casually?

## 7. Decision Log Updates

The decision log SHOULD be updated when a decision is:

* durable;
* likely to affect implementation;
* likely to prevent future drift;
* not yet large enough for an ADR;
* useful for future handoff.

If a decision affects architecture, module boundaries, runtime strategy, product identity, or long-term governance, prefer an ADR.

## 8. Current State Updates

Current state files SHOULD distinguish:

* completed work;
* active work;
* next work;
* blocked work;
* known risks;
* accepted decisions;
* verification status.

## 9. Context Pack Updates

A context pack SHOULD be compact but complete enough for AI rehydration.

It SHOULD include:

* project identity;
* current epic and work packet;
* locked decisions;
* canonical read order;
* recent completed work;
* next action;
* verification command;
* user preferences relevant to execution.

## 10. Session Chronicles

Session chronicles SHOULD record meaningful session-level progress.

They SHOULD NOT become noisy transcripts.

A good session chronicle captures:

* date;
* session objective;
* work completed;
* files changed;
* decisions made;
* verification performed;
* next action.

## 11. AI Context Safety

Context files MUST NOT include secrets.

Context files SHOULD NOT include sensitive personal information unless explicitly necessary and intentionally provided.

Context files SHOULD avoid vague claims that cannot be verified from the repository.

## 12. Context Before Handoff

Before a major handoff:

1. Update current state.
2. Update latest handoff.
3. Update latest context pack.
4. Update decision log if needed.
5. Run relevant verification.
6. Provide the next work packet and read order.

## 13. Context After Commit

After a commit, context MAY be updated to include:

* commit hash;
* completed work packet;
* verification command result;
* next recommended work packet.

This is especially useful before a conversation handoff.

## 14. Maintenance Rules

Context standards SHOULD evolve as Monad's context bridge becomes executable, indexed, and validated by automated checks.
