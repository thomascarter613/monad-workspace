---
title: Language-Aware Add Workflow
status: complete
epic: E13
---

# Language-Aware Add Workflow

Epic E13 extends `monad add` from generic component scaffolds to language-aware component scaffolds.

## Command shape

```bash
monad add <kind> <name> --language <language> --dry-run
monad add <kind> <name> --language <language> --yes
```

Supported languages:

```text
rust
typescript
python
go
```

Supported kinds:

```text
app
package
service
tool
```

## Safety workflow

Start with dry-run:

```bash
monad add service api --language rust --dry-run
```

Review the plan.

Then apply:

```bash
monad add service api --language rust --yes
```

## What Monad does

Monad writes local scaffold files only.

## What Monad does not do

Monad does not:

- run Cargo;
- run Bun, npm, pnpm, or yarn;
- run pip, uv, poetry, or hatch;
- run `go mod tidy`;
- mutate root workspace manifests;
- mutate lockfiles;
- initialize Git;
- run Git commands;
- call remote template registries;
- make network calls.

## Rust examples

```bash
monad add service api --language rust --dry-run
monad add service api --language rust --yes
```

For `app`, `service`, and `tool`, Rust creates:

```text
README.md
Cargo.toml
src/main.rs
```

For `package`, Rust creates:

```text
README.md
Cargo.toml
src/lib.rs
```

## TypeScript examples

```bash
monad add app web --language typescript --dry-run
monad add app web --language typescript --yes
```

TypeScript creates:

```text
README.md
package.json
tsconfig.json
src/index.ts
```

## Python examples

```bash
monad add service worker --language python --dry-run
monad add service worker --language python --yes
```

Python creates:

```text
README.md
pyproject.toml
src/<module_name>/__init__.py
tests/test_smoke.py
```

Names are normalized for Python module paths.

Example:

```text
my-worker -> my_worker
```

## Go examples

```bash
monad add tool repo-lint --language go --dry-run
monad add tool repo-lint --language go --yes
```

Go creates:

```text
README.md
go.mod
main.go
```

The initial local module path is:

```text
monad.local/<component-name>
```

## Deferred integrations

The following remain deferred beyond E13:

- root `Cargo.toml` workspace member mutation;
- package-manager workspace glob mutation;
- Python virtual environment management;
- `go.work` mutation;
- lockfile generation;
- framework-specific scaffolds;
- remote template registries.
