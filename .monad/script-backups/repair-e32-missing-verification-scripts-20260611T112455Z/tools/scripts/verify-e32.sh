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
