---
title: "E2 — Repository Intelligence Foundation"
document_type: "epic"
status: "in-progress"
version: "0.2.0"
created: "2026-05-24"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E2"
tags:
  - epic
  - repository-intelligence
  - inspection
  - graph
  - foundation
---

# E2 — Repository Intelligence Foundation

## Product Area

Repository Intelligence

## Objective

Establish Monad's repository intelligence foundation so the runtime can inspect a workspace, summarize repository structure, detect important project surfaces, and prepare for graphing and deeper repository analysis.

## Rationale

E1 gave Monad a working runtime, manifest loading, CLI commands, diagnostics, repository-contract checks, and text/JSON output. E2 now makes Monad more useful by moving from static foundation checks into repository inspection.

Repository intelligence is the bridge between a basic CLI and Monad's larger goal: understanding, verifying, and safely evolving software repositories.

## Scope

E2 includes:

- repository inspection data model;
- file and directory inventory primitives;
- important path classification;
- workspace surface detection;
- initial repository summary command behavior;
- JSON-friendly inspection output;
- future graph foundation preparation.

## Out of Scope

E2 does not include:

- full dependency graphing;
- language-specific parsing;
- LSP integration;
- AI provider integration;
- code modification;
- plugin marketplace behavior;
- full policy engine behavior.

## Work Packets

| Work Packet | Title | Status |
|---|---|---|
| WP-E2-001 | Establish repository inspection foundation | In Progress |

## Expected Result After Verification

Monad can inspect repository structure beyond the initial contract checks and expose a typed foundation for future repository graph and intelligence features.

## Verification

Run:

```bash
tools/scripts/verify.sh
```

Expected result:

```text
Verification baseline passed.
```

## Priority

High

## Size

XL
