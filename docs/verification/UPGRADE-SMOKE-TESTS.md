---
title: Upgrade Smoke Tests
status: complete
epic: E17
---

# Upgrade Smoke Tests

Run:

```bash
tools/scripts/verify-upgrade.sh
```

This verifies:

- upgrade dry-run writes no files;
- JSON dry-run works;
- guarded apply writes generated metadata/evidence only;
- missing manifest blocks guarded apply;
- unsafe overwrite conflicts are refused.
