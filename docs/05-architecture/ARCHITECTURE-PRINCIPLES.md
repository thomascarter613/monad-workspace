---
title: "Architecture Principles"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - architecture
  - principles
related:
  - docs/05-architecture/SYSTEM-OVERVIEW.md
  - docs/05-architecture/MODULE-BOUNDARIES.md
  - docs/01-project/01-charter/PRODUCT-CHARTER.md
  - docs/02-product/MVP-SCOPE.md
  - docs/06-adrs/ADR-0001-use-rust-for-core-runtime.md
  - docs/06-adrs/ADR-0008-coordinate-native-tools-rather-than-replace-them.md
---

# Architecture Principles

## Purpose

This document defines the architecture principles that guide Monad.

These principles are intended to prevent architectural drift while still allowing implementation experience to refine the design.

## Principle 1: The Repository Is the Source of Truth

Monad should treat the repository as the canonical home for durable project knowledge.

### Implications

- Product docs belong in `docs/`.
- Workflow standards belong in `docs/`.
- Context handoff artifacts belong in `docs/`, `work/`, or `.monad/`.
- Architecture decisions belong in ADRs.
- Generated reports should be reviewable.
- Important knowledge should not live only in chat history.

### Avoid

- Hidden state that cannot be reviewed.
- Cloud-only source of truth.
- Undocumented conventions.
- Binding decisions that exist only in external tools.

## Principle 2: Local-First Before Cloud

Monad’s core value must work locally.

### Implications

- The CLI should be useful without a hosted account.
- Repo inspection should run locally.
- Context generation should run locally.
- Verification should coordinate local native tools.
- Safe evolution should work on local files.
- Cloud services may extend Monad later but must not be required for core value.

### Avoid

- Requiring login for local functionality.
- Designing the core around a hosted control plane.
- Blocking MVP on cloud infrastructure.

## Principle 3: Human in Command

Monad may assist, plan, draft, verify, and recommend, but the human remains responsible for consequential actions.

### Implications

- Risky file writes require review.
- Destructive operations require explicit approval.
- Agent workflows must be supervised.
- Plans should be visible.
- Verification evidence should be visible.
- The user should understand what Monad is doing.

### Avoid

- Silent background changes.
- Unapproved commits.
- Unapproved pushes.
- Unrestricted command execution.
- Treating AI output as inherently correct.

## Principle 4: Verification Over Vibes

Monad should earn trust through evidence.

### Implications

- Checks should produce structured results.
- Reports should explain what was run.
- Evidence packets should record outcomes.
- Exit codes should be meaningful.
- Generated changes should be verified.
- Failures should be visible and actionable.

### Avoid

- “Looks good” without checks.
- Hiding command failures.
- Overstating confidence.
- Producing plans that cannot be verified.

## Principle 5: Coordinate Native Tools Rather Than Replace Them

Monad should coordinate the tools each ecosystem already uses.

### Implications

- Rust projects should still use Cargo.
- JavaScript projects should still use Bun, npm, pnpm, or yarn.
- Go projects should still use Go tooling.
- Python projects should still use uv, pip, Poetry, or related tooling.
- Monad should inspect, orchestrate, verify, and report around those tools.

### Avoid

- Reimplementing package managers.
- Replacing language-native build systems without strong reason.
- Creating a worse duplicate of established tools.

## Principle 6: Durable Core, Thin Surfaces

Long-lived product logic belongs in `monad-core`.

User interfaces should remain thin.

### Implications

- CLI command handlers should delegate to core logic.
- Future desktop/web/MCP surfaces should reuse the same core concepts.
- Domain models should not depend on terminal rendering.
- Core modules should be testable without invoking the CLI.

### Avoid

- Business logic hidden in CLI parsing.
- Terminal-only representations inside domain models.
- Duplicating logic across future interfaces.

## Principle 7: Safe File Operations First

File writes are trust-critical.

Monad should model planned file operations before applying them.

### Implications

- Dry-run mode should come before apply mode.
- Conflicts should be explicit.
- Existing files should not be overwritten casually.
- Destructive operations should be rare and gated.
- Generated files should be reviewable.
- Diffs or previews should be available where practical.

### Avoid

- Silent overwrites.
- Hidden deletes.
- Applying generated changes without preview.
- Treating templates as harmless by default.

## Principle 8: Context Is Infrastructure

Project context is not an optional convenience.

It is core infrastructure for human and AI continuity.

### Implications

- Current-state files matter.
- Handoff files matter.
- Bootstrap prompts matter.
- Context packs matter.
- Decision logs matter.
- Generated context needs trust boundaries.

### Avoid

- Relying on chat history as the only project memory.
- Generating context without indicating source or status.
- Mixing draft/generated context with accepted doctrine.

## Principle 9: AI-Native but Provider-Agnostic

Monad should be designed for AI-assisted workflows without requiring a specific provider.

### Implications

- Model-provider abstractions should be explicit.
- Local and hosted providers should be possible.
- MVP should not depend on paid AI access.
- AI output should be treated as proposed, not verified.
- MCP compatibility should be considered, but not allowed to dominate early design.

### Avoid

- Hard-coding one provider into core logic.
- Making AI mandatory for basic repository inspection.
- Confusing provider output with project truth.

## Principle 10: Make State Explicit

Monad should prefer explicit state over implicit assumptions.

### Implications

- Work status should be visible.
- Context state should be visible.
- Check results should be visible.
- Generated artifacts should be labeled.
- Decisions should be recorded.
- Configuration should be inspectable.

### Avoid

- Hidden magic.
- Unclear defaults.
- Undocumented generated state.
- Silent behavior changes.

## Principle 11: Additive and Non-Destructive by Default

Monad should prefer additive changes during early evolution workflows.

### Implications

- Create missing baseline files.
- Skip or report existing files.
- Avoid destructive edits unless explicitly requested.
- Prefer dry-run summaries.
- Preserve user-authored files.

### Avoid

- Deleting user files.
- Reformatting entire repositories unexpectedly.
- Replacing existing configs without review.

## Principle 12: Small Slices, High Confidence

Monad should be built in small, verifiable slices.

### Implications

- Work packets should be bounded.
- Each slice should have verification commands.
- Each implementation slice should be atomically committed.
- Tests should accompany core behavior.
- Documentation should be updated when behavior changes.

### Avoid

- Large unreviewable changes.
- Multi-feature commits.
- Skipping tests because the feature is early.
- Building complex frameworks before concrete use cases.

## Principle 13: Beginner-Readable Rust

Monad’s implementation should be production-minded but teachable.

### Implications

- Prefer clear names.
- Add comments for non-obvious Rust concepts.
- Avoid clever abstractions too early.
- Keep modules focused.
- Write tests that explain expected behavior.
- Explain ownership, borrowing, traits, and errors as they appear.

### Avoid

- Overly abstract generic code without need.
- Macro-heavy design before the domain is stable.
- Unexplained lifetime complexity.
- Hiding core concepts behind cleverness.

## Principle 14: Architecture as a Living Contract

Architecture docs and ADRs should guide implementation, and implementation should refine architecture when evidence demands it.

### Implications

- Accepted decisions should be followed.
- New major decisions should produce ADRs.
- Architecture docs should be updated after meaningful implementation discoveries.
- Drift should be treated as a project risk.

### Avoid

- Treating architecture docs as decorative.
- Making major design changes without documentation.
- Following stale architecture when implementation proves it wrong.

## Principle 15: MVP Discipline

Monad’s vision is large, but the MVP must stay focused.

### Implications

- Prove local CLI value first.
- Prove repo inspection first.
- Prove context generation first.
- Prove verification first.
- Prove safe dry-run evolution first.
- Defer cloud, billing, marketplace, and enterprise features.

### Avoid

- Building the long-term control plane before local value works.
- Implementing full autonomous agents too early.
- Premature plugin ecosystem design.
- Replacing too many native tools.

## Current Status

These architecture principles are a draft. They should guide early implementation and be promoted or refined through ADRs as core decisions become accepted.
