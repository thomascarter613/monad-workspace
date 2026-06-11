# Web Workbench Foundation

E35 adds Monad's Web Workbench Foundation.

## Command surface

```bash
monad web-workbench-plan --dry-run
monad web-workbench-plan --dry-run --format=json
monad web-workbench-plan --yes
monad web-workbench --dry-run
monad web-ui --dry-run
```

## What this foundation models

- Local web workbench architecture
- Local server/API foundation
- Repository graph view
- Work-packet and report views
- Approval/context viewer foundation
- Web workbench smoke tests

## Safety boundaries

This foundation does **not** start an HTTP server, open sockets, add browser or
frontend dependencies, call GitHub, access networks, invoke package managers, or
mutate user-owned source files.

`--yes` writes generated web workbench evidence only under `.monad/reports`.
