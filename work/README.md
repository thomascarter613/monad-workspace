---
title: "Work Directory"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - work
  - workflow
related:
  - docs/07-workflow/OPERATING-MODEL.md
  - docs/07-workflow/WORK-HIERARCHY.md
  - docs/07-workflow/WORK-PACKET-STANDARD.md
---

# Work Directory

## Purpose

This directory stores repo-native work planning and delivery records when they need to live in the repository.

GitHub Issues and GitHub Projects are used for active execution tracking. The repository remains the canonical place for durable workflow standards, accepted decisions, and long-lived work records.

## Directory Layout

```text
work/
  epics/       Repo-resident epic records when needed
  packets/     Repo-resident work packet records when needed
  tasks/       Repo-resident task records when needed
  records/     Delivery records, summaries, and closeout notes
````

## Rule

Do not duplicate every GitHub Issue into this directory by default.

Use `work/` when the record should be versioned with the repository, referenced by docs, used for handoff, or preserved beyond GitHub issue workflow.

## Current Status

The initial work system is tracked primarily through GitHub Issues and Projects. This directory exists for repo-native records that need to be committed.
