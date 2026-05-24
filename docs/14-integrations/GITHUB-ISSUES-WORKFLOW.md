---
title: "GitHub Issues Workflow"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - integrations
  - github
  - issues
  - workflow
related:
  - docs/07-workflow/OPERATING-MODEL.md
  - docs/07-workflow/WORK-HIERARCHY.md
  - docs/07-workflow/WORK-PACKET-STANDARD.md
  - docs/14-integrations/GITHUB-PROJECTS-WORKFLOW.md
---

# GitHub Issues Workflow

## Purpose

This document defines how Monad uses GitHub Issues.

GitHub Issues are the active execution tracking system for epics, work packets, tasks, bugs, research items, ADR candidates, and risks.

The repository remains the canonical source of durable product, architecture, workflow, and implementation truth.

## Core Rule

GitHub Issues track work.

Repository docs preserve durable truth.

If an issue contains a decision that affects Monad long term, that decision must be promoted into a repository document or ADR.

## Issue Types

Monad uses the following issue types:

```text
Epic
Work Packet
Task
Bug
Research
ADR Candidate
Risk
```

## Epic Issues

Epic issues represent large outcomes composed of multiple work packets.

Example:

```text
[Epic]: E1 — Rust Core Foundation
```

An epic issue should include:

- Product Area;
- Objective;
- User Value;
- Scope;
- Expected Work Packets;
- Deliverables;
- Verification Strategy;
- Risks / Open Questions;
- Priority;
- Size.

## Work Packet Issues

Work packet issues are the primary delivery unit.

Example:

```text
[Work Packet]: WP-E1-003 — Add core error and diagnostic model
```

A work packet should include:

- Work Packet ID;
- Parent Epic ID;
- Work Packet Title;
- Product Area;
- Objective;
- User Value;
- Scope;
- Expected Files or Directories Affected;
- Tasks;
- Deliverables;
- Verification Commands / Evidence;
- Expected Result After Verification;
- Definition of Done;
- Recommended Conventional Commit;
- Risks / Blockers / Open Questions;
- Priority;
- Size.

Product Area appears before Objective.

Priority and Size appear at the end.

## Task Issues

Task issues should be used sparingly.

Most tasks should remain checklist items inside work packets.

Create a separate task issue only when the task:

- is independently assignable;
- is blocked separately;
- requires separate discussion;
- is too large for a checklist item;
- needs independent tracking.

## Bug Issues

Bug issues track broken behavior.

Bug issues should include:

- observed behavior;
- expected behavior;
- reproduction steps;
- environment;
- relevant command output;
- suspected area;
- severity;
- verification needed after fix.

## Research Issues

Research issues track investigation.

Research issues should include:

- question;
- reason the question matters;
- sources to inspect;
- expected output;
- decision or recommendation needed;
- follow-up work if applicable.

## ADR Candidate Issues

ADR candidate issues track decisions before they become accepted ADRs.

Use an ADR candidate when a decision affects:

- architecture;
- workflow;
- product identity;
- core technology;
- safety;
- verification;
- agent behavior;
- file operation model;
- command execution model.

Accepted decisions must be written into `docs/06-adrs/`.

## Risk Issues

Risk issues track known project risks.

Risk issues should include:

- risk description;
- probability;
- impact;
- mitigation;
- owner;
- review cadence;
- related epics or work packets.

## Labels

Recommended labels:

```text
type:epic
type:work-packet
type:task
type:bug
type:research
type:adr
type:risk

area:core
area:cli
area:repo-intelligence
area:context-bridge
area:verification
area:evolution
area:agents
area:docs
area:workflow
area:business
area:security
area:integrations
area:operations

priority:p0
priority:p1
priority:p2
priority:p3

needs-verification
context-update-required
rust-learning
status:blocked
status:needs-decision
good-first-issue
```

## Sub-Issue Structure

Preferred relationship:

```text
Epic
  → Work Packet
    → Task checklist or task issue
```

For example:

```text
E1 — Rust Core Foundation
  → WP-E1-001 — Create Rust workspace crates
  → WP-E1-002 — Add CLI shell
  → WP-E1-003 — Add core error and diagnostic model
```

## Issue Title Standards

Use these title formats:

```text
[Epic]: E1 — Rust Core Foundation
[Work Packet]: WP-E1-003 — Add core error and diagnostic model
[Bug]: Short bug title
[Research]: Short research title
[ADR Candidate]: Short decision topic
[Risk]: Short risk title
```

Use an em dash between IDs and titles.

## Issue Body Standards

Issue bodies should be structured enough for humans and AI assistants.

Avoid vague issue bodies such as:

```text
Do the thing.
```

Prefer clear fields, scope, and verification.

## Closing Issues

An issue should be closed only when its Definition of Done is met.

For a work packet, this normally means:

- scope complete;
- verification passed;
- expected result achieved;
- docs updated if needed;
- context updated if needed;
- atomic commit completed.

## GitHub CLI Automation

GitHub CLI may be used to create issues in bulk.

Example:

```bash
gh issue create \
  --repo OWNER/REPO \
  --title "[Work Packet]: WP-E1-003 — Add core error and diagnostic model" \
  --label "type:work-packet,area:core,priority:p0" \
  --body-file /tmp/wp-e1-003.md
```

Automation may create issues, labels, and project items, but manual cleanup may still be needed for:

- sub-issue relationships;
- custom project fields;
- final status review.

## Current Status

This GitHub Issues workflow is a draft. It is authoritative enough for early Monad planning and should be updated if GitHub project structure changes.
