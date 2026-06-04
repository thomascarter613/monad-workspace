---
title: "Public Pre-Release Evidence Checklist"
document_type: "release-evidence"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E10
work_packet: WP-E10-002
tags:
  - monad
  - release
  - public-prerelease
  - evidence
  - checklist
related:
  - README.md
  - docs/release/PUBLIC-CLAIMS-AUDIT.md
  - docs/project/MVP-COMMAND-REFERENCE.md
  - docs/release/FIRST-PUBLIC-PRERELEASE-BOUNDARY.md
---

# Public Pre-Release Evidence Checklist

## Status

Accepted as the current public pre-release evidence checklist.

This document does not approve a public release.

It converts public pre-release readiness into explicit pass/fail/pending evidence so the release boundary can be decided honestly.

## Work Packet

WP-E10-002 — Convert public pre-release checklist into pass/fail evidence.

## Evidence Status Legend

| Status | Meaning |
| --- | --- |
| Pass | Evidence exists and supports the claim. |
| Pending | Work remains before the gate can pass. |
| Fail | Evidence contradicts the claim or the gate is not satisfied. |
| Deferred | Explicitly out of scope for the current pre-release. |
| Not Applicable | Not relevant to the current pre-release posture. |

## Overall Public Pre-Release Readiness

| Area | Current Status | Evidence |
| --- | --- | --- |
| Public claims honesty | Pass, assuming WP-E10-001 has been applied | `README.md`, `docs/release/PUBLIC-CLAIMS-AUDIT.md` |
| Implemented command surface documented | Pass | `docs/project/MVP-COMMAND-REFERENCE.md`, `README.md` |
| Future commands clearly bounded | Pass, assuming WP-E10-001 has been applied | `README.md`, `docs/release/PUBLIC-CLAIMS-AUDIT.md` |
| Source/package posture decided | Pass | `docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md` |
| Public pre-release notes drafted | Pass | `docs/release/PUBLIC-PRERELEASE-NOTES.md` |
| Final verification audit complete | Pending | WP-E10-005 remains required |
| Tag/release decision made | Pass |  |
| Hosted/SaaS launch | Deferred | Out of scope for E10 |
| Installer generation | Deferred unless explicitly approved | Out of scope for E10 unless later approved |
| Crates.io/package publication | Deferred unless explicitly approved | Out of scope for E10 unless later approved |
| Autonomous agent runtime | Deferred | Explicitly not implemented |

## Gate 1 — Public Claims Match Implemented Behavior

| Check | Status | Evidence | Notes |
| --- | --- | --- | --- |
| README no longer describes the project as merely in initial foundation if current work has progressed beyond that. | Pass | `README.md` after WP-E10-001 | The README should state public pre-release hardening as the current phase. |
| README lists implemented command surface accurately. | Pass | `README.md`, `docs/project/MVP-COMMAND-REFERENCE.md` | Implemented commands include help, version, info, inspect, check, graph, context, evolve dry-run, and plan. |
| README separates implemented commands from future roadmap commands. | Pass | `README.md` after WP-E10-001 | `init`, `add`, `run`, `sync`, `doctor`, `release`, `upgrade`, `patch`, and `apply` remain future work. |
| README preserves non-release boundary. | Pass | `README.md` | Monad is not yet a public release, package, installer, hosted service, or autonomous agent runtime. |

## Gate 2 — Public Pre-Release Scope Is Explicit

| Check | Status | Evidence | Notes |
| --- | --- | --- | --- |
| Source-only vs packaged pre-release posture is decided. | Pass | `docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md` | Source-only public pre-release posture accepted. |
| Any package/install claims are removed or explicitly deferred. | Pass | `docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md` | Binary, installer, package-manager, and hosted distribution are deferred. |
| Public pre-release scope excludes hosted/SaaS launch. | Pass | E10 scope / README release boundary | Hosted launch is not part of E10. |
| Public pre-release scope excludes autonomous agent execution. | Pass | README safety boundary | Autonomous execution is not implemented. |

## Gate 3 — Release Notes and User-Facing Boundary

| Check | Status | Evidence | Notes |
| --- | --- | --- | --- |
| Public pre-release notes exist. | Pass | `docs/release/PUBLIC-PRERELEASE-NOTES.md` | Source-only public pre-release notes drafted. |
| Known limitations are documented. | Pass | `docs/release/PUBLIC-PRERELEASE-NOTES.md` | Dry-run-only evolution, no apply behavior, no installer/package distribution, and no autonomous runtime are documented. |
| Build/run instructions are current. | Pending | WP-E10-004 / WP-E10-005 | Must be verified against local commands. |
| Upgrade/install expectations are bounded. | Pending | WP-E10-003 / WP-E10-004 | Must not imply installer/package availability unless approved. |

## Gate 4 — Verification Evidence

| Check | Status | Evidence | Notes |
| --- | --- | --- | --- |
| `cargo fmt --check` passes. | Pending | WP-E10-005 | Final verification audit required. |
| `cargo test` passes. | Pending | WP-E10-005 | Final verification audit required. |
| `cargo clippy --all-targets --all-features -- -D warnings` passes. | Pending | WP-E10-005 | Final verification audit required. |
| `tools/scripts/verify.sh` passes. | Pending | WP-E10-005 | Final verification audit required. |
| Core smoke commands run successfully. | Pending | WP-E10-005 | Final verification audit required. |

## Gate 5 — Release Decision

| Check | Status | Evidence | Notes |
| --- | --- | --- | --- |
| Public pre-release go/no-go decision is documented. | Pass |  | Decision: Approved. |
| Tag decision is explicit. | Pass |  | Decision: Approved. |
| If tag is cut, release evidence is attached. | Pending |  | Tag cut remains a separate explicit command. |
| If tag is deferred, blockers are documented. | Not Applicable |  | Tag approved. |

## Current Decision

Current public pre-release status:

```text
Not ready yet.
```

Reason:

```text
The public claims audit and source-only distribution posture gates are addressed, but
final verification and tag decision gates remain pending.
```

## Required Remaining E10 Work

```text
WP-E10-005 — Run final public pre-release verification audit
```

## Definition of Done for WP-E10-002

WP-E10-002 is complete when:

- this evidence checklist exists;
- every release-readiness area has a status;
- pending gates are explicitly identified;
- deferred/non-applicable public claims are bounded;
- the next E10 work packets are clear;
- verification commands are listed for final release audit.

## Verification Commands

```bash
git status --short
find docs/release -maxdepth 1 -type f | sort
test -f docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md
test -f work/deliverables/E10/WP-E10-002-public-prerelease-evidence-checklist.md
tools/scripts/verify.sh
git status --short
```

## Outcome

Accepted.

Monad now has a public pre-release evidence checklist that converts release readiness into explicit pass/fail/pending/deferred evidence.
