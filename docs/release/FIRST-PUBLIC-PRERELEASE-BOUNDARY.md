---
title: First Public Pre-Release Boundary
description: Explicit go/no-go decision and boundary definition for Monad's first public pre-release.
status: draft
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
phase: Post-MVP Candidate Stabilization
epic: E9
work_packet: WP-E9-006
decision: no-go-defer-to-e10
---

# First Public Pre-Release Boundary

## 1. Decision

```text
NO-GO: defer first public pre-release to a dedicated E10 public-pre-release hardening epic.
```

Monad has completed an internal MVP candidate and post-candidate stabilization review, but the project should not yet cut a public pre-release tag.

## 2. Rationale

The internal MVP candidate proves the repository has a meaningful foundation, but a responsible public pre-release should require a stronger public-facing boundary.

E9 established or reviewed:

* public-readiness gaps
* generated artifact policy
* context freshness policy
* public pre-release checklist
* repository hygiene posture
* security/dependency alert status

The next step should be a focused public-pre-release hardening epic, not immediate public release.

## 3. What can be public-facing after E9

The following may be presented as true, if the repository state supports it:

* Monad is a Rust-based local-first monorepo runtime/developer-experience CLI.
* Monad has an internal MVP candidate milestone.
* Monad can inspect repository structure.
* Monad can render repository information and graphs.
* Monad can generate repo-native context artifacts.
* Monad supports dry-run planning/evolution foundations.
* Monad is not yet a public release.
* Monad is not yet packaged or installer-distributed.

## 4. What must remain internal or explicitly non-public

The following must remain internal or explicitly not claimed for public pre-release:

* public package publication
* installer distribution
* hosted/SaaS launch
* plugin marketplace
* autonomous agent execution
* apply/write evolution behavior
* MCP server public release
* enterprise features
* public stability guarantees
* compatibility promises beyond current documented constraints

## 5. Public pre-release blockers remaining

| Priority | Blocker                                                                                    | Current state                                      | Required next action                                  |
| -------- | ------------------------------------------------------------------------------------------ | -------------------------------------------------- | ----------------------------------------------------- |
| P0       | Public release notes do not yet exist                                                      | Not done                                           | Create public pre-release notes in E10.               |
| P0       | Public README may still over/under-state capabilities                                      | Needs final review                                 | Perform README capability audit in E10.               |
| P1       | Public pre-release checklist is not fully satisfied                                        | Checklist exists, gates pending                    | Convert checklist into E10 tasks.                     |
| P1       | Install/package story is intentionally absent                                              | Deferred                                           | Decide whether source-only pre-release is acceptable. |
| P1       | Context generator still reports old active epic/work packet if work metadata remains stale | Needs follow-up if still occurring                 | Stabilize work metadata source or generator inputs.   |
| P1       | Repository hygiene has been reviewed but may need final pass                               | In progress/resolved depending on WP-E9-005 commit | Recheck before public tag.                            |

## 6. Required E10 before public pre-release

Recommended next epic:

```text
E10 — Public Pre-Release Hardening and Boundary Enforcement
```

Minimum E10 work packets:

```text
WP-E10-001 — Audit README and public claims against implemented capability
WP-E10-002 — Convert public pre-release checklist into pass/fail evidence
WP-E10-003 — Decide source-only versus packaged pre-release posture
WP-E10-004 — Draft public pre-release notes
WP-E10-005 — Run final public pre-release verification audit
WP-E10-006 — Decide and cut first public pre-release tag, if approved
```

## 7. Conditions for future GO decision

A future public pre-release may be approved only if:

* Dependabot high/critical alerts are clear or formally risk-accepted
* root verification passes
* README is capability-accurate
* release notes are capability-accurate
* public pre-release checklist gates are satisfied or explicitly deferred
* license/contribution/security posture is clear
* release tag points to a verified commit
* public release posture does not imply unsupported capabilities

## 8. Explicit non-authorization

This decision does not authorize:

* publishing to crates.io
* creating installers
* creating a public release tag
* launching hosted services
* marketing launch
* claiming autonomous agent operation
* claiming apply/write evolution support

## 9. Decision result

```text
First public pre-release is deferred.
```

The project should proceed into E10.
