# Interactive Workbench / TUI Foundation

E34 adds Monad's Interactive Workbench / TUI Foundation.

## Command surface

```bash
monad workbench-plan --dry-run
monad workbench-plan --dry-run --format=json
monad workbench-plan --yes
monad workbench --dry-run
monad tui --dry-run
```

## What this foundation models

- TUI navigation model
- TUI shell proof of concept
- Issue/work-packet view
- Plan/report/context viewer
- Approval review screen foundation
- TUI smoke tests

## Safety boundaries

This foundation does **not** add a terminal UI dependency, enter raw terminal
mode, start an interactive event loop, call GitHub, access networks, invoke
package managers, or mutate user-owned source files.

`--yes` writes generated workbench evidence only under `.monad/reports`.
