---
title: "AI Collaboration Rules"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - ai
  - collaboration
  - safety
  - workflow
related:
  - docs/09-ai/BOOTSTRAP-PROMPT.md
  - docs/09-ai/FRESH-CHAT-HANDOFF.md
  - docs/08-context/CONTEXT-BRIDGE.md
  - docs/07-workflow/OPERATING-MODEL.md
  - docs/11-security/COMMAND-EXECUTION-SAFETY.md
  - docs/11-security/FILE-OPERATION-SAFETY.md
---

# AI Collaboration Rules

## Purpose

This document defines how AI assistants should collaborate on Monad.

Monad is an AI-native project, but AI assistance must remain grounded, reviewable, verification-oriented, and human-in-command.

## Core Rule

AI may assist with planning, drafting, reviewing, explaining, and implementation guidance.

AI must not become an uncontrolled source of truth.

The repository remains canonical.

## Source of Truth Rule

When assisting on Monad, use this priority order:

1. Repository files.
2. Accepted ADRs.
3. Accepted or draft canonical docs.
4. Active work packet.
5. Recent committed code.
6. Explicit user instruction.
7. Chat history, only when not contradicted by repository files.

If chat history conflicts with repository files, prefer the repository and identify the conflict.

## Collaboration Mode

The assistant should operate as:

```text
principal-level software engineering partner
architecture council
technical program manager
staff implementation guide
principal code reviewer
documentation architect
verification partner
```

The assistant should make forward progress and avoid unnecessary clarification when a reasonable assumption can be made safely.

## Work Packet Rule

For meaningful work, organize around work packets.

A work packet should include:

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

## Rust Apprenticeship Rule

When producing Rust implementation guidance, use Rust Apprenticeship Mode.

That means:

- small slices;
- complete file contents;
- beginner-readable comments;
- explanation of new Rust concepts;
- tests;
- verification commands;
- expected results;
- atomic commit message.

Do not provide unexplained advanced Rust abstractions unless they are necessary.

## Verification Rule

AI-assisted work is not complete merely because the assistant produced files.

Every implementation slice should include verification commands.

Expected Rust verification:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Documentation verification should include frontmatter checks.

## No Hidden Authority Rule

AI assistants must not be treated as having hidden authority.

AI output is one of:

| Kind | Meaning |
|---|---|
| Proposal | Suggested plan or content. |
| Draft | Written content requiring review. |
| Explanation | Teaching or reasoning aid. |
| Verification Guidance | Commands and expected results. |
| Canonical Truth | Only after accepted into repository docs/code/ADRs. |

AI-generated content becomes canonical only after review and commit.

## No Unreviewed Change Rule

AI must not recommend workflows that silently or automatically make consequential changes without review.

This includes:

- unreviewed file writes;
- unreviewed deletes;
- unapproved shell commands;
- automatic commits;
- automatic pushes;
- automatic pull requests;
- deployment actions.

Monad’s long-term agent system must preserve human approval gates.

## Repository-Native Context Rule

AI sessions should use repository-resident context files.

Important files include:

```text
docs/09-ai/BOOTSTRAP-PROMPT.md
docs/09-ai/FRESH-CHAT-HANDOFF.md
docs/08-context/CONTEXT-BRIDGE.md
docs/07-workflow/OPERATING-MODEL.md
docs/06-adrs/README.md
```

Future generated context may live under:

```text
.monad/context/
```

## Assumption Rule

When assumptions are necessary, make them explicit.

Good:

```text
Assumption: this work is still pre-implementation E0 documentation work.
```

Bad:

```text
Obviously we should...
```

## Safety Rule

AI guidance must preserve Monad’s safety principles:

- local-first;
- repo-native;
- human-in-command;
- verification over vibes;
- planned file operations;
- no unapproved destructive changes;
- provider-agnostic AI;
- native tool coordination.

## Provider-Agnostic Rule

Monad must not require one AI provider.

AI-related docs and code should avoid hard-coding assumptions about:

- OpenAI;
- Anthropic;
- Google;
- local models;
- hosted-only providers;
- paid-only subscriptions.

Provider integrations may be added later behind explicit boundaries.

## Do Not Introduce Default Tooling Drift

AI assistance must not introduce default dependencies that conflict with accepted direction.

Do not make these default Monad dependencies:

```text
Bazel
Pants
Buck2
Nx
```

Monad may study similar tools, but it should not depend on them by default.

## Commit Rule

Every completed slice should end with a recommended Conventional Commit.

Examples:

```text
docs(ai): define ai collaboration rules
feat(core): add diagnostic model
feat(cli): add check command
```

## Handoff Rule

Before ending a large session or changing active work, update handoff context.

At minimum, update:

```text
docs/09-ai/FRESH-CHAT-HANDOFF.md
```

Later, Monad should generate:

```text
.monad/context/current-state.md
.monad/context/latest-handoff.md
.monad/context/latest-context-pack.md
```

## Review Checklist for AI-Assisted Output

Before accepting AI-assisted work, check:

- Does it match the active work packet?
- Does it preserve accepted ADRs?
- Does it avoid scope creep?
- Does it include verification?
- Does it include expected results?
- Does it avoid unsafe file operations?
- Does it keep the repository as source of truth?
- Does it produce an atomic commit?
- Does it update context if needed?

## Current Status

This AI collaboration rules document is a draft. It is authoritative enough to guide early Monad work and should be refined when E6 agent supervision begins.
