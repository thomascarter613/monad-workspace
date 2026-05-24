---
title: "ADR 0000: Template"
document_type: "adr"
status: accepted
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - adr
  - template
related:
  - docs/06-adrs/README.md
  - docs/00-meta/DOCUMENTATION-STANDARD.md
  - docs/00-meta/FRONTMATTER-STANDARD.md
---

# ADR 0000: Template

## Status

Accepted.

## Context

Monad uses Architecture Decision Records to preserve consequential project decisions in the repository.

This template defines the expected structure for future ADRs.

## Decision

Future Monad ADRs should follow this structure unless there is a clear reason to deviate:

```text
# ADR NNNN: Title

## Status

## Context

## Decision

## Rationale

## Alternatives Considered

## Consequences

## Implementation Notes

## Related Documents

## Review / Supersession Notes
````

## Rationale

A consistent ADR structure makes decisions easier to:

* write;
* review;
* search;
* compare;
* cite;
* update;
* supersede;
* use as AI-readable project memory.

## Required Sections

### Status

States whether the ADR is:

```text
draft
review
accepted
superseded
archived
```

### Context

Explains the situation that makes the decision necessary.

### Decision

States the actual decision clearly.

This section should be direct.

### Rationale

Explains why this decision was chosen.

### Alternatives Considered

Lists meaningful alternatives and why they were not chosen.

### Consequences

Explains what follows from the decision.

Include both positive and negative consequences.

### Implementation Notes

Explains what implementation work should follow from the decision.

### Related Documents

Lists related docs, ADRs, work packets, or implementation artifacts.

### Review / Supersession Notes

Explains when this ADR should be revisited or what supersedes it.

## ADR Writing Rules

An ADR should:

* make one major decision;
* state the decision plainly;
* avoid hiding uncertainty;
* avoid pretending trade-offs do not exist;
* be specific enough to guide implementation;
* link related docs;
* record consequences;
* avoid excessive rhetoric.

## ADR Numbering Rule

ADR numbers are permanent.

Do not reuse or renumber ADRs after commit.

## Current Status

This template is accepted as the initial ADR structure for Monad.
