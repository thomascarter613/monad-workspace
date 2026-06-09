---
title: AI Context Workflow
status: complete
epic: E18
---

# AI Context Workflow

## 1. Preview

```bash
monad ai-context --dry-run
```

## 2. Preview as JSON

```bash
monad ai-context --dry-run --format=json
```

## 3. Generate local artifacts

```bash
monad ai-context --yes
```

## 4. Review before sharing

Review:

```text
.monad/context/assistant-handoff.md
.monad/context/ai-context-snapshot.md
.monad/ai/memory/
```

Do not share secrets or private data.
