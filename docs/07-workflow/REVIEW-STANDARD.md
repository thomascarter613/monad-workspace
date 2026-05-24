---
title: "Review Standard"
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
* review
* quality
* governance

---

# Review Standard

## 1. Purpose

This document defines Monad's review standard.

Review protects the product from drift, accidental complexity, broken verification, and unexamined AI output.

## 2. Core Review Principle

A review is not only a style check.

A review asks whether the change is:

* correct;
* necessary;
* aligned with product direction;
* consistent with ADRs;
* verifiable;
* maintainable;
* safe;
* understandable by future contributors and AI sessions.

## 3. Review Inputs

A reviewer SHOULD inspect:

* work packet objective;
* changed files;
* `git diff`;
* relevant ADRs;
* relevant requirements;
* verification output;
* context updates;
* generated artifacts;
* dependency changes.

## 4. Review Checklist

For every non-trivial change, review:

### Scope

* Does the change match the work packet?
* Are unrelated changes excluded?
* Is the product area clear?
* Are assumptions explicit?

### Architecture

* Does the change respect module boundaries?
* Does it align with accepted ADRs?
* Does it avoid premature coupling?
* Does it keep durable logic in the correct layer?

### Documentation

* Are docs updated where needed?
* Do Markdown files have YAML frontmatter?
* Are references accurate?
* Are placeholders removed or clearly marked?

### Verification

* Are verification commands provided?
* Do expected results match actual results?
* Are failures documented?
* Is AI-generated work independently checked?

### Security and Safety

* Are secrets excluded?
* Are unsafe operations avoided?
* Are file operations planned before writes?
* Are user-controlled inputs treated carefully?

### Maintainability

* Is the change readable?
* Is complexity justified?
* Are names clear?
* Are future changes made easier rather than harder?

## 5. AI-Assisted Review

When reviewing AI-assisted output, check for:

* invented files;
* invented commands;
* invented dependencies;
* stale assumptions;
* architecture drift;
* unverified claims;
* overly broad edits;
* hidden placeholders;
* inconsistent terminology.

AI output is proposed, not verified.

## 6. Review Outcomes

Review outcomes SHOULD be one of:

| Outcome           | Meaning                                         |
| ----------------- | ----------------------------------------------- |
| Accept            | Change is ready                                 |
| Accept with Notes | Change is ready but follow-up is captured       |
| Request Changes   | Change must be revised                          |
| Block             | Change cannot proceed until blocker is resolved |
| Split             | Change is too broad and should be divided       |
| Supersede         | Change is replaced by a better approach         |

## 7. Self-Review

Before asking for review or committing, perform self-review:

```bash
git status --short
git diff --check
git diff --stat
```

Then inspect the actual diff.

## 8. Review Comments

Review comments SHOULD be:

* specific;
* actionable;
* tied to a file, section, command, or requirement;
* clear about severity.

Avoid vague comments like:

> This feels wrong.

Prefer:

> This adds CLI behavior directly in `monad-cli`; durable manifest validation should live in `monad-core` per the current module boundary.

## 9. Review and Context

A change SHOULD update context files when it affects:

* current state;
* next work packet;
* accepted decisions;
* project direction;
* known blockers;
* completed deliverables;
* verification status.

## 10. Review Completion

Review is complete when:

* requested changes are resolved;
* verification is acceptable;
* context updates are complete if needed;
* commit boundaries are clear;
* follow-up work is captured.

## 11. Maintenance Rules

This review standard SHOULD evolve as Monad introduces pull request templates, code owners, automated checks, and CI gates.
