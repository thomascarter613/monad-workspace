---
title: monad upgrade
status: complete
epic: E17
---

# `monad upgrade`

`monad upgrade` plans or applies controlled repository evolution for Monad-managed repositories.

## Commands

```bash
monad upgrade --dry-run
monad upgrade --dry-run --format=json
monad upgrade --yes
```

## Safety contract

E17 upgrade does not:

- run destructive migrations;
- rewrite user source code;
- upgrade dependency versions;
- execute arbitrary third-party migration scripts;
- perform remote/cloud upgrades;
- publish packages;
- use autonomous agent-driven repo rewrites.

## Dry-run

```bash
monad upgrade --dry-run
```

Dry-run:

- detects the current repository contract version;
- compares it to the supported target version;
- resolves deterministic upgrade steps;
- reports blockers and no-op state;
- writes no files.

## Guarded apply

```bash
monad upgrade --yes
```

Guarded apply writes generated metadata/evidence only.

Approved generated outputs:

```text
.monad/upgrade/contract-version
.monad/upgrade/README.md
.monad/reports/upgrade-report.md
.monad/reports/upgrade-report.json
```

If a generated metadata file already exists with different content, upgrade refuses the overwrite and reports a conflict.
