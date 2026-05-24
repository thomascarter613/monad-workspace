---
title: "E1 Task Records"
document_type: "task-index"
status: "draft"
version: "0.8.0"
created: "2026-05-23"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
tags:

* tasks
* runtime
* rust
* index

---

# E1 Task Records

This directory contains durable task records for E1 — Runtime Foundation.

## Current Task Records

| Task            | Parent Work Packet | Title                                      | Status      |
| --------------- | ------------------ | ------------------------------------------ | ----------- |
| T-WP-E1-001-001 | WP-E1-001          | Create Rust workspace crates               | Complete    |
| T-WP-E1-001-002 | WP-E1-001          | Add minimal core runtime identity          | Complete    |
| T-WP-E1-001-003 | WP-E1-001          | Add thin CLI entrypoint                    | Complete    |
| T-WP-E1-001-004 | WP-E1-001          | Add Rust verification to baseline          | Complete    |
| T-WP-E1-002-001 | WP-E1-002          | Add diagnostics module                     | Complete    |
| T-WP-E1-002-002 | WP-E1-002          | Export diagnostics from core runtime       | Complete    |
| T-WP-E1-002-003 | WP-E1-002          | Update E1 records and context              | Complete    |
| T-WP-E1-003-001 | WP-E1-003          | Add core error module                      | Complete    |
| T-WP-E1-003-002 | WP-E1-003          | Export core error model                    | Complete    |
| T-WP-E1-003-003 | WP-E1-003          | Update E1 records and context              | Complete    |
| T-WP-E1-004-001 | WP-E1-004          | Add workspace context module               | Complete    |
| T-WP-E1-004-002 | WP-E1-004          | Export workspace context from core runtime | Complete    |
| T-WP-E1-004-003 | WP-E1-004          | Update E1 records and context              | Complete    |
| T-WP-E1-005-001 | WP-E1-005          | Add root Monad manifest                    | Complete    |
| T-WP-E1-005-002 | WP-E1-005          | Add manifest model module                  | Complete    |
| T-WP-E1-005-003 | WP-E1-005          | Export manifest model from core runtime    | Complete    |
| T-WP-E1-005-004 | WP-E1-005          | Update E1 records and context              | Complete    |
| T-WP-E1-006-001 | WP-E1-006          | Add manifest parsing dependencies          | Complete    |
| T-WP-E1-006-002 | WP-E1-006          | Add manifest loading                       | Complete    |
| T-WP-E1-006-003 | WP-E1-006          | Update E1 records and context              | Complete    |
| T-WP-E1-007-001 | WP-E1-007          | Add CLI command parser                     | Complete    |
| T-WP-E1-007-002 | WP-E1-007          | Add CLI info rendering                     | Complete    |
| T-WP-E1-007-003 | WP-E1-007          | Update E1 records and context              | In Progress |
|                 |                    |                                            |             |

cat > work/deliverables/E1/D-WP-E1-007-001-cli-info-command.md <<'MD'
---
title: "D-WP-E1-007-001 — CLI Info Command"
document_type: "deliverable"
status: "complete"
version: "0.1.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-007"
deliverable: "D-WP-E1-007-001"
tags:
  - deliverable
  - cli
  - rust
---

# D-WP-E1-007-001 — CLI Info Command

## Product Area

CLI Experience

## Objective

Add `monad info` to the CLI.

## Source Work Packet

WP-E1-007 — Establish CLI Info Command Foundation

## Deliverable Type

Source Code

## Artifact Path

`crates/monad-cli/src/main.rs`

## Expected Result After Verification

`monad info` discovers the workspace, loads `monad.toml`, and prints workspace/project/runtime information.

## Verification

Run:

```bash
cargo run --quiet -p monad-cli -- info
tools/scripts/verify.sh
````

## Status

Complete
