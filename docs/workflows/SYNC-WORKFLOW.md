---
title: Sync Workflow
status: complete
epic: E14
---

# Sync Workflow

Use sync to understand repository drift before taking action.

## 1. Preview

```bash
monad sync --dry-run
```

## 2. Review

Read the findings:

- matches show expected state;
- missing items show expected state that was not found;
- stale items show incomplete or inconsistent state;
- unsupported items show recognized state that Monad will not rewrite automatically.

## 3. Write evidence

```bash
monad sync --yes
```

This writes generated reports only:

```text
.monad/reports/sync-report.md
.monad/reports/sync-report.json
```

## 4. Fix manually or with later approved commands

E14 does not rewrite native manifests. Future epics may add narrowly approved reconciliation commands.
