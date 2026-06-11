# GitHub Integration and PR Workflow Foundation

E36 adds Monad's GitHub Integration and PR Workflow Foundation.

## Command surface

```bash
monad github-plan --dry-run
monad github-plan --dry-run --format=json
monad github-plan --yes
monad github-workflow --dry-run
monad pr-plan --dry-run
```

## What this foundation models

- GitHub integration boundary
- GitHub issue sync/export model
- Branch and PR planning model
- PR description and review-pack generation
- Issue closeout/evidence helpers
- GitHub workflow smoke tests

## Safety boundaries

This foundation does **not** call GitHub APIs, create branches, open pull
requests, close issues, access networks, invoke package managers, or mutate
remote state.

`--yes` writes generated GitHub workflow evidence only under `.monad/reports`.
