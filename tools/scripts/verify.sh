#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "${ROOT_DIR}"

echo "==> Checking git diff whitespace"
git diff --check

echo "==> Checking required foundation and runtime paths"
python3 tools/scripts/check-required-paths.py

echo "==> Checking Markdown frontmatter"
python3 tools/scripts/check-markdown-frontmatter.py

echo "==> Checking context records"
python3 tools/scripts/check-context-records.py

echo "==> Checking work records"
python3 tools/scripts/check-work-records.py

echo "==> Checking task records"
python3 tools/scripts/check-task-records.py

echo "==> Checking deliverable records"
python3 tools/scripts/check-deliverable-records.py

echo "==> Formatting Rust code"
cargo fmt --check

echo "==> Running Rust tests"
cargo test

echo "==> Running CLI info smoke test"
cargo run --quiet -p monad-cli -- info >/dev/null

echo "==> Running CLI info JSON smoke test"
cargo run --quiet -p monad-cli -- info --format=json >/dev/null

echo "==> Running CLI check smoke test"
cargo run --quiet -p monad-cli -- check >/dev/null

echo "==> Running CLI check JSON smoke test"
cargo run --quiet -p monad-cli -- check --format=json >/dev/null

echo "==> Running CLI inspect smoke test"
cargo run --quiet -p monad-cli -- inspect >/dev/null

echo "==> Running CLI inspect JSON smoke test"
cargo run --quiet -p monad-cli -- inspect --format=json >/dev/null

echo "==> Running CLI graph smoke test"
cargo run --quiet -p monad-cli -- graph >/dev/null

echo "==> Running CLI graph JSON smoke test"
cargo run --quiet -p monad-cli -- graph --format=json >/dev/null

echo "==> Running CLI graph Mermaid smoke test"
cargo run --quiet -p monad-cli -- graph --format=mermaid >/dev/null

echo "==> Running CLI graph DOT smoke test"
cargo run --quiet -p monad-cli -- graph --format=dot >/dev/null

echo "==> Running CLI context smoke test"
cargo run --quiet -p monad-cli -- context >/dev/null

echo "==> Running CLI context Markdown smoke test"
cargo run --quiet -p monad-cli -- context --format=markdown >/dev/null

echo "==> Running CLI context JSON smoke test"
cargo run --quiet -p monad-cli -- context --format=json >/dev/null

echo "==> Running CLI context md alias smoke test"
cargo run --quiet -p monad-cli -- context --format=md >/dev/null

echo "==> Running CLI context text alias smoke test"
cargo run --quiet -p monad-cli -- context --format=text >/dev/null

echo "Verification baseline passed."
