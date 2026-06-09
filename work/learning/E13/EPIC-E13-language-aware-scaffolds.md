---
title: Epic E13 Learning Note
epic: E13
---

# Epic E13 Learning Note: Language-Aware Component Scaffolds

Epic E13 turns `monad add` from a generic folder scaffold into a language-aware component scaffold.

The main design lesson is that language-aware generation can still be safe and local-first.

Monad now understands:

```text
rust
typescript
python
go
```

but it still does not run package managers, mutate root manifests, generate lockfiles, or call the network.

## How it works

The core model is:

```rust
ComponentLanguage
AddPlanOptions
ComponentScaffoldTemplate
```

The CLI parses:

```bash
--language <language>
--language=<language>
```

Then `monad-core` selects the template set based on:

```text
component kind + optional language
```

## Why this is safe

Dry-run uses the same planning path as writes.

Writes use the same conflict detection as generic E12 adds.

No language template is allowed to bypass the guarded file-operation model.

## What changed from packet-by-packet learning

This file compresses the learning note for the rest of E13 because the project is switching to faster epic-level scripts.
