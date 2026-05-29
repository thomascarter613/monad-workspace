---
title: DeepWiki Dump README
description: Imported DeepWiki dump README for Monad workspace reference.
status: imported
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
source: DeepWiki
---

# DeepWiki Dump Runner for `thomascarter613/monad-workspace`

This package dumps the DeepWiki-generated documentation for:

```text
thomascarter613/monad-workspace
```

It writes the export into your repo under:

```text
docs/deepwiki/thomascarter613/monad-workspace/
```

## Usage

From the root of your local `monad-workspace` repository:

```bash
unzip deepwiki-dump-monad-workspace.zip -d /tmp/deepwiki-dump-monad-workspace
bash /tmp/deepwiki-dump-monad-workspace/run-dump.sh "$PWD"
```

Or from anywhere:

```bash
bash /tmp/deepwiki-dump-monad-workspace/run-dump.sh /path/to/monad-workspace
```

## Output files

```text
docs/deepwiki/thomascarter613/monad-workspace/
  README.md
  tools.raw.json
  structure.md
  structure.raw.json
  contents.md
  contents.raw.json
```

## Requirement

You need Node.js/npm installed, because the script uses the official MCP TypeScript SDK client.

## Troubleshooting

If the script says the repo is not indexed, open the repository on DeepWiki first, wait for indexing, and rerun the script.
