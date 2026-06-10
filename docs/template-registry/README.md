# Template Registry and Preset Evolution

E29 adds Monad's local template registry and preset evolution foundation.

## What this foundation does

- Defines the template registry evolution contract.
- Adds template metadata schema.
- Adds preset metadata schema.
- Validates template/preset compatibility.
- Plans preset upgrades without applying them.
- Adds template registry smoke tests.

## Command surface

```bash
monad template-registry --dry-run
monad template-registry --dry-run --format=json
monad template-registry --yes
```

Aliases:

```bash
monad templates --dry-run
monad presets --dry-run
```

## Safety boundaries

This foundation does **not** fetch remote templates, render templates, apply presets, upgrade presets, invoke package managers, or rewrite user-owned source files.

`--yes` writes generated registry evidence only under `.monad/reports`.
