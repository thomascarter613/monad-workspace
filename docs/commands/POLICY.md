---
title: monad policy
status: complete
epic: E19
---

# `monad policy`

`monad policy` previews or writes generated policy/approval-gate evidence.

## Commands

```bash
monad policy --dry-run
monad policy --dry-run --format=json
monad policy --yes
```

## Safety contract

`policy` does not:

- execute commands;
- rewrite user source files;
- publish releases;
- call AI providers;
- automatically approve risky operations;
- use a remote policy service;
- claim OPA/Rego support.

## Generated evidence

`monad policy --yes` writes generated evidence only:

```text
.monad/reports/policy-report.md
.monad/reports/policy-report.json
```
