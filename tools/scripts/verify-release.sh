#!/usr/bin/env bash
set -euo pipefail

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

echo "==> verify-release: repo root: $REPO_ROOT"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

(
  cd "$tmpdir"

  echo "==> verify release dry-run in temp repo"
  cat > Cargo.toml <<'TOML'
[package]
name = "example"
version = "0.1.0"
edition = "2024"
TOML

  mkdir -p docs/release tools/scripts
  cat > docs/release/PUBLIC-PRERELEASE-NOTES.md <<'MD'
# Notes

## Implemented

- Example.

## Deferred

- Example.

## Not implemented

- Example.
MD
  cat > CHANGELOG.md <<'MD'
# Changelog

## Unreleased
MD
  touch tools/scripts/verify.sh
  touch tools/scripts/verify-e15.sh
  touch tools/scripts/package-release.sh

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- release --dry-run >/tmp/monad-e16-release.out
  grep -q "Monad release dry-run plan" /tmp/monad-e16-release.out
  grep -q "decision:" /tmp/monad-e16-release.out
  grep -q "No tags were created." /tmp/monad-e16-release.out
  grep -q "No packages were published." /tmp/monad-e16-release.out

  cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- release --dry-run --format=json >/tmp/monad-e16-release.json
  grep -q '"command":"release"' /tmp/monad-e16-release.json
  grep -q '"mode":"dry-run"' /tmp/monad-e16-release.json
  grep -q '"tag":"v0.1.0"' /tmp/monad-e16-release.json

  if cargo run --manifest-path "$REPO_ROOT/Cargo.toml" -p monad-cli -- release >/tmp/monad-e16-release-no-mode.out 2>&1; then
    echo "Expected release without --dry-run to fail" >&2
    exit 1
  fi
  grep -q "release currently requires --dry-run" /tmp/monad-e16-release-no-mode.out

  test ! -e .monad/releases
)

echo "==> verify package script syntax"
bash -n "$REPO_ROOT/tools/scripts/package-release.sh"

echo "verify-release: PASS"
