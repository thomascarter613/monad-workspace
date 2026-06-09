---
title: "Monad Add Workflow"
document_type: "workflow-guide"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-006
tags:
  - monad
  - add
  - workflow
  - scaffold
  - safety
related:
  - docs/commands/ADD.md
  - docs/verification/ADD-SMOKE-TESTS.md
  - tools/scripts/verify-add.sh
  - crates/monad-core/src/component_add.rs
  - crates/monad-cli/src/main.rs
---

# Monad Add Workflow

## Status

Accepted.

## Purpose

This guide explains how to use the initial `monad add` workflow.

`monad add` adds a new component scaffold to an initialized Monad workspace.

## Preconditions

Before using `monad add`, the current directory must be inside a Monad workspace.

A Monad workspace has a repository baseline created by:

```bash
monad init --yes
```

or previewed by:

```bash
monad init --dry-run
```

If `monad add` is run outside a Monad workspace, it should fail safely.

## Supported Component Kinds

E12 supports four initial component kinds:

```text
app
package
service
tool
```

## Path Mapping

| Command | Component root |
| --- | --- |
| `monad add app web` | `apps/web` |
| `monad add package shared-ui` | `packages/shared-ui` |
| `monad add service api` | `services/api` |
| `monad add tool repo-lint` | `tools/repo-lint` |

## Component Name Rules

Component names must be filesystem-safe.

Allowed:

```text
web
api
shared-ui
repo-lint
worker-1
```

Rejected:

```text
../web
apps/web
Web
web app
web/
web;
-web
web-
```

## Preview First

Use dry-run to preview:

```bash
monad add app web --dry-run
```

Expected behavior:

- no files are written;
- target paths are shown;
- conflicts are reported;
- next steps are shown.

## Apply After Review

Use `--yes` to apply:

```bash
monad add app web --yes
```

Expected files:

```text
apps/web/README.md
apps/web/.gitkeep
```

## Conflict Behavior

If a target file already exists, `monad add --yes` refuses to continue.

Example:

```text
apps/web/README.md already exists
```

Expected behavior:

- command fails;
- no partial scaffold should be written;
- existing files are preserved.

## Safety Boundary

`monad add` does not:

- run Git commands;
- commit changes;
- push changes;
- install packages;
- modify package manager lockfiles;
- call remote services;
- create GitHub issues;
- publish packages;
- deploy anything.

## Recommended Workflow

```bash
monad add app web --dry-run
monad add app web --yes
git status --short
git diff
git add apps/web
git commit -m "feat: add web app scaffold"
```

Monad creates the scaffold.

The user remains responsible for Git operations.

## Verification

Run:

```bash
tools/scripts/verify-add.sh
```

For full E12 closeout verification:

```bash
tools/scripts/verify-e12.sh
```

## Outcome

Accepted.

The initial `monad add` workflow is documented and ready for follow-on component/language work.
