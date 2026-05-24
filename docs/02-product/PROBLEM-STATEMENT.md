---
title: "Problem Statement"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - product
  - problem
  - strategy
related:
  - docs/01-project/00-vision/PRODUCT-VISION.md
  - docs/01-project/01-charter/PRODUCT-CHARTER.md
  - docs/02-product/TARGET-USERS.md
  - docs/02-product/VALUE-PROPOSITION.md
  - docs/02-product/MVP-SCOPE.md
---

# Problem Statement

## Purpose

This document defines the core problem Monad exists to solve.

Monad is not merely a tool for writing code faster. Monad exists because modern software projects are difficult to understand, verify, and safely evolve.

## Core Problem

Software repositories accumulate complexity faster than most teams can preserve understanding.

As a project grows, knowledge becomes scattered across:

- source code;
- package manifests;
- build scripts;
- test suites;
- CI configuration;
- documentation;
- architecture diagrams;
- issues;
- pull requests;
- chat history;
- personal memory;
- local scripts;
- deployment notes;
- undocumented conventions.

Developers often need to change a repository without fully understanding its structure, constraints, history, tooling, verification expectations, or current state.

This creates friction, risk, repeated work, and low confidence.

## AI Has Not Solved This Yet

AI coding tools can generate code, but code generation alone does not solve the deeper repository problem.

AI-generated changes are risky when the assistant lacks:

- durable project context;
- architectural constraints;
- current work state;
- verified requirements;
- knowledge of accepted decisions;
- awareness of native tool behavior;
- reliable test and check execution;
- safe file operation boundaries;
- evidence that the change is correct enough to review.

The result is often code that appears plausible but is not grounded enough in the real project.

Monad’s thesis is that the missing layer is not “more code generation.”

The missing layer is **repo-native understanding, context preservation, verification, and safe evolution**.

## Primary Pain Points

### 1. Repositories are hard to understand

Developers often clone a repository and must manually answer:

- What kind of project is this?
- Which languages and tools does it use?
- How is the workspace organized?
- Which commands should I run?
- What is the architecture?
- Which files matter first?
- What is safe to change?
- What is currently broken?

This creates slow onboarding and repeated context discovery.

### 2. Project context is fragile

Important project knowledge often lives in:

- old chats;
- issue comments;
- one developer’s memory;
- outdated docs;
- scattered notes;
- previous AI sessions;
- hidden local assumptions.

When context is not repo-native, it is easy to lose.

### 3. Verification is inconsistent

Many repositories do not have a single trustworthy way to answer:

- Did formatting pass?
- Did tests pass?
- Did linting pass?
- Did typechecking pass?
- Did contract checks pass?
- Did documentation stay aligned?
- Did architecture rules remain valid?
- What evidence proves the change is ready?

Without consistent verification, developers rely on guesswork.

### 4. AI output is difficult to trust

AI-generated code may be syntactically plausible but still wrong because it can:

- ignore project conventions;
- miss architectural boundaries;
- invent APIs;
- skip tests;
- fail to update docs;
- overlook security implications;
- break build assumptions;
- produce changes too large to review.

Trust requires evidence.

### 5. Repository evolution is unsafe

Common repo improvements are repetitive but risky:

- adding verification baselines;
- creating docs structure;
- adding context handoff files;
- adding CI checks;
- refactoring workspace structure;
- adding new packages;
- adding services;
- updating tooling;
- adding policies;
- introducing templates.

Developers need tools that can plan, preview, and verify these changes before applying them.

### 6. Toolchains are fragmented

Modern projects often combine multiple ecosystems:

- Rust;
- TypeScript;
- Python;
- Go;
- Java;
- shell scripts;
- Docker;
- CI;
- databases;
- cloud infrastructure.

Each tool has its own conventions. Developers need coordination, not another tool that ignores native ecosystems.

### 7. Documentation decays

Documentation often starts strong and becomes stale because it is not connected to:

- work packets;
- verification;
- context handoffs;
- architecture decisions;
- implementation changes;
- release processes.

Monad treats documentation as a living, repo-native project artifact.

## Consequences of the Problem

When this problem is not solved, teams experience:

- slower onboarding;
- repeated explanations;
- fragile AI sessions;
- low confidence in changes;
- longer review cycles;
- failing CI;
- architecture drift;
- duplicated setup work;
- stale documentation;
- risky refactors;
- inconsistent release readiness;
- more time spent understanding the project than improving it.

## The Opportunity

The opportunity is to create a developer tool that treats the repository as a living system.

Monad should help developers answer:

- What is this repository?
- What does it contain?
- What is the current state?
- What decisions govern it?
- What checks prove it is healthy?
- What should happen next?
- What is the safest way to make this improvement?
- What evidence exists that the work is ready to review?

## Problem Monad Should Solve First

The first problem Monad should solve is:

> Given a repository, help a developer understand its structure, current state, tooling, context readiness, and verification readiness.

The second problem is:

> Given a desired improvement, help plan and preview a safe, reviewable change.

The third problem is:

> Given AI assistance, keep the human in command and require evidence before trust.

## Problem Monad Should Not Solve First

Monad should not initially try to solve:

- complete autonomous software development;
- enterprise-wide workflow management;
- cloud-hosted agent orchestration;
- billing and marketplace infrastructure;
- replacing every native tool;
- replacing the IDE;
- replacing CI/CD;
- replacing all project management systems.

Those may become adjacent later, but the first product must prove local repository value.

## Current Status

This problem statement is a draft. It is sufficient to guide early MVP scope and should be refined as Monad is tested against real repositories.
