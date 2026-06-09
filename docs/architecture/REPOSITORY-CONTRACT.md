---
title: Repository Contract
status: complete
epic: E14
---

# Repository Contract

The repository contract is Monad's bounded understanding of what a healthy repository should look like.

E14 establishes the first MVP-safe version of that contract.

## Contract sources

The initial contract uses:

- `monad.toml` as declared Monad intent;
- `.monad/` as Monad operational state;
- component family directories:
  - `apps/`
  - `packages/`
  - `services/`
  - `tools/`
- supported native manifests discovered in components:
  - `Cargo.toml`
  - `package.json`
  - `pyproject.toml`
  - `go.mod`

## Finding severities

| Severity | Meaning |
| --- | --- |
| `match` | Expected state exists. |
| `missing` | Expected state is absent. |
| `extra` | State exists but is outside the first sync contract. |
| `stale` | State exists but appears incomplete or inconsistent. |
| `unsupported` | State is recognized but not automatically rewritten by sync. |

## Non-destructive rule

The contract may report drift, but sync does not silently fix user-owned files.

E14 only permits generated evidence writes under:

```text
.monad/reports/
```

Native manifest rewriting is intentionally deferred.
