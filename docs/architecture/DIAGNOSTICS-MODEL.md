---
title: Diagnostics Model
status: complete
epic: E15
---

# Diagnostics Model

Monad diagnostics are read-only reports that explain environment and repository readiness.

## Core model

The E15 doctor model is:

```text
DoctorReport
DoctorCheck
DoctorCategory
DoctorSeverity
```

A report is a deterministic list of checks.

A check has:

- stable ID;
- category;
- severity;
- subject;
- message;
- optional remediation.

## Non-mutating principle

Diagnostics may inspect. They must not mutate.

This means diagnostics must not:

- install missing tools;
- edit user configuration;
- rewrite native manifests;
- generate lockfiles;
- run package installs;
- upload telemetry.

## Relationship to sync

E14 sync reports repository-contract drift.

E15 doctor reports whether the user's local environment and repository state are ready for Monad workflows.
