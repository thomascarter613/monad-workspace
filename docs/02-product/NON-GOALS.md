---
title: "Non-Goals"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - product
  - non-goals
  - scope
related:
  - docs/02-product/MVP-SCOPE.md
  - docs/01-project/01-charter/PRODUCT-CHARTER.md
  - docs/01-project/00-vision/PRODUCT-VISION.md
---

# Non-Goals

## Purpose

This document defines what Monad is intentionally not trying to do, especially during MVP development.

Non-goals protect focus. Monad is ambitious enough that scope control is essential.

## Core Non-Goal

Monad is not trying to build the entire Holy Grail developer tool in the first version.

The MVP must prove a focused local-first foundation before expanding into advanced agents, cloud services, marketplace features, or enterprise governance.

## Product Non-Goals

### Monad is not an IDE replacement

Monad may integrate with editors later, but the MVP is a CLI-centered local tool.

Monad should not initially attempt to replace:

- VS Code;
- JetBrains IDEs;
- Vim/Neovim;
- Zed;
- Cursor;
- other editors.

### Monad is not a CI/CD replacement

Monad can run local checks and produce evidence, but it is not initially replacing CI/CD systems.

It may later integrate with:

- GitHub Actions;
- GitLab CI;
- Buildkite;
- CircleCI;
- other CI platforms.

### Monad is not a package manager replacement

Monad should coordinate native package managers, not replace them.

Examples:

- Cargo remains responsible for Rust packages.
- Bun/npm/pnpm/yarn remain responsible for JavaScript packages.
- Go tooling remains responsible for Go modules.
- uv/pip/Poetry remain responsible for Python packages.
- Maven/Gradle remain responsible for Java builds.

### Monad is not a monorepo tool clone

Monad may include monorepo intelligence, graphing, and task coordination, but it should not merely clone Nx, Turborepo, Bazel, Pants, Buck2, or Moon.

Monad’s broader value is repo understanding, verification, context, and safe evolution.

### Monad is not a black-box AI coding agent

Monad should not make unreviewed changes or ask users to trust opaque model output.

AI-assisted work must remain:

- planned;
- bounded;
- reviewable;
- verified;
- human-approved.

### Monad is not cloud-only SaaS

Monad’s core value must work locally.

A hosted control plane may exist later, but the local CLI and repo-native workflow must remain useful without it.

### Monad is not a project-management app

Monad may use GitHub Issues and Projects for planning, but it is not trying to replace all project-management systems.

Work management exists to support verified repository evolution, not to become a separate PM SaaS.

### Monad is not a documentation wiki product

Monad treats documentation as a first-class repo artifact, but the goal is not to build a general-purpose wiki.

Docs exist to support product clarity, architecture, workflow, verification, context, and implementation.

## MVP Non-Goals

During MVP, Monad should not implement:

- billing;
- marketplace;
- hosted accounts;
- enterprise SSO;
- team dashboard;
- remote execution;
- multi-tenant cloud agents;
- plugin signing;
- production-grade installer;
- background daemon;
- desktop app;
- web control plane;
- automatic PR creation;
- deployment workflows;
- full MCP server;
- full semantic retrieval;
- vector database integration;
- complete project graph analysis;
- full static analysis;
- complete security scanning;
- full dependency auditing;
- autonomous repair loops.

## Technical Non-Goals

### Do not rewrite native tools

Monad should not reimplement every tool it coordinates.

It should call, inspect, organize, verify, and report around native tools.

### Do not overfit to one ecosystem

Monad should start with Rust and JavaScript detection because they are immediately relevant, but the architecture should not assume those are the only ecosystems.

### Do not require one AI provider

Monad must not require one AI model vendor or one paid AI subscription.

### Do not require hidden chat memory

Monad context must live in the repo, not only in an assistant conversation.

### Do not make unsafe file changes

Monad should not write or delete files without clear planning, dry-run behavior, conflict handling, and approval where appropriate.

### Do not add complex abstractions too early

Monad should avoid elaborate abstractions before concrete commands prove what is needed.

## Business Non-Goals for MVP

The MVP should not require:

- final pricing;
- incorporated business structure;
- sales funnel;
- billing platform;
- enterprise contracts;
- support team;
- paid hosted service;
- formal marketplace.

Business thinking is important, but MVP development should focus on local product proof.

## Documentation Non-Goals

The documentation foundation should not require every future document to be fully final before coding begins.

The goal is:

- create the tree;
- fill critical foundation docs;
- stub future docs;
- promote important decisions as they become real.

Documentation should guide implementation, not prevent it.

## Agent Non-Goals

For early agent work, Monad should not allow:

- unapproved writes;
- unapproved command execution;
- production deployment;
- secret access;
- unrestricted shell access;
- silent background work;
- hidden model calls;
- autonomous commits;
- autonomous pushes;
- autonomous PR creation.

Agents assist. Humans approve.

## What This Protects

These non-goals protect Monad from:

- scope creep;
- premature enterprise complexity;
- unsafe automation;
- rewriting too many tools;
- becoming a generic AI wrapper;
- building SaaS before local value;
- losing the repo-native source-of-truth principle.

## Reconsideration Rule

A non-goal may become a goal later only when:

- the core local value is proven;
- the use case is clearly justified;
- there is a work packet or ADR;
- verification and safety expectations are defined;
- the addition does not compromise the product’s core principles.

## Current Status

This non-goals document is a draft. It is authoritative enough to constrain MVP planning and should be revised when new product or architecture decisions are accepted.
