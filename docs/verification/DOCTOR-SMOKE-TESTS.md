---
title: Doctor Smoke Tests
status: complete
epic: E15
---

# Doctor Smoke Tests

Run:

```bash
tools/scripts/verify-doctor.sh
```

This verifies:

- `monad doctor` renders a text report;
- `monad doctor --format=json` renders JSON;
- doctor states the non-mutating safety contract;
- doctor works in an uninitialized directory;
- doctor does not create Monad files, lockfiles, or package-manager artifacts.

Full E15 verification:

```bash
tools/scripts/verify-e15.sh
```
