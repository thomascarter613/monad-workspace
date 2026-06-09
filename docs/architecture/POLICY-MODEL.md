---
title: Policy Model
status: complete
epic: E19
---

# Policy Model

E19 defines Monad's first policy and approval-gate foundation.

## Core concepts

```text
OperationKind
OperationMutability
RiskLevel
ApprovalGate
OperationClassification
ApprovalPlan
PolicyFinding
PolicyReport
GatedWriteRequest
GatedWriteResult
```

## Approval gates

- `none`
- `dry-run-review`
- `explicit-yes`
- `forbidden`

## Principles

- read-only operations are low risk;
- generated writes require explicit approval;
- source mutations are blocked in MVP policy;
- publishing and remote side effects remain dry-run or blocked;
- risky operations are never approved automatically.
