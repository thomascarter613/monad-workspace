---
title: Doctor Workflow
status: complete
epic: E15
---

# Doctor Workflow

Use doctor before running larger Monad workflows.

## 1. Run doctor

```bash
monad doctor
```

## 2. Review warnings and failures

Failures usually mean a required tool or basic environment condition is missing.

Warnings usually mean optional tooling or repository context is incomplete.

## 3. Export JSON when needed

```bash
monad doctor --format=json
```

Use JSON output for issue evidence, handoffs, dashboards, and AI-readable diagnostics.

## 4. Fix manually

Doctor does not repair automatically. It gives remediation hints.
