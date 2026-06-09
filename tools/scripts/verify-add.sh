#!/usr/bin/env bash
set -euo pipefail

# Verify Monad add behavior.
#
# Run from the Monad repository root:
#
#   tools/scripts/verify-add.sh
#
# This script writes only to temporary directories.
#
# It verifies:
# - add dry-run from the real repository workspace;
# - add requires an initialized Monad workspace for writes;
# - init + add --yes works in a temporary workspace;
# - add --dry-run writes no component files;
# - add refuses existing component target files;
# - add rejects conflicting mode flags.

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

assert_path_missing() {
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

write_temp=""
dry_temp=""
conflict_temp=""
uninitialized_temp=""
trap 'rm -rf "${write_temp:-}" "${dry_temp:-}" "${conflict_temp:-}" "${uninitialized_temp:-}"' EXIT

echo "==> Verifying add dry-run from repository workspace"

dry_run_output="$(run_from_repo cargo run -p monad-cli -- add app web --dry-run)"
assert_output_contains "${dry_run_output}" "Monad add dry-run plan"
assert_output_contains "${dry_run_output}" "kind: app"
assert_output_contains "${dry_run_output}" "name: web"
assert_output_contains "${dry_run_output}" "apps/web/README.md"
assert_output_contains "${dry_run_output}" "No files were written."

echo "==> Verifying add write path requires a Monad workspace"

uninitialized_temp="$(mktemp -d)"

if run_from_temp "${uninitialized_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- add app web --yes; then
  echo "Expected add --yes to fail in an uninitialized temp directory." >&2
  exit 1
fi

assert_path_missing "${uninitialized_temp}/apps/web/README.md"
assert_path_missing "${uninitialized_temp}/apps/web/.gitkeep"

echo "==> Verifying init + add --yes in a temporary Monad workspace"

write_temp="$(mktemp -d)"

run_from_temp "${write_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --yes
add_output="$(run_from_temp "${write_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- add app web --yes)"

assert_output_contains "${add_output}" "Monad add applied"
assert_output_contains "${add_output}" "kind: app"
assert_output_contains "${add_output}" "name: web"
assert_output_contains "${add_output}" "No Git commands were run."

assert_file_exists "${write_temp}/monad.toml"
assert_file_exists "${write_temp}/apps/web/README.md"
assert_file_exists "${write_temp}/apps/web/.gitkeep"

echo "==> Verifying add --dry-run writes no component files"

dry_temp="$(mktemp -d)"

run_from_temp "${dry_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --yes
dry_temp_output="$(run_from_temp "${dry_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- add service api --dry-run)"

assert_output_contains "${dry_temp_output}" "Monad add dry-run plan"
assert_output_contains "${dry_temp_output}" "kind: service"
assert_output_contains "${dry_temp_output}" "services/api/README.md"
assert_output_contains "${dry_temp_output}" "No files were written."

assert_path_missing "${dry_temp}/services/api/README.md"
assert_path_missing "${dry_temp}/services/api/.gitkeep"

echo "==> Verifying add refuses existing component target files"

conflict_temp="$(mktemp -d)"

run_from_temp "${conflict_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --yes
mkdir -p "${conflict_temp}/apps/api"
printf '# Existing API\n' > "${conflict_temp}/apps/api/README.md"

if run_from_temp "${conflict_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- add app api --yes; then
  echo "Expected add --yes to fail when apps/api/README.md already exists." >&2
  exit 1
fi

assert_file_exists "${conflict_temp}/apps/api/README.md"
assert_path_missing "${conflict_temp}/apps/api/.gitkeep"

echo "==> Verifying add rejects conflicting mode flags"

if run_from_repo cargo run -p monad-cli -- add app web --dry-run --yes; then
  echo "Expected add --dry-run --yes to fail because modes conflict." >&2
  exit 1
fi

echo
echo "Monad add smoke verification passed."
