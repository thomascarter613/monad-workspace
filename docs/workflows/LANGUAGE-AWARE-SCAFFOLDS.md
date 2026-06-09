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
