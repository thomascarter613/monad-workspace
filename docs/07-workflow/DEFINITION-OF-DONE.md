---
title: "Definition Of Done"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - workflow
  - definition-of-done
related:
  - docs/07-workflow/OPERATING-MODEL.md
  - docs/07-workflow/WORK-PACKET-STANDARD.md
  - docs/07-workflow/DEFINITION-OF-READY.md
  - docs/07-workflow/VERIFICATION-STANDARD.md
  - docs/07-workflow/COMMIT-STANDARD.md
---

# Definition Of Done

## Purpose

This document defines when Monad work is complete.

Done means more than “files were edited.” Done means the work achieved its objective, was verified, was committed atomically, and left enough context for future continuation.

## Core Rule

A work packet is done when:

```text
Scope complete
  + Verification passed
  + Expected result achieved
  + Docs/context updated as needed
  + Atomic commit completed
````

## Universal Done Checklist

A work packet is Done when:

* [ ] Objective achieved.
* [ ] In-scope work completed.
* [ ] Out-of-scope boundaries respected.
* [ ] Deliverables exist.
* [ ] Verification commands were run.
* [ ] Verification passed or exceptions are documented.
* [ ] Expected Result After Verification is true.
* [ ] Documentation updated if needed.
* [ ] Context updated if needed.
* [ ] No known blocker remains.
* [ ] Changes are reviewable.
* [ ] Atomic Conventional Commit completed.
* [ ] Git working tree is clean or remaining changes are intentionally deferred.

## Documentation Done Checklist

Documentation work is Done when:

* [ ] Files are in the correct docs area.
* [ ] Frontmatter exists.
* [ ] Frontmatter status is accurate.
* [ ] Title matches the document topic.
* [ ] Purpose is clear.
* [ ] Scope or expected contents are clear.
* [ ] Related docs are listed where known.
* [ ] Content does not make unsupported final claims.
* [ ] Verification confirms frontmatter.
* [ ] Documentation commit is atomic.

## Rust Implementation Done Checklist

Rust implementation work is Done when:

* [ ] Code compiles.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes with warnings denied.
* [ ] Public module boundaries are intentional.
* [ ] Error handling is explicit.
* [ ] New behavior has tests where practical.
* [ ] Rust concepts are explained in surrounding guidance when needed.
* [ ] CLI remains thin if CLI is involved.
* [ ] Core logic lives in `monad-core` when reusable.
* [ ] Commit is atomic.

Minimum Rust verification:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## CLI Work Done Checklist

CLI work is Done when:

* [ ] Command appears in help where appropriate.
* [ ] Command behavior matches objective.
* [ ] Output is readable.
* [ ] Exit code behavior is defined.
* [ ] Core logic is not buried in CLI handler.
* [ ] Relevant command has been manually run.
* [ ] Tests exist where practical.

## Verification Work Done Checklist

Verification work is Done when:

* [ ] Check model or command behavior is clear.
* [ ] Pass/fail/skipped behavior is represented.
* [ ] Evidence or report output is understandable.
* [ ] Native command failures are not hidden.
* [ ] Exit code behavior is predictable.
* [ ] Tests cover result behavior where practical.

## Context Work Done Checklist

Context work is Done when:

* [ ] Generated or authored context file exists.
* [ ] Source and status are clear.
* [ ] Generated context is not confused with accepted doctrine.
* [ ] Handoff information is sufficient for a future session.
* [ ] Related docs are linked where practical.

## File Operation / Evolution Work Done Checklist

Evolution work is Done when:

* [ ] Planned operations are visible.
* [ ] Dry-run behavior works before apply behavior.
* [ ] Conflicts are represented clearly.
* [ ] Existing files are not overwritten unexpectedly.
* [ ] Destructive behavior is absent or explicitly gated.
* [ ] Verification confirms expected file behavior.

## Agent Work Done Checklist

Agent-related work is Done when:

* [ ] Human-in-command boundaries are preserved.
* [ ] No unapproved writes are performed.
* [ ] No unapproved command execution is performed.
* [ ] Model output is treated as proposed, not verified.
* [ ] Approval/audit expectations are clear.
* [ ] Provider lock-in is avoided.

## Commit Done Rule

A work packet is not Done until the relevant change is committed.

Recommended final check:

```bash
git status --short
```

After commit, the working tree should be clean unless there are intentionally deferred changes.

## Exceptions

If verification cannot pass, the work packet may not be marked Done unless:

* the failure is unrelated and documented;
* a follow-up bug/work packet is created;
* the risk is explicitly accepted;
* the current work still satisfies its scoped objective.

Exceptions should be rare.

## Current Status

This Definition of Done is a draft. It is authoritative enough for initial Monad work and should be refined as implementation practices mature.
