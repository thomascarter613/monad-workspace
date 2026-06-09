#!/usr/bin/env bash
set -euo pipefail

# Complete WP-E12-006 — Document add workflow and close E12.
#
# Learning-first closeout packet.
#
# This packet does not add new runtime behavior.
# It documents the add workflow, adds E12 closeout verification, and records
# what was completed across E12.

echo "==> WP-E12-006"
echo "Goal: document monad add workflow and close E12."
echo "Mental model: feature epics need implementation, verification, documentation, and closeout evidence."
echo

mkdir -p docs/workflows
mkdir -p docs/verification
mkdir -p tools/scripts
mkdir -p work/learning/E12
mkdir -p work/deliverables/E12

cat > docs/workflows/ADD-WORKFLOW.md <<'EOF'
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
EOF

cat > tools/scripts/verify-e12.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

# Verify E12 — Component Add and Polyglot Scaffold Foundation.
#
# This script is intended as an E12 closeout check.
#
# It runs:
# - formatting check;
# - full test suite;
# - clippy with warnings denied;
# - add-command smoke verification;
# - general repo verification.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "${REPO_ROOT}"

echo "==> E12 verification: formatting"
cargo fmt --check

echo "==> E12 verification: tests"
cargo test

echo "==> E12 verification: clippy"
cargo clippy --all-targets --all-features -- -D warnings

echo "==> E12 verification: add smoke tests"
tools/scripts/verify-add.sh

echo "==> E12 verification: general repo verification"
tools/scripts/verify.sh

echo
echo "E12 verification passed."
EOF

chmod +x tools/scripts/verify-e12.sh

cat > docs/verification/E12-CLOSEOUT.md <<'EOF'
---
title: "E12 Closeout Verification"
document_type: "epic-closeout"
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
  - e12
  - closeout
  - verification
related:
  - docs/workflows/ADD-WORKFLOW.md
  - docs/verification/ADD-SMOKE-TESTS.md
  - tools/scripts/verify-e12.sh
  - tools/scripts/verify-add.sh
---

# E12 Closeout Verification

## Epic

E12 — Component Add and Polyglot Scaffold Foundation.

## Status

Accepted.

## Purpose

This document records the closeout verification path for E12.

E12 adds the initial `monad add` command surface.

## Completed Work Packets

| Work Packet | Outcome |
| --- | --- |
| WP-E12-001 | Defined `monad add` UX and safety contract. |
| WP-E12-002 | Added `monad add <kind> <name> --dry-run`. |
| WP-E12-003 | Added embedded component scaffold templates. |
| WP-E12-004 | Added guarded `monad add <kind> <name> --yes` write path. |
| WP-E12-005 | Added add-command smoke verification. |
| WP-E12-006 | Documented workflow and closeout evidence. |

## Implemented Command Surface

```bash
monad add app web --dry-run
monad add app web --yes

monad add package shared-ui --dry-run
monad add package shared-ui --yes

monad add service api --dry-run
monad add service api --yes

monad add tool repo-lint --dry-run
monad add tool repo-lint --yes
```

## Safety Properties

E12 preserves these safety properties:

- dry-run writes no files;
- `--yes` is required for writes;
- existing files block writes;
- uninitialized workspaces fail safely;
- no Git commands are run;
- no package managers are run;
- no remote services are called.

## Verification Script

Run:

```bash
tools/scripts/verify-e12.sh
```

This runs:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-add.sh
tools/scripts/verify.sh
```

## Manual Verification Commands

```bash
cargo run -p monad-cli -- add app web --dry-run
cargo run -p monad-cli -- add package shared-ui --dry-run
cargo run -p monad-cli -- add service api --dry-run
cargo run -p monad-cli -- add tool repo-lint --dry-run
```

Guarded write verification should be performed only in a temporary initialized workspace:

```bash
tmpdir="$(mktemp -d)"
(
  cd "$tmpdir"
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- init --yes
  cargo run --manifest-path /data/monad-workspace/Cargo.toml -p monad-cli -- add app web --yes
  test -f apps/web/README.md
  test -f apps/web/.gitkeep
)
rm -rf "$tmpdir"
```

## Closeout Criteria

E12 may be closed when:

- all E12 work-packet deliverables exist;
- `docs/workflows/ADD-WORKFLOW.md` exists;
- `docs/verification/ADD-SMOKE-TESTS.md` exists;
- `tools/scripts/verify-add.sh` exists;
- `tools/scripts/verify-e12.sh` passes;
- the E12 epic issue has a closeout comment.

## Outcome

Accepted.

E12 is ready to close after successful verification and commit.
EOF

cat > work/learning/E12/WP-E12-006-document-add-workflow-close-e12.md <<'EOF'
---
title: "Learning Note — WP-E12-006 Document Add Workflow and Close E12"
document_type: "learning-note"
status: active
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-006
tags:
  - learning
  - documentation
  - closeout
  - verification
  - monad
---

# Learning Note — WP-E12-006 Document Add Workflow and Close E12

## What You Are Doing

You are closing an implementation epic.

That means the work is not only code.

It includes:

```text
command contract
implementation
tests
smoke verification
workflow docs
closeout evidence
```

## Why Closeout Matters

A project can have working code and still be hard to continue.

Closeout docs make future work easier because they answer:

- What was built?
- What is supported?
- What is intentionally unsupported?
- How do I verify it?
- What should the next epic build on?

## What E12 Built

E12 added the first component scaffold command:

```bash
monad add <kind> <name> --dry-run
monad add <kind> <name> --yes
```

Supported kinds:

```text
app
package
service
tool
```

## What to Read

Read these files in order:

```text
docs/commands/ADD.md
docs/workflows/ADD-WORKFLOW.md
docs/verification/ADD-SMOKE-TESTS.md
docs/verification/E12-CLOSEOUT.md
```

## What to Inspect

```bash
git diff -- docs/workflows/ADD-WORKFLOW.md
git diff -- tools/scripts/verify-e12.sh
git diff -- docs/verification/E12-CLOSEOUT.md
```

## Main Lesson

A feature epic is complete only when a future version of you can understand, verify, and safely extend it.

That is why E12 ends with documentation and closeout verification rather than more feature code.

## Verification

Run:

```bash
tools/scripts/verify-e12.sh
```

Then commit the closeout docs and close the E12 issue.
EOF

cat > work/deliverables/E12/WP-E12-006-document-add-workflow-close-e12.md <<'EOF'
---
title: "WP-E12-006 Document Add Workflow and Close E12 Deliverable"
document_type: "deliverable-record"
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
  - e12
  - documentation
  - closeout
  - workflow
related:
  - docs/workflows/ADD-WORKFLOW.md
  - docs/verification/E12-CLOSEOUT.md
  - tools/scripts/verify-e12.sh
  - work/learning/E12/WP-E12-006-document-add-workflow-close-e12.md
---

# WP-E12-006 Document Add Workflow and Close E12 Deliverable

## Work Packet

WP-E12-006 — Document add workflow and close E12.

## Outcome

Implemented.

## Summary

This work packet documents the `monad add` workflow, adds E12 closeout verification, and records the E12 closeout evidence.

## Deliverables

- `docs/workflows/ADD-WORKFLOW.md`
- `docs/verification/E12-CLOSEOUT.md`
- `tools/scripts/verify-e12.sh`
- `work/learning/E12/WP-E12-006-document-add-workflow-close-e12.md`
- `work/deliverables/E12/WP-E12-006-document-add-workflow-close-e12.md`

## Verification

Run:

```bash
git status --short
tools/scripts/verify-e12.sh
git status --short
```

## Recommended Commit

```bash
git commit -m "docs(add): document add workflow and close E12"
```

## Closeout Note

WP-E12-006 is complete once the workflow docs, closeout verification, and deliverable record are committed.

After WP-E12-006 is closed, the E12 epic can be closed.

## Suggested E12 Closeout Comment

```text
Completed E12. Monad now supports a documented and verified `monad add` workflow for app, package, service, and tool components, including dry-run planning, embedded component scaffold templates, guarded `--yes` writes, conflict refusal, smoke verification, and closeout evidence.
```

## Next Epic

```text
E13 — Language-Aware Component Scaffolds
```
EOF

# Append a cross-reference to docs/commands/ADD.md without duplicating it.
if [[ -f docs/commands/ADD.md ]] && ! grep -q "WP-E12-006 Workflow Closeout Note" docs/commands/ADD.md; then
  cat >> docs/commands/ADD.md <<'EOF'

## WP-E12-006 Workflow Closeout Note

The add workflow guide is:

```text
docs/workflows/ADD-WORKFLOW.md
```

The E12 closeout verification record is:

```text
docs/verification/E12-CLOSEOUT.md
```

The reusable E12 verification command is:

```bash
tools/scripts/verify-e12.sh
```
EOF
fi

echo
echo "WP-E12-006 files updated:"
echo "  docs/workflows/ADD-WORKFLOW.md"
echo "  docs/verification/E12-CLOSEOUT.md"
echo "  tools/scripts/verify-e12.sh"
echo "  work/learning/E12/WP-E12-006-document-add-workflow-close-e12.md"
echo "  work/deliverables/E12/WP-E12-006-document-add-workflow-close-e12.md"
echo
echo "Learning checkpoint:"
echo "  Read work/learning/E12/WP-E12-006-document-add-workflow-close-e12.md before committing."
echo
echo "Next verification:"
echo "  tools/scripts/verify-e12.sh"
