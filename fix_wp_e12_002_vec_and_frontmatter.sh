#!/usr/bin/env bash
set -euo pipefail

# Fix WP-E12-002 Vec-vs-array compile error.
#
# What failed:
#   FileOperationPlan::from_operations([...])
#
# Why:
#   The function expects Vec<PlannedFileOperation>, but [...] creates a fixed-size array.
#
# Fix:
#   FileOperationPlan::from_operations(vec![...])
#
# This script also optionally adds minimal YAML frontmatter to docs/mvp-capabilities.md
# if that file exists and still lacks frontmatter, because tools/scripts/verify.sh
# reported that as a separate repository verification issue.

python3 - <<'PY'
from pathlib import Path

component_path = Path("crates/monad-core/src/component_add.rs")
text = component_path.read_text(encoding="utf-8")

old = "FileOperationPlan::from_operations([\n"
new = "FileOperationPlan::from_operations(vec![\n"

if old not in text:
    raise SystemExit("Expected from_operations([ pattern not found in component_add.rs")

text = text.replace(old, new, 1)
component_path.write_text(text, encoding="utf-8")

learning_path = Path("work/learning/E12/WP-E12-002-add-dry-run-plan.md")
if learning_path.exists():
    learning = learning_path.read_text(encoding="utf-8")
    if "## Fix Note — Vec Versus Array" not in learning:
        learning += """

## Fix Note — Vec Versus Array

The first version of this work packet used:

```rust
FileOperationPlan::from_operations([
    operation_one,
    operation_two,
])
```

That creates a fixed-size Rust array.

The existing Monad API expects:

```rust
Vec<PlannedFileOperation>
```

So the correct code is:

```rust
FileOperationPlan::from_operations(vec![
    operation_one,
    operation_two,
])
```

The `vec![]` macro creates a growable vector.

This is a common Rust distinction:

```text
[T; N]  = fixed-size array with exactly N items
Vec<T>  = growable heap-allocated list
```

In this repo, `FileOperationPlan::from_operations` wants ownership of a `Vec` because plans may contain any number of operations.
"""
        learning_path.write_text(learning, encoding="utf-8")

# Optional cleanup for the unrelated verification failure reported by tools/scripts/verify.sh.
mvp_path = Path("docs/mvp-capabilities.md")
if mvp_path.exists():
    mvp = mvp_path.read_text(encoding="utf-8")
    if not mvp.lstrip().startswith("---"):
        mvp_path.write_text(
            """---
title: "MVP Capabilities"
document_type: "capability-reference"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
tags:
  - monad
  - mvp
  - capabilities
---

""" + mvp,
            encoding="utf-8",
        )
PY

cargo fmt

echo
echo "Applied WP-E12-002 compile fix:"
echo "  - changed from_operations([...]) to from_operations(vec![...])"
echo "  - appended a Vec-vs-array explanation to the learning note"
echo
echo "If docs/mvp-capabilities.md existed without frontmatter, minimal frontmatter was added."
echo
echo "Run:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  cargo run -p monad-cli -- add app web --dry-run"
echo "  tools/scripts/verify.sh"
