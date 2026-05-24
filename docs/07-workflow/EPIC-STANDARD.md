---
title: "Epic Standard"
document_type: "workflow-standard"
status: "draft"
version: "0.1.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E0"
work_packet: "WP-E0-004"
tags:
  - workflow
  - epics
  - planning
  - delivery
---

# Epic Standard

## 1. Purpose

This document defines the standard for Monad epics.

An epic is a major body of product, platform, documentation, architecture, governance, or implementation work that is too large to complete safely as a single work packet.

Epics exist to keep Monad's delivery model explicit, traceable, and reviewable.

## 2. Definition

An epic is a coherent outcome-oriented container composed of work packets.

An epic MUST:

- represent a meaningful product, platform, or operational capability;
- have a clear objective;
- contain bounded work packets;
- be traceable to product strategy, requirements, roadmap, or architecture;
- have clear completion criteria;
- avoid becoming an unbounded theme or vague bucket.

An epic MUST NOT:

- be used as a dumping ground for unrelated tasks;
- bypass work packets;
- contain implementation work that cannot be independently verified;
- mix unrelated product areas without a clear unifying outcome.

## 3. Epic Identifier Format

Epic identifiers MUST use this format:

```text
E<number>
````

Examples:

```text
E0
E1
E2
E3
```

The epic identifier MUST remain stable after creation.

## 4. Required Epic Fields

Each epic record SHOULD contain:

| Field                 | Requirement            |
| --------------------- | ---------------------- |
| Epic ID               | Required               |
| Title                 | Required               |
| Product Area          | Required               |
| Objective             | Required               |
| Rationale             | Required               |
| Scope                 | Required               |
| Out of Scope          | Required               |
| Work Packets          | Required               |
| Deliverables          | Required               |
| Verification Strategy | Required               |
| Definition of Done    | Required               |
| Dependencies          | Required, may be empty |
| Risks                 | Required, may be empty |
| Status                | Required               |
| Priority              | Required               |
| Size                  | Required               |

## 5. Product Area

Product Area MUST appear before Objective.

Product Area describes the part of Monad affected by the epic.

Examples:

* Project Foundation
* Core Runtime
* CLI Experience
* Repository Intelligence
* Verification and Policy
* Context Bridge
* AI Integration
* Templates and Generators
* Documentation and Developer Experience
* Release and Distribution

## 6. Objective

The Objective MUST describe the durable outcome of the epic.

Good objective:

> Establish Monad's repository-native context bridge so future sessions, agents, and contributors can resume work from canonical repo state.

Weak objective:

> Add some context docs.

## 7. Epic Scope

Scope MUST describe what the epic includes.

Scope SHOULD be written as concrete capability boundaries.

Scope MUST NOT hide open-ended implementation work.

## 8. Out of Scope

Every epic MUST define what is excluded.

Out of Scope prevents drift and protects the project from absorbing unrelated work too early.

## 9. Work Packet Relationship

Epics are delivered through work packets.

Every significant epic outcome MUST map to one or more work packets.

A work packet SHOULD be independently reviewable and verifiable.

## 10. Epic Status Values

Allowed status values:

| Status      | Meaning                                  |
| ----------- | ---------------------------------------- |
| Proposed    | Suggested but not accepted               |
| Accepted    | Approved for planning                    |
| Planned     | Broken down into work packets            |
| In Progress | At least one work packet is active       |
| Blocked     | Cannot proceed until blocker is resolved |
| Complete    | All work packets are done and verified   |
| Superseded  | Replaced by a later epic or decision     |
| Deferred    | Intentionally postponed                  |

## 11. Epic Completion Criteria

An epic is complete only when:

* all required work packets are complete;
* all expected deliverables exist;
* verification passes;
* context files are updated;
* relevant ADRs are created or updated;
* known follow-up work is captured;
* the repository is in a clean, reviewable state.

## 12. Change Control

Changing an accepted epic SHOULD require one of:

* a documented update to the epic record;
* a follow-up work packet;
* an ADR if the change affects architecture;
* a context update if the change affects project state.

## 13. Maintenance Rules

Epic records SHOULD be updated when:

* work packet scope changes;
* a major dependency is added or removed;
* completion criteria change;
* an epic is blocked, deferred, superseded, or completed.

Epic records MUST NOT be silently rewritten in a way that destroys project history.


