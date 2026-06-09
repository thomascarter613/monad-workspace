---
title: Sync Smoke Tests
status: complete
epic: E14
---

# Sync Smoke Tests

Run:

```bash
tools/scripts/verify-sync.sh
```

This verifies:

- `monad sync --dry-run` writes no sync evidence;
- `monad sync --dry-run --format=json` renders JSON;
- `monad sync --yes` writes only generated sync evidence reports;
- missing command mode fails safely.

Full E14 verification:

```bash
tools/scripts/verify-e14.sh
```
