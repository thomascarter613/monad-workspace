---
title: Release Smoke Tests
status: complete
epic: E16
---

# Release Smoke Tests

Run:

```bash
tools/scripts/verify-release.sh
```

This verifies:

- `monad release --dry-run` renders a readiness plan;
- `monad release --dry-run --format=json` renders JSON;
- release without `--dry-run` fails safely;
- no tags are created;
- no packages are published;
- package script syntax is valid.
