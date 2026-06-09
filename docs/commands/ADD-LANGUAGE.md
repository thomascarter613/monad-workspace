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
