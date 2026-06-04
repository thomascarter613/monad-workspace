---
title: "Public Pre-Release Notes"
document_type: "release-notes"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E10
work_packet: WP-E10-004
tags:
  - monad
  - release
  - prerelease
  - source-only
  - notes
related:
  - README.md
  - docs/release/PUBLIC-CLAIMS-AUDIT.md
  - docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md
  - docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md
  - docs/project/MVP-COMMAND-REFERENCE.md
---

# Public Pre-Release Notes

## Status

Drafted and accepted as the current public pre-release notes artifact.

These notes do not cut, approve, or publish a release by themselves.

The final public pre-release decision remains governed by:

```text
WP-E10-005 — Run final public pre-release verification audit
WP-E10-006 — Decide and cut first public pre-release tag, if approved
```

## Release Posture

Monad's first public pre-release posture is:

```text
Source-only public pre-release.
```

This means the first public pre-release, if approved, is intended for repository inspection and local Cargo-based evaluation.

It is not:

- a general availability release;
- a production release;
- a packaged binary release;
- an installer release;
- a Crates.io/package-manager publication;
- a hosted service;
- a SaaS launch;
- an autonomous agent runtime.

## What Monad Is

Monad is a Rust-first, repo-native, local-first developer tool being built toward an AI-native Software Foundry OS for understanding, verifying, and safely evolving software repositories.

The current implementation focuses on:

- repository inspection;
- repository graph rendering;
- workspace checks;
- context artifact generation;
- context pack generation/export;
- context verification;
- dry-run baseline evolution planning;
- supervised no-write planning.

## Implemented Command Surface

The current implemented local command surface is:

```text
monad --help
monad --version
monad help
monad version
monad info
monad info --format=json
monad inspect
monad inspect --format=json
monad check
monad check --format=json
monad graph
monad graph --format=json
monad graph --format=mermaid
monad graph --format=dot
monad context
monad context --format=json
monad context --write
monad context generate current-state
monad context generate handoff
monad context generate bootstrap
monad context pack
monad context verify
monad evolve verify-baseline --dry-run
monad evolve context-baseline --dry-run
monad plan "<intent>"
```

## Local Evaluation

Clone the repository and run commands through Cargo.

```bash
cargo run -p monad-cli -- --help
cargo run -p monad-cli -- version
cargo run -p monad-cli -- info
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- check
cargo run -p monad-cli -- graph
cargo run -p monad-cli -- context
cargo run -p monad-cli -- context verify
cargo run -p monad-cli -- plan "explain this repository"
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run
```

## Verification Commands

Before approving any public pre-release tag, run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- --help
cargo run -p monad-cli -- version
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- check
cargo run -p monad-cli -- plan "explain this repository"
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run
tools/scripts/verify.sh
git status --short
```

Final verification evidence belongs to WP-E10-005.

## Safety Boundaries

Current safety boundaries:

- `plan` is no-write.
- `plan` does not run shell commands.
- `plan` does not call a real model provider or external AI API.
- `evolve verify-baseline` is dry-run only.
- `evolve context-baseline` is dry-run only.
- Write behavior is limited to explicit context export/generation commands.
- No apply command exists yet.
- No autonomous agent execution exists yet.
- No remote Git operation exists yet.
- No MCP server exists yet.
- No deployment command exists yet.

## Not Yet Implemented

The following command families are roadmap work and are not part of the current implemented command surface:

```text
monad init
monad add
monad run
monad sync
monad doctor
monad release
monad upgrade
monad patch
monad apply
```

## Deferred Distribution Channels

The following are explicitly deferred beyond this first public pre-release boundary:

- binary artifacts;
- installer scripts/packages;
- Crates.io publication;
- Homebrew/Apt/Nix/Winget distribution;
- Docker images;
- hosted/SaaS control plane;
- marketplace features;
- autonomous agent runtime claims.

## Known Limitations

Known limitations for the source-only public pre-release:

- The CLI is still early.
- The release is source-only.
- The user must build/run locally with Cargo.
- Evolution behavior is dry-run only.
- There is no apply/write evolution command yet.
- The planning command is deterministic/no-write and does not call an AI provider.
- There is no hosted control plane.
- There is no installer or package-manager distribution.
- Future roadmap commands are documented as future work only.

## Current Public Pre-Release Readiness

Current readiness status:

```text
Not ready for tag yet.
```

Reason:

```text
Public claims have been audited, distribution posture has been decided, and release notes are drafted.
The final verification audit and tag/go-no-go decision remain pending.
```

## Required Remaining E10 Work

```text
WP-E10-005 — Run final public pre-release verification audit
WP-E10-006 — Decide and cut first public pre-release tag, if approved
```

## Changelog Summary

Current pre-release notes summarize the internal MVP candidate/public-readiness path:

- E0 — Project Foundation
- E1 — Rust Core Foundation
- E2 — Repo Intelligence
- E3 — Context Bridge
- E4 — Verification Engine
- E5 — Evolution Engine
- E6 — Agent Supervision
- E7 — MVP Hardening
- E8 — MVP Candidate Cut and Release Preparation
- E9 — Post-MVP Candidate Stabilization and Public-Readiness Gap Closure
- E10 — Public Pre-Release Hardening and Boundary Enforcement

## Release Decision

No public pre-release tag is approved by this document.

The tag decision is deferred to WP-E10-006 after final verification in WP-E10-005.

## Outcome

Accepted as drafted public pre-release notes for E10.

