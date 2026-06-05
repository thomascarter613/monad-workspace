---
title: "Learning Note — WP-E12-003 Embedded Component Scaffold Templates"
document_type: "learning-note"
status: active
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E12
work_packet: WP-E12-003
tags:
  - learning
  - rust
  - templates
  - dry-run
  - monad
---

# Learning Note — WP-E12-003 Embedded Component Scaffold Templates

## What Changed

WP-E12-002 built the `add` plan directly from hard-coded paths.

WP-E12-003 introduces a tiny embedded template layer:

```text
component.readme
component.gitkeep
```

The command still does not write files.

## Why This Matters

Hard-coded paths are fine for the first slice, but they do not scale.

Templates give us a place to attach:

- a stable ID;
- a relative path;
- a description;
- content for a future write path.

## Main File to Read

```text
crates/monad-core/src/component_add.rs
```

Start with:

```text
ComponentScaffoldTemplate
COMPONENT_SCAFFOLD_TEMPLATES
component_scaffold_templates
```

## New Mental Model

The add flow is now:

```text
component kind + name
  -> component root
  -> embedded component templates
  -> target paths
  -> dry-run file operation plan
  -> rendered dry-run output
```

## Rust Concept: Struct for Template Metadata

`ComponentScaffoldTemplate` is a struct.

It groups related template facts:

```text
id
relative_path
description
content
```

That keeps the plan builder from spreading template details across unrelated code.

## Rust Concept: Constants

The templates are stored in a constant array:

```text
COMPONENT_SCAFFOLD_TEMPLATES
```

This means the templates are compiled into the binary.

No network fetch.

No plugin install.

No runtime template discovery.

## Rust Concept: Placeholder Replacement

The README template uses simple placeholders:

```text
{{component_kind}}
{{component_name}}
{{component_root}}
```

The renderer replaces them with safe values.

This is not a full templating engine.

That is intentional.

## What to Inspect

```bash
git diff -- crates/monad-core/src/component_add.rs
```

Look for:

```text
ComponentScaffoldTemplate
component_scaffold_templates
render_content
build_add_plan
```

## Verification Commands

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- add app web --dry-run
cargo run -p monad-cli -- add service api --dry-run
```
