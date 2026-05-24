---
title: "Context Pack Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - context
  - context-pack
  - standard
  - ai
related:
  - docs/08-context/CONTEXT-BRIDGE.md
  - docs/08-context/HANDOFF-STANDARD.md
  - docs/09-ai/BOOTSTRAP-PROMPT.md
  - docs/09-ai/FRESH-CHAT-HANDOFF.md
---

# Context Pack Standard

## Purpose

This document defines the standard for Monad context packs.

A context pack is a compact, structured bundle of project context intended to help a human or AI assistant understand the repository and continue work.

## Core Rule

A context pack should be:

```text
small enough to read
large enough to orient
grounded in repository files
clear about trust level
explicit about next action
```

## Context Pack Purpose

A context pack helps answer:

- What is this project?
- What is the current state?
- What decisions matter?
- What work is active?
- What files should be read first?
- What verification exists?
- What is the next recommended action?

## Recommended Location

Generated context packs should live under:

```text
.monad/context/
```

Recommended generated file:

```text
.monad/context/latest-context-pack.md
```

Human-authored standards live under:

```text
docs/08-context/
```

## Required Context Pack Sections

A context pack should include:

```text
Metadata
Project Identity
Current Status
Active Work
Accepted Decisions
Important Documents
Architecture Summary
Workflow Summary
Verification Summary
Risks and Blockers
Next Recommended Action
Source Files Used
Trust Notes
```

## Section: Metadata

Include:

- generated or authored status;
- date;
- tool or process;
- reviewed status;
- source.

Example:

```yaml
generated: true
generated_at: 2026-05-23
reviewed: false
source: repository
```

## Section: Project Identity

State:

- project name;
- product definition;
- current mission.

Example:

```text
Monad is an AI-native, repo-native, local-first Software Foundry OS for understanding, verifying, and safely evolving software repositories.
```

## Section: Current Status

Summarize where the project stands.

Keep this short and factual.

## Section: Active Work

Include:

- active epic;
- active work packet;
- current slice;
- current goal;
- expected next commit.

## Section: Accepted Decisions

Include accepted ADRs and durable decisions.

Examples:

- ADR-0001: Use Rust for Core Runtime.
- ADR-0002: Use Monad as Unified Product Name.
- Repository is the source of truth.
- Work packets are primary delivery units.

## Section: Important Documents

List the most important files to read.

Example:

```text
docs/01-project/01-charter/PRODUCT-CHARTER.md
docs/05-architecture/SYSTEM-OVERVIEW.md
docs/07-workflow/OPERATING-MODEL.md
docs/08-context/CONTEXT-BRIDGE.md
```

## Section: Architecture Summary

Summarize major architecture boundaries.

Example:

```text
Monad begins with monad-cli and monad-core. The CLI is thin. Durable product logic belongs in monad-core. The repository is the canonical source of truth.
```

## Section: Workflow Summary

Summarize the work model.

Example:

```text
Monad uses epics and work packets. Work packets include product area, objective, scope, verification commands, expected result after verification, priority, and size.
```

## Section: Verification Summary

Include recent verification results.

If none exist, say:

```text
No verification evidence recorded in this context pack.
```

## Section: Risks and Blockers

List current blockers or risks.

If none are known, say:

```text
No known blockers.
```

## Section: Next Recommended Action

State the next action clearly.

A context pack should not end with ambiguity.

## Section: Source Files Used

List files used to create the context pack.

This makes generated context auditable.

Example:

```text
docs/01-project/01-charter/PRODUCT-CHARTER.md
docs/05-architecture/SYSTEM-OVERVIEW.md
docs/07-workflow/WORK-PACKET-STANDARD.md
```

## Section: Trust Notes

State whether the context pack is:

- human-authored;
- generated;
- reviewed;
- unreviewed;
- partial;
- stale.

## Context Pack Size Guidance

A context pack should normally be concise.

Preferred size:

```text
1,000 to 3,000 words
```

For very large projects, a context pack may link to deeper docs rather than copy everything.

## What to Exclude

A context pack should exclude:

- secrets;
- credentials;
- raw logs unless necessary;
- unnecessary long file listings;
- speculative claims presented as facts;
- outdated information;
- huge pasted docs that should be linked instead.

## Generated Context Pack Requirements

When Monad generates a context pack, it should:

- identify itself as generated;
- list source files;
- distinguish accepted decisions from draft docs;
- identify assumptions;
- identify missing information;
- avoid secrets;
- be deterministic where practical;
- be reviewable in Git.

## Context Pack Template

```markdown
---
title: "Latest Context Pack"
status: generated
generated_at: YYYY-MM-DD
reviewed: false
source: repository
---

# Latest Context Pack

## Project Identity

## Current Status

## Active Work

## Accepted Decisions

## Important Documents

## Architecture Summary

## Workflow Summary

## Verification Summary

## Risks and Blockers

## Next Recommended Action

## Source Files Used

## Trust Notes
```

## Current Status

This context pack standard is a draft. It is authoritative enough for E0 manual context work and should guide E3 implementation.
