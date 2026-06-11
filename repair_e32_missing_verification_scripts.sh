#!/usr/bin/env bash
set -euo pipefail

# Focused E32 repair:
# Restores missing verification scripts:
# - tools/scripts/verify-local-ai-retrieval.sh
# - tools/scripts/verify-e32.sh

if [[ ! -f "Cargo.toml" ]]; then
  echo "Run this script from the monad-workspace repository root." >&2
  exit 1
fi

BACKUP_DIR=".monad/script-backups/repair-e32-missing-verification-scripts-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$BACKUP_DIR/tools/scripts"
mkdir -p tools/scripts

backup_if_exists() {
  local path="$1"
  if [[ -e "$path" ]]; then
    mkdir -p "$BACKUP_DIR/$(dirname "$path")"
    cp -a "$path" "$BACKUP_DIR/$path"
  fi
}

backup_if_exists "tools/scripts/verify-local-ai-retrieval.sh"
backup_if_exists "tools/scripts/verify-e32.sh"

cat > tools/scripts/verify-local-ai-retrieval.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail

cargo check -p monad-core
cargo test -p monad-core --lib local_ai_retrieval
cargo test -p monad-cli retrieval_plan

text_output="$(mktemp)"
json_output="$(mktemp)"
trap 'rm -f "$text_output" "$json_output"' EXIT

cargo run -p monad-cli -- retrieval-plan --dry-run > "$text_output"
grep -q "Monad local AI retrieval and vector memory plan" "$text_output"
grep -q "deterministic-local" "$text_output"
grep -q "No AI model provider is called by Monad" "$text_output"

cargo run -p monad-cli -- retrieval-plan --dry-run --format=json > "$json_output"
grep -q '"command": "retrieval-plan"' "$json_output"
grep -q 'deterministic-local' "$json_output"

echo "Local AI retrieval verification passed."
SH

cat > tools/scripts/verify-e32.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail

tools/scripts/verify-local-ai-retrieval.sh

test -f crates/monad-core/src/local_ai_retrieval.rs
test -f docs/local-ai-retrieval/README.md
test -f docs/roadmap/epic-32-local-ai-retrieval-vector-memory.md

grep -q "Local AI Retrieval and Vector Memory" docs/local-ai-retrieval/README.md
grep -q "WP-E32-001" docs/roadmap/epic-32-local-ai-retrieval-vector-memory.md
grep -q "WP-E32-006" docs/roadmap/epic-32-local-ai-retrieval-vector-memory.md

echo "E32 verification passed."
SH

chmod +x tools/scripts/verify-local-ai-retrieval.sh
chmod +x tools/scripts/verify-e32.sh

echo "Restored E32 verification scripts."
echo "Backup directory: $BACKUP_DIR"
echo
echo "Run:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-local-ai-retrieval.sh"
echo "  tools/scripts/verify-e32.sh"
