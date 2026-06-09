#!/usr/bin/env bash
set -euo pipefail

# Verify Monad add behavior safely in temporary directories.
#
# This script should be run from the Monad repository root.
# It writes only to temporary directories.
#
# It verifies:
# - add dry-run from the real Monad repo
# - init + add guarded write in a temp workspace
# - add conflict refusal in an initialized temp workspace

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

add_temp=""
conflict_temp=""
trap 'rm -rf "${add_temp:-}" "${conflict_temp:-}"' EXIT

echo "==> Verifying add dry-run from repository workspace"

dry_run_output="$(run_from_repo cargo run -p monad-cli -- add app web --dry-run)"
assert_output_contains "${dry_run_output}" "Monad add dry-run plan"
assert_output_contains "${dry_run_output}" "kind: app"
assert_output_contains "${dry_run_output}" "name: web"
assert_output_contains "${dry_run_output}" "apps/web/README.md"
assert_output_contains "${dry_run_output}" "No files were written."

echo "==> Verifying init + add --yes in temporary workspace"

add_temp="$(mktemp -d)"

run_from_temp "${add_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --yes
run_from_temp "${add_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- add app web --yes

assert_file_exists "${add_temp}/monad.toml"
assert_file_exists "${add_temp}/apps/web/README.md"
assert_file_exists "${add_temp}/apps/web/.gitkeep"

echo "==> Verifying add refuses existing component files"

conflict_temp="$(mktemp -d)"

run_from_temp "${conflict_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- init --yes
mkdir -p "${conflict_temp}/apps/api"
printf '# Existing API\n' > "${conflict_temp}/apps/api/README.md"

if run_from_temp "${conflict_temp}" cargo run --manifest-path "${CARGO_MANIFEST}" -p monad-cli -- add app api --yes; then
  echo "Expected add --yes to fail when apps/api/README.md already exists." >&2
  exit 1
fi

assert_file_exists "${conflict_temp}/apps/api/README.md"

echo
echo "Monad add verification passed."
