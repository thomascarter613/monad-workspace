#!/usr/bin/env bash
set -Eeuo pipefail

printf '[verify-e22] checking E22 files\n'
test -f crates/monad-core/src/contract_schema.rs
test -f docs/contract-schema/README.md
test -f docs/roadmap/epic-22-repo-contract-schema-validation.md
test -x tools/scripts/verify-contract-schema.sh

grep -q 'pub mod contract_schema;' crates/monad-core/src/lib.rs
grep -q 'ContractSchemaPlan' crates/monad-core/src/lib.rs
grep -q 'Contract {' crates/monad-cli/src/main.rs
grep -q 'contract --dry-run' crates/monad-cli/src/main.rs

printf '[verify-e22] running contract schema verification\n'
tools/scripts/verify-contract-schema.sh

printf '[verify-e22] ok\n'
