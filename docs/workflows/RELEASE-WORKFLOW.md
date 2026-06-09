---
title: Release Workflow
status: complete
epic: E16
---

# Release Workflow

## 1. Verify first

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-e16.sh
```

## 2. Plan release readiness

```bash
monad release --dry-run
```

## 3. Build release binary

```bash
cargo build --release -p monad-cli
```

## 4. Package locally

```bash
tools/scripts/package-release.sh
```

## 5. Draft release manually

Use:

```text
docs/release/GITHUB-RELEASE-DRAFT.md
```

Do not publish until the evidence is reviewed.
