---
title: "ADR 0002: Use Monad as Unified Product Name"
document_type: "adr"
status: accepted
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - adr
  - naming
  - product
related:
  - docs/00-meta/NAMING-STANDARD.md
  - docs/01-project/00-vision/PRODUCT-VISION.md
  - docs/01-project/00-vision/HOLY-GRAIL-VISION.md
  - docs/01-project/01-charter/PRODUCT-CHARTER.md
---

# ADR 0002: Use Monad as Unified Product Name

## Status

Accepted.

## Context

The product vision evolved through several related names and concepts, including:

- AionX;
- Foundry;
- Charon;
- Context Bridge;
- repo-native memory;
- AI-guided SDLC;
- supervised repository execution;
- work packets;
- verification;
- safe evolution;
- agent supervision.

These concepts were valuable, but keeping them as separate product identities creates confusion.

The project needs one canonical product name.

## Decision

Use **Monad** as the unified product and umbrella name.

Monad is the canonical name for the complete system.

Former names may survive as internal concepts, historical references, subsystem inspirations, or implementation metaphors, but they are not competing product names.

## Rationale

A single product name improves:

- clarity;
- documentation;
- repository structure;
- GitHub issue organization;
- product positioning;
- future packaging;
- user understanding;
- AI-readable context;
- implementation focus.

Monad is broad enough to hold the full product vision:

- local-first developer tool;
- repo intelligence;
- context bridge;
- verification engine;
- evolution engine;
- supervised agents;
- future software foundry control plane.

## Naming Consequences

Use:

```text
Monad
````

as the product name in canonical docs.

Use:

```text
monad
```

for the repository name and CLI command.

Use crate/package names such as:

```text
monad-cli
monad-core
monad-mcp
```

Do not use AionX, Foundry, or Charon as competing product names.

## Former Names

### AionX

AionX was previously used as an umbrella concept for AI-guided SDLC, Foundry, Charon, repo-native memory, supervised execution, and continuous project evolution.

That umbrella meaning is now absorbed into Monad.

### Foundry

Foundry was previously used for monorepo generation, repo evolution, scaffolding, and software factory workflows.

Those ideas now become part of Monad’s Evolution Engine and broader Software Foundry OS direction.

### Charon

Charon was previously used for the context bridge concept.

Those ideas now become Monad’s Context Bridge capability.

### Mnemosyne, Argos, Anamnesis, Themis, Hephaestus, Athena, Clio, Daedalus

These names may remain useful as internal metaphors or future subsystem codenames, but they should not be introduced as separate products during MVP development.

## Alternatives Considered

### Keep AionX as the umbrella

This would preserve previous naming, but it is less directly connected to the current Rust CLI and repo tool direction.

### Keep Foundry as the product name

Foundry is descriptive for generators and evolution workflows, but it does not fully capture context, verification, agents, and repo intelligence.

### Keep Charon as the context product

Charon is strong for context handoff but too narrow for the unified system.

### Use multiple product names

This would preserve nuance but create avoidable confusion.

The project needs focus.

## Consequences

### Positive Consequences

* Documentation has one canonical name.
* The repository has one product identity.
* GitHub issues and epics are easier to organize.
* Product positioning becomes clearer.
* Future users encounter less naming confusion.
* AI assistants have one project name to anchor context.

### Negative Consequences

* Some prior conceptual names must be retired or demoted.
* Existing notes may need to be updated.
* Historical references may need clarification.
* Some subsystem names may be tempting to reintroduce prematurely.

### Required Mitigations

The docs should clarify that former concepts are absorbed into Monad.

Naming standards should prevent drift.

Future docs should use Monad as the product name unless explicitly discussing historical context.

## Implementation Notes

Update or create docs to use Monad as canonical name:

* product vision;
* product charter;
* documentation map;
* naming standard;
* GitHub issue titles;
* work packets;
* ADRs;
* README files.

Use `monad` for the CLI command:

```bash
monad --help
monad inspect
monad check
```

Use `monad-core` and `monad-cli` for initial Rust crates.

## Related Documents

* `docs/00-meta/NAMING-STANDARD.md`
* `docs/01-project/00-vision/PRODUCT-VISION.md`
* `docs/01-project/00-vision/HOLY-GRAIL-VISION.md`
* `docs/01-project/01-charter/PRODUCT-CHARTER.md`

## Review / Supersession Notes

This ADR should remain accepted unless the project undergoes a deliberate product rename.

Do not reintroduce AionX, Foundry, or Charon as separate product identities without a new ADR.
