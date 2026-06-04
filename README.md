# Monad

Monad is a Rust-first, repo-native, local-first developer tool being built toward an AI-native Software Foundry OS for understanding, verifying, and safely evolving software repositories.

The current implementation helps repositories become easier to inspect, explain, verify, document, hand off, and improve without overstating future capabilities that are not implemented yet.

## Current Status

Monad is currently in public pre-release hardening.

The repository has progressed beyond the initial project foundation phase into an internal MVP-candidate/public-readiness track. The current focus is:

1. keep public-facing claims aligned with implemented capability;
2. preserve clear release and safety boundaries;
3. verify the current command surface;
4. document source-only versus packaged pre-release posture;
5. prepare truthful public pre-release notes;
6. explicitly decide whether a public pre-release tag should be cut.

## Product Thesis

Modern software development is not blocked only by the speed of writing code.

It is blocked by scattered context, fragile verification, toolchain complexity, architectural drift, stale documentation, repeated setup work, and low trust in generated changes.

Monad exists to help solve those problems by combining:

- repository intelligence;
- repo-native context;
- verification evidence;
- safe file operations;
- native tool coordination;
- supervised AI assistance;
- durable project memory;
- human-in-command workflows.

## Source of Truth

The repository is the canonical source of truth.

Durable product, architecture, workflow, implementation, and context decisions should live in repository files, especially:

```text
docs/
work/
.monad/
docs/06-adrs/
```

External systems such as GitHub Issues, GitHub Projects, and private planning tools may support the workflow, but accepted decisions must be promoted into repository docs, ADRs, source code, or committed context artifacts.

## Initial Repository Layout

```text
.
├── docs/                 Canonical project documentation
├── work/                 Epics, work packets, tasks, and delivery records
├── .monad/               Monad-maintained local/generated state, context, and reports
├── Cargo.toml            Rust workspace manifest
├── rust-toolchain.toml   Rust toolchain policy
├── .editorconfig         Editor formatting baseline
├── .gitignore            Git ignore rules
├── LICENSE               Project license
└── README.md             Project entrypoint
```

## Recommended Reading Order

For a new human or AI-assisted session, read:

```text
docs/09-ai/BOOTSTRAP-PROMPT.md
docs/09-ai/FRESH-CHAT-HANDOFF.md
docs/01-project/01-charter/PRODUCT-CHARTER.md
docs/01-project/00-vision/PRODUCT-VISION.md
docs/02-product/MVP-SCOPE.md
docs/05-architecture/SYSTEM-OVERVIEW.md
docs/05-architecture/ARCHITECTURE-PRINCIPLES.md
docs/05-architecture/MODULE-BOUNDARIES.md
docs/06-adrs/README.md
docs/07-workflow/OPERATING-MODEL.md
docs/07-workflow/WORK-PACKET-STANDARD.md
```

## Foundation and Public-Readiness Epics

```text
E0 — Project Foundation
E1 — Rust Core Foundation
E2 — Repo Intelligence
E3 — Context Bridge
E4 — Verification Engine
E5 — Evolution Engine
E6 — Agent Supervision
E7 — MVP Hardening
E8 — MVP Candidate Cut and Release Preparation
E9 — Post-MVP Candidate Stabilization and Public-Readiness Gap Closure
E10 — Public Pre-Release Hardening and Boundary Enforcement
```

## Implemented MVP Command Surface

The current implemented local command surface is:

```text
monad --help
monad --version
monad help
monad version
monad info
monad info --format=json
monad inspect
monad inspect --format=json
monad check
monad check --format=json
monad graph
monad graph --format=json
monad graph --format=mermaid
monad graph --format=dot
monad context
monad context --format=json
monad context --write
monad context generate current-state
monad context generate handoff
monad context generate bootstrap
monad context pack
monad context verify
monad evolve verify-baseline --dry-run
monad evolve context-baseline --dry-run
monad plan "<intent>"
```

The following command families are future roadmap work and should not be treated as implemented yet:

```text
monad init
monad add
monad run
monad sync
monad doctor
monad release
monad upgrade
monad patch
monad apply
```

Safety boundary: `plan` is no-write, `evolve` commands are dry-run only, and write behavior is currently limited to explicit context export/generation commands.

## Development Prerequisites

Initial local development requires:

```text
git
rustup
cargo
rustfmt
clippy
python3
```

GitHub issue/project automation also uses:

```text
gh
```

After the Rust crates are created, the standard Rust verification command set will be:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

During documentation-only setup, verify docs with:

```bash
find docs -type f | sort
```

and the frontmatter verification script described in the documentation standards.

## License

Monad is licensed under the Apache License, Version 2.0.

See:

```text
LICENSE
```

## Local development

Monad is currently preparing for an internal MVP candidate cut.

Local development documentation:

- [Local Build Guide](docs/development/LOCAL-BUILD.md)
- [Local Verification Guide](docs/development/LOCAL-VERIFY.md)
- [MVP Command Reference](docs/project/MVP-COMMAND-REFERENCE.md)
- [MVP Scope Freeze](docs/project/MVP-SCOPE-FREEZE.md)

Run the CLI locally through Cargo:

```bash
cargo run -p monad-cli -- --help
cargo run -p monad-cli -- version
cargo run -p monad-cli -- inspect
```

Run the baseline verification set:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

Monad is not yet a public release, published package, installer distribution, hosted service, or autonomous agent runtime.
