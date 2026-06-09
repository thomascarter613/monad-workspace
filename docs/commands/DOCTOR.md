---
title: monad doctor
status: complete
epic: E15
---

# `monad doctor`

`monad doctor` diagnoses local environment and repository readiness without mutating the machine or repository.

## Commands

```bash
monad doctor
monad doctor --format=json
```

## Safety contract

`monad doctor` does not:

- install tools;
- edit shell profiles;
- mutate PATH;
- run package-manager install commands;
- modify native manifests;
- modify Monad context files;
- upload telemetry;
- call cloud services.

## Diagnostic categories

- environment
- core tooling
- ecosystem tooling
- repository
- monad context
- repository contract

## Severity levels

| Severity | Meaning |
| --- | --- |
| `pass` | A required or relevant check passed. |
| `warn` | The user should review or fix something, but doctor did not block execution. |
| `fail` | A required readiness check failed. |
| `info` | Informational state. |
| `skipped` | Optional check was not required by detected repo state. |

## JSON output

```bash
monad doctor --format=json
```

JSON output is intended for dashboards, issue evidence, and AI-readable session handoffs.
