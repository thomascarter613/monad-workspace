# Work-packet execution workflow

Monad work-packet workflow support is a local-first foundation for planning, verifying, evidencing, and handing off roadmap work packets.

## Command surface

```bash
monad work-packet --dry-run
monad work-packet --dry-run --format=json
monad work-packet --yes
```

## What E21 does

E21 adds a deterministic work-packet execution model inside `monad-core`:

- work-packet lifecycle status labels;
- work-packet metadata records;
- simple Markdown metadata parsing;
- implementation-plan generation;
- verification checklist rendering;
- evidence checklist rendering;
- generated closeout and handoff records;
- smoke tests and verification scripts.

## Safety boundaries

The work-packet command does not execute implementation commands, mutate GitHub issues, close work packets remotely, contact hosted services, or rewrite user-owned source files. `monad work-packet --yes` writes generated Monad workflow evidence only, through E19 generated-write approval gates.

## Generated evidence

`monad work-packet --yes` writes:

- `.monad/reports/work-packet-plan.md`
- `.monad/reports/work-packet-plan.json`
- `.monad/work-packets/e21-closeout-handoff.md`

Existing files with different content are not silently overwritten.
