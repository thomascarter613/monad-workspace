---
title: "Definition Of Ready"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - workflow
  - definition-of-ready
related:
  - docs/07-workflow/OPERATING-MODEL.md
  - docs/07-workflow/WORK-PACKET-STANDARD.md
  - docs/07-workflow/DEFINITION-OF-DONE.md
---

# Definition Of Ready

## Purpose

This document defines when Monad work is ready to begin.

A work packet should not become `Ready` or `Active` until it is clear enough to execute without guessing at the objective, scope, deliverables, or verification.

## Ready Means

Ready means:

> The work is sufficiently understood, bounded, and verifiable that implementation or documentation work can begin.

Ready does not mean all details are known.

Ready means unknowns are either resolved or explicitly captured.

## Work Packet Ready Checklist

A work packet is ready when it has:

- [ ] Work Packet ID.
- [ ] Parent Epic ID.
- [ ] Work Packet Title.
- [ ] Product Area.
- [ ] Objective.
- [ ] User Value.
- [ ] Scope.
- [ ] Expected Files or Directories Affected.
- [ ] Tasks.
- [ ] Deliverables.
- [ ] Verification Commands / Evidence.
- [ ] Expected Result After Verification.
- [ ] Definition of Done.
- [ ] Recommended Conventional Commit.
- [ ] Risks / Blockers / Open Questions.
- [ ] Priority.
- [ ] Size.

## Product Area Requirement

Product Area must be identified before Objective.

This keeps work connected to Monad’s product structure and makes planning easier.

## Objective Requirement

The objective must state what will be accomplished.

Weak objective:

```text
Work on docs.
````

Ready objective:

```text
Define Monad’s workflow foundation by writing the operating model, work hierarchy, work packet standard, Definition of Ready, and Definition of Done.
```

## Scope Requirement

Scope must define both:

```text
In scope
Out of scope
```

This prevents work packets from becoming uncontrolled containers for related ideas.

## Verification Requirement

A ready work packet must define verification before work starts.

Verification may be:

* automated;
* manual;
* file existence checks;
* command output;
* tests;
* generated reports;
* review checklist.

## Expected Result Requirement

A ready work packet must include Expected Result After Verification.

This is required because passing a command is not always enough. The work packet should explain what the passing result proves.

## Blocker Rule

A work packet is not ready if:

* it depends on an unresolved decision;
* it has no verification path;
* the scope is too broad;
* expected result is unclear;
* priority is missing;
* size is unknown without a discovery task;
* the next action is ambiguous.

If uncertainty remains, create or complete a research/discovery work packet first.

## Ready for Documentation Work

Documentation work is ready when:

* target files are known;
* topic boundaries are clear;
* status is known;
* related docs are identified where practical;
* verification includes frontmatter and file checks.

## Ready for Rust Implementation

Rust implementation work is ready when:

* target crate/module is known;
* expected behavior is clear;
* tests are identified;
* verification commands are listed;
* new Rust concepts can be explained;
* commit message is defined.

## Ready for File Operation Work

File operation work is ready when:

* writes are planned;
* dry-run behavior is defined if applicable;
* conflict behavior is defined;
* destructive behavior is excluded or explicitly gated;
* verification confirms file behavior.

## Ready for Agent Work

Agent-related work is ready when:

* human approval boundaries are clear;
* no unapproved writes are allowed;
* model provider assumptions are explicit;
* verification distinguishes model output from verified truth.

## Current Status

This Definition of Ready is a draft. It is authoritative enough for initial Monad workflow and GitHub planning.
