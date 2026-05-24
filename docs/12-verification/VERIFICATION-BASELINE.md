---
title: "Verification Baseline"
document_type: "verification-standard"
status: "current"
version: "1.1.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-001"
tags:
  - verification
  - quality
  - repository-contract
  - workflow
  - rust
  - work-packets
  - tasks
  - deliverables
  - epics
  - adrs
  - context
---

# Verification Baseline

## 1. Purpose

This document defines Monad's initial repository verification baseline.

The baseline exists so foundational repository work and early Rust runtime work can be checked from durable repo-resident scripts.

## 2. Current Baseline Command

Run from the repository root:

```bash
tools/scripts/verify.sh
````

## 3. Current Checks

The baseline currently verifies:

1. Git diff whitespace validity.
2. Required foundation and runtime paths.
3. YAML frontmatter presence for Markdown files under:

   * `docs/`
   * `work/`
   * `.monad/`
4. Work packet record structure.
5. Task record structure.
6. Deliverable record structure.
7. Epic record structure.
8. ADR record structure.
9. E0 closure and E1 context handoff records.
10. Rust formatting with `cargo fmt --check`.
11. Rust tests with `cargo test`.
12. Current working tree status.

## 4. Scripts

| Script                                        | Purpose                                          |
| --------------------------------------------- | ------------------------------------------------ |
| `tools/scripts/verify.sh`                     | Main verification entrypoint                     |
| `tools/scripts/check-required-paths.py`       | Checks required foundation and runtime files     |
| `tools/scripts/check-markdown-frontmatter.py` | Checks Markdown YAML frontmatter presence        |
| `tools/scripts/check-work-records.py`         | Checks work packet record structure              |
| `tools/scripts/check-task-records.py`         | Checks task record structure                     |
| `tools/scripts/check-deliverable-records.py`  | Checks deliverable record structure              |
| `tools/scripts/check-epic-records.py`         | Checks epic record structure                     |
| `tools/scripts/check-adr-records.py`          | Checks ADR record structure                      |
| `tools/scripts/check-context-records.py`      | Checks E0 closure and E1 handoff context records |

## 5. Expected Successful Result

A successful run should include:

```text
All required foundation and runtime paths exist.
All docs/work/.monad Markdown files have YAML frontmatter.
All work packet records satisfy the required structure.
All task records satisfy the required baseline structure.
All deliverable records satisfy the required baseline structure.
All epic records satisfy the required baseline structure.
All ADR records satisfy the required baseline structure.
All context records satisfy the E0 closure and E1 handoff baseline.
cargo fmt --check
cargo test
Verification baseline passed.
```

The final `git status --short` output may be empty or may show intentional uncommitted changes before a commit.

## 6. Rust Verification

The Rust portion of the baseline currently requires:

```bash
cargo fmt --check
cargo test
```

The CLI can also be manually checked with:

```bash
cargo run -p monad-cli
```

Expected CLI output:

```text
Monad runtime foundation ready (crate: monad-core, model: local-first)
```

## 7. Failure Meaning

A failure means at least one foundational repository or runtime expectation is not satisfied.

Common causes include:

* a required file was not created;
* a file was created at the wrong path;
* a Markdown file is missing YAML frontmatter;
* a work packet record is missing a required section;
* a task record is missing a required section;
* a deliverable record is missing a required section;
* an epic record is missing a required planning section;
* an ADR record is missing required structure;
* context files do not identify E0, E1, WP-E1-001, and Runtime Foundation;
* Rust code is not formatted;
* Rust tests fail;
* trailing whitespace or whitespace errors are present in the diff.

## 8. Design Notes

The non-Rust verification scripts intentionally avoid external Python dependencies.

The Rust verification uses standard Cargo commands.

## 9. Future Expansion

Future verification work should add checks for:

* crate boundary rules;
* manifest validation;
* repository contract validation;
* generated artifact drift;
* CI parity;
* security checks;
* ADR index consistency;
* ADR status transition rules;
* epic, work packet, task, and deliverable consistency;
* deliverable artifact existence checks.

## 10. Maintenance Rules

This document must be updated when:

* `tools/scripts/verify.sh` changes materially;
* new baseline checks are added;
* existing baseline checks are removed;
* expected successful output changes;
* Rust verification requirements change.
