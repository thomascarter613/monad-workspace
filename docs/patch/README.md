# Patch planning and supervised apply

Monad patch support begins as a local-first, governance-first foundation. The first implementation is intentionally conservative: it models patch/change-set behavior, renders deterministic dry-run output, validates conflicts, and applies only generated Monad evidence artifacts after explicit approval.

## Command surface

```bash
monad patch --dry-run
monad patch --dry-run --format=json
monad patch --yes
```

## Safety boundaries

The E20 foundation does **not** perform autonomous patch application. It does not fetch remote patches, execute arbitrary scripts, call AI providers, delete files, silently overwrite existing files, or mutate user-owned source files.

`monad patch --yes` is supervised and local. In this foundation slice, it writes generated Monad-owned patch/evidence artifacts only, through the E19 generated-write approval gate. If a target file already exists with different content, the operation is blocked rather than overwritten.

## What is modeled now

- deterministic patch/change-set identity;
- file-level change kind, path, before hash, after hash, and rationale;
- validation findings with info/warning/blocked severity;
- conflict detection for existing files with different content;
- representation of user-owned source mutation as a blocked planning concept;
- generated evidence reports under `.monad/reports`.

## What remains for later epics

Future patch work can add explicit user-supplied patch inputs, richer diff parsing, review workflows, source mutation approval policy, and stronger provenance. Those later additions must preserve the E20 safety boundaries unless a new ADR explicitly changes them.
