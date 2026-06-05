#!/usr/bin/env bash
set -euo pipefail

# Complete WP-E12-001 — Define monad add UX and safety contract.
#
# This is a design/contract packet, not code implementation.
#
# It creates:
# - docs/commands/ADD.md
# - work/deliverables/E12/WP-E12-001-add-ux-safety-contract.md
#
# It does not add the add command implementation. That begins in WP-E12-002.

mkdir -p docs/commands
mkdir -p work/deliverables/E12

cat > docs/commands/ADD.md <<'EOF'
---
title: "monad add Command Contract"
document_type: "command-contract"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-001
tags:
  - monad
  - command
  - add
  - component
  - scaffold
  - safety
related:
  - docs/commands/INIT.md
  - docs/commands/INIT-PRESETS.md
  - docs/project/MVP-COMMAND-REFERENCE.md
---

# `monad add` Command Contract

## Status

Accepted as the initial command contract for E12.

This document defines the intended `monad add` behavior before implementation begins.

## Work Packet

WP-E12-001 — Define `monad add` UX and safety contract.

## Purpose

`monad add` adds a new component to an already initialized Monad-aware repository.

Where `monad init` creates the repository foundation, `monad add` creates a component scaffold inside that repository.

A component may be an app, package, service, tool, library, CLI, worker, or future supported component type.

## Product Intent

`monad add` should be:

- local-first;
- reviewable before writes;
- dry-run-first during early implementation;
- explicit about every file it proposes to create;
- safe in existing repositories;
- compatible with native language tooling;
- compatible with future language adapters;
- compatible with future repo contract validation.

## Non-Goals for E12

`monad add` must not become a full framework generator in E12.

E12 should not implement:

- full framework project generation;
- package installation;
- dependency installation;
- remote repository creation;
- GitHub issue creation;
- CI provider mutation;
- package publication;
- deployment configuration;
- hosted control plane integration;
- autonomous AI execution;
- destructive overwrites.

Those belong to later epics.

## Command Shape

Target command family:

```bash
monad add <kind> <name> --dry-run
monad add app web --dry-run
monad add package shared-ui --dry-run
monad add service api --dry-run
monad add tool repo-lint --dry-run
```

When guarded write behavior is implemented later in E12:

```bash
monad add app web --yes
monad add package shared-ui --yes
monad add service api --yes
monad add tool repo-lint --yes
```

Optional future flags:

```bash
--language rust
--language typescript
--language python
--language go
--language java
--path apps/web
--preset basic
--preset empty
```

## Initial Component Kinds

Initial E12 should support a small set:

| Kind | Intended path family | Purpose |
| --- | --- | --- |
| `app` | `apps/<name>` | User-facing application component. |
| `package` | `packages/<name>` | Shared library/package component. |
| `service` | `services/<name>` | Backend/service component. |
| `tool` | `tools/<name>` | Repo-local tool component. |

These kinds match the `polyglot-minimal` init scaffold.

## Name Rules

Component names must be conservative and filesystem-safe.

Initial rules:

- lowercase ASCII letters;
- numbers;
- hyphens;
- must start with a letter or number;
- must not be empty;
- must not contain `/`, `\`, `..`, spaces, shell metacharacters, or control characters.

Valid examples:

```text
web
api
shared-ui
repo-lint
worker-1
```

Invalid examples:

```text
../web
apps/web
Web App
web/
web;rm-rf
```

## Safety Contract

### Rule 1 — Dry-run before writes

The first implementation must support dry-run planning before any write path exists.

During early E12 implementation, `monad add` may require `--dry-run`.

### Rule 2 — Write path requires explicit approval

When write behavior is added, it must require:

```bash
--yes
```

### Rule 3 — No silent overwrite

`monad add` must never silently overwrite existing files or directories.

If the component root exists, the plan must report a conflict unless future adoption behavior is explicitly designed.

### Rule 4 — No Git mutation

For E12, `monad add` must not automatically run:

```bash
git add
git commit
git push
```

### Rule 5 — No package installation

For E12, `monad add` must not automatically run:

```bash
npm install
bun install
pnpm install
cargo add
go get
pip install
mvn
gradle
```

### Rule 6 — Native tools are respected

`monad add` may scaffold basic component files, but it must not rewrite native manifests unless a later command contract or ADR explicitly allows it.

### Rule 7 — Component operations are reviewable

The dry-run plan must show:

- component kind;
- component name;
- target root;
- proposed files;
- safety status;
- conflicts;
- next steps.

### Rule 8 — Failure must be actionable

Errors must explain:

- what input was invalid;
- what target path blocked the operation;
- what the user can do next.

## Initial Dry-Run Plan Model

A future `monad add` dry-run should render:

```text
Monad add dry-run plan

Component:
  kind: app
  name: web
  root: apps/web

Proposed file operations:
  would-create apps/web/README.md
  would-create apps/web/.gitkeep

Safety:
  mode: dry-run
  writes: disabled
  conflicts: 0

Next:
  rerun with --yes after reviewing the plan
```

## Initial Scaffold Shape

For E12, the scaffold should stay minimal.

### `app`

```text
apps/<name>/README.md
apps/<name>/.gitkeep
```

### `package`

```text
packages/<name>/README.md
packages/<name>/.gitkeep
```

### `service`

```text
services/<name>/README.md
services/<name>/.gitkeep
```

### `tool`

```text
tools/<name>/README.md
tools/<name>/.gitkeep
```

Language-specific scaffolds belong to later E12 slices or future language-adapter epics.

## Relationship to `monad init`

`monad init` creates the repository baseline.

`monad add` assumes a repository baseline exists or can be evaluated.

If the repository is not initialized, the command should provide an actionable message such as:

```text
This repository does not appear to be initialized for Monad. Run `monad init --dry-run` first.
```

The initial implementation may be permissive and work without a manifest, but it should not pretend that full repository contract validation exists until implemented.

## Relationship to Future Commands

`monad add` should not absorb responsibilities of:

```text
monad run
monad sync
monad doctor
monad release
monad upgrade
monad patch
monad apply
```

Those remain separate future command contracts.

## Acceptance Criteria

WP-E12-001 is complete when:

- this command contract exists;
- component kinds are named;
- component name validation rules are documented;
- dry-run-first behavior is established;
- guarded write behavior is defined but not implemented;
- non-goals are explicit;
- relationship to `monad init` is clear.

## Verification

```bash
git status --short
test -f docs/commands/ADD.md
grep -n "Dry-run before writes" docs/commands/ADD.md
grep -n "No silent overwrite" docs/commands/ADD.md
grep -n "Initial Component Kinds" docs/commands/ADD.md
tools/scripts/verify.sh
git status --short
```

## Outcome

Accepted.

E12 may proceed to WP-E12-002 — Add add-command dry-run plan.
EOF

cat > work/deliverables/E12/WP-E12-001-add-ux-safety-contract.md <<'EOF'
---
title: "WP-E12-001 Add UX and Safety Contract Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-001
tags:
  - monad
  - e12
  - add
  - safety
  - command-contract
related:
  - docs/commands/ADD.md
---

# WP-E12-001 Add UX and Safety Contract Deliverable

## Work Packet

WP-E12-001 — Define `monad add` UX and safety contract.

## Outcome

Completed.

## Summary

This work packet establishes the design and safety contract for `monad add` before implementation begins.

The contract defines:

- dry-run-first behavior;
- no silent overwrite rule;
- explicit `--yes` approval boundary for future writes;
- initial component kinds;
- component name validation rules;
- initial scaffold shape;
- relationship to `monad init`;
- non-goals and command boundaries.

## Initial Component Kinds

```text
app
package
service
tool
```

## Initial Target Path Families

```text
apps/<name>
packages/<name>
services/<name>
tools/<name>
```

## Safety Boundary

`monad add` must:

- preview before writes;
- require `--yes` for future writes;
- refuse to overwrite existing component paths;
- avoid Git mutation;
- avoid package installation;
- avoid remote service calls.

## Deliverables

- `docs/commands/ADD.md`
- `work/deliverables/E12/WP-E12-001-add-ux-safety-contract.md`

## Verification

Run:

```bash
git status --short
test -f docs/commands/ADD.md
grep -n "Dry-run before writes" docs/commands/ADD.md
grep -n "No silent overwrite" docs/commands/ADD.md
grep -n "Initial Component Kinds" docs/commands/ADD.md
tools/scripts/verify.sh
git status --short
```

## Recommended Commit

```bash
git commit -m "docs(add): define add ux and safety contract"
```

## Closeout Note

WP-E12-001 is complete once the command contract is committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E12-002 — Add add-command dry-run plan
```
EOF

echo
echo "WP-E12-001 files written:"
echo "  docs/commands/ADD.md"
echo "  work/deliverables/E12/WP-E12-001-add-ux-safety-contract.md"
echo
echo "Next verification:"
echo "  git status --short"
echo "  test -f docs/commands/ADD.md"
echo "  tools/scripts/verify.sh"
