#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

log() {
  printf '\n==> %s\n' "$1"
}

log "Checking git diff whitespace"
git diff --check

log "Checking required foundation and runtime paths"
python3 tools/scripts/check-required-paths.py

log "Checking Markdown YAML frontmatter"
python3 tools/scripts/check-markdown-frontmatter.py

log "Checking work packet record structure"
python3 tools/scripts/check-work-records.py

log "Checking task record structure"
python3 tools/scripts/check-task-records.py

log "Checking deliverable record structure"
python3 tools/scripts/check-deliverable-records.py

log "Checking epic record structure"
python3 tools/scripts/check-epic-records.py

log "Checking ADR record structure"
python3 tools/scripts/check-adr-records.py

log "Checking context handoff records"
python3 tools/scripts/check-context-records.py

log "Checking Rust formatting"
cargo fmt --check

log "Running Rust tests"
cargo test

log "Reporting working tree status"
git status --short

printf '\nVerification baseline passed.\n'
