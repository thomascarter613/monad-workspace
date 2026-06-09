#!/usr/bin/env bash
set -euo pipefail

echo "[verify-language-adapters] cargo test -p monad-core --lib language_adapters"
cargo test -p monad-core --lib language_adapters

echo "[verify-language-adapters] adapters dry-run smoke"
cargo run -q -p monad-cli -- adapters --dry-run > /tmp/monad-adapters.txt
grep -q "Monad language adapter registry" /tmp/monad-adapters.txt
grep -q "Rust" /tmp/monad-adapters.txt
grep -q "Node/Bun" /tmp/monad-adapters.txt
grep -q "Python" /tmp/monad-adapters.txt
grep -q "Go" /tmp/monad-adapters.txt
grep -q "Java" /tmp/monad-adapters.txt
grep -q "no commands were executed" /tmp/monad-adapters.txt

echo "[verify-language-adapters] adapters json smoke"
cargo run -q -p monad-cli -- adapters --dry-run --format=json > /tmp/monad-adapters.json
grep -q '"command":"adapters"' /tmp/monad-adapters.json
grep -q '"id":"rust"' /tmp/monad-adapters.json
grep -q '"id":"node-bun"' /tmp/monad-adapters.json
grep -q '"id":"python"' /tmp/monad-adapters.json
grep -q '"id":"go"' /tmp/monad-adapters.json
grep -q '"id":"java"' /tmp/monad-adapters.json

echo "[verify-language-adapters] ok"
