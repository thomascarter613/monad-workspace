---
title: "Security"
status: approved
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:

* monad
* security
* safety
  related:
* docs/11-security/COMMAND-EXECUTION-SAFETY.md
* docs/11-security/FILE-OPERATION-SAFETY.md
* docs/09-ai/AI-COLLABORATION-RULES.md

---

# Security

## Purpose

This directory defines Monad’s security and safety standards.

Monad works near files, commands, generated context, and future AI-assisted actions. These areas require conservative safety rules.

## Belongs Here

* Security model.
* Threat model.
* Secret handling.
* Sandboxing principles.
* Command execution safety.
* File operation safety.
* Agent safety model.
* MCP safety boundaries.
* Supply-chain security.
* Responsible disclosure.

## Does Not Belong Here

* General product scope.
* Non-security implementation docs.
* Generated verification reports.

## Start Here

```text
COMMAND-EXECUTION-SAFETY.md
FILE-OPERATION-SAFETY.md
AGENT-SAFETY-MODEL.md
MCP-SAFETY-BOUNDARIES.md
SECRET-HANDLING.md
```
