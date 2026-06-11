# Agent Workflow Sandbox

E33 adds Monad's Agent Workflow Sandbox Foundation.

## Command surface

```bash
monad sandbox-plan --dry-run
monad sandbox-plan --dry-run --format=json
monad sandbox-plan --yes
monad agent-sandbox --dry-run
monad sandbox-verify --dry-run
```

## Safety boundaries

This foundation does **not** execute agent actions, apply patches, run
verification commands, promote sandbox changes, access the network, invoke
package managers, or mutate user-owned source files.

`--yes` writes generated sandbox evidence only under `.monad/reports`.
