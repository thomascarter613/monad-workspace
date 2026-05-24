---
title: "Monad Local State"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - local-state
  - context
  - reports
related:
  - docs/08-context/CONTEXT-BRIDGE.md
  - docs/12-verification/EVIDENCE-PACKET-STANDARD.md
  - docs/11-security/FILE-OPERATION-SAFETY.md
---

# Monad Local State

## Purpose

This directory is reserved for Monad-maintained context, reports, local state, generated artifacts, and operational files.

Some files under `.monad/` may be committed when they are useful for review or handoff. Other files are local-only and ignored by Git.

## Directory Layout

```text
.monad/
  context/   Current state, handoffs, context packs, and session chronicles
  reports/   Verification reports, evidence packets, and generated summaries
  cache/     Local cache files ignored by Git
  tmp/       Temporary files ignored by Git
  local/     Machine-local state ignored by Git
```

## Source of Truth Rule

Generated `.monad/` artifacts are useful, but they are not automatically accepted doctrine.

Accepted project truth should live in:

```text
docs/
docs/06-adrs/
source code
committed manifests
```

## Safety Rule

Do not place secrets, credentials, API keys, or private tokens in `.monad/` artifacts.

## Current Status

This directory is established during E0 as part of the repository foundation.
