#!/usr/bin/env bash
set -Eeuo pipefail

printf '[verify-contract-schema] cargo test -p monad-core --lib contract_schema\n'
cargo test -p monad-core --lib contract_schema

printf '[verify-contract-schema] cargo run -p monad-cli -- contract --dry-run\n'
cargo run -p monad-cli -- contract --dry-run >/tmp/monad-contract-schema-dry-run.txt

grep -q 'Monad repo contract schema validation plan' /tmp/monad-contract-schema-dry-run.txt
grep -q 'Safety:' /tmp/monad-contract-schema-dry-run.txt

printf '[verify-contract-schema] cargo run -p monad-cli -- contract --dry-run --format=json\n'
cargo run -p monad-cli -- contract --dry-run --format=json >/tmp/monad-contract-schema-dry-run.json

grep -q '"command":"contract"' /tmp/monad-contract-schema-dry-run.json
grep -q '"remote_calls":false' /tmp/monad-contract-schema-dry-run.json
grep -q '"destructive_migrations":false' /tmp/monad-contract-schema-dry-run.json

printf '[verify-contract-schema] ok\n'
