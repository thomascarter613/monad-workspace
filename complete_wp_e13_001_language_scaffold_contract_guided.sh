#!/usr/bin/env bash
set -euo pipefail

# WP-E13-001 — Define language-aware scaffold UX and safety contract
#
# This guided script is intentionally docs-only.
# It does not edit Rust source code.
# It does not change Cargo files.
# It does not run package managers.
# It establishes the behavior contract that WP-E13-002+ will implement.

echo "==> WP-E13-001: Language-aware scaffold UX and safety contract"

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

cd "$REPO_ROOT"

echo "==> Repo root: $REPO_ROOT"

mkdir -p \
  docs/commands \
  docs/workflows \
  work/learning/E13 \
  work/deliverables/E13

echo "==> Writing docs/commands/ADD-LANGUAGE.md"
cat > docs/commands/ADD-LANGUAGE.md <<'EOF'
---
title: monad add --language
status: draft
epic: E13
work_packet: WP-E13-001
---

# `monad add --language`

`monad add --language` is the planned E13 extension of the E12 `monad add` workflow.

E12 introduced generic component scaffolds. E13 makes those scaffolds language-aware while preserving Monad's safety model: dry-run first, explicit confirmation for writes, no hidden installs, no network calls, no Git side effects, and no lockfile mutations.

This document is a UX and safety contract. It defines target behavior before implementation.

## Current generic behavior from E12

When `--language` is omitted, `monad add` keeps the E12 generic scaffold behavior.

```bash
monad add app web --dry-run
monad add app web --yes
```

Generic E12 output remains intentionally small:

```text
README.md
.gitkeep
```

E13 must not break this default behavior.

## Target CLI shape

Language-aware scaffolds use this shape:

```bash
monad add <kind> <name> --language <language> --dry-run
monad add <kind> <name> --language <language> --yes
```

Examples:

```bash
monad add app web --language typescript --dry-run
monad add app web --language typescript --yes

monad add service api --language rust --dry-run
monad add service api --language rust --yes

monad add service worker --language python --dry-run
monad add service worker --language python --yes

monad add tool repo-lint --language rust --dry-run
monad add package shared-ui --language typescript --dry-run
```

## Supported component kinds

E13 inherits the E12 component kinds and path mapping.

| Kind | Directory |
| --- | --- |
| `app` | `apps/<name>` |
| `package` | `packages/<name>` |
| `service` | `services/<name>` |
| `tool` | `tools/<name>` |

## Supported initial language IDs

WP-E13-001 defines the initial language IDs as:

| Language ID | Intended use |
| --- | --- |
| `rust` | Rust apps, services, tools, and packages |
| `typescript` | TypeScript apps, packages, services, and tools |
| `python` | Python services, packages, tools, and worker-style components |
| `go` | Go services, tools, and packages |

Future aliases may be added later, but are not required for the first implementation:

| Alias | Canonical language |
| --- | --- |
| `ts` | `typescript` |
| `py` | `python` |

Until aliases are implemented, users should use canonical language IDs.

## Name validation

E13 must preserve E12 component name rules:

- lowercase letters;
- numbers;
- hyphens;
- must start with a lowercase letter or number;
- no path separators;
- no `..`;
- no spaces;
- no uppercase;
- no repeated hyphens;
- no trailing hyphens.

Language-aware scaffolds must not weaken path safety.

## Dry-run behavior

Dry-run is required to show the planned scaffold without writing files.

```bash
monad add service api --language rust --dry-run
```

Dry-run must:

- validate the workspace;
- validate the component kind;
- validate the component name;
- validate the language ID;
- resolve the target directory;
- build a deterministic file-operation plan;
- print the plan;
- write no files;
- create no directories;
- run no package manager;
- run no language toolchain;
- make no network calls;
- make no Git changes.

## Write behavior

Writes require `--yes`.

```bash
monad add service api --language rust --yes
```

Write mode must:

- require an initialized Monad workspace;
- require a valid component kind;
- require a valid component name;
- require a supported language ID when `--language` is present;
- refuse to overwrite existing target files;
- create only the files shown in the plan;
- avoid package installs;
- avoid lockfile updates;
- avoid root workspace manifest mutation unless a later work packet explicitly adds it;
- avoid Git operations;
- avoid network calls.

## Unsupported behavior

The first language-aware implementation must not do these things:

- run `cargo`;
- run `bun`, `npm`, `pnpm`, or `yarn`;
- run `pip`, `uv`, `poetry`, or `hatch`;
- run `go mod tidy`;
- mutate the root `Cargo.toml`;
- mutate root package-manager workspaces;
- mutate lockfiles;
- auto-install dependencies;
- auto-initialize Git;
- call remote template registries;
- fetch templates from the network.

Monad may generate files that are compatible with native tooling, but the user remains in control of when native tools are executed.

## Planned scaffold file sets

### Generic scaffold

Used when `--language` is omitted.

```text
README.md
.gitkeep
```

### Rust scaffold

Target file set:

```text
README.md
Cargo.toml
src/lib.rs
```

For executable-style component kinds such as `app`, `service`, and `tool`, a later implementation may choose `src/main.rs` instead of `src/lib.rs`.

Initial implementation may keep Rust package scaffolds standalone and defer root workspace membership changes.

Deferred root mutation:

```text
Cargo.toml workspace members
Cargo.lock
```

### TypeScript scaffold

Target file set:

```text
README.md
package.json
tsconfig.json
src/index.ts
```

Deferred root mutation:

```text
package-manager workspace globs
bun.lockb
package-lock.json
pnpm-lock.yaml
yarn.lock
```

### Python scaffold

Target file set:

```text
README.md
pyproject.toml
src/<module_name>/__init__.py
tests/test_smoke.py
```

Python module names must be normalized from component names.

Example:

```text
component name: my-service
module name:   my_service
```

Deferred root mutation:

```text
shared Python workspace configuration
lockfiles
virtual environments
```

### Go scaffold

Target file set:

```text
README.md
go.mod
main.go
```

A later implementation may choose a package-style layout for `package` components.

Deferred root mutation:

```text
go.work
go.sum
```

## Error behavior

Unsupported language IDs must fail before planning writes.

Example:

```bash
monad add service api --language ruby --dry-run
```

Expected behavior:

- command exits non-zero;
- error explains that `ruby` is unsupported;
- output lists supported language IDs;
- no files are written.

## Design rule

Language-aware scaffolding should be additive.

E13 should build on the E12 planner/write path rather than fork a separate command path. The implementation should keep one safety model for generic and language-aware scaffolds.

## Implementation sequence

Recommended E13 sequence:

1. Define this UX and safety contract.
2. Add language option parsing and dry-run planning.
3. Add Rust templates.
4. Add TypeScript templates.
5. Add Python templates.
6. Add Go templates.
7. Add smoke verification.
8. Document and close E13.

## Non-goals for E13

E13 is not trying to solve every workspace integration problem.

Non-goals:

- dependency installation;
- root workspace membership management;
- lockfile management;
- template registry downloads;
- project-specific framework selection;
- advanced test harness generation;
- remote scaffolding;
- AI-generated scaffolds.

Those may become future epics after the safe local scaffold foundation is proven.
EOF

echo "==> Writing docs/workflows/LANGUAGE-AWARE-SCAFFOLDS.md"
cat > docs/workflows/LANGUAGE-AWARE-SCAFFOLDS.md <<'EOF'
---
title: Language-Aware Scaffolds
status: draft
epic: E13
work_packet: WP-E13-001
---

# Language-Aware Scaffolds

Language-aware scaffolds let Monad create the first files for a component using a known language template while still respecting the repository's existing toolchains.

Monad does not replace native tools. It coordinates them by creating predictable repository structure and leaving execution to the user.

## Why this matters

A polyglot repository runtime needs more than generic folders.

A real repository usually contains:

- TypeScript web apps and packages;
- Rust CLIs and runtime modules;
- Python AI, ingestion, and worker services;
- Go infrastructure, connector, or backend services.

The E12 scaffold gives a safe generic baseline. E13 adds language identity so future commands can reason about the component more clearly.

## Safety-first workflow

The expected user workflow is:

```bash
monad add service api --language rust --dry-run
```

Then inspect the plan.

If the plan is correct:

```bash
monad add service api --language rust --yes
```

Then inspect the repository:

```bash
git diff -- docs work
git status --short
```

For implementation packets after WP-E13-001, inspect the created component files too.

## Design principles

### 1. Preserve the generic path

Language support must not force users to pick a language.

This remains valid:

```bash
monad add app web --dry-run
```

### 2. Make language explicit

Language-aware scaffolds should be explicit:

```bash
monad add app web --language typescript --dry-run
```

This keeps the command predictable and avoids guessing from component names.

### 3. Use canonical language IDs first

Initial canonical IDs:

```text
rust
typescript
python
go
```

Aliases such as `ts` and `py` are useful, but they can be implemented later.

### 4. Do not execute native toolchains

Scaffolding should write files only.

It should not execute:

```text
cargo
bun
npm
pnpm
yarn
pip
uv
poetry
hatch
go
```

### 5. Do not mutate root manifests yet

Root manifest mutation is powerful but risky.

E13 should defer automatic changes to:

```text
Cargo.toml workspace members
package-manager workspace globs
go.work
lockfiles
```

This keeps early language-aware scaffolds reversible and easy to review.

### 6. Keep plans deterministic

The same command should produce the same ordered file-operation plan.

That makes dry-runs useful, tests stable, and AI/runtime reasoning easier.

## Initial scaffold matrix

| Language | Main generated files | Deferred |
| --- | --- | --- |
| Generic | `README.md`, `.gitkeep` | language-specific metadata |
| Rust | `README.md`, `Cargo.toml`, `src/lib.rs` or `src/main.rs` | root `Cargo.toml`, `Cargo.lock` |
| TypeScript | `README.md`, `package.json`, `tsconfig.json`, `src/index.ts` | workspace globs, package-manager lockfiles |
| Python | `README.md`, `pyproject.toml`, `src/<module_name>/__init__.py`, `tests/test_smoke.py` | virtualenvs, lockfiles |
| Go | `README.md`, `go.mod`, `main.go` | `go.work`, `go.sum`, `go mod tidy` |

## Example dry-run outputs to aim for

Exact wording may evolve, but dry-run output should communicate:

- component kind;
- component name;
- language;
- target directory;
- files that would be created;
- confirmation that no files were written.

Example intent:

```text
Plan: add rust service api
Target: services/api
Would create:
- services/api/README.md
- services/api/Cargo.toml
- services/api/src/main.rs

Dry run only. No files were written.
```

## Example write output to aim for

Example intent:

```text
Created rust service api
Target: services/api
Created:
- services/api/README.md
- services/api/Cargo.toml
- services/api/src/main.rs
```

## Review checklist

Before implementation, confirm:

- generic E12 behavior remains unchanged;
- unsupported language IDs fail safely;
- `--dry-run` writes nothing;
- `--yes` is required for writes;
- existing files block writes;
- native toolchains are not executed;
- root workspace manifests are not mutated;
- generated files are deterministic;
- docs explain deferred behavior clearly.

## Learning goal

This packet teaches that a CLI feature should have a written contract before code changes.

The contract protects the implementation from scope creep and makes later tests easier to write.
EOF

echo "==> Writing work/learning/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md"
cat > work/learning/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md <<'EOF'
---
title: WP-E13-001 Learning Note
epic: E13
work_packet: WP-E13-001
---

# Learning Note: Language-Aware Scaffold UX and Safety Contract

## What this packet teaches

This packet teaches how to define a CLI feature before implementing it.

The important lesson is that a command should have a behavior contract before code is changed. The contract answers:

- What command shape will users type?
- What inputs are valid?
- What will happen during dry-run?
- What will happen during confirmed writes?
- What is intentionally not supported yet?
- What files should be generated?
- What side effects are forbidden?

## Why this matters for Monad

Monad is a local-first polyglot repo runtime. It should be powerful, but it should also be predictable.

`monad add --language` will eventually create language-specific files. That can become risky if it also installs packages, changes lockfiles, edits workspace manifests, or calls remote template sources.

This packet draws a boundary:

- write local files only;
- preserve dry-run;
- require `--yes` for writes;
- do not run native tools;
- do not mutate root manifests yet;
- keep E12 generic scaffolds working.

## The key design move

E13 should be additive.

Instead of replacing the E12 generic `monad add` flow, language-aware scaffolding should extend it.

That means:

```bash
monad add app web --dry-run
```

still means generic scaffold, while:

```bash
monad add app web --language typescript --dry-run
```

means language-aware scaffold.

## What to inspect after running this script

Run:

```bash
git diff -- docs/commands/ADD-LANGUAGE.md
git diff -- docs/workflows/LANGUAGE-AWARE-SCAFFOLDS.md
git diff -- work/learning/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md
git diff -- work/deliverables/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md
```

Focus on the safety boundaries.

The most important boundaries are:

- dry-run writes no files;
- `--yes` is required for writes;
- no package installs;
- no network calls;
- no Git operations;
- no root manifest mutation in the first implementation.

## How this prepares WP-E13-002

WP-E13-002 can now implement only the next narrow slice:

```bash
monad add <kind> <name> --language <language> --dry-run
```

It should parse the language option and produce a dry-run plan.

It should not need to solve every language template yet.
EOF

echo "==> Writing work/deliverables/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md"
cat > work/deliverables/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md <<'EOF'
---
title: WP-E13-001 Deliverable Record
epic: E13
work_packet: WP-E13-001
status: complete
---

# Deliverable Record: WP-E13-001

## Work packet

WP-E13-001 — Define language-aware scaffold UX and safety contract.

## Purpose

Create the documentation contract for E13 before changing Rust source code.

## Files created

```text
docs/commands/ADD-LANGUAGE.md
docs/workflows/LANGUAGE-AWARE-SCAFFOLDS.md
work/learning/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md
work/deliverables/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md
```

## Scope completed

This packet defines:

- supported initial language IDs;
- target CLI shape;
- safety boundaries;
- unsupported behavior;
- dry-run behavior;
- `--yes` write behavior;
- planned scaffold file sets;
- deferred root manifest and lockfile behavior;
- recommended E13 implementation order.

## Supported initial language IDs

```text
rust
typescript
python
go
```

## Explicitly deferred

The following are deferred beyond WP-E13-001:

- Rust implementation;
- language option parsing;
- language-aware dry-run plans;
- language-specific file writes;
- root workspace membership mutation;
- package installs;
- native toolchain execution;
- lockfile generation;
- template registries;
- network calls.

## Verification

Recommended verification after this packet:

```bash
test -f docs/commands/ADD-LANGUAGE.md
test -f docs/workflows/LANGUAGE-AWARE-SCAFFOLDS.md
test -f work/learning/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md
test -f work/deliverables/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md
grep -R "rust" docs/commands/ADD-LANGUAGE.md
grep -R "typescript" docs/commands/ADD-LANGUAGE.md
grep -R "python" docs/commands/ADD-LANGUAGE.md
grep -R "go" docs/commands/ADD-LANGUAGE.md
git diff --check
```

If E12 verification has not been run recently, also run:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-add.sh
tools/scripts/verify-e12.sh
tools/scripts/verify.sh
git status --short
```

## Next packet

WP-E13-002 — Add language option parsing and dry-run model.
EOF

echo "==> WP-E13-001 docs written."

echo
echo "Next: inspect the changes"
echo "  git diff -- docs/commands/ADD-LANGUAGE.md"
echo "  git diff -- docs/workflows/LANGUAGE-AWARE-SCAFFOLDS.md"
echo "  git diff -- work/learning/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md"
echo "  git diff -- work/deliverables/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md"

echo
echo "Verify:"
echo "  test -f docs/commands/ADD-LANGUAGE.md"
echo "  test -f docs/workflows/LANGUAGE-AWARE-SCAFFOLDS.md"
echo "  test -f work/learning/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md"
echo "  test -f work/deliverables/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md"
echo "  grep -R \"rust\" docs/commands/ADD-LANGUAGE.md"
echo "  grep -R \"typescript\" docs/commands/ADD-LANGUAGE.md"
echo "  grep -R \"python\" docs/commands/ADD-LANGUAGE.md"
echo "  grep -R \"go\" docs/commands/ADD-LANGUAGE.md"
echo "  git diff --check"
echo
echo "Done."
