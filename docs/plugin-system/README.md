# Plugin and Extension System

E30 adds Monad's plugin and extension system foundation.

## What this foundation does

- Defines the plugin boundary and trust model.
- Adds a local plugin manifest schema.
- Adds an extension point registry foundation.
- Adds an adapter/plugin loading plan model.
- Adds disabled-by-default plugin safety checks.
- Adds plugin contract tests and documentation.

## Command surface

```bash
monad plugin-plan --dry-run
monad plugin-plan --dry-run --format=json
monad plugin-plan --yes
```

Aliases:

```bash
monad plugins --dry-run
monad extensions --dry-run
```

## Local plugin metadata

Plugins are discovered from:

```text
plugins/**/plugin.toml
plugins/**/plugin.json
plugins/**/metadata.toml
plugins/**/metadata.json
```

## Safety boundaries

This foundation does **not**:

- load plugin code;
- open dynamic libraries;
- execute plugin binaries;
- contact remote plugin registries;
- enable plugins automatically;
- invoke package managers;
- rewrite user-owned source files.

`--yes` writes generated plugin-system evidence only under `.monad/reports`.
