---
title: Upgrade Workflow
status: complete
epic: E17
---

# Upgrade Workflow

## 1. Preview

```bash
monad upgrade --dry-run
```

## 2. Review JSON if needed

```bash
monad upgrade --dry-run --format=json
```

## 3. Apply guarded generated metadata

```bash
monad upgrade --yes
```

## 4. Review evidence

```text
.monad/reports/upgrade-report.md
.monad/reports/upgrade-report.json
```

## 5. Commit deliberately

Upgrade does not commit automatically.
