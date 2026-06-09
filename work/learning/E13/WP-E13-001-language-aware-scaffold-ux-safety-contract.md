---
title: WP-E13-001 Learning Note
epic: E13
work_packet: WP-E13-001
---

# Learning Note: Language-Aware Scaffold UX and Safety Contract

## What this packet teaches

This packet teaches how to define a CLI feature before implementing it.

The important lesson is that a command should have a behavior contract before code is changed. The contract answers:

- What command shape will users type?
- What inputs are valid?
- What will happen during dry-run?
- What will happen during confirmed writes?
- What is intentionally not supported yet?
- What files should be generated?
- What side effects are forbidden?

## Why this matters for Monad

Monad is a local-first polyglot repo runtime. It should be powerful, but it should also be predictable.

`monad add --language` will eventually create language-specific files. That can become risky if it also installs packages, changes lockfiles, edits workspace manifests, or calls remote template sources.

This packet draws a boundary:

- write local files only;
- preserve dry-run;
- require `--yes` for writes;
- do not run native tools;
- do not mutate root manifests yet;
- keep E12 generic scaffolds working.

## The key design move

E13 should be additive.

Instead of replacing the E12 generic `monad add` flow, language-aware scaffolding should extend it.

That means:

```bash
monad add app web --dry-run
```

still means generic scaffold, while:

```bash
monad add app web --language typescript --dry-run
```

means language-aware scaffold.

## What to inspect after running this script

Run:

```bash
git diff -- docs/commands/ADD-LANGUAGE.md
git diff -- docs/workflows/LANGUAGE-AWARE-SCAFFOLDS.md
git diff -- work/learning/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md
git diff -- work/deliverables/E13/WP-E13-001-language-aware-scaffold-ux-safety-contract.md
```

Focus on the safety boundaries.

The most important boundaries are:

- dry-run writes no files;
- `--yes` is required for writes;
- no package installs;
- no network calls;
- no Git operations;
- no root manifest mutation in the first implementation.

## How this prepares WP-E13-002

WP-E13-002 can now implement only the next narrow slice:

```bash
monad add <kind> <name> --language <language> --dry-run
```

It should parse the language option and produce a dry-run plan.

It should not need to solve every language template yet.
