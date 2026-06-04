#!/usr/bin/env bash
set -euo pipefail

# Fix WP-E11-004 missing init apply imports/exports.
#
# Error being fixed:
# - cannot find function `apply_init_plan` in this scope
# - cannot find function `render_init_apply_result` in this scope
#
# Root cause:
# The guarded write functions exist in monad-core/src/init.rs, but the re-export
# and/or CLI import list did not include them.

python3 - <<'PY'
from pathlib import Path
import re

lib_path = Path("crates/monad-core/src/lib.rs")
main_path = Path("crates/monad-cli/src/main.rs")
init_path = Path("crates/monad-core/src/init.rs")

init_text = init_path.read_text(encoding="utf-8")
missing = [
    name
    for name in ["pub fn apply_init_plan", "pub fn render_init_apply_result"]
    if name not in init_text
]
if missing:
    raise SystemExit(
        "Missing expected functions in crates/monad-core/src/init.rs: "
        + ", ".join(missing)
    )

lib = lib_path.read_text(encoding="utf-8")

export_block = """pub use init::{
    InitApplyResult, InitPlanOptions, InitPreset, apply_init_plan, build_init_plan,
    render_init_apply_result, render_init_dry_run,
};"""

if re.search(r"pub use init::\{.*?\};", lib, flags=re.S):
    lib = re.sub(r"pub use init::\{.*?\};", export_block, lib, flags=re.S)
else:
    if "pub mod init;" not in lib:
        lib = lib.replace("pub mod git;\n", "pub mod git;\npub mod init;\n")
    anchor = "pub use git::{"
    idx = lib.find(anchor)
    if idx == -1:
        raise SystemExit("Could not find git export anchor in lib.rs")

    # Insert after the git export block.
    start = idx
    semi = lib.find("};", start)
    if semi == -1:
        raise SystemExit("Could not find end of git export block in lib.rs")
    semi += 3
    lib = lib[:semi] + "\n" + export_block + lib[semi:]

lib_path.write_text(lib, encoding="utf-8")

main = main_path.read_text(encoding="utf-8")

match = re.search(r"use monad_core::\{(?P<body>.*?)\};", main, flags=re.S)
if not match:
    raise SystemExit("Could not find `use monad_core::{...};` import block in crates/monad-cli/src/main.rs")

body = match.group("body")
for name in ["apply_init_plan", "render_init_apply_result"]:
    if name not in body:
        # Put apply near other build/apply functions and render near render functions.
        if name == "apply_init_plan":
            body = " apply_init_plan," + body
        else:
            body = body.rstrip() + f", {name}\n"

new_block = "use monad_core::{" + body + "};"
main = main[:match.start()] + new_block + main[match.end():]

main_path.write_text(main, encoding="utf-8")
PY

cargo fmt

echo
echo "Applied WP-E11-004 import/export fix."
echo
echo "Run:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  cargo run -p monad-cli -- init --dry-run"
