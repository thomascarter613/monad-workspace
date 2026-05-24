---
title: "Latest Context Pack"
document_type: "context-pack"
status: "current"
version: "1.6.0"
created: "2026-05-23"
updated: "2026-05-24"
owner: "Monad Project"
epic: "E1"
work_packet: "WP-E1-007"
tags:

* context-pack
* e1
* runtime-foundation
* cli-info

---

# Latest Context Pack

## Identity

Monad is an AI-native, repo-native, local-first Software Foundry OS.

## Completed

E0 — Project Foundation is complete.

WP-E1-001 — Establish Rust Workspace Runtime Foundation is complete.

WP-E1-002 — Establish Core Diagnostics Foundation is complete.

WP-E1-003 — Establish Core Error Foundation is complete.

WP-E1-004 — Establish Workspace Context Foundation is complete.

WP-E1-005 — Establish Manifest Model Foundation is complete.

WP-E1-006 — Establish Manifest Loading Foundation is complete.

## Current Epic

E1 — Runtime Foundation

## Current Work Packet

WP-E1-007 — Establish CLI Info Command Foundation

## Runtime Foundation State

The Rust workspace contains:

```text
crates/
  monad-cli/
  monad-core/
```

Manifest Loading added:

* `serde`;
* `toml`;
* `MonadManifest::from_toml_str`;
* `MonadManifest::load_from_path`;
* `MonadManifest::load_from_workspace`.

CLI Info adds:

* early command parsing;
* `monad help`;
* `monad info`;
* workspace discovery from current directory;
* rendering loaded manifest state.

## Verification

Run:

```bash
cargo fmt --check
cargo test
cargo run --quiet -p monad-cli -- info
tools/scripts/verify.sh
```

