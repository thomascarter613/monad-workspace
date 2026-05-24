---
title: "ADR 0001: Use Rust for Core Runtime"
status: accepted
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - adr
  - rust
  - core-runtime
  - architecture
related:
  - docs/01-project/01-charter/PRODUCT-CHARTER.md
  - docs/05-architecture/SYSTEM-OVERVIEW.md
  - docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
  - docs/05-architecture/MODULE-BOUNDARIES.md
  - docs/10-engineering/RUST-CODING-STANDARD.md
---

# ADR 0001: Use Rust for Core Runtime

## Status

Accepted.

## Context

Monad is intended to become a local-first, repo-native software foundry for understanding, verifying, and safely evolving software repositories.

The product needs a durable core runtime that can eventually support:

- command-line usage;
- repository inspection;
- workspace resolution;
- file operation planning;
- native command execution;
- verification checks;
- context artifact generation;
- safe repository evolution;
- provider-agnostic AI workflows;
- future MCP or daemon surfaces;
- possible single-binary distribution.

The implementation language for the core runtime is therefore a foundational decision.

## Decision

Monad’s durable local core runtime will be implemented in Rust.

The initial MVP will use a Rust workspace with at least:

```text
crates/monad-cli/
crates/monad-core/
````

`monad-cli` will provide the command-line interface.

`monad-core` will contain durable product logic.

## Rationale

Rust is the best fit for Monad’s core runtime because Monad needs to operate close to the filesystem, run native tools, inspect repositories, produce reliable local output, and eventually support single-binary distribution.

Rust provides:

* strong compile-time safety;
* explicit error handling;
* strong filesystem and process-control capabilities;
* good performance;
* low runtime overhead;
* reliable cross-platform binary potential;
* good fit for developer tooling;
* strong ecosystem for CLIs;
* good long-term maintainability for trust-critical local tools.

Monad’s core functions involve operations where correctness and safety matter:

* discovering repository roots;
* reading manifests;
* planning file writes;
* running commands;
* capturing outputs;
* producing verification evidence;
* avoiding destructive behavior.

Rust’s safety model is well aligned with these needs.

## Alternatives Considered

### TypeScript / Node.js

TypeScript would be faster for early development and familiar to the maintainer.

It has strong advantages:

* excellent CLI ecosystem;
* easy JSON/YAML/TOML handling;
* strong developer familiarity;
* easy integration with JavaScript projects;
* fast iteration.

It was not chosen for the core runtime because Monad’s durable local engine should prioritize:

* filesystem safety;
* reliable binary distribution;
* low runtime assumptions;
* long-term trust-critical operation;
* suitability for command execution and local tooling.

TypeScript remains appropriate for future surfaces, SDKs, integrations, web UI, and provider-specific adapters.

### Go

Go would be a reasonable choice for a local CLI and daemon.

It has advantages:

* easy static binaries;
* simple concurrency;
* strong tooling;
* fast builds;
* straightforward syntax.

It was not chosen because Rust offers stronger safety guarantees and better alignment with the long-term trust-critical local runtime.

### Python

Python would be fast for prototyping and strong for AI/data workflows.

It was not chosen for the core runtime because Monad should eventually be a reliable local binary without requiring Python environment management.

Python may still be useful for auxiliary scripts or future indexing experiments.

### Hybrid TypeScript-first approach

A TypeScript-first implementation could produce faster visible product progress, but it risks needing a later rewrite for the trust-critical core.

Monad should avoid building the wrong core twice.

## Consequences

### Positive Consequences

* Monad begins with a durable local systems foundation.
* File and command operations can be modeled with strong safety.
* Single-binary distribution remains plausible.
* Core logic can be reused by future interfaces.
* Rust strengthens Monad’s credibility as serious developer infrastructure.
* The system can grow into higher-assurance local workflows.

### Negative Consequences

* Development may be slower while the maintainer learns Rust.
* Some integrations may be easier in TypeScript than Rust.
* The Rust ecosystem requires careful dependency selection.
* Early implementation must avoid overcomplicated abstractions.

### Required Mitigations

Monad implementation must use Rust Apprenticeship Mode:

* small implementation slices;
* clear comments;
* complete file contents;
* tests with each slice;
* verification commands;
* beginner-readable explanations;
* explicit Rust concept teaching when new concepts appear.

## Implementation Notes

Initial implementation should create:

```text
Cargo.toml
crates/monad-cli/Cargo.toml
crates/monad-cli/src/main.rs
crates/monad-core/Cargo.toml
crates/monad-core/src/lib.rs
```

The first verification baseline should include:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

The CLI should remain thin.

Durable logic should move into `monad-core`.

## Related Documents

* `docs/01-project/01-charter/PRODUCT-CHARTER.md`
* `docs/05-architecture/SYSTEM-OVERVIEW.md`
* `docs/05-architecture/ARCHITECTURE-PRINCIPLES.md`
* `docs/05-architecture/MODULE-BOUNDARIES.md`
* `docs/10-engineering/RUST-CODING-STANDARD.md`

## Review / Supersession Notes

This ADR should be revisited only if implementation evidence shows that Rust is preventing Monad from delivering its core local value.

Future TypeScript, web, desktop, cloud, or integration layers do not supersede this ADR unless they replace the core runtime decision itself.
