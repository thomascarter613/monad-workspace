---
title: MVP Candidate Tag Record
description: Internal MVP candidate tag record for Monad.
status: draft
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
phase: MVP Candidate Cut
epic: E8
work_packet: WP-E8-006
---

# MVP Candidate Tag Record

## 1. Purpose

This document records the internal MVP candidate tag for Monad.

This is not a public release record. It does not authorize public package publishing, installer distribution, hosted launch, or marketing announcement.

## 2. Internal candidate tag

```text
v0.1.0-internal-mvp-candidate.1
```

## 3. Release posture

| Field                              | Value                  |
| ---------------------------------- | ---------------------- |
| Release type                       | Internal MVP candidate |
| Public release                     | No                     |
| Package published                  | No                     |
| Installer available                | No                     |
| Hosted service launched            | No                     |
| Autonomous agent execution claimed | No                     |
| Apply/write evolution claimed      | No                     |

## 4. Candidate scope

The candidate scope is controlled by:

```text
docs/project/MVP-SCOPE-FREEZE.md
```

## 5. Verification gate

The tag is authorized only if this file records a passing audit:

```text
docs/release/MVP-CANDIDATE-VERIFICATION-AUDIT.md
```

## 6. Final verification command set

```bash
git status --short
git log --oneline --max-count=12
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
git status --short
```

## 7. Tag command

```bash
git tag -a v0.1.0-internal-mvp-candidate.1 -m "v0.1.0 internal MVP candidate 1"
```

## 8. Push command

```bash
git push origin v0.1.0-internal-mvp-candidate.1
```

## 9. Notes

This tag marks an internal milestone only.

Future public release preparation requires separate release scope, documentation, verification, and approval.
