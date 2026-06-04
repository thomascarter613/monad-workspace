---
title: "Public Claims Audit"
document_type: "release-audit"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E10
work_packet: WP-E10-001
tags:
  - monad
  - release
  - public-readiness
  - claims-audit
  - e10
related:
  - README.md
  - docs/project/MVP-COMMAND-REFERENCE.md
  - docs/release/FIRST-PUBLIC-PRERELEASE-BOUNDARY.md
  - docs/release/E9-CLOSEOUT.md
---

# Public Claims Audit

## Status

Accepted.

## Work Packet

WP-E10-001 — Audit README and public claims against implemented capability.

## Purpose

This audit checks Monad's public-facing repository claims against implemented capability before any public pre-release tag is cut.

The purpose is release honesty:

- do not claim implemented behavior that does not exist;
- distinguish current capability from roadmap intent;
- state safety boundaries plainly;
- make public pre-release posture explicit;
- preserve evidence for future release decisions.

## Audit Sources

This audit reviewed:

- `README.md`
- `docs/project/MVP-COMMAND-REFERENCE.md`
- current CLI command surface in `crates/monad-cli/src/main.rs`
- E9 public-readiness closeout posture
- E10 public pre-release hardening objective

## Current Implemented CLI Capability

The current implemented command surface is:

```text
monad --help
monad --version
monad help
monad version
monad info
monad info --format=json
monad check
monad check --format=json
monad inspect
monad inspect --format=json
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

## Current Safety Boundaries

Monad currently has the following public-facing safety boundaries:

- local-first execution only;
- `plan` is no-write;
- `plan` does not run shell commands;
- `plan` does not call a real model provider or external AI API;
- `evolve verify-baseline` is dry-run only;
- `evolve context-baseline` is dry-run only;
- no apply command exists yet;
- no autonomous agent execution exists yet;
- no remote Git operation exists yet;
- no MCP server exists yet;
- no deployment command exists yet;
- no public package has been published;
- no installer distribution exists.

## Claims Audit Table

| Public Claim Area | Previous / Existing Claim | Audit Finding | Required Action | Result |
| --- | --- | --- | --- | --- |
| Product identity | Monad is an AI-native, repo-native, local-first Software Foundry OS. | Directionally correct as product vision, but too strong if read as fully implemented current capability. | Qualify as "being built toward" this product vision. | README updated. |
| Current status | Monad is currently in the project foundation phase. | Outdated. E0-E9 foundation/MVP-candidate/public-readiness work has been completed or substantially closed, and E10 is active. | Replace with public pre-release hardening status. | README updated. |
| Initial focus | Establish repository foundation, docs architecture, context bridge, workflow standards, product canon, then Rust core. | Outdated as a current-status list. | Replace with current E10 focus. | README updated. |
| Planned MVP epics | E0-E6 only. | Outdated because E7-E10 have since been added for MVP hardening, candidate cut, public-readiness closure, and public pre-release hardening. | Replace with E0-E10 foundation/release-readiness sequence. | README updated. |
| Planned MVP commands | Commands were described as targeted and "most commands do not exist yet." | Outdated. Many MVP commands now exist, while future commands remain unimplemented. | Replace with implemented command surface and explicit future-command boundary. | README updated. |
| Public release | Monad is not yet a public release, package, installer, hosted service, or autonomous agent runtime. | Accurate and important. | Preserve this statement. | Preserved. |

## Current Public README Boundary

After this audit, `README.md` should communicate:

1. Monad is being built toward the larger Software Foundry OS vision.
2. The current repository is in public pre-release hardening, not early project foundation.
3. The implemented command surface is explicit.
4. Future roadmap commands are not implied to exist.
5. Safety boundaries are clear.
6. Monad is not yet a public release, package, installer, hosted service, or autonomous agent runtime.

## Future Commands Not Yet Implemented

The following command families belong to later epics and should not be described as currently available:

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

## Definition of Done Evidence

WP-E10-001 is complete when:

- this audit exists;
- README public claims are updated;
- the implemented command surface is not overstated;
- future commands are clearly labeled as future work;
- release and safety boundaries are preserved;
- standard verification passes or blockers are recorded.

## Verification Commands

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

## Outcome

Accepted.

Monad's public-facing README claims have been brought back into alignment with implemented capability and explicit public pre-release boundaries.
