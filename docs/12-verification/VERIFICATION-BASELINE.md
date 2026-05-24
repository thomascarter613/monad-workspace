---
title: "Verification Baseline"
document_type: "verification-standard"
status: "draft"
version: "0.1.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E0"
work_packet: "WP-E0-005"
tags:
  - verification
  - quality
  - repository-contract
  - workflow
---

# Verification Baseline

## 1. Purpose

This document defines Monad's initial repository verification baseline.

The baseline exists so foundational repository work can be checked from durable repo-resident scripts instead of pasted one-off commands.

## 2. Current Baseline Command

Run from the repository root:

```bash
tools/scripts/verify.sh
````

## 3. Current Checks

The baseline currently verifies:

1. Git diff whitespace validity.
2. Required E0 foundation paths.
3. YAML frontmatter presence for Markdown files under:

   * `docs/`
   * `work/`
   * `.monad/`
4. Current working tree status.

## 4. Scripts

| Script                                        | Purpose                                             |
| --------------------------------------------- | --------------------------------------------------- |
| `tools/scripts/verify.sh`                     | Main verification entrypoint                        |
| `tools/scripts/check-required-paths.py`       | Checks required E0 foundation files and directories |
| `tools/scripts/check-markdown-frontmatter.py` | Checks Markdown YAML frontmatter presence           |

## 5. Expected Successful Result

A successful run should include:

```text
All required E0 foundation paths exist.
All docs/work/.monad Markdown files have YAML frontmatter.
Verification baseline passed.
```

The final `git status --short` output may be empty or may show intentional uncommitted changes before a commit.

## 6. Failure Meaning

A failure means at least one foundational repository expectation is not satisfied.

Common causes include:

* a required file was not created;
* a file was created at the wrong path;
* a Markdown file is missing YAML frontmatter;
* trailing whitespace or whitespace errors are present in the diff;
* the script is being run from an unexpected repository state.

## 7. Design Notes

This baseline intentionally avoids external dependencies.

It requires only:

* Bash;
* Git;
* Python 3.

The baseline should remain small, readable, and portable during the E0 foundation phase.

## 8. Future Expansion

Future verification work should add checks for:

* ADR structure;
* work packet records;
* Rust formatting;
* Rust tests;
* crate boundaries;
* manifest validation;
* repository contract validation;
* generated artifact drift;
* CI parity;
* security checks.

## 9. Maintenance Rules

This document must be updated when:

* `tools/scripts/verify.sh` changes materially;
* new baseline checks are added;
* existing baseline checks are removed;
* expected successful output changes;
* verification requirements move from documentation-only checks into code, build, or runtime checks.
