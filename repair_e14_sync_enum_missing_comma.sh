#!/usr/bin/env bash
set -euo pipefail

# Repair — E14 sync enum missing comma
#
# Symptom:
#   error: expected one of `,`, `=`, or `}`, found doc comment
#   crates/monad-cli/src/main.rs
#
# Cause:
#   The E14 repair script inserted a struct-like enum variant:
#
#     Sync { ... }
#
#   but the closing brace before the next variant needs a comma:
#
#     Sync { ... },
#
# This script fixes missing commas between struct-like variants inside
# enum CliCommand only, then formats the repo.

echo "==> Repair: E14 CliCommand enum missing comma"

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

cd "$REPO_ROOT"

CLI_FILE="crates/monad-cli/src/main.rs"

if [ ! -f "$CLI_FILE" ]; then
  echo "ERROR: expected file not found: $CLI_FILE" >&2
  exit 1
fi

mkdir -p .monad/script-backups/E14/REPAIR-sync-enum-comma
BACKUP_STAMP="$(date +%Y%m%d%H%M%S)"
cp "$CLI_FILE" ".monad/script-backups/E14/REPAIR-sync-enum-comma/main.rs.$BACKUP_STAMP.bak"

python3 <<'PY'
from pathlib import Path

path = Path("crates/monad-cli/src/main.rs")
text = path.read_text()

enum_start = text.find("enum CliCommand {")
if enum_start == -1:
    raise SystemExit("ERROR: could not find enum CliCommand")

brace_start = text.find("{", enum_start)
if brace_start == -1:
    raise SystemExit("ERROR: could not find opening brace for enum CliCommand")

depth = 0
enum_end = None
for index in range(brace_start, len(text)):
    char = text[index]
    if char == "{":
        depth += 1
    elif char == "}":
        depth -= 1
        if depth == 0:
            enum_end = index
            break

if enum_end is None:
    raise SystemExit("ERROR: could not find closing brace for enum CliCommand")

before = text[:brace_start + 1]
body = text[brace_start + 1:enum_end]
after = text[enum_end:]

# Work line-by-line inside the enum only. If a line is an indented closing brace
# for a struct-like variant and the next meaningful line is another enum variant
# doc comment or variant name, the closing brace must end with a comma.
lines = body.splitlines(keepends=True)
fixed_lines = []

for i, line in enumerate(lines):
    stripped = line.strip()

    if stripped == "}":
        # Look ahead for the next non-empty line.
        next_non_empty = ""
        for future in lines[i + 1:]:
            if future.strip():
                next_non_empty = future.strip()
                break

        # If another variant follows, this is a struct-like variant close.
        # The final enum close is outside this body slice and is not touched.
        if next_non_empty.startswith("///") or next_non_empty.startswith("#[") or next_non_empty[:1].isupper():
            line = line.rstrip("\n") + ",\n"

    fixed_lines.append(line)

fixed_body = "".join(fixed_lines)
fixed_text = before + fixed_body + after

path.write_text(fixed_text)
PY

echo "==> Formatting Rust code"
cargo fmt

echo
echo "==> Enum comma repair complete."
echo
echo "Focused verification:"
echo "  cargo test -p monad-cli"
echo
echo "Then continue E14 verification:"
echo "  cargo test -p monad-core --lib sync"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-sync.sh"
echo "  tools/scripts/verify-e14.sh"
