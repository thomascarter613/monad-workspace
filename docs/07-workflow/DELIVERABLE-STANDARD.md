---
title: "Deliverable Standard"
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
* deliverables
* verification
* documentation

---

# Deliverable Standard

## 1. Purpose

This document defines the standard for Monad deliverables.

A deliverable is a concrete artifact produced by an epic, work packet, or task.

Deliverables make progress visible and auditable.

## 2. Definition

A deliverable is an artifact that can be reviewed, verified, and preserved.

Deliverables may include:

* source files;
* documentation files;
* ADRs;
* tests;
* scripts;
* schemas;
* generated reports;
* templates;
* configuration files;
* context handoff files;
* release artifacts.

## 3. Deliverable Requirements

A deliverable MUST:

* have a clear purpose;
* belong to a specific product area or workflow area;
* be located in the appropriate repository path;
* be traceable to an epic, work packet, task, requirement, or ADR;
* be reviewable through `git diff`;
* be verifiable by an explicit method.

A deliverable MUST NOT:

* be hidden outside the repository without a pointer;
* be generated without documenting how to regenerate it;
* duplicate canonical truth without a reason;
* silently contradict an ADR or accepted standard.

## 4. Required Deliverable Fields

When deliverables are listed in a structured work packet, each SHOULD include:

| Field               | Requirement |
| ------------------- | ----------- |
| Path or Name        | Required    |
| Type                | Required    |
| Purpose             | Required    |
| Owner               | Required    |
| Source Work Packet  | Required    |
| Verification Method | Required    |
| Expected Result     | Required    |
| Status              | Required    |

## 5. Deliverable Types

Recommended deliverable types:

| Type                | Meaning                                             |
| ------------------- | --------------------------------------------------- |
| Documentation       | Markdown or other project docs                      |
| Architecture Record | ADR or architecture decision artifact               |
| Source Code         | Runtime or library implementation                   |
| Test                | Unit, integration, contract, acceptance, or fixture |
| Script              | Automation or verification script                   |
| Schema              | Machine-readable contract or validation schema      |
| Template            | Reusable generated starting point                   |
| Report              | Generated verification or analysis output           |
| Context Artifact    | Handoff, current state, decision log, context pack  |
| Configuration       | Tool, workspace, CI, or runtime configuration       |

## 6. Canonical Location

Deliverables SHOULD live in canonical repository locations.

Examples:

| Deliverable             | Canonical Area                                       |
| ----------------------- | ---------------------------------------------------- |
| Product vision          | `docs/01-project/00-vision/`                         |
| Product charter         | `docs/01-project/01-charter/`                        |
| Requirements            | `docs/03-requirements/`                              |
| Domain model            | `docs/04-domain/`                                    |
| Architecture            | `docs/05-architecture/`                              |
| ADRs                    | `docs/06-adrs/`                                      |
| Workflow standards      | `docs/07-workflow/`                                  |
| AI/context docs         | `docs/08-context/`, `docs/09-ai/`, `.monad/context/` |
| Engineering standards   | `docs/10-engineering/`                               |
| Security standards      | `docs/11-security/`                                  |
| Verification docs       | `docs/12-verification/`                              |
| Work records            | `work/`                                              |
| Monad operational state | `.monad/`                                            |

## 7. Documentation Deliverables

Markdown deliverables under `docs/`, `work/`, and `.monad/` MUST include YAML frontmatter.

Markdown deliverables SHOULD include:

* purpose;
* scope;
* maintenance rules;
* relationship to other artifacts;
* verification notes when applicable.

## 8. Generated Deliverables

Generated deliverables MUST identify:

* source input;
* generation command;
* expected output path;
* whether generated output is committed;
* how drift is detected.

Generated deliverables SHOULD be deterministic.

## 9. Deliverable Verification

Every deliverable SHOULD have an explicit verification method.

Examples:

* Markdown frontmatter check;
* schema validation;
* unit test;
* snapshot comparison;
* manual review checklist;
* `git diff --check`;
* build command;
* CLI command output.

## 10. Deliverable Completion

A deliverable is complete only when:

* it exists in the expected location;
* it satisfies the work packet definition;
* it passes applicable verification;
* it does not violate accepted ADRs or standards;
* context state is updated if the deliverable changes project direction or current state.

## 11. Maintenance Rules

Deliverables SHOULD be updated when their source of truth changes.

Deprecated deliverables SHOULD be marked as superseded rather than silently deleted when history matters.
