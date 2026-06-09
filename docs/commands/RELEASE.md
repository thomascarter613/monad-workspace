---
title: monad release
status: complete
epic: E16
---

# `monad release`

`monad release` prepares an evidence-backed release readiness plan.

## Commands

```bash
monad release --dry-run
monad release --dry-run --format=json
monad release --dry-run --release-version=0.1.0
```

## Safety contract

E16 release planning is dry-run only.

It does not:

- create Git tags;
- push Git tags;
- publish to Crates.io;
- publish to npm;
- publish to Homebrew;
- create installers;
- create GitHub releases automatically;
- upload artifacts;
- launch hosted services.

## Release readiness checks

The release plan checks:

- version shape;
- release tag shape;
- local tag conflict;
- verification script presence;
- local release binary presence;
- packaging script presence;
- release notes presence;
- changelog presence;
- GitHub release draft workflow or documentation presence;
- explicit publishing boundaries.

## Packaging

Use the bounded local packaging script:

```bash
tools/scripts/package-release.sh
```

This creates local artifacts and checksums only. It does not publish.
