---
title: Public Pre-Release Hardening and Boundary Enforcement
epic: E10
status: in-progress
priority: high
size: large
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
product_area: Public Pre-Release / Release Governance
---

# E10 — Public Pre-Release Hardening and Boundary Enforcement

## Objective

Harden Monad for a responsible first public pre-release by enforcing the public pre-release boundary, auditing public claims, producing release evidence, and deciding whether a verified public pre-release tag should be cut.

## Scope

E10 is focused on final public-pre-release readiness.

It converts the E9 findings into concrete pass/fail evidence and prevents Monad from publishing a public pre-release before the repository, documentation, verification, and release boundary are ready.

## In scope

- Audit README and public claims against implemented capability.
- Convert the public pre-release checklist into pass/fail evidence.
- Decide source-only versus packaged pre-release posture.
- Draft public pre-release notes.
- Run final public pre-release verification audit.
- Decide whether to cut the first public pre-release tag.

## Out of scope

- Hosted launch.
- SaaS launch.
- Installer generation unless explicitly approved.
- Crates.io publishing unless explicitly approved.
- Autonomous agent execution.
- Apply/write evolution behavior.
- Plugin marketplace.
- Enterprise feature launch.

## Work packets

| Work Packet | Name | Status |
|---|---|---|
| WP-E10-001 | Audit README and public claims against implemented capability | planned |
| WP-E10-002 | Convert public pre-release checklist into pass/fail evidence | planned |
| WP-E10-003 | Decide source-only versus packaged pre-release posture | planned |
| WP-E10-004 | Draft public pre-release notes | planned |
| WP-E10-005 | Run final public pre-release verification audit | planned |
| WP-E10-006 | Decide and cut first public pre-release tag, if approved | planned |

## Definition of done

E10 is complete when:

- public-facing claims match implemented behavior;
- public pre-release checklist has pass/fail evidence;
- source-only/package/installer posture is explicit;
- public pre-release notes exist;
- final public pre-release verification audit passes or records blockers;
- the first public pre-release is either approved and tagged or explicitly deferred.

## Verification

Run:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```
