---
title: Generated Artifact Policy
description: Policy for generated, imported, vendored, and operational artifacts in the Monad repository.
status: draft
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
phase: Post-MVP Candidate Stabilization
epic: E9
work_packet: WP-E9-001
---

# Generated Artifact Policy

## 1. Purpose

This policy defines how Monad treats generated, imported, vendored, and operational artifacts.

## 2. Policy

Generated or imported artifacts must not be allowed to masquerade as first-party source code, first-party dependencies, or first-party documentation records.

## 3. DeepWiki imports

DeepWiki Markdown exports may be retained as reference documentation when useful.

DeepWiki tooling metadata and dependency trees must not be committed as first-party repository dependencies.

Do not commit the following under `docs/wiki/`:

- `node_modules/`
- `package.json`
- `package-lock.json`
- `pnpm-lock.yaml`
- `bun.lockb`
- `yarn.lock`

## 4. Security rationale

Dependency scanners treat committed package manifests as active dependency surfaces.

If an imported documentation dump contains a `package.json`, Dependabot may correctly flag vulnerabilities even though the package is not part of Monad's runtime. To avoid false public-readiness blockers, imported package metadata should be removed or quarantined outside the committed first-party docs tree.

## 5. Verification expectation

The root verifier should check first-party records and ignore generated or vendored artifacts.

This policy does not weaken first-party verification. It clarifies what is and is not first-party.
