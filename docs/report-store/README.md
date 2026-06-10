# Local Artifact and Report Store

E28 adds Monad's local artifact/report store foundation.

## What this foundation does

- Defines the `.monad/reports` and `.monad/artifacts` contract.
- Adds report metadata schema.
- Adds artifact metadata schema.
- Adds report writing and retention policy documentation.
- Adds a local report/artifact index foundation.
- Adds artifact/report store smoke tests.

## Command surface

```bash
monad report-store --dry-run
monad report-store --dry-run --format=json
monad report-store --yes
```

Aliases:

```bash
monad reports --dry-run
monad artifacts --dry-run
```

## Store paths

```text
.monad/reports
.monad/artifacts
.monad/reports/report-store-index.md
.monad/reports/report-store-index.json
```

## Safety boundaries

This foundation does **not**:

- upload reports or artifacts;
- delete reports or artifacts;
- rewrite existing report or artifact content;
- contact remote storage;
- call AI providers;
- invoke package managers.

`--yes` writes generated index evidence only under `.monad/reports`.
