---
title: "Frontmatter Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - documentation
  - frontmatter
  - standard
related:
  - docs/00-meta/DOCUMENTATION-STANDARD.md
  - docs/00-meta/STATUS-STANDARD.md
  - docs/07-workflow/DEFINITION-OF-DONE.md
---

# Frontmatter Standard

## Purpose

This document defines the required YAML frontmatter format for canonical Monad Markdown documentation.

Frontmatter exists so Monad documentation can be read by humans, indexed by tools, checked by verification scripts, and used by future AI-assisted context workflows.

## Core Rule

Every canonical Markdown document in `docs/` must begin with YAML frontmatter.

A document without frontmatter is incomplete.

## Minimum Required Frontmatter

Every canonical Markdown document must include:

```yaml
---
title: ""
status: stub
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags: []
related: []
---
```

## Field Definitions

### `title`

Human-readable document title.

Example:

```yaml
title: "Product Vision"
```

The title should usually match the first `# Heading` in the document.

### `status`

The current maturity state of the document.

Allowed values:

```text
stub
draft
review
accepted
superseded
archived
```

See:

```text
docs/00-meta/STATUS-STANDARD.md
```

### `owner`

The person or role responsible for maintaining the document.

For early Monad work:

```yaml
owner: "Thomas Carter"
```

Later, this may become a role such as:

```yaml
owner: "Monad Maintainers"
```

### `created`

The date the document was first created.

Format:

```text
YYYY-MM-DD
```

Example:

```yaml
created: 2026-05-23
```

### `updated`

The date the document was last meaningfully updated.

Format:

```text
YYYY-MM-DD
```

Example:

```yaml
updated: 2026-05-23
```

### `version`

The document version.

Initial documents should use:

```yaml
version: 0.1.0
```

The version should change when the document changes materially.

### `tags`

A list of short classification tags.

Example:

```yaml
tags:
  - monad
  - architecture
  - verification
```

Tags should be lowercase kebab-case unless there is a strong reason otherwise.

### `related`

A list of related documents.

Example:

```yaml
related:
  - docs/05-architecture/SYSTEM-OVERVIEW.md
  - docs/06-adrs/ADR-0001-use-rust-for-core-runtime.md
```

Use repository-relative paths.

## Recommended Optional Fields

Some documents may include additional fields.

### `adr`

For documents related to a specific ADR:

```yaml
adr: ADR-0001
```

### `epic`

For documents closely tied to an epic:

```yaml
epic: E1
```

### `work_packet`

For documents produced by a specific work packet:

```yaml
work_packet: WP-E0-002
```

### `supersedes`

For a document replacing earlier guidance:

```yaml
supersedes:
  - docs/old/path.md
```

### `superseded_by`

For a document that has been replaced:

```yaml
superseded_by:
  - docs/new/path.md
```

## Status-Specific Expectations

### `stub`

A stub must include:

* frontmatter;
* title heading;
* purpose;
* status note;
* expected contents.

### `draft`

A draft must include meaningful content that can guide work.

### `review`

A review document should be complete enough for focused review.

### `accepted`

An accepted document is canonical.

### `superseded`

A superseded document must identify the replacing document when known.

### `archived`

An archived document should explain why it is retained.

## Example Stub

```markdown
---
title: "Example Document"
status: stub
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - example
related: []
---

# Example Document

## Purpose

This document is the canonical Monad location for the example topic.

## Status

This file is a stub.

## Expected Contents

- Topic explanation.
- Relevant decisions.
- Related documents.
- Verification notes where applicable.
```

## Example Draft

```markdown
---
title: "Example Draft"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - example
related:
  - docs/00-meta/DOCUMENTATION-STANDARD.md
---

# Example Draft

## Purpose

This document defines...

## Scope

### In scope

- ...

### Out of scope

- ...
```

## Validation Rules

A future verification script should check that every `docs/**/*.md` file:

* starts with `---`;
* has a closing `---`;
* includes `title`;
* includes `status`;
* includes `owner`;
* includes `created`;
* includes `updated`;
* includes `version`;
* includes `tags`;
* includes `related`;
* uses an allowed `status`;
* uses repository-relative paths in `related`.

## Verification Command

Basic current verification:

```bash
python3 - <<'PY'
from pathlib import Path

missing = []
for path in sorted(Path("docs").rglob("*.md")):
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        missing.append(str(path))

if missing:
    print("Files missing frontmatter:")
    for item in missing:
        print(f"  {item}")
    raise SystemExit(1)

print("All docs Markdown files have YAML frontmatter.")
PY
```

## Current Status

This standard is a draft. It is authoritative enough for the initial Monad documentation foundation and should be refined when automated documentation verification is added.
