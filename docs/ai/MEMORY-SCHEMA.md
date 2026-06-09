---
title: AI Memory Schema
status: complete
epic: E18
---

# AI Memory Schema

Monad memory records are Markdown files with frontmatter.

## Kinds

- decision
- preference
- constraint
- status
- question

## Required frontmatter

```yaml
id: memory-0001
kind: decision
title: Example decision
source: generated:E18
freshness: current
```

## Safety

Do not store secrets.

Review memory before exporting context to any assistant or provider.
