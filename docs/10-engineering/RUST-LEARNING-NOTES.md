---
title: "Rust Learning Notes"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - engineering
  - rust
  - learning
related:
  - docs/10-engineering/RUST-CODING-STANDARD.md
  - docs/10-engineering/RUST-VERIFICATION.md
  - docs/06-adrs/ADR-0001-use-rust-for-core-runtime.md
---

# Rust Learning Notes

## Purpose

This document records Rust concepts that matter while building Monad.

Monad should be a production-minded Rust project, but the maintainer is learning Rust during implementation. These notes help connect Rust syntax and concepts to the actual Monad codebase.

## Learning Rule

Every major Rust concept should be introduced when it becomes useful.

Avoid learning Rust as abstract theory disconnected from the project.

## The Initial Mental Model

Rust code is organized around:

- packages;
- crates;
- modules;
- functions;
- structs;
- enums;
- traits;
- implementations;
- tests.

Monad begins with a workspace containing multiple crates.

## Package

A package is defined by a `Cargo.toml`.

Example:

```text
crates/monad-core/Cargo.toml
```

A package can contain a library crate, binary crate, or both.

## Crate

A crate is a compilation unit.

Monad initially uses:

```text
monad-cli   binary crate
monad-core  library crate
```

`monad-cli` produces the executable.

`monad-core` provides reusable logic.

## Module

A module organizes code inside a crate.

Example:

```rust
pub mod diagnostics;
```

This tells Rust that the crate has a module named `diagnostics`.

Depending on file layout, Rust expects either:

```text
src/diagnostics.rs
```

or:

```text
src/diagnostics/mod.rs
```

## `pub`

`pub` means public.

Without `pub`, items are private to the module.

Example:

```rust
pub struct Diagnostic {
    pub message: String,
}
```

This means other modules can use `Diagnostic` and read its `message`.

## `struct`

A struct groups named data.

Example:

```rust
pub struct Diagnostic {
    pub message: String,
}
```

This means a diagnostic has a message.

Structs are useful for Monad concepts such as:

- workspace context;
- diagnostic;
- check result;
- command output;
- file operation.

## `enum`

An enum represents one value from a fixed set of possibilities.

Example:

```rust
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}
```

Enums are useful for Monad concepts such as:

- diagnostic severity;
- check status;
- file operation kind;
- output format.

## `impl`

An `impl` block defines methods for a type.

Example:

```rust
impl Diagnostic {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
```

`Self` means the type currently being implemented.

## Ownership

Rust has ownership rules to manage memory safely without a garbage collector.

A value has one owner.

When the owner goes out of scope, the value is dropped.

This matters for Monad because file paths, strings, command outputs, and parsed data must be passed safely between functions.

## Borrowing

Borrowing lets code use a value without taking ownership.

Example:

```rust
fn print_message(message: &str) {
    println!("{message}");
}
```

`&str` means borrowed string slice.

The function can read the message but does not own it.

## `String` vs `&str`

`String` is an owned growable string.

`&str` is a borrowed string slice.

Use `String` when the struct needs to own the text.

Use `&str` when a function only needs to read text temporarily.

## `PathBuf` vs `&Path`

`PathBuf` is an owned filesystem path.

`&Path` is a borrowed filesystem path.

Use `PathBuf` in structs that store paths.

Use `&Path` in functions that read a path temporarily.

## `Result`

`Result` represents success or failure.

```rust
Result<T, E>
```

means either:

```text
Ok(T)
Err(E)
```

Monad should use `Result` for fallible operations such as:

- reading files;
- parsing manifests;
- resolving workspace roots;
- running commands;
- writing reports.

## The `?` Operator

The `?` operator returns early if a `Result` is an error.

Example:

```rust
let text = std::fs::read_to_string(path)?;
```

If reading succeeds, `text` gets the file contents.

If reading fails, the function returns the error.

## Tests

Rust tests can live in the same file.

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_diagnostic() {
        let diagnostic = Diagnostic::new("hello".to_string());
        assert_eq!(diagnostic.message, "hello");
    }
}
```

`#[cfg(test)]` means the module only compiles during tests.

`use super::*;` imports items from the parent module.

## Cargo Commands

Common commands:

```bash
cargo fmt
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- --help
```

## Learning Priorities for Monad

Learn these concepts as they appear:

1. Workspace and crates.
2. Modules and exports.
3. Structs.
4. Enums.
5. `impl` methods.
6. Ownership and borrowing.
7. `Result` and error handling.
8. Tests.
9. Traits.
10. Filesystem paths.
11. Command execution.
12. Serialization.
13. Lifetimes only when needed.
14. Generics only when needed.
15. Async only when justified.

## Avoid Early Complexity

Avoid advanced Rust patterns until they are needed:

- complex lifetimes;
- macro-heavy APIs;
- deeply generic abstractions;
- async everywhere;
- trait object architecture before concrete types are known;
- premature crate splitting.

## Current Status

This Rust learning notes document is a draft. It should be updated as implementation introduces new Rust concepts.
