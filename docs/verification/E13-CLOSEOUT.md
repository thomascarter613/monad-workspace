---
title: E13 Closeout
status: complete
epic: E13
---

# E13 Closeout — Language-Aware Component Scaffolds

Epic E13 is complete when the following pass:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-add-language.sh
tools/scripts/verify-e13.sh
```

## Completed capability

`monad add` now supports language-aware scaffolds:

```bash
monad add service api --language rust --dry-run
monad add service api --language rust --yes

monad add app web --language typescript --dry-run
monad add app web --language typescript --yes

monad add service worker --language python --dry-run
monad add service worker --language python --yes

monad add tool repo-lint --language go --dry-run
monad add tool repo-lint --language go --yes
```

## Supported languages

```text
rust
typescript
python
go
```

## Safety retained

E13 keeps the E12 safety posture:

- `--dry-run` writes no files;
- `--yes` is required for writes;
- existing target files block writes;
- no Git commands are run;
- no package managers are run;
- no native toolchains are run;
- no network calls are made;
- root workspace manifests are not mutated;
- lockfiles are not generated.

## Deferred work

Future epics may add:

- root workspace membership management;
- package-manager workspace integration;
- lockfile strategy;
- framework presets;
- template registry support;
- component metadata in `monad.toml`;
- language-aware validation commands.
