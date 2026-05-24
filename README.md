# Monad

Monad is an AI-native, repo-native, local-first Software Foundry OS for understanding, verifying, and safely evolving software repositories.

Monad is being built as a Rust-first developer tool that helps repositories become easier to inspect, explain, verify, document, hand off, and improve.

## Current Status

Monad is currently in the project foundation phase.

The initial focus is:

1. establish the repository foundation;
2. establish the documentation architecture;
3. establish the context bridge foundation;
4. establish workflow standards;
5. establish the initial product canon;
6. then begin the Rust core foundation.

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

## Planned MVP Epics

```text
E0 — Project Foundation
E1 — Rust Core Foundation
E2 — Repo Intelligence
E3 — Context Bridge
E4 — Verification Engine
E5 — Evolution Engine
E6 — Agent Supervision
```

## Planned MVP Commands

The MVP roadmap targets these commands:

```text
monad --help
monad --version
monad info
monad inspect
monad graph
monad context generate
monad context verify
monad check
monad evolve verify-baseline --dry-run
monad evolve context-baseline --dry-run
monad plan "<intent>"
```

Most commands do not exist yet. They will be implemented through the MVP work packets.

## Local Development

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
