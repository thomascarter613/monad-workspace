---
title: GitHub Release Draft Process
status: complete
epic: E16
---

# GitHub Release Draft Process

This is a bounded manual process for the first Monad release foundation.

## Preflight

```bash
monad release --dry-run
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-e16.sh
```

## Local artifact

```bash
cargo build --release -p monad-cli
tools/scripts/package-release.sh
```

## Draft contents

Include:

- version;
- tag;
- target platform;
- verification evidence;
- checksum;
- implemented features;
- deferred features;
- not implemented features;
- known risks.

## Explicit non-claims

Do not claim:

- Crates.io availability;
- Homebrew availability;
- npm availability;
- installer availability;
- hosted/SaaS launch;
- signed artifacts;
- SBOM availability;

unless those features are actually implemented and verified.
