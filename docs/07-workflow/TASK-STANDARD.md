---
title: "Task Standard"
document_type: "workflow-standard"
status: "draft"
version: "0.1.0"
created: "2026-05-23"
updated: "2026-05-23"
owner: "Monad Project"
epic: "E0"
work_packet: "WP-E0-004"
tags:

* workflow
* tasks
* execution
* planning

---

# Task Standard

## 1. Purpose

This document defines the standard for Monad tasks.

Tasks are the smallest planned units of execution inside a work packet.

Tasks exist to make work explicit, reviewable, and verifiable without over-fragmenting delivery.

## 2. Definition

A task is a concrete action required to complete a work packet.

A task MUST:

* be specific;
* be actionable;
* contribute directly to a work packet objective;
* have a clear expected result;
* be small enough to review;
* be verifiable by inspection, command, test, or documented result.

A task MUST NOT:

* be a vague intention;
* contain multiple unrelated outcomes;
* bypass the work packet definition;
* hide risky changes behind generic wording.

## 3. Task Identifier Format

Task identifiers SHOULD use this format:

```text
T-<work-packet-id>-<number>
```

Example:

```text
T-WP-E0-004-001
```

For GitHub issue task lists, simpler checkbox labels MAY be used if the parent work packet is unambiguous.

## 4. Required Task Fields

Each task SHOULD include:

| Field               | Requirement                                |
| ------------------- | ------------------------------------------ |
| Task ID             | Required when recorded outside a checklist |
| Title               | Required                                   |
| Parent Work Packet  | Required                                   |
| Product Area        | Required                                   |
| Objective           | Required                                   |
| Expected Result     | Required                                   |
| Verification Method | Required                                   |
| Dependencies        | Required, may be empty                     |
| Status              | Required                                   |
| Priority            | Required                                   |
| Size                | Required                                   |

## 5. Product Area

Product Area MUST appear before Objective when a task is represented as a structured record.

This keeps task records aligned with the work packet standard.

## 6. Objective

The task objective MUST describe the specific outcome of the task.

Good objective:

> Add YAML frontmatter to all workflow standard documents.

Weak objective:

> Update docs.

## 7. Expected Result

Every task SHOULD state what will be true after completion.

Examples:

* The target file exists.
* The target document has valid YAML frontmatter.
* The verification command passes.
* The generated output is deterministic.
* The implementation compiles.
* The test fails before the fix and passes after the fix.

## 8. Verification Method

Each task SHOULD identify how completion will be checked.

Accepted verification methods include:

* file inspection;
* automated test;
* lint command;
* typecheck command;
* build command;
* script output;
* documentation review;
* `git diff` review;
* reproducible manual command.

## 9. Task Size

Task sizes SHOULD use:

| Size | Meaning                               |
| ---- | ------------------------------------- |
| XS   | Single small edit or check            |
| S    | Small contained change                |
| M    | Moderate change across a few files    |
| L    | Large change requiring careful review |
| XL   | Too large for a normal task; split it |

Tasks SHOULD usually be XS, S, or M.

## 10. Task Status Values

Allowed status values:

| Status      | Meaning                  |
| ----------- | ------------------------ |
| Todo        | Not started              |
| In Progress | Currently being worked   |
| Blocked     | Cannot proceed           |
| Review      | Ready for review         |
| Done        | Completed and verified   |
| Superseded  | Replaced by another task |
| Deferred    | Intentionally postponed  |

## 11. Task Discipline

A task is done only when:

* the intended change is complete;
* the expected result is true;
* verification has been run or the reason it cannot be run is documented;
* no unrelated changes are hidden in the task;
* any follow-up work is captured.

## 12. Relationship to Commits

A commit MAY contain one or more closely related tasks from the same work packet.

A commit SHOULD NOT mix tasks from unrelated work packets.

## 13. Maintenance Rules

Tasks SHOULD be updated when:

* scope changes;
* a task is split;
* a task is blocked;
* verification changes;
* the task is completed.

Completed task records SHOULD remain historically understandable.
