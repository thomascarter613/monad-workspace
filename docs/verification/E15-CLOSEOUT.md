---
title: E15 Closeout
status: complete
epic: E15
---

# E15 Closeout — Doctor and Environment Diagnostics Foundation

E15 is complete when:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-doctor.sh
tools/scripts/verify-e15.sh
```

## Completed capability

```bash
monad doctor
monad doctor --format=json
```

## Safety retained

Doctor does not:

- install tools;
- mutate environment configuration;
- run package-manager installs;
- modify user source files;
- upload telemetry;
- call cloud services.

## Next epic

E16.
