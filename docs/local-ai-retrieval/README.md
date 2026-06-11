# Local AI Retrieval and Vector Memory

E32 adds Monad's local AI retrieval and vector memory foundation.

## Command surface

```bash
monad retrieval-plan --dry-run
monad retrieval-plan --dry-run --format=json
monad retrieval-plan --yes
monad local-retrieval --dry-run
monad vector-memory --dry-run
```

## Safety boundaries

This foundation does **not** call AI model providers, contact vector databases,
access networks, start background indexers, invoke package managers, or mutate
user-owned source files.
