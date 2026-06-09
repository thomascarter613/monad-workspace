# Dependency Graph and Impact Analysis

E25 adds Monad's first local-only dependency graph and impact-analysis foundation.

## Command surface

```bash
monad impact --dry-run
monad impact --dry-run --format=json
monad impact --yes
monad dependency-impact --dry-run
```

## Changed-file manifest

Monad does not shell out to Git in this foundation. Changed-file impact analysis
reads an optional local file:

```text
.monad/changed-files.txt
```

If that file is absent, Monad emits conservative full-verification recommendations.

## Safety boundaries

This foundation does not execute Git, build tools, package managers, language
servers, remote calls, AI providers, or source rewrites. `--yes` writes generated
evidence only under `.monad/reports`.
