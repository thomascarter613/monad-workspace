---
title: MVP Readiness Report
description: Concrete readiness report for treating Monad as an MVP candidate after E7 MVP hardening.
status: draft
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
phase: MVP Hardening
epic: E7
work_packet: WP-E7-006
---

# MVP Readiness Report

## 1. Executive summary

Monad has completed the foundation and MVP hardening pass required to evaluate whether the repository can be treated as an MVP candidate.

At this stage, Monad should be considered an **internal MVP candidate** if the final verification commands pass.

It should **not yet** be treated as a public release, published package, installer-ready binary, hosted product, or autonomous agent system.

## 2. MVP candidate decision

### Current decision

**Decision:** Internal MVP candidate after verification.

### Conditions

Monad may be treated as an internal MVP candidate only when all of the following pass:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

### MVP candidate means

For this repository, “MVP candidate” means:

* the core command surface exists
* command behavior is documented
* command smoke tests exist
* dry-run and planning safety boundaries are explicit
* repo-native context artifacts exist
* project state can be handed off to a future AI/developer session
* verification can be run locally and reviewed

### MVP candidate does not mean

“MVP candidate” does not mean:

* public release
* crates.io publication
* binary installer distribution
* hosted SaaS launch
* autonomous coding agent execution
* remote Git mutation
* real model-provider orchestration
* marketplace/plugin system
* enterprise-ready platform

## 3. E7 hardening summary

E7 was the MVP hardening epic.

Its purpose was to stabilize the foundation after E0 through E6 and identify the final boundary between “implemented MVP candidate” and “future product work.”

### WP-E7-001 — Foundation closure audit

Outcome:

* Created a foundation closure audit process.
* Recorded verification status.
* Identified root verification as the main hardening focus.
* Established that blockers should be recorded honestly rather than hidden.

Evidence artifact:

```text
docs/project/FOUNDATION-CLOSURE-AUDIT.md
```

### WP-E7-002 — CLI help and command UX

Outcome:

* Normalized CLI help output.
* Documented current command surface in help text.
* Added `plan` to help output.
* Added dry-run evolution commands to help output.
* Improved command-specific dry-run-required errors.
* Preserved actionable missing-argument behavior.

Evidence artifact:

```text
docs/project/WP-E7-002-CLI-UX-EVIDENCE.md
```

### WP-E7-003 — Command smoke tests

Outcome:

* Added integration-style command smoke tests.
* Covered help/version behavior.
* Covered inspect/check behavior.
* Covered plan behavior.
* Covered dry-run evolution behavior.
* Covered expected failure behavior.
* Adjusted tests to run from the repository root where workspace discovery succeeds.

Evidence artifact:

```text
docs/project/WP-E7-003-COMMAND-SMOKE-TEST-EVIDENCE.md
```

### WP-E7-004 — Documentation alignment

Outcome:

* Added current MVP command reference.
* Aligned documentation with implemented behavior.
* Clarified package name, binary name, local command usage, safety boundaries, and current limitations.
* Avoided overstating future capabilities as implemented behavior.

Evidence artifact:

```text
docs/project/MVP-COMMAND-REFERENCE.md
docs/project/WP-E7-004-DOC-ALIGNMENT-EVIDENCE.md
```

### WP-E7-005 — Dry-run and no-write guarantees

Outcome:

* Added or strengthened no-write assertions for `plan`.
* Added or strengthened dry-run assertions for evolution commands.
* Added a script to verify no-write commands do not mutate Git status.
* Integrated no-write verification into root verification where appropriate.

Evidence artifact:

```text
docs/project/WP-E7-005-DRY-RUN-NO-WRITE-EVIDENCE.md
tools/scripts/verify-no-write-commands.sh
```

### WP-E7-006 — MVP readiness report

Outcome:

* Defines the MVP candidate cut line.
* Summarizes hardening results.
* Identifies MVP-ready capabilities.
* Identifies non-MVP future capabilities.
* Defines final verification evidence.
* Recommends the next milestone.

Evidence artifact:

```text
docs/project/MVP-READINESS-REPORT.md
```

## 4. MVP-ready capabilities

The following capabilities are in the MVP candidate boundary, assuming final verification passes.

### 4.1 Workspace discovery and repository inspection

Monad can discover and inspect a repository workspace from the CLI.

Representative commands:

```bash
cargo run -p monad-cli -- info
cargo run -p monad-cli -- inspect
```

### 4.2 Workspace checks

Monad can run workspace checks and produce a check report.

Representative command:

```bash
cargo run -p monad-cli -- check
```

Important behavior:

* text mode writes latest check evidence
* JSON mode prints JSON output
* check evidence should be treated as a generated review artifact

### 4.3 Repository graph rendering

Monad can render repository graph information in multiple formats.

Representative commands:

```bash
cargo run -p monad-cli -- graph
cargo run -p monad-cli -- graph --format=json
cargo run -p monad-cli -- graph --format=mermaid
cargo run -p monad-cli -- graph --format=dot
```

### 4.4 Repo-native context rendering and writing

Monad can render and write repo-native context artifacts.

Representative commands:

```bash
cargo run -p monad-cli -- context
cargo run -p monad-cli -- context --write
cargo run -p monad-cli -- context generate current-state
cargo run -p monad-cli -- context generate handoff
cargo run -p monad-cli -- context generate bootstrap
cargo run -p monad-cli -- context pack
cargo run -p monad-cli -- context verify
```

### 4.5 Supervised planning

Monad can produce a supervised no-write plan from a user intent.

Representative command:

```bash
cargo run -p monad-cli -- plan "explain this repository"
```

Important current boundaries:

* no files are written
* no shell commands are run
* no Git state is changed
* no real model provider or external AI API is called

### 4.6 Dry-run evolution previews

Monad can preview baseline evolution operations without applying them.

Representative commands:

```bash
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run
```

Important current boundaries:

* dry-run only
* no apply behavior
* no file writes
* no branch creation
* no remote Git mutation

### 4.7 Verification baseline

Monad has a local verification culture centered on:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

## 5. Non-MVP future capabilities

The following are intentionally **outside** the current MVP candidate boundary.

### 5.1 Apply/write evolution

Not MVP-ready:

* applying dry-run file operation plans
* writing generated evolution files
* conflict-resolution workflows
* branch/worktree creation
* patch application
* rollback workflows

### 5.2 Autonomous agents

Not MVP-ready:

* autonomous code modification
* unsupervised tool execution
* multi-agent orchestration
* long-running agent sessions
* agent lifecycle management

### 5.3 Real provider execution

Not MVP-ready:

* hosted LLM provider calls
* local model provider execution
* provider credential management
* provider routing
* provider billing/account handling

### 5.4 MCP runtime/server behavior

Not MVP-ready:

* MCP server execution
* MCP client integration
* remote tool invocation
* MCP permission prompts
* MCP tool result auditing

### 5.5 Distribution and release

Not MVP-ready:

* crates.io publishing
* binary installer creation
* package signing
* release automation
* semantic version release pipeline
* public release notes

### 5.6 Enterprise/platform features

Not MVP-ready:

* RBAC
* SSO
* hosted control plane
* multi-user audit dashboards
* SaaS onboarding
* marketplace/plugin billing

## 6. Release blockers

The following must be resolved before Monad can move from internal MVP candidate to public release candidate.

### Blocker 1 — Final verification must pass

Required:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

Status:

* Pending final WP-E7-006 verification.

### Blocker 2 — Working tree must be clean after verification

Required:

```bash
git status --short
```

Expected:

* no unrelated changes
* generated artifacts either intentionally committed or ignored
* no accidental report churn

Status:

* Pending final WP-E7-006 verification.

### Blocker 3 — MVP candidate scope must remain explicit

Required:

* internal MVP candidate only
* no public release claims
* no autonomous-agent claims
* no apply/write claims

Status:

* Documented in this report.

## Scope freeze

The internal MVP candidate scope is formally frozen in:

```text
docs/project/MVP-SCOPE-FREEZE.md
```

That document is controlling for included capabilities, deferred capabilities, and prohibited release claims during E8.

## 7. Final MVP cut line

The final MVP cut line is:

> Monad is an internal, local-first, Rust-based monorepo runtime and governance-grade developer-experience CLI that can inspect a repository, run checks, render graphs, generate repo-native context artifacts, produce supervised no-write plans, and preview safe baseline evolution operations in dry-run mode.

Anything beyond that statement is outside the current MVP candidate.

## 8. Required final verification evidence

Before closing E7, run:

```bash
find docs/project .monad/context -maxdepth 4 -type f | sort
git status --short

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh

git status --short
```

Record the result in the commit message, PR/issue comment, or final handoff.

## 9. Recommended next milestone

The recommended next milestone is:

```text
E8 — MVP Candidate Cut and Release Preparation
```

### Recommended E8 purpose

E8 should prepare Monad for an actual MVP candidate cut without expanding core scope.

### Recommended E8 work packets

1. **WP-E8-001 — Freeze MVP candidate scope**

   * convert this report into a formal release cut line
   * mark deferred capabilities explicitly

2. **WP-E8-002 — Add changelog and release notes foundation**

   * create `CHANGELOG.md`
   * create release notes template
   * document current unreleased changes

3. **WP-E8-003 — Add version and build metadata hardening**

   * confirm package version
   * confirm binary version output
   * decide pre-release version tag policy

4. **WP-E8-004 — Add installation/build documentation**

   * document local build
   * document local run
   * document supported development environment

5. **WP-E8-005 — Run release-candidate verification audit**

   * run clean-clone style verification
   * record exact evidence
   * document blockers

6. **WP-E8-006 — Cut internal MVP candidate tag**

   * tag only after verification passes
   * no public distribution unless separately approved

## 10. E7 closeout status

E7 may be closed when:

* WP-E7-006 is committed
* final verification has been run
* remaining blockers are documented
* context artifacts are updated if needed
* all E7 issues are closed or explicitly carried forward

