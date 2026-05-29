---
title: Public Pre-Release Checklist
description: Checklist defining the gates required before Monad may move from internal MVP candidate to public pre-release.
status: draft
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
phase: Post-MVP Candidate Stabilization
epic: E9
work_packet: WP-E9-004
---

# Public Pre-Release Checklist

## 1. Purpose

This checklist defines what must be true before Monad may move from an internal MVP candidate to a first public pre-release.

This document does not authorize a public release. It creates the release gate.

## 2. Current decision

```text
NO-GO for public pre-release.
```

Monad remains internal until every P0 and P1 gate below is passed or explicitly deferred through a documented release decision.

## 3. Release posture gates

| Gate                               | Requirement                                                                                                                                                            | Status  |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------- |
| Public scope is explicit           | Public-facing scope must clearly state what Monad does and does not do.                                                                                                | Pending |
| Internal-vs-public status is clear | Docs must distinguish internal MVP candidate from public pre-release.                                                                                                  | Pending |
| No misleading capability claims    | Docs must not imply autonomous agent execution, apply/write evolution, installer distribution, package publication, or hosted service availability unless implemented. | Pending |
| Release boundary is approved       | First public pre-release boundary must be documented.                                                                                                                  | Pending |

## 4. Security gates

| Gate                            | Requirement                                                                                       | Status                        |
| ------------------------------- | ------------------------------------------------------------------------------------------------- | ----------------------------- |
| Dependabot high/critical alerts | No unresolved high or critical dependency alerts on default branch unless formally risk-accepted. | Passed as of WP-E9-001 triage |
| Security policy                 | `SECURITY.md` must exist or public vulnerability reporting policy must be documented.             | Pending                       |
| Generated artifact boundaries   | Imported/generated artifacts must not create false dependency surfaces.                           | In progress                   |
| Secrets posture                 | Repository must not contain committed secrets or credential material.                             | Pending                       |
| Risk acceptance                 | Any known material risk must be documented before public pre-release.                             | Pending                       |

## 5. Repository hygiene gates

| Gate               | Requirement                                                                      | Status  |
| ------------------ | -------------------------------------------------------------------------------- | ------- |
| License            | Repository license must be present and intentional.                              | Pending |
| Contribution guide | Contribution expectations must be documented.                                    | Pending |
| Issue/PR hygiene   | Issue templates and PR expectations should be present or intentionally deferred. | Pending |
| Code ownership     | Ownership expectations should be documented or intentionally deferred.           | Pending |
| README accuracy    | README must match actual current capability.                                     | Pending |

## 6. Build and install gates

| Gate             | Requirement                                                                | Status  |
| ---------------- | -------------------------------------------------------------------------- | ------- |
| Local build      | Public user can build from documented steps.                               | Pending |
| Local test       | Public user can run test suite from documented steps.                      | Pending |
| Rust toolchain   | Required Rust toolchain expectations are documented.                       | Pending |
| Binary execution | Public user can run `monad` locally from source.                           | Pending |
| Installer status | Installer absence is clearly documented if no installer exists.            | Pending |
| Package status   | Package registry absence is clearly documented if no package is published. | Pending |

## 7. Verification gates

| Gate                 | Requirement                                                        | Status  |
| -------------------- | ------------------------------------------------------------------ | ------- |
| Format               | `cargo fmt --check` passes.                                        | Pending |
| Tests                | `cargo test` passes.                                               | Pending |
| Clippy               | `cargo clippy --all-targets --all-features -- -D warnings` passes. | Pending |
| Root verifier        | `tools/scripts/verify.sh` passes.                                  | Pending |
| Context verifier     | Context records satisfy continuity and release-context checks.     | Pending |
| Frontmatter verifier | First-party Markdown frontmatter checks pass.                      | Pending |

## 8. Documentation gates

| Gate                     | Requirement                                                          | Status  |
| ------------------------ | -------------------------------------------------------------------- | ------- |
| Public README            | README clearly describes actual current Monad capability.            | Pending |
| Local build guide        | Local build instructions exist.                                      | Pending |
| Local verification guide | Verification instructions exist.                                     | Pending |
| Changelog                | Changelog exists and distinguishes internal/public status.           | Pending |
| Release notes template   | Release notes template exists.                                       | Pending |
| Public pre-release notes | Candidate-specific public pre-release notes exist before public tag. | Pending |
| Deferred capabilities    | Deferred capabilities are clearly listed.                            | Pending |

## 9. Context and handoff gates

| Gate                  | Requirement                                                                               | Status      |
| --------------------- | ----------------------------------------------------------------------------------------- | ----------- |
| Current state         | Current state context is refreshed after release-boundary decision.                       | Pending     |
| Handoff               | Fresh handoff is generated after release-boundary decision.                               | Pending     |
| Context pack          | Context pack is refreshed after release-boundary decision.                                | Pending     |
| Release-context state | Current release-context state is discoverable.                                            | In progress |
| Bootstrap prompt      | Bootstrap prompt is refreshed if public pre-release work changes onboarding expectations. | Pending     |

## 10. Public pre-release no-go criteria

A public pre-release must not proceed if any of the following are true:

* high or critical dependency vulnerabilities remain unresolved without formal risk acceptance
* README claims capabilities that are not implemented
* release notes imply package publication when no package is published
* release notes imply installer availability when no installer exists
* release notes imply autonomous agent execution when none exists
* release notes imply apply/write evolution behavior when only dry-run planning exists
* root verification fails
* context handoff is stale or misleading
* license status is unclear
* security reporting path is unclear

## 11. Required pre-release command set

Before any public pre-release tag, run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
git status --short
```

## 12. Required release decision

Before any public pre-release tag, create or update:

```text
docs/release/FIRST-PUBLIC-PRERELEASE-BOUNDARY.md
```

The release decision must state:

* what is included
* what is excluded
* what is deferred
* what risks remain
* whether public pre-release is approved or deferred
