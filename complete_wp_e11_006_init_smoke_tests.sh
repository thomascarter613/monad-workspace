#!/usr/bin/env bash
set -euo pipefail

# Complete WP-E11-006 — Add init smoke tests and verification evidence.
#
# This packet adds a reusable init smoke verification script and evidence docs.
# It does not add new init behavior.

mkdir -p tools/scripts
mkdir -p docs/verification
mkdir -p work/deliverables/E11

cat > tools/scripts/verify-init.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
CARGO_MANIFEST="${REPO_ROOT}/Cargo.toml"

if [[ ! -f "${CARGO_MANIFEST}" ]]; then
  echo "Unable to locate Cargo manifest at ${CARGO_MANIFEST}" >&2
  exit 1
fi

run_from_repo() {
  (
    cd "${REPO_ROOT}"
    "$@"
  )
}

run_from_temp() {
  local temp_dir="$1"
  shift
  (
    cd "${temp_dir}"
    "$@"
  )
}

assert_file_exists() {
  local path="$1"
  if [[ ! -f "${path}" ]]; then
    echo "Expected file to exist: ${path}" >&2
    exit 1
  fi
}

assert_file_missing() {
  local path="$1"
  if [[ -e "${path}" ]]; then
    echo "Expected path to be missing: ${path}" >&2
    exit 1
  fi
}

assert_output_contains() {
  local output="$1"
  local expected="$2"
  if ! grep -Fq "${expected}" <<<"${output}"; then
    echo "Expected output to contain: ${expected}" >&2
    echo "Actual output:" >&2
    echo "${output}" >&2
    exit 1
  fi
}

basic_temp=""
polyglot_temp=""
conflict_temp=""
trap 'rm -rf "${basic_temp:-}" "${polyglot_temp:-}" "${conflict_temp:-}"' EXIT

echo "==> Verifying init dry-run commands"

basic_dry_run="$(run_from_repo cargo run -p monad-cli -- init --preset=basic --dry-run)"
assert_output_contains "${basic_dry_run}" "Monad init dry-run plan"
assert_output_contains "${basic_dry_run}" "preset: minimal"
assert_output_contains "${basic_dry_run}" "Template source:"
assert_output_contains "${basic_dry_run}" "No files were written."

minimal_dry_run="$(run_from_repo cargo run -p monad-cli -- init --preset=minimal --dry-run)"
assert_output_contains "${minimal_dry_run}" "Monad init dry-run plan"
assert_output_contains "${minimal_dry_run}" "preset: minimal"

polyglot_dry_run="$(run_from_repo cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run)"
assert_output_contains "${polyglot_dry_run}" "Monad init dry-run plan"
assert_output_contains "${polyglot_dry_run}" "preset: polyglot-minimal"
assert_output_contains "${polyglot_dry_run}" "apps/.gitkeep"
assert_output_contains "${polyglot_dry_run}" "packages/.gitkeep"
assert_output_contains "${polyglot_dry_run}" "services/.gitkeep"
assert_output_contains "${polyglot_dry_run}" "tools/.gitkeep"

echo "==> Verifying init --yes basic apply in temporary directory"

basic_temp="$(mktemp -d)"
run_from_temp "${basic_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --preset=basic --yes

assert_file_exists "${basic_temp}/monad.toml"
assert_file_exists "${basic_temp}/README.md"
assert_file_exists "${basic_temp}/docs/README.md"
assert_file_exists "${basic_temp}/work/README.md"
assert_file_exists "${basic_temp}/.monad/.gitignore"

echo "==> Verifying init --yes polyglot-minimal apply in temporary directory"

polyglot_temp="$(mktemp -d)"
run_from_temp "${polyglot_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --preset=polyglot-minimal --yes

assert_file_exists "${polyglot_temp}/monad.toml"
assert_file_exists "${polyglot_temp}/README.md"
assert_file_exists "${polyglot_temp}/docs/README.md"
assert_file_exists "${polyglot_temp}/work/README.md"
assert_file_exists "${polyglot_temp}/.monad/.gitignore"
assert_file_exists "${polyglot_temp}/apps/.gitkeep"
assert_file_exists "${polyglot_temp}/packages/.gitkeep"
assert_file_exists "${polyglot_temp}/services/.gitkeep"
assert_file_exists "${polyglot_temp}/tools/.gitkeep"

echo "==> Verifying init refuses conflicts"

conflict_temp="$(mktemp -d)"
printf '# Existing README\n' > "${conflict_temp}/README.md"

if run_from_temp "${conflict_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --yes; then
  echo "Expected init --yes to fail when README.md already exists." >&2
  exit 1
fi

assert_file_exists "${conflict_temp}/README.md"
assert_file_missing "${conflict_temp}/monad.toml"

echo "==> Verifying init rejects conflicting mode flags"

if run_from_repo cargo run -p monad-cli -- init --dry-run --yes; then
  echo "Expected init --dry-run --yes to fail because modes conflict." >&2
  exit 1
fi

echo "==> Verifying init help mentions presets"

help_output="$(run_from_repo cargo run -p monad-cli -- --help)"
assert_output_contains "${help_output}" "init --dry-run"
assert_output_contains "${help_output}" "init --preset=basic --dry-run"
assert_output_contains "${help_output}" "init --yes"

echo
echo "Monad init smoke verification passed."
EOF

chmod +x tools/scripts/verify-init.sh

cat > docs/verification/INIT-SMOKE-TESTS.md <<'EOF'
---
title: "Init Smoke Tests"
document_type: "verification-evidence"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E11
work_packet: WP-E11-006
tags:
  - monad
  - init
  - smoke-tests
  - verification
related:
  - tools/scripts/verify-init.sh
  - docs/commands/INIT.md
  - docs/commands/INIT-PRESETS.md
  - crates/monad-core/src/init.rs
  - crates/monad-cli/src/main.rs
---

# Init Smoke Tests

## Status

Accepted.

## Work Packet

WP-E11-006 — Add init smoke tests and verification evidence.

## Purpose

This document records the smoke-test evidence path for `monad init`.

The goal is to verify that the E11 init foundation behaves safely and predictably before closing the epic.

## Verification Script

The reusable smoke-test script is:

```text
tools/scripts/verify-init.sh
```

Run it from the repository root:

```bash
tools/scripts/verify-init.sh
```

## Coverage

The script verifies:

| Area | Coverage |
| --- | --- |
| Dry-run command | `monad init --preset=basic --dry-run` |
| Minimal alias | `monad init --preset=minimal --dry-run` |
| Polyglot dry-run | `monad init --preset=polyglot-minimal --dry-run` |
| Basic guarded write | `monad init --preset=basic --yes` in a temporary directory |
| Polyglot guarded write | `monad init --preset=polyglot-minimal --yes` in a temporary directory |
| Conflict refusal | Existing `README.md` blocks `init --yes` |
| Mode conflict | `init --dry-run --yes` fails |
| Help text | Help output mentions init, basic preset, and `--yes` |

## Safety Properties Verified

The smoke test verifies that:

- dry-run writes no files;
- guarded write creates the expected scaffold in an empty temp directory;
- guarded write refuses to overwrite existing target files;
- conflicting mode flags fail;
- polyglot-minimal creates expected monorepo placeholder directories;
- no verification command requires writing into the Monad repository itself.

## Expected Basic Scaffold

The `basic` preset creates:

```text
monad.toml
README.md
docs/README.md
work/README.md
.monad/.gitignore
```

## Expected Polyglot-Minimal Scaffold

The `polyglot-minimal` preset creates the basic scaffold plus:

```text
apps/.gitkeep
packages/.gitkeep
services/.gitkeep
tools/.gitkeep
```

## Full E11 Verification Command Set

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-init.sh
tools/scripts/verify.sh
git status --short
```

## Acceptance Criteria

WP-E11-006 is complete when:

- `tools/scripts/verify-init.sh` exists;
- the script verifies dry-run, guarded write, presets, conflict refusal, and help text;
- this verification evidence document exists;
- the E11 deliverable record exists;
- all normal repository verification commands pass.

## Outcome

Accepted.

Monad now has a reusable init smoke verification script and E11 verification evidence.
EOF

cat > work/deliverables/E11/WP-E11-006-init-smoke-tests-verification-evidence.md <<'EOF'
---
title: "WP-E11-006 Init Smoke Tests and Verification Evidence Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E11
work_packet: WP-E11-006
tags:
  - monad
  - e11
  - init
  - smoke-tests
  - verification
related:
  - tools/scripts/verify-init.sh
  - docs/verification/INIT-SMOKE-TESTS.md
  - docs/commands/INIT.md
  - docs/commands/INIT-PRESETS.md
---

# WP-E11-006 Init Smoke Tests and Verification Evidence Deliverable

## Work Packet

WP-E11-006 — Add init smoke tests and verification evidence.

## Outcome

Implemented.

## Summary

This work packet adds a reusable smoke-test script for the `monad init` foundation and records verification evidence.

The script verifies:

- dry-run behavior;
- basic preset behavior;
- minimal preset behavior;
- polyglot-minimal preset behavior;
- guarded write behavior in temporary directories;
- conflict refusal;
- conflicting mode flag rejection;
- help text coverage.

## Deliverables

- `tools/scripts/verify-init.sh`
- `docs/verification/INIT-SMOKE-TESTS.md`
- `work/deliverables/E11/WP-E11-006-init-smoke-tests-verification-evidence.md`

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify-init.sh
tools/scripts/verify.sh
git status --short
```

## Recommended Commit

```bash
git commit -m "test(init): add init smoke verification"
```

## Closeout Note

WP-E11-006 is complete once the init smoke verification script and evidence docs are committed and the corresponding GitHub work-packet issue or tracking item is closed.

After WP-E11-006 is closed, E11 may be closed.

## Next Epic

```text
E12 — Component Add and Polyglot Scaffold Foundation
```
EOF

echo
echo "WP-E11-006 files written:"
echo "  tools/scripts/verify-init.sh"
echo "  docs/verification/INIT-SMOKE-TESTS.md"
echo "  work/deliverables/E11/WP-E11-006-init-smoke-tests-verification-evidence.md"
echo
echo "Next verification:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-init.sh"
echo "  tools/scripts/verify.sh"
