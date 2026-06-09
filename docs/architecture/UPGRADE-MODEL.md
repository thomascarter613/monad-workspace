---
title: Upgrade Model
status: complete
epic: E17
---

# Upgrade Model

E17 introduces a first safe repository evolution model.

## Core types

```text
UpgradePlan
UpgradeStatus
UpgradeStep
UpgradeStepSafety
UpgradeApplyResult
```

## Status model

- `missing-manifest`
- `unknown`
- `upgrade-needed`
- `up-to-date`
- `unsupported-future`

## Step registry

Upgrade steps are deterministic and ordered.

The E17 registry includes generated metadata/evidence steps and explicit no-op documentation for unsafe categories such as source-code rewrites.
