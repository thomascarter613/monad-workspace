---
title: Policy Smoke Tests
status: complete
epic: E19
---

# Policy Smoke Tests

Run:

```bash
tools/scripts/verify-policy.sh
```

This verifies:

- policy dry-run output;
- policy JSON output;
- generated policy evidence writes;
- dry-run writes no files;
- blocked and approval-required examples are represented.
