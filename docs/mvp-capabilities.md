---
title: "MVP Capabilities"
document_type: "capability-reference"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
tags:
  - monad
  - mvp
  - capabilities
---

# Monad Capabilities: Current Binary vs. First MVP Release Target

## Orientation

**Current state:** Monad is currently a working local-first Rust CLI foundation, but for our first release MVP we want it to be a full monorepo planner, initializea, anad evolver.

**Key distinction:** Current Monad can **inspect, verify, graph, plan, and generate context for an existing repo**. First MVP should also be able to **initialize or baseline a repo in a controlled way**.
**Important correction:** `monad init` is not currently implemented.

The current CLI enum includes `Help`, `Version`, `Info`, `Check`, `Inspect`, `Graph`, `Context`, `ContextGenerate`, `ContextPack`, `ContextVerify`, `EvolveVerifyBaseline`, `EvolveContextBaseline`, and `Plan`; there is no `Init` command in the parser today. 

## 1. What Monad can currently do

| Area                          | Current capability                                                 | Command                                        |
| ----------------------------- | ------------------------------------------------------------------ | ---------------------------------------------- |
| Basic CLI identity            | Show help/version/runtime identity.                                | `monad --help`, `monad version`                |
| Workspace summary             | Read `monad.toml` and summarize workspace/project identity.        | `monad info`                                   |
| Workspace checks              | Run Monad’s current repository checks and render text/JSON output. | `monad check`, `monad check --format=json`     |
| Repository inspection         | Inspect repository structure and summarize what it detects.        | `monad inspect`, `monad inspect --format=json` |
| Repository graphing           | Build/render a repository graph in supported formats.              | `monad graph --format text/json/mermaid/dot`   |
| Context rendering/export      | Render or export AI-readable repository context pack data.         | `monad context`, `monad context --write`       |
| Context current state         | Generate `.monad/context/current-state.md`.                        | `monad context generate current-state`         |
| Context handoff               | Generate `.monad/context/latest-handoff.md`.                       | `monad context generate handoff`               |
| Bootstrap prompt              | Generate `docs/ai/BOOTSTRAP-PROMPT.md`.                            | `monad context generate bootstrap`             |
| Context pack                  | Assemble/write `.monad/context/latest-context-pack.md`.            | `monad context pack`                           |
| Context verification          | Verify context files structurally.                                 | `monad context verify`                         |
| Safe evolution preview        | Preview verify-baseline changes, but only in dry-run mode.         | `monad evolve verify-baseline --dry-run`       |
| Safe context baseline preview | Preview context-baseline changes, but only in dry-run mode.        | `monad evolve context-baseline --dry-run`      |
| AI-style planning             | Produce a supervised plan from a user intent.                      | `monad plan "explain this repo"`               |

The current CLI explicitly parses `context generate current-state`, `context generate handoff`, and `context generate bootstrap`, plus context pack/verify and dry-run evolve commands.  The `evolve` commands currently require `--dry-run`; write/apply behavior is intentionally not implemented yet. 

## 2. What Monad cannot currently do yet, but what we want ASAP in the first release MVP.

| Missing capability                          | Current status                                                           |
| ------------------------------------------- | ------------------------------------------------------------------------ |
| `monad init`                                | Not implemented.                                                         |
| Scaffold a new monorepo from nothing        | Not implemented.                                                         |
| Add apps/packages/services from templates   | Not implemented.                                                         |
| Write repo changes from evolve commands     | Not implemented; dry-run only.                                           |
| Apply patches                               | Not implemented.                                                         |
| Manage Git branches/worktrees automatically | Not implemented.                                                         |
| Publish packages                            | Not implemented.                                                         |
| Generate installers                         | Not implemented.                                                         |
| Run autonomous agents                       | Not implemented and should not be claimed.                               |
| MCP server public release                   | Not implemented as a public release capability.                          |
| Plugin/provider ecosystem                   | Not implemented.                                                         |
| Cloud/SaaS/hosted mode                      | Not implemented.                                                         |
| Full polyglot package-manager orchestration | Not implemented.                                                         |
| Cross-language task runner                  | Not implemented.                                                         |
| Release/download channel                    | Only beginning via build artifact workflow, not a formal release system. |

## 3. What Monad should be able to do at the first MVP release

I would split the first MVP release into **must-have**, **should-have**, and **explicitly not MVP**.

## Must-have for first MVP

| Area                                       | First MVP target                                                                                                        |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------- |
| Install/run locally                        | User can download/build/run a `monad` binary locally.                                                                   |
| Project identity                           | `monad info` reliably reads `monad.toml`.                                                                               |
| Repository inspection                      | `monad inspect` explains an existing repository clearly.                                                                |
| Toolchain detection                        | Detect common repo signals for Rust, Node/JavaScript, and other early supported ecosystems.                             |
| Graph output                               | `monad graph` renders useful text/JSON plus Mermaid/DOT if already supported.                                           |
| Verification                               | `monad check` produces clear pass/fail/warn evidence.                                                                   |
| Context generation                         | `monad context generate current-state`, `handoff`, `bootstrap`, and `context pack` produce durable repo-native context. |
| Context verification                       | `monad context verify` validates generated context structure.                                                           |
| Safe dry-run evolution                     | `monad evolve verify-baseline --dry-run` and `context-baseline --dry-run` preview changes without writing.              |
| No-write guarantee                         | Any command that could mutate must be obviously dry-run or explicitly blocked.                                          |
| Accurate docs                              | README/docs must not claim unimplemented features.                                                                      |
| Release hygiene                            | License, security policy, contributing guide, issue/PR templates, and public pre-release checklist exist.               |
| Binary artifact                            | A downloadable Linux binary artifact exists for testing.                                                                |
| Public release boundary                    | The project clearly states whether the release is source-only, binary-only, or package-published.                       |
| Full polyglot build orchestration          | Too broad for first MVP.                                                                                                |
| Full task runner replacement               | Monad should coordinate native tools first.                                                                             |
| Dependency manager replacement             | Not MVP; use native tools.                                                                                              |
| Installer generation                       | Nice-to-have after release boundary is proven.                                                                          |
| Enterprise governance mode                 | Post-MVP.                                                                                                               |

E7’s purpose was to stabilize the E0–E6 foundation into a coherent local-first CLI MVP, with a focus on command behavior, verification, documentation, repo contracts, safety guarantees, and release readiness. 

## Should-have for first MVP, but can be narrowly scoped

| Area                         | First MVP target                                                                                                    |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `monad init`                 | A minimal initializer that creates `monad.toml`, `.monad/context/`, docs stubs, `work/`, and verification baseline. |
| Init dry-run                 | `monad init --dry-run` shows what would be created before writing.                                                  |
| Init no-overwrite protection | `monad init` refuses to overwrite existing files unless a future approved write mode exists.                        |
| Presets                      | At least one preset: `basic` or `polyglot-minimal`.                                                                 |
| Generated artifact policy    | Generated/vendor/imported content is clearly ignored or bounded.                                                    |
| Release notes                | Public release notes distinguish implemented, deferred, and not-implemented capabilities.                           |

My recommendation: **promote a minimal `monad init --dry-run` plus guarded `monad init` to the first public MVP**. Without it, Monad is useful as a repo inspector/context tool, but it does not yet satisfy the “monorepo runtime that can start a repo” expectation.

## Explicitly not first MVP

| Capability                                 | Why not MVP                                               |
| ------------------------------------------ | --------------------------------------------------------- |
| Autonomous agent execution                 | Too risky and outside current safety boundary.            |
| Apply/write evolution beyond approved init | Needs stronger approval gates.                            |
| Plugin marketplace                         | Post-MVP ecosystem feature.                               |
| Hosted SaaS/control plane                  | Not needed for local-first MVP.                           |
| Crates.io/npm/homebrew publishing          | Useful later, but source/binary artifact is enough first. |

WP-E9-006 explicitly kept publishing, public tag creation, package publication, installer generation, and hosted launch out of scope for the current boundary decision. 

## 4. Best first MVP capability statement

The first MVP should truthfully be:

> Monad is a local-first Rust CLI for understanding, verifying, documenting, and safely preparing software repositories for structured evolution. It can inspect a repository, summarize its structure, generate repo-native context, run verification checks, preview baseline improvements, and initialize a minimal Monad-managed monorepo without overwriting existing work.

That last clause, **“initialize a minimal Monad-managed monorepo”**, is the missing piece.

## 5. Recommended next epic after E10

```text
E11 — Init Command and Monorepo Scaffold Foundation
```

Recommended work packets:

| Work Packet | Name                                           |
| ----------- | ---------------------------------------------- |
| WP-E11-001  | Define `monad init` UX and safety contract     |
| WP-E11-002  | Add init dry-run plan                          |
| WP-E11-003  | Add minimal embedded scaffold templates        |
| WP-E11-004  | Add guarded init write path                    |
| WP-E11-005  | Add basic/polyglot-minimal preset              |
| WP-E11-006  | Add init smoke tests and verification evidence |

The first version of `monad init` should create only the foundational skeleton:

```text
monad.toml
README.md
docs/
work/
.monad/context/
tools/scripts/verify.sh
.github/workflows/ci.yml
```

It should try to generate a full enterprise-grade polyglot monorepo on the first MVP release
