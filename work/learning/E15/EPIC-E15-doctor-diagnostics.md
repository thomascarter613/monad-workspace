---
title: Epic E15 Learning Note
epic: E15
---

# Epic E15 Learning Note: Doctor Diagnostics

E15 adds `monad doctor`.

The main concept is simple:

```text
doctor inspects; doctor does not repair
```

Doctor is a read-only readiness report for:

- PATH and environment basics;
- Git, Rust, and Cargo;
- optional ecosystem tools;
- Git repository state;
- Monad manifest/context state;
- sync evidence state.

## Why this matters

Before users run larger workflows, they need one command that explains what is ready and what is missing.

Doctor becomes the support and onboarding command for Monad.
