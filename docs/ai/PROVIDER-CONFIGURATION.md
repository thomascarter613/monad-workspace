---
title: AI Provider Configuration
status: complete
epic: E18
---

# AI Provider Configuration

AI usage in Monad is optional and provider-agnostic.

Supported modes:

- disabled
- local
- self-hosted
- hosted

Provider config must not store secrets directly in committed files.

Secrets should live outside repo-committed config, such as:

- shell environment;
- local secret manager;
- untracked local file;
- organization-managed secret store.

E18 does not call providers.
