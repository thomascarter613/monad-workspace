---
title: WP-E13-002 Learning Note
epic: E13
work_packet: WP-E13-002
---

# Learning Note: Language Option Parsing and Dry-Run Model

## What this packet teaches

This packet teaches how to extend a CLI feature without turning it into a large rewrite.

The important move is small:

```bash
monad add service api --language rust --dry-run
```

now has a language model, but language-specific files are still deferred.

That means the command can validate language intent and show it in dry-run output before it starts writing Rust, TypeScript, Python, or Go templates.

## Why this matters

A safe local-first CLI should not jump straight from "I parsed an option" to "I generated a full project and modified workspaces."

Instead, Monad now has a staged model:

1. parse a supported language ID;
2. store it in the core `AddPlanOptions`;
3. render it in dry-run output;
4. reject language-aware writes until templates are explicitly implemented.

This keeps the user in control.

## New core model

The new core enum is:

```rust
ComponentLanguage
```

Supported canonical IDs:

```text
rust
typescript
python
go
```

The add planner now carries:

```rust
Option<ComponentLanguage>
```

This preserves generic scaffolds when no language is selected.

## Important safety decision

This packet intentionally rejects:

```bash
monad add service api --language rust --yes
```

for now.

Why?

Because WP-E13-002 only adds language parsing and dry-run planning. It does not yet define the language-specific write files.

Generic E12 writes still work:

```bash
monad add app web --yes
```

Language-aware writes begin in later packets after each language template is added deliberately.

## What to inspect

Inspect the parser changes:

```bash
git diff -- crates/monad-cli/src/main.rs
```

Look for:

- `ComponentLanguage` import;
- `language: Option<ComponentLanguage>`;
- `--language` and `--language=<value>` parsing;
- validation that `--language` only applies to `add`;
- updated help text.

Inspect the core model:

```bash
git diff -- crates/monad-core/src/component_add.rs
```

Look for:

- `ComponentLanguage`;
- `AddPlanOptions::with_language`;
- dry-run output showing `language: ...`;
- guarded rejection of language-aware writes.

Inspect the public export:

```bash
git diff -- crates/monad-core/src/lib.rs
```

Look for:

```rust
ComponentLanguage
```

## How this prepares WP-E13-003

WP-E13-003 can now focus only on Rust templates.

It does not need to invent CLI language parsing. That foundation already exists.
