---
title: "Terminology"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - reference
  - terminology
  - glossary
related:
  - docs/00-meta/GLOSSARY.md
  - docs/01-project/04-glossary/PRODUCT-GLOSSARY.md
  - docs/07-workflow/WORK-HIERARCHY.md
  - docs/08-context/CONTEXT-BRIDGE.md
---

# Terminology

## Purpose

This document defines key Monad terms.

It is a reference for humans and AI assistants.

## Monad

The unified product name.

Monad is an AI-native, repo-native, local-first Software Foundry OS for understanding, verifying, and safely evolving software repositories.

## Software Foundry OS

The long-term product category Monad is aiming toward.

It means a developer operating system for repository understanding, verification, context, safe evolution, and supervised AI-assisted software development.

## Repo-Native

Information or behavior that lives in, operates on, or is grounded in the repository.

Repo-native artifacts are reviewable, versioned, and available to future sessions.

## Local-First

Core Monad value should work on the user’s machine without requiring a hosted service.

## Context Bridge

Monad’s system for preserving current state, handoff, decisions, active work, and AI-readable continuity in repository artifacts.

## Current State

A document or generated artifact that summarizes the present project state, active work, blockers, and next recommended action.

## Handoff

A structured artifact that allows a future human or AI assistant to resume work.

## Fresh Chat Handoff

A handoff specifically designed to start a new AI conversation with enough project context.

## Bootstrap Prompt

A reusable prompt that tells a new AI assistant how to orient from repository files.

## Context Pack

A compact bundle of important project context.

## Work Packet

Monad’s primary delivery unit.

A work packet is bounded, actionable, verifiable, and usually commit-sized.

## Epic

A large project outcome composed of multiple work packets.

## Product Area

A major functional or planning area of Monad.

Examples:

```text
Core Runtime
CLI
Repo Intelligence
Context Bridge
Verification
Evolution Engine
Agent Supervision
```

## Verification

The process of checking whether work behaves as expected and producing evidence.

## Evidence Packet

A reviewable report that records what checks ran, what passed, what failed, what was skipped, and what conclusion is supported.

## Check

A defined verification action.

Examples:

```text
rust.fmt
rust.test
rust.clippy
docs.frontmatter
```

## Check Registry

The catalog of checks Monad knows how to describe, select, run, skip, and report.

## Native Tool

An ecosystem tool that Monad coordinates rather than replaces.

Examples:

```text
cargo
bun
npm
go
pytest
maven
gradle
```

## Adapter

A module or boundary that helps Monad detect, understand, or coordinate a native tool or ecosystem.

## Evolution Engine

Monad’s system for safely planning and preparing repository changes.

## File Operation Plan

A structured plan describing proposed file creates, updates, skips, conflicts, or deletes before writes occur.

## Dry Run

A mode that shows what would happen without changing files.

## Agent Supervision

Monad’s human-in-command approach to AI-assisted planning, drafting, verification, and approval.

## Provider-Agnostic

Designed so Monad does not depend on one AI model provider, API, subscription, or hosted service.

## MCP

Model Context Protocol.

A future integration path for exposing Monad capabilities to compatible AI tools while preserving safety boundaries.

## ADR

Architecture Decision Record.

A repository document that records a consequential decision, its context, rationale, alternatives, consequences, and supersession notes.

## Rust Apprenticeship Mode

Monad’s Rust implementation mode.

It means small slices, complete file contents, comments, tests, verification commands, expected results, and clear explanations for a maintainer learning Rust.

## Thin CLI

A CLI architecture where command parsing and rendering happen in the CLI crate, but durable product logic lives in `monad-core`.

## Core Runtime

The durable local engine that owns reusable product logic.

For Monad, this is initially `monad-core`.

## Current Status

This terminology document is a draft. It should be expanded as new Monad concepts are accepted.
