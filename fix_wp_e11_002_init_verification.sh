#!/usr/bin/env bash
set -euo pipefail

# Fix WP-E11-002 verification failures:
# 1. Ensure help_text() includes "init --dry-run".
# 2. Replace manual InitPreset Default impl with #[derive(Default)] + #[default].

python3 - <<'PY'
from pathlib import Path
import re

init_path = Path("crates/monad-core/src/init.rs")
main_path = Path("crates/monad-cli/src/main.rs")

init_text = init_path.read_text(encoding="utf-8")

# Clippy wants this manual Default implementation derived instead.
init_text = init_text.replace(
    "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum InitPreset {",
    "#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]\npub enum InitPreset {",
)

# Mark Minimal as the enum default variant.
init_text = init_text.replace(
    "    /// Smallest useful Monad-aware repository baseline.\n    Minimal,",
    "    /// Smallest useful Monad-aware repository baseline.\n    #[default]\n    Minimal,",
)

# Remove the manual impl Default block if present.
init_text = re.sub(
    r"\nimpl Default for InitPreset \{\n    fn default\(\) -> Self \{\n        Self::Minimal\n    \}\n\}\n",
    "\n",
    init_text,
)

init_path.write_text(init_text, encoding="utf-8")

main_text = main_path.read_text(encoding="utf-8")

# The test expects help_text() to mention init --dry-run. Insert the help line
# under "Core commands:" if it is not already present.
if 'init --dry-run' not in main_text:
    main_text = main_text.replace(
        '        "Core commands:",\n',
        '        "Core commands:",\n'
        '        "  init --dry-run                            Preview repository initialization plan",\n',
        1,
    )

main_path.write_text(main_text, encoding="utf-8")
PY

cargo fmt

echo
echo "WP-E11-002 verification fixes applied:"
echo "  - help_text now includes init --dry-run"
echo "  - InitPreset derives Default with #[default] Minimal"
echo
echo "Run:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
