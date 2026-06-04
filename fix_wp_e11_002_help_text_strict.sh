#!/usr/bin/env bash
set -euo pipefail

# Fix WP-E11-002 help-text test failure.
#
# Cause:
# The previous fixer checked whether "init --dry-run" existed anywhere in
# crates/monad-cli/src/main.rs. It did exist in the test assertion, so the script
# skipped inserting the actual help_text() command line.
#
# This patch checks for the exact rendered help line and inserts it under
# "Core commands:" if missing.

python3 - <<'PY'
from pathlib import Path
import re

path = Path("crates/monad-cli/src/main.rs")
text = path.read_text(encoding="utf-8")

help_line = '        "  init --dry-run                            Preview repository initialization plan",\n'

if help_line not in text:
    anchor = '        "Core commands:",\n'
    if anchor not in text:
        raise SystemExit('Could not find help_text() anchor: "Core commands:"')

    text = text.replace(anchor, anchor + help_line, 1)

# Also make sure InitPreset uses derived Default if the earlier Clippy fix did
# not apply for any reason.
init_path = Path("crates/monad-core/src/init.rs")
if init_path.exists():
    init_text = init_path.read_text(encoding="utf-8")

    init_text = init_text.replace(
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum InitPreset {",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]\npub enum InitPreset {",
    )

    init_text = init_text.replace(
        "    /// Smallest useful Monad-aware repository baseline.\n    Minimal,",
        "    /// Smallest useful Monad-aware repository baseline.\n    #[default]\n    Minimal,",
    )

    init_text = re.sub(
        r"\nimpl Default for InitPreset \{\n    fn default\(\) -> Self \{\n        Self::Minimal\n    \}\n\}\n",
        "\n",
        init_text,
    )

    init_path.write_text(init_text, encoding="utf-8")

path.write_text(text, encoding="utf-8")
PY

cargo fmt

echo
echo "Applied strict WP-E11-002 help_text fix."
echo
echo "Confirm the help text contains init --dry-run:"
echo "  cargo run -p monad-cli -- --help | grep 'init --dry-run'"
echo
echo "Then run:"
echo "  cargo test -p monad-cli --bin monad"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
