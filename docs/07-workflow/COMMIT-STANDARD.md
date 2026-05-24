---
title: "Commit Standard"
document_type: "workflow-standard"
status: "draft"
version: "0.1.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E0"
work_packet: "WP-E0-004"
tags:

* workflow
* git
* commits
* conventional-commits

---

# Commit Standard

## 1. Purpose

This document defines Monad's commit standard.

Commits are project records. They should be atomic, reviewable, traceable, and understandable later.

Monad uses Conventional Commits.

## 2. Core Rule

Every commit SHOULD represent one coherent unit of work.

A commit SHOULD answer:

* What changed?
* Why did it change?
* What work packet does it support?
* How was it verified?

## 3. Conventional Commit Format

Commit messages SHOULD use:

```text
<type>(<scope>): <description>
```

Examples:

```text
docs(workflow): establish workflow standards
chore(repo): add repository foundation files
feat(cli): add workspace inspection command
fix(manifest): reject unsupported schema versions
test(core): add manifest validation tests
```

## 4. Allowed Types

Recommended types:

| Type     | Use                                            |
| -------- | ---------------------------------------------- |
| feat     | New user-facing or product capability          |
| fix      | Bug fix                                        |
| docs     | Documentation-only change                      |
| test     | Tests or fixtures                              |
| refactor | Code restructure without behavior change       |
| chore    | Maintenance, setup, or repository housekeeping |
| build    | Build system or dependency changes             |
| ci       | CI/CD changes                                  |
| perf     | Performance improvement                        |
| security | Security hardening                             |
| revert   | Revert a prior commit                          |

## 5. Scope

Scope SHOULD identify the affected area.

Examples:

* repo
* docs
* workflow
* context
* adr
* cli
* core
* manifest
* graph
* policy
* verification
* templates
* release

## 6. Description

The description MUST be:

* lowercase unless using a proper noun;
* imperative or outcome-oriented;
* concise;
* specific.

Good:

```text
docs(workflow): establish workflow standards
```

Weak:

```text
update stuff
```

## 7. Atomicity

A commit SHOULD contain only related changes.

Avoid mixing:

* docs and unrelated code;
* formatting churn and feature changes;
* multiple work packets;
* unrelated refactors;
* generated artifacts without their source change.

## 8. Work Packet Traceability

When practical, the commit body SHOULD reference the work packet.

Example:

```text
docs(workflow): establish workflow standards

Covers WP-E0-004.
Adds standards for epics, tasks, deliverables, verification, commits, branching, review, and context updates.
```

## 9. Verification Notes

Important commits SHOULD include verification notes in the commit body.

Example:

```text
Verification:
- git diff --check
- python3 frontmatter check
```

## 10. Breaking Changes

Breaking changes MUST be explicit.

Use:

```text
BREAKING CHANGE: <description>
```

Breaking changes SHOULD reference an ADR or work packet.

## 11. Commit Preparation Checklist

Before committing:

```bash
git status --short
git diff --check
```

Then run relevant project verification.

For documentation-only changes, run the Markdown frontmatter check.

## 12. Commit Message Template

Recommended full form:

```text
<type>(<scope>): <description>

Covers <work-packet-id>.

Summary:
- <change 1>
- <change 2>

Verification:
- <command 1>
- <command 2>
```

## 13. Noisy Change Rule

Large formatting-only changes SHOULD be isolated in their own commit.

Generated changes SHOULD be committed with their source change when that helps review, or separately when generated output is large.

## 14. Maintenance Rules

This standard SHOULD be updated when Monad adopts automated commit linting, release automation, or a stricter branching model.
