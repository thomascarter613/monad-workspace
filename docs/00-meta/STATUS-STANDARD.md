---
title: "Status Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - documentation
  - status
  - standard
related:
  - docs/00-meta/FRONTMATTER-STANDARD.md
  - docs/07-workflow/WORK-HIERARCHY.md
  - docs/07-workflow/DEFINITION-OF-DONE.md
---

# Status Standard

## Purpose

This document defines status values used in Monad documentation and project workflow.

Monad uses two related but different kinds of status:

1. **Documentation status** — the maturity of a Markdown document.
2. **Work status** — the workflow state of an epic, work packet, task, bug, research item, or ADR candidate.

These should not be confused.

## Documentation Status Values

Documentation status appears in Markdown frontmatter:

```yaml
status: draft
```

Allowed documentation statuses:

```text
stub
draft
review
accepted
superseded
archived
```

## `stub`

A `stub` document is a planned knowledge location.

It means:

* the file exists;
* the topic has a known home;
* the content is incomplete;
* the document should not be treated as canonical guidance yet.

A stub should include:

* title;
* purpose;
* status note;
* expected contents;
* related docs if known.

## `draft`

A `draft` document has meaningful content but is still expected to change.

It may guide active work, but readers should understand that it is not final.

Use `draft` when:

* the document is useful;
* the content has enough detail to guide work;
* open questions remain;
* implementation may still refine the document.

## `review`

A `review` document is ready for focused review.

Use `review` when:

* the content is believed to be complete;
* the author wants review before acceptance;
* the document may become canonical soon.

## `accepted`

An `accepted` document is canonical project guidance.

Use `accepted` only when:

* the document has been reviewed enough for current needs;
* the project intends future work to follow it;
* changing it should require deliberate action.

Accepted documents are not immutable, but they should not be changed casually.

## `superseded`

A `superseded` document has been replaced by newer guidance.

A superseded document should identify the replacing document when possible.

Use frontmatter such as:

```yaml
superseded_by:
  - docs/05-architecture/NEW-DOCUMENT.md
```

## `archived`

An `archived` document is retained for history but is no longer active guidance.

Archived documents should explain why they are archived.

## Documentation Status Transitions

Recommended transitions:

```text
stub → draft → review → accepted
draft → superseded
accepted → superseded
draft → archived
superseded → archived
```

Do not move a document to `accepted` unless it is intended to guide future work.

## Work Status Values

Work status is used in GitHub Projects and planning records.

Allowed work statuses:

```text
Inbox
Ready
Active
Blocked
Review
Done
Deferred
```

## `Inbox`

Newly captured item that has not yet been triaged, clarified, prioritized, or assigned to an epic/work packet.

Use `Inbox` for:

* raw ideas;
* possible tasks;
* rough bugs;
* research questions;
* untriaged work.

## `Ready`

Item has been reviewed and is clear enough to start.

For a work packet, `Ready` means it satisfies the Definition of Ready:

* objective is clear;
* product area is identified;
* scope is bounded;
* deliverables are known;
* verification is defined;
* expected result after verification is stated;
* priority and size are set.

## `Active`

Work is currently in progress.

Use `Active` sparingly. Monad should normally have very few active work packets at once.

A work packet should become `Active` only when someone is actually working it.

## `Blocked`

Work cannot continue because something unresolved prevents progress.

Examples:

* missing decision;
* unavailable credential;
* broken tooling;
* unclear requirement;
* dependency on another work packet;
* failing verification that must be resolved first.

Blocked work should state the blocker clearly.

## `Review`

Work is substantially complete and awaiting inspection, verification, acceptance, merge, or final approval.

Use `Review` when:

* work has been implemented or drafted;
* verification evidence exists or is pending final check;
* someone must inspect the result before `Done`.

## `Done`

Work is complete, verified, accepted, and no longer requires action.

For implementation work, `Done` usually means:

* scope completed;
* verification passed;
* docs updated if needed;
* context updated if needed;
* atomic commit completed;
* issue/work packet can be closed.

## `Deferred`

Work is valid but intentionally postponed.

Deferred does not mean rejected. It means the work is not part of the current focus, milestone, or MVP slice.

## Work Status Transition Guidance

Typical flow:

```text
Inbox → Ready → Active → Review → Done
```

Blocked flow:

```text
Active → Blocked → Active
```

Deferred flow:

```text
Inbox → Deferred
Ready → Deferred
Deferred → Ready
```

## Status Ownership

Documentation status is owned by the repository.

Work status is owned by the active planning system, currently GitHub Projects.

If there is disagreement between GitHub status and repo context files, the discrepancy should be resolved by updating the stale source.

## Status and Context Handoff

Before a session handoff, update the active context files to reflect:

* active epic;
* active work packet;
* completed work;
* blocked work if any;
* next recommended action.

This prevents future sessions from relying on stale status.

## Current Status

This status standard is a draft. It is authoritative enough for the initial Monad documentation and GitHub planning foundation.
