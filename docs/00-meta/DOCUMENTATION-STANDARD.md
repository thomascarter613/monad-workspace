---
title: "Documentation Standard"
status: draft
owner: "Thomas Carter"
created: 2026-05-23
updated: 2026-05-23
version: 0.1.0
tags:
  - monad
  - documentation
  - standard
  - meta
related:
  - docs/00-meta/DOCUMENTATION-MAP.md
  - docs/00-meta/FRONTMATTER-STANDARD.md
  - docs/00-meta/NAMING-STANDARD.md
  - docs/07-workflow/DEFINITION-OF-DONE.md
---

# Documentation Standard

## Purpose

This document defines how Monad documentation should be written, organized, maintained, and promoted from rough notes into canonical project knowledge.

Monad’s documentation is part of the product foundation. It exists to make the repository understandable, verifiable, maintainable, and resumable by humans and AI assistants.

## Core Rule

Documentation must make the project easier to continue.

A document is useful when it helps someone answer at least one of these questions:

- What is Monad?
- Why does this exist?
- What decision was made?
- What is the current plan?
- How should work proceed?
- What is in scope?
- What is out of scope?
- How do we verify this?
- What should a future session read first?
- What should not be changed casually?

## Required Frontmatter

Every canonical Markdown document in `docs/` must begin with YAML frontmatter.

Minimum required frontmatter:

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

## Document Statuses

Monad uses these statuses:

| Status       | Meaning                                     |
| ------------ | ------------------------------------------- |
| `stub`       | Planned location for future knowledge.      |
| `draft`      | Meaningful content exists but is not final. |
| `review`     | Ready for review before acceptance.         |
| `accepted`   | Canonical project guidance.                 |
| `superseded` | Replaced by newer guidance.                 |
| `archived`   | Retained for history only.                  |

A document should not be marked `accepted` unless it is intended to guide future work.

## File Naming Rules

Use uppercase kebab-style names for canonical topic documents:

```text
PRODUCT-VISION.md
MVP-SCOPE.md
SYSTEM-OVERVIEW.md
WORK-PACKET-STANDARD.md
```

Use numbered ADR names for Architecture Decision Records:

```text
ADR-0001-use-rust-for-core-runtime.md
```

Use `README.md` for directory overviews.

## Directory Rules

Each major documentation area should contain a `README.md`.

The `README.md` should explain:

* what belongs in that area;
* what does not belong in that area;
* which documents are most important;
* when that area should be updated.

## Writing Style

Monad documentation should be:

* direct;
* structured;
* explicit;
* durable;
* reviewable;
* useful to humans;
* useful to AI assistants;
* free of unnecessary hype;
* clear about assumptions and uncertainty.

Avoid vague phrases such as:

* “handle this later”;
* “obviously”;
* “standard stuff”;
* “etc.” when the omitted details matter;
* “just” when the work may be complex;
* “simple” when the implementation may not be simple.

## Canonical Decision Rule

If a decision affects architecture, workflow, safety, product scope, implementation direction, or long-term maintainability, it should be recorded in one of these places:

1. an ADR;
2. a canonical standard document;
3. a current-state or decision-log context file if still temporary.

Durable decisions should not live only in chat history.

## Stub Rule

Stubs are allowed and expected during early project setup.

A stub must include:

* frontmatter;
* title;
* purpose;
* status;
* expected contents;
* notes about future expansion.

A stub should not pretend to be complete.

## Draft Rule

A draft should include enough detail to guide work.

A draft should usually include:

* purpose;
* scope;
* non-goals where relevant;
* definitions;
* expected workflow or behavior;
* verification expectations where relevant;
* related documents.

## Accepted Document Rule

An accepted document should be treated as canonical unless superseded.

Accepted documents should be changed carefully and, when appropriate, through a new work packet or ADR.

## AI-Readable Documentation Rule

Because Monad is AI-native and repo-native, documentation should be easy for an AI assistant to use as context.

That means:

* use clear headings;
* avoid burying decisions in prose;
* keep lists explicit;
* distinguish facts from plans;
* distinguish accepted decisions from drafts;
* link related docs;
* keep current-state and handoff docs updated.

## Generated Documentation Rule

Generated documentation or generated context must be clearly distinguishable from human-authored canonical guidance.

Generated files should indicate:

* when they were generated;
* what source data was used if known;
* whether humans have reviewed them;
* whether they are safe to treat as canonical.

## Documentation Update Rule

Documentation should be updated when:

* a work packet changes behavior;
* a new architectural decision is made;
* a workflow rule changes;
* a command is added or renamed;
* a safety rule is added;
* a verification expectation changes;
* a context handoff becomes stale;
* implementation reveals that a prior assumption was wrong.

## Work Packet Documentation Rule

Every meaningful work packet should define:

* product area;
* objective;
* user value;
* scope;
* tasks;
* deliverables;
* verification commands;
* expected result after verification;
* Definition of Done;
* recommended commit message;
* priority;
* size.

Work packet records may live in GitHub Issues, but canonical or durable workflow standards belong in the repository.

## Documentation Review Checklist

Before committing a documentation change, check:

* Does every Markdown file have frontmatter?
* Is the status accurate?
* Is the document in the correct directory?
* Does the title match the filename?
* Are important assumptions explicit?
* Are related docs listed when known?
* Does the content help future work?
* Does the content avoid unsupported final claims?
* Is the change atomic?

## Verification

Basic documentation verification should include:

```bash
find docs -type f | sort
```

and:

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

This documentation standard is a draft. It is strong enough to guide the initial Monad documentation foundation and should be refined as the repository matures.
