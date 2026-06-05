---
title: "WP-E12-002 Add Command Dry-Run Plan Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-002
tags:
  - monad
  - e12
  - add
  - dry-run
  - component
related:
  - crates/monad-core/src/component_add.rs
  - crates/monad-cli/src/main.rs
  - work/learning/E12/WP-E12-002-add-dry-run-plan.md
---

# WP-E12-002 Add Command Dry-Run Plan Deliverable

## Work Packet

WP-E12-002 — Add add-command dry-run plan.

## Outcome

Implemented.

## Summary

This work packet adds the first dry-run planning implementation for:

```bash
monad add <kind> <name> --dry-run
```

Supported component kinds:

```text
app
package
service
tool
```

## Deliverables

- `crates/monad-core/src/component_add.rs`
- `crates/monad-core/src/lib.rs`
- `crates/monad-cli/src/main.rs`
- `work/learning/E12/WP-E12-002-add-dry-run-plan.md`
- `work/deliverables/E12/WP-E12-002-add-dry-run-plan.md`

## Safety Boundary

WP-E12-002 is dry-run only.

It does not:

- write files;
- create directories;
- install packages;
- run Git commands;
- call remote services;
- support `--yes`.

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- add app web --dry-run
cargo run -p monad-cli -- add package shared-ui --dry-run
cargo run -p monad-cli -- add service api --dry-run
cargo run -p monad-cli -- add tool repo-lint --dry-run
cargo run -p monad-cli -- add app ../bad --dry-run
tools/scripts/verify.sh
git status --short
```

The unsafe-name command should fail.

## Recommended Commit

```bash
git commit -m "feat(add): add component dry-run plan"
```

## Closeout Note

WP-E12-002 is complete once the add dry-run plan is committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E12-003 — Add embedded component scaffold templates
```
