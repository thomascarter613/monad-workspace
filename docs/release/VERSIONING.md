---
title: Versioning Policy
description: Version and build metadata policy for Monad internal MVP candidate preparation.
status: draft
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
phase: MVP Candidate Cut
epic: E8
work_packet: WP-E8-003
---

# Versioning Policy

## 1. Purpose

This document defines Monad's version and build metadata policy during internal MVP candidate preparation.

It exists to make version claims explicit, reviewable, and aligned with the current scope freeze.

## 2. Current release posture

Monad is preparing for an **internal MVP candidate**.

This is not:

- a public release
- a crates.io publication
- an installer release
- a hosted product launch
- an enterprise release
- an autonomous-agent release

## 3. Current package version

The current workspace package version should remain:

```text
0.1.0
```

This version identifies the early internal MVP candidate line. It does not imply public API stability or public distribution readiness.

## 4. Internal MVP candidate identifier

The recommended internal candidate identifier is:

```text
0.1.0-internal-mvp-candidate
```

This identifier should be used in documentation, release notes, and tag planning until E8 explicitly approves the final internal tag.

## 5. Recommended internal tag shape

For WP-E8-006, the recommended internal tag shape is:

```text
v0.1.0-internal-mvp-candidate.1
```

Rules:

* Use the `v` prefix for Git tags.
* Use `internal-mvp-candidate` to avoid implying public release.
* Use a numeric suffix for repeated internal candidate cuts.
* Do not create or push the tag until final verification passes.
* Do not reuse a tag once pushed.

## 6. Public release policy

Public release requires a later explicit decision.

Before any public release, Monad needs at minimum:

* final public release scope
* public install/build documentation
* license review
* package metadata review
* release notes approval
* clean verification audit
* tag policy approval
* publishing target decision

## 7. Cargo workspace version policy

During internal MVP preparation:

* workspace package version remains centralized in the root `Cargo.toml`
* crates inherit the workspace version where practical
* CLI `version` output must reflect the compiled package version
* release candidate labels may be documented without changing Cargo package version
* pre-public labels must not imply public SemVer stability

## 8. CLI version output policy

The CLI version command should answer:

* product name
* package name if available
* version
* runtime/build identity already implemented by Monad

The version command should not claim:

* public release status
* production readiness
* autonomous execution support
* write/apply evolution support
* hosted service availability

## 9. Changelog alignment

`CHANGELOG.md` may contain a pending internal MVP candidate section:

```text
[0.1.0-internal-mvp-candidate] - Pending
```

That section does not authorize public distribution.

## 10. Verification commands

```bash
cargo run -p monad-cli -- version
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

## 11. Scope-change rule

Any change to versioning policy that affects public release status, package publication, installer distribution, or tag semantics requires a later release-preparation work packet or ADR.
