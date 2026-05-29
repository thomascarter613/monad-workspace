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
work_packet: WP-E9-002
---

# Generated Artifact Policy

## 1. Purpose

This policy defines how Monad treats generated, imported, vendored, and operational artifacts.

The goal is to preserve useful evidence and reference material without allowing generated or imported files to masquerade as first-party source code, first-party documentation records, or first-party dependency surfaces.

## 2. Policy

Generated, imported, vendored, and operational artifacts must be explicitly bounded.

They may be retained only when they are useful, reviewable, and do not destabilize the repository’s verification, dependency scanning, or release-readiness posture.

## 3. First-party records

First-party records include hand-authored or intentionally maintained Monad files such as:

- source code
- tests
- architecture documents
- release documents
- work packets
- context handoffs
- ADRs
- verification scripts
- repository policy documents

First-party Markdown records should carry YAML frontmatter when they live under documentation, work, or context record trees.

## 4. Generated and imported artifacts

Generated and imported artifacts include:

- tool output
- report output
- local audit logs
- generated context scratch output
- imported DeepWiki dumps
- vendored package metadata
- dependency trees
- copied third-party Markdown

These artifacts must not be allowed to break first-party documentation checks.

## 5. DeepWiki imports

DeepWiki Markdown exports may be retained as reference documentation when useful.

DeepWiki tooling metadata and dependency trees must not be committed as first-party repository dependencies.

Do not commit the following under `docs/wiki/`:

- `node_modules/`
- `package.json`
- `package-lock.json`
- `pnpm-lock.yaml`
- `bun.lockb`
- `yarn.lock`

## 6. Security rationale

Dependency scanners treat committed package manifests as active dependency surfaces.

If an imported documentation dump contains a `package.json`, Dependabot may correctly flag vulnerabilities even though the package is not part of Monad's runtime. To avoid false public-readiness blockers, imported package metadata should be removed or quarantined outside the committed first-party docs tree.

## 7. Verification rationale

The root verifier should check first-party records and ignore generated or vendored artifacts.

This policy does not weaken first-party verification. It clarifies what is and is not first-party.

## 8. Required verifier behavior

Verification scripts should:

- check first-party Markdown frontmatter
- ignore generated `.artifacts` paths
- ignore vendored `node_modules` paths
- ignore generated `.monad/reports` paths
- ignore generated `.monad/context/generated` paths
- preserve strict checks for first-party docs, work records, and context records

## 9. Release-readiness requirement

Before any public pre-release, generated/imported artifact policy must be enforced so that:

- dependency scanning reflects real first-party dependency surfaces
- documentation checks are not polluted by imported third-party Markdown
- generated reports do not create false verification failures
- security alerts are triaged against the correct ownership boundary
