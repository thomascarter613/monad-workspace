---
title: "Branching Standard"
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
* branches
* delivery

---

# Branching Standard

## 1. Purpose

This document defines Monad's branching standard.

The goal is to keep repository history clean while supporting safe, reviewable work.

## 2. Default Model

Monad SHOULD use a simple trunk-based model.

The default permanent branch is:

```text
main
```

Short-lived work branches SHOULD be used for reviewable changes when needed.

## 3. Branch Naming

Branch names SHOULD use:

```text
<type>/<work-packet-id>-<short-description>
```

Examples:

```text
docs/wp-e0-004-workflow-standards
feat/wp-e1-001-workspace-inspection
fix/wp-e2-003-manifest-validation
chore/wp-e0-001-repo-foundation
```

## 4. Allowed Branch Types

Recommended branch types:

| Type     | Use                    |
| -------- | ---------------------- |
| docs     | Documentation changes  |
| feat     | Product capability     |
| fix      | Bug fix                |
| chore    | Repository maintenance |
| refactor | Internal restructuring |
| test     | Test-focused change    |
| ci       | CI/CD change           |
| security | Security hardening     |

## 5. Branch Scope

A branch SHOULD map to one work packet or one tightly related fix.

A branch SHOULD NOT accumulate unrelated work.

## 6. Main Branch Expectations

The `main` branch SHOULD remain:

* buildable;
* testable;
* understandable;
* free of known broken verification unless explicitly documented;
* aligned with accepted ADRs.

## 7. Local Work Without Branches

During early solo development, direct work on `main` MAY be acceptable if:

* changes are small;
* commits remain atomic;
* verification is run;
* history remains understandable;
* no collaborative review flow is being bypassed.

As Monad matures, review branches SHOULD become the default.

## 8. Rebasing and Merging

For short-lived branches:

* rebase MAY be used to keep history linear;
* merge commits MAY be used when preserving branch context is useful;
* squash merge MAY be used for noisy local history.

The chosen approach SHOULD preserve traceability to work packets.

## 9. Long-Lived Branches

Long-lived branches SHOULD be avoided.

If a long-lived branch is necessary, it MUST have:

* a clear purpose;
* an owner;
* a rebase or sync plan;
* explicit merge criteria;
* known divergence documented.

## 10. Release Branches

Release branches are not required during the earliest foundation phase.

When introduced, release branches SHOULD follow a documented release standard.

Possible format:

```text
release/v<major>.<minor>
```

## 11. Hotfix Branches

Hotfix branches MAY use:

```text
hotfix/<short-description>
```

A hotfix MUST be followed by verification and context updates if it affects product state.

## 12. Deleting Branches

Merged work branches SHOULD be deleted after merge unless retained for a clear reason.

## 13. Maintenance Rules

This standard SHOULD be revisited before public release, multi-contributor development, or automated release management.
