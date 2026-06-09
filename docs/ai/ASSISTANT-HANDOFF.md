---
title: Assistant Handoff
status: complete
epic: E18
---

# Assistant Handoff

E18 assistant handoff is supervised and provider-agnostic.

The generated handoff file gives a human-reviewed assistant enough context to help without relying on fragile chat history.

Use:

```bash
monad ai-context --yes
```

Then review:

```text
.monad/context/assistant-handoff.md
```

Before sharing with any assistant, remove anything private or sensitive.
