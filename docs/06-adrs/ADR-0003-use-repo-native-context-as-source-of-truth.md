---
title: "ADR 0003: Use Repo-Native Context as Source of Truth"
document_type: "adr"
status: accepted
owner: "Thomas Carter"
created: 2026-06-03
updated: 2026-06-03
version: 1.0.0
tags:
  - monad
  - adr
related:
  - docs/06-adrs/README.md
---

# ADR 0003: Use Repo-Native Context as Source of Truth

## Status

Accepted.

## Context

Monad needs durable foundational decisions that are kept in the repository rather than scattered across chat history, private notes, or issue comments.

This ADR records one of the baseline project decisions needed to complete the E0 ADR foundation.

## Decision

Monad will treat repo-native context artifacts as the source of truth for project memory and handoff state.

## Rationale

Repo-native context is durable, reviewable, versioned, citeable, portable, AI-readable, and resilient across chat/session boundaries.

## Alternatives Considered

### Leave the decision implicit

Rejected because implicit decisions are difficult to review, cite, verify, and preserve across sessions.

### Record the decision only in issue comments

Rejected because issue comments are useful operational history but do not replace repo-native architectural decision records.

## Consequences

Context artifacts require maintenance, generated context must be clearly identified, and stale context can mislead if it is not verified.

## Implementation Notes

Future implementation work should follow this decision unless a later accepted ADR supersedes it.

## Related Documents

- \

## Review / Supersession Notes

Revisit this ADR if implementation experience shows that the decision creates avoidable friction or if a later project phase requires a more precise policy.
