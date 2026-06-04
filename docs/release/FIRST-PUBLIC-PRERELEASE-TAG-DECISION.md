---
title: "First Public Pre-Release Tag Decision"
document_type: "release-decision"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E10
work_packet: WP-E10-006
tags:
  - monad
  - release
  - public-prerelease
  - tag-decision
  - source-only
related:
  - README.md
  - docs/release/PUBLIC-CLAIMS-AUDIT.md
  - docs/release/PUBLIC-PRERELEASE-EVIDENCE-CHECKLIST.md
  - docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-POSTURE.md
  - docs/release/PUBLIC-PRERELEASE-NOTES.md
  - docs/release/PUBLIC-PRERELEASE-VERIFICATION-AUDIT.md
---

# First Public Pre-Release Tag Decision

## Status

Accepted.

## Work Packet

WP-E10-006 — Decide and cut first public pre-release tag, if approved.

## Decision

Approved.

## Decision Summary

Approved to cut a source-only public pre-release tag, subject to committing this decision record and using a clean working tree.

## Release Posture

Monad's first public pre-release posture remains:

```text
Source-only public pre-release.
```

This decision does not approve:

- binary artifact publication;
- installer publication;
- Crates.io publication;
- package-manager distribution;
- hosted service launch;
- SaaS launch;
- autonomous agent runtime claims;
- production-readiness claims.

## Verification Audit Dependency

The decision is based on:

```text
docs/release/PUBLIC-PRERELEASE-VERIFICATION-AUDIT.md
```

Verification audit status detected by this script:

| Field | Value |
| --- | --- |
| Audit Status | Pass |
| Failed Checks Zero | yes |
| Decision | Approved |

## Approved Tag Boundary

If the decision is Approved, the first public pre-release tag should use a pre-release-style tag name such as:

```text
v0.1.0-public-prerelease.0
```

The tag message should make the source-only boundary explicit:

```text
Monad v0.1.0 public pre-release 0 — source-only
```

## If Approved

The maintainer may cut the tag only after:

1. this decision record is committed;
2. the verification audit is committed;
3. release notes are committed;
4. the working tree is clean;
5. no final blockers remain.

Recommended command:

```bash
git tag -a v0.1.0-public-prerelease.0 -m "Monad v0.1.0 public pre-release 0 — source-only"
git push origin v0.1.0-public-prerelease.0
```

## If Deferred

If the decision is Deferred, do not cut a public pre-release tag.

Instead:

1. resolve the failing verification items;
2. rerun WP-E10-005;
3. rerun WP-E10-006;
4. create a new or updated decision record.

## Public Claim Boundary

Public-facing language may say:

```text
source-only public pre-release
```

Public-facing language must not imply:

```text
general availability
production readiness
installer availability
package publication
hosted service availability
autonomous agent runtime
```

## Definition of Done for WP-E10-006

WP-E10-006 is complete when:

- this decision record exists;
- the decision is explicit;
- if approved, the tag is cut only after the decision record is committed and the working tree is clean;
- if deferred, blockers are explicit;
- the E10 closeout state is clear.

## Outcome

Approved.
