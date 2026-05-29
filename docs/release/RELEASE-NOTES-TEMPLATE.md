---
title: Release Notes Template
description: Template for Monad internal MVP candidate and future release notes.
status: draft
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
phase: MVP Candidate Cut
epic: E8
work_packet: WP-E8-002
---

# Release Notes Template

## Release identifier

`<version-or-tag>`

## Release date

`<YYYY-MM-DD>`

## Release type

Choose one:

- Internal MVP candidate
- Internal release candidate
- Public pre-release
- Public release

## Release status

Choose one:

- Draft
- Candidate
- Approved
- Blocked
- Released

## Summary

Briefly describe what this release candidate or release includes.

## Scope statement

State the authorized scope.

Example:

> This is an internal MVP candidate for Monad. It is not a public release, package publication, installer distribution, hosted launch, or marketing launch.

## Included capabilities

List implemented and verified capabilities only.

- `<capability>`
- `<capability>`
- `<capability>`

## Excluded or deferred capabilities

List deferred capabilities explicitly.

- `<deferred capability>`
- `<deferred capability>`
- `<deferred capability>`

## Breaking changes

- None known.

## Added

- `<added item>`

## Changed

- `<changed item>`

## Fixed

- `<fixed item>`

## Security and safety notes

- `<security or safety note>`

## Verification evidence

Paste or link the verification command set and result.

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
git status --short
```

## Known blockers

* `<blocker or "None">`

## Known limitations

* `<limitation>`

## Approval notes

Record who approved the candidate/release and under what constraints.

## Tag

`<tag name>`

## Related documents

* `CHANGELOG.md`
* `docs/project/MVP-SCOPE-FREEZE.md`
* `docs/project/MVP-READINESS-REPORT.md`
