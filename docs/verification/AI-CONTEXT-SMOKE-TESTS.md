---
title: AI Context Smoke Tests
status: complete
epic: E18
---

# AI Context Smoke Tests

Run:

```bash
tools/scripts/verify-ai-context.sh
```

This verifies:

- dry-run writes no files;
- JSON dry-run works;
- generated local artifacts are written with `--yes`;
- no provider calls are made;
- unsafe overwrites are refused.
