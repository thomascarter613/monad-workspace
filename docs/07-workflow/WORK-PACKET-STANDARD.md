---
title: "Work Packet Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - workflow
  - work-packets
  - standard
related:
  - docs/07-workflow/OPERATING-MODEL.md
  - docs/07-workflow/WORK-HIERARCHY.md
  - docs/07-workflow/DEFINITION-OF-READY.md
  - docs/07-workflow/DEFINITION-OF-DONE.md
  - docs/07-workflow/VERIFICATION-STANDARD.md
---

# Work Packet Standard

## Purpose

This document defines the standard structure for Monad work packets.

A work packet is Monad’s primary delivery unit. It converts a larger epic into a bounded, verifiable, reviewable slice of work.

## Core Rule

A work packet must be clear enough to start and specific enough to verify.

A work packet should not be vague intent. It should define what will change, how completion will be proven, and what commit should result.

## Required Work Packet Fields

Each work packet should include these sections, in this order:

```text
Work Packet ID
Parent Epic ID
Work Packet Title
Product Area
Objective
User Value
Scope
Expected Files or Directories Affected
Tasks
Deliverables
Verification Commands / Evidence
Expected Result After Verification
Definition of Done
Recommended Conventional Commit
Risks / Blockers / Open Questions
Priority
Size
````

## Field: Work Packet ID

The ID follows:

```text
WP-E<epic-number>-NNN
```

Example:

```text
WP-E1-003
```

## Field: Parent Epic ID

The parent epic ID.

Example:

```text
E1
```

## Field: Work Packet Title

The title should be action-oriented and specific.

Good:

```text
Add core error and diagnostic model
```

Weak:

```text
Diagnostics stuff
```

## Field: Product Area

The product area must appear before Objective.

Examples:

```text
Core Runtime
CLI
Repo Intelligence
Context Bridge
Verification
Evolution Engine
Agent Supervision
Documentation
Workflow
```

## Field: Objective

The objective states what this work packet accomplishes.

It should be concise but specific.

Example:

```text
Add Monad’s foundational diagnostic model in monad-core so future commands can return structured warnings, errors, and informational messages.
```

## Field: User Value

User value explains why the work matters.

For internal foundation work, user value may describe future maintainability, safety, verification, or developer experience.

## Field: Scope

Scope should include:

```text
In scope
Out of scope
```

This protects the work packet from expanding during execution.

## Field: Expected Files or Directories Affected

List likely files and directories.

This list can evolve during implementation, but it should provide a starting boundary.

## Field: Tasks

Tasks are checkboxes.

Example:

```markdown
- [ ] Create diagnostics module.
- [ ] Define severity enum.
- [ ] Define diagnostic struct.
- [ ] Add tests.
- [ ] Export module from monad-core.
```

## Field: Deliverables

Deliverables are concrete outputs.

Examples:

* Rust module;
* Markdown doc;
* CLI command;
* test file;
* generated report;
* verification script.

## Field: Verification Commands / Evidence

Every work packet must define how completion will be verified.

Examples:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

For documentation:

```bash
find docs -type f | sort
python3 scripts/check-doc-frontmatter.py
```

## Field: Expected Result After Verification

This field is required.

It explains what should be true after verification passes.

Example:

```text
All Rust formatting, tests, and Clippy checks pass. The diagnostics module exists, is exported from monad-core, and tests prove that info, warning, and error diagnostics can be constructed.
```

## Field: Definition of Done

The work packet Definition of Done is a checklist.

It should include:

* scope complete;
* verification passed;
* expected result achieved;
* docs updated if needed;
* context updated if needed;
* atomic commit completed.

## Field: Recommended Conventional Commit

Every work packet should include a recommended commit.

Example:

```bash
git commit -m "feat(core): add diagnostic model"
```

## Field: Risks / Blockers / Open Questions

This section records known uncertainty.

Examples:

* boundary may change after implementation;
* needs ADR before acceptance;
* depends on prior work packet;
* may require fixture design;
* may need follow-up work.

## Field: Priority

Priority must be near the end.

Allowed values:

```text
P0 Critical
P1 High
P2 Normal
P3 Low
```

## Field: Size

Size must be at the end.

Allowed values:

```text
XS
S
M
L
XL
Unknown
```

## Work Packet Template

````markdown
## Work Packet ID

WP-E0-000

## Parent Epic ID

E0

## Work Packet Title

Title

## Product Area

Documentation

## Objective

State the objective.

## User Value

Explain why this matters.

## Scope

### In scope

- ...

### Out of scope

- ...

## Expected Files or Directories Affected

- ...

## Tasks

- [ ] ...

## Deliverables

- ...

## Verification Commands / Evidence

```bash
command
````

## Expected Result After Verification

State the expected result.

## Definition of Done

* [ ] Scope complete.
* [ ] Verification passes.
* [ ] Expected result achieved.
* [ ] Docs updated if needed.
* [ ] Context updated if needed.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git commit -m "type(scope): summary"
```

## Risks / Blockers / Open Questions

* ...

## Priority

P1 High

## Size

M

```

## Sizing Guidance

| Size | Meaning |
|---|---|
| XS | Tiny, obvious change. |
| S | Small focused work packet. |
| M | Normal work packet. |
| L | Large or caution-worthy. |
| XL | Should probably be split. |
| Unknown | Needs discovery. |

## Splitting Rule

Split a work packet when it:

- touches too many unrelated areas;
- cannot be verified clearly;
- has more than one primary objective;
- would produce an unreviewable commit;
- mixes docs, architecture, implementation, and refactor work without a clear reason;
- is likely to take multiple sessions without a handoff point.

## Current Status

This work packet standard is a draft. It is authoritative for initial Monad work planning and GitHub issue content.
