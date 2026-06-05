#!/usr/bin/env bash
set -euo pipefail

# Fix WP-E12-002 clippy items_after_test_module error.
#
# What failed:
#   pub use component_add::{...};
# was appended after `#[cfg(test)] mod tests` in crates/monad-core/src/lib.rs.
#
# Why Clippy rejects it:
#   Public module items should appear before the test module so tests stay at
#   the bottom of the file.
#
# Fix:
#   Remove any component_add export block from the bottom and reinsert it before
#   the first #[cfg(test)] test module.

python3 - <<'PY'
from pathlib import Path
import re

path = Path("crates/monad-core/src/lib.rs")
text = path.read_text(encoding="utf-8")

export_block = """pub use component_add::{
    AddPlanOptions, ComponentKind, ComponentName, build_add_plan, render_add_dry_run,
};
"""

# Remove every existing component_add export block, wherever it landed.
text = re.sub(
    r"\n*pub use component_add::\{\n\s+AddPlanOptions, ComponentKind, ComponentName, build_add_plan, render_add_dry_run,\n\};\n*",
    "\n",
    text,
)

# Ensure the module declaration exists with the other module declarations.
if "pub mod component_add;" not in text:
    anchor = "pub mod checks;\n"
    if anchor not in text:
        raise SystemExit("Could not find `pub mod checks;` anchor in lib.rs")
    text = text.replace(anchor, anchor + "pub mod component_add;\n", 1)

# Insert export block before the test module.
test_marker = "#[cfg(test)]"
idx = text.find(test_marker)
if idx == -1:
    text = text.rstrip() + "\n\n" + export_block
else:
    before = text[:idx].rstrip()
    after = text[idx:].lstrip()
    text = before + "\n\n" + export_block + "\n" + after

path.write_text(text, encoding="utf-8")

learning_path = Path("work/learning/E12/WP-E12-002-add-dry-run-plan.md")
if learning_path.exists():
    learning = learning_path.read_text(encoding="utf-8")
    marker = "## Fix Note — Rust Items Before Test Modules"
    if marker not in learning:
        learning += "\n\n" + "\n".join([
            marker,
            "",
            "Clippy reported `items_after_test_module` because a public export was appended after `#[cfg(test)] mod tests` in `lib.rs`.",
            "",
            "The Rust convention is:",
            "",
            "```text",
            "module declarations",
            "public exports",
            "normal code",
            "test module at the bottom",
            "```",
            "",
            "The fix moves:",
            "",
            "```rust",
            "pub use component_add::{...};",
            "```",
            "",
            "above the test module.",
            "",
            "This does not change runtime behavior. It only restores normal Rust file organization.",
            "",
        ])
        learning_path.write_text(learning, encoding="utf-8")
PY

cargo fmt

echo
echo "Applied WP-E12-002 lib.rs export-order fix."
echo
echo "Run:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  cargo run -p monad-cli -- add app web --dry-run"
echo "  tools/scripts/verify.sh"
