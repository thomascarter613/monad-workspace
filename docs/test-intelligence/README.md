# Test Intelligence and Verification Planning

E26 adds Monad's first local-only test-intelligence and verification-planning
foundation.

## What this foundation does

- Defines a test intelligence model.
- Discovers likely test commands from local manifests.
- Maps discovered commands to components/packages.
- Generates targeted verification recommendations.
- Adds confidence and evidence metadata.
- Writes generated verification-planning evidence only when explicitly approved.

## Command surface

```bash
monad verify-plan --dry-run
monad verify-plan --dry-run --format=json
monad verify-plan --yes
```

Aliases:

```bash
monad test-intelligence --dry-run
monad verification-plan --dry-run
```

## Safety boundaries

This foundation does **not**:

- execute tests;
- execute Git;
- invoke package managers;
- install tools;
- call remote services;
- call AI providers;
- rewrite user-owned source files.

`--yes` writes generated evidence only under `.monad/reports`.
