---
name: Bug report
about: Report a reproducible Monad bug
title: "bug: "
labels: bug
assignees: ""
---

## Summary

Describe the bug clearly.

## Environment

- OS:
- Shell:
- Rust version:
- Monad commit or version:

## Steps to reproduce

1.
2.
3.

## Expected behavior

What should happen?

## Actual behavior

What happened instead?

## Verification attempted

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

## Additional context

Add logs, screenshots, or notes.
