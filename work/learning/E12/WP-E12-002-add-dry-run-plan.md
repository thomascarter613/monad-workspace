---
title: "Learning Note — WP-E12-002 Add Dry-Run Plan"
document_type: "learning-note"
status: active
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-002
tags:
  - learning
  - rust
  - cli
  - dry-run
  - monad
---

# Learning Note — WP-E12-002 Add Dry-Run Plan

## What You Are Building

You are adding the first implementation layer for:

```bash
monad add app web --dry-run
```

This command does not create files yet.

It previews what Monad would create later.

## Mental Model

The flow is:

```text
CLI arguments
  -> parse command
  -> validate component kind
  -> validate component name
  -> build file operation plan
  -> evaluate plan against the filesystem
  -> render dry-run output
```

## Main Rust File to Read First

Read:

```text
crates/monad-core/src/component_add.rs
```

Start with these types:

```text
ComponentKind
ComponentName
AddPlanOptions
```

Then read:

```text
build_add_plan
render_add_dry_run
```

## What `ComponentKind` Teaches

`ComponentKind` is an enum.

An enum is a type that can be one of several named variants:

```text
App
Package
Service
Tool
```

The enum maps user words to known component families.

## What `ComponentName` Teaches

`ComponentName` is a wrapper around `String`.

It exists so unsafe names are rejected once at the boundary.

That means the rest of the code can trust that the name is safe.

## What `AddPlanOptions` Teaches

`AddPlanOptions` is a small data object.

It carries:

```text
kind
name
```

It also knows how to compute:

```text
apps/web
packages/shared-ui
services/api
tools/repo-lint
```

## What `build_add_plan` Teaches

`build_add_plan` does not touch the filesystem.

It only builds an in-memory plan:

```text
would-create apps/web/README.md
would-create apps/web/.gitkeep
```

This is the same pattern used for `init`.

## What `render_add_dry_run` Teaches

`render_add_dry_run` evaluates the plan against the current workspace.

That is what lets Monad say:

```text
would-create
conflict
no-op
```

without writing files.

## What to Inspect After Running the Script

Run:

```bash
git diff -- crates/monad-core/src/component_add.rs
git diff -- crates/monad-cli/src/main.rs
```

Then ask yourself:

1. Where does user input enter?
2. Where is the input validated?
3. Where is the file plan built?
4. Where is the filesystem checked?
5. Where is text rendered for the user?

## Verification Commands

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- add app web --dry-run
cargo run -p monad-cli -- add service api --dry-run
cargo run -p monad-cli -- add app ../bad --dry-run
```

The last command should fail.


## Fix Note — Vec Versus Array

The first version of this work packet used:

```rust
FileOperationPlan::from_operations([
    operation_one,
    operation_two,
])
```

That creates a fixed-size Rust array.

The existing Monad API expects:

```rust
Vec<PlannedFileOperation>
```

So the correct code is:

```rust
FileOperationPlan::from_operations(vec![
    operation_one,
    operation_two,
])
```

The `vec![]` macro creates a growable vector.

This is a common Rust distinction:

```text
[T; N]  = fixed-size array with exactly N items
Vec<T>  = growable heap-allocated list
```

In this repo, `FileOperationPlan::from_operations` wants ownership of a `Vec` because plans may contain any number of operations.


## Fix Note — Testing Safety Properties, Not Brittle Strings

The first test for `monad add app web --yes` expected one exact error phrase.

That was too brittle.

The important behavior is not the exact sentence.

The important safety property is that `monad add` remains dry-run only in WP-E12-002.

The improved test checks that the command fails and that the error mentions the add/write-safety boundary.

This makes the test less fragile while still protecting the safety rule.


## Fix Note — Parser Validation Order

The `monad add app web --yes` parser test revealed a useful parser-design detail.

The command fails before the parser reaches the `add` command arm because `--yes` is currently globally allowed only for `init`.

That means the actual error is:

```text
--yes is only supported for init command
```

For WP-E12-002, this is still correct because `monad add` has no write path yet.

The test should therefore verify the safety boundary:

```text
--yes is rejected for monad add
```

It should not require the parser to reach the `add` command-specific error path.


## Fix Note — Rust Items Before Test Modules

Clippy reported `items_after_test_module` because a public export was appended after `#[cfg(test)] mod tests` in `lib.rs`.

The Rust convention is:

```text
module declarations
public exports
normal code
test module at the bottom
```

The fix moves:

```rust
pub use component_add::{...};
```

above the test module.

This does not change runtime behavior. It only restores normal Rust file organization.
