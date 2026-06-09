---
title: Language Adapter Foundation
status: active
owner: Monad
last_reviewed: 2026-06-09
---

# Language Adapter Foundation

E23 introduces Monad's first language adapter registry.

Adapters are local, deterministic descriptions of language ecosystems. They tell
Monad how to recognize a language and which ecosystem-native commands may be
suggested in supervised plans. They do **not** execute commands by themselves.

## Initial adapters

- Rust / Cargo
- Node/Bun
- Python
- Go
- Java / Gradle / Maven

## Safety boundary

The E23 registry is planning and evidence only:

- no tool installation;
- no package-manager invocation;
- no arbitrary command execution;
- no remote registry calls;
- no AI provider calls;
- no user-owned source rewrites;
- no autonomous agent execution.

Generated evidence may be written under `.monad/reports` only when the caller
uses `monad adapters --yes`.
