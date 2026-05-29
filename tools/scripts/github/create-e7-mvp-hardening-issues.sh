#!/usr/bin/env bash
set -euo pipefail

# Creates the next Monad issue set:
#
#   E7 — MVP Hardening
#   WP-E7-001 through WP-E7-006
#
# Requirements:
#   - GitHub CLI installed: gh
#   - authenticated with permission to create issues and labels
#   - run from anywhere
#
# Usage:
#   bash tools/scripts/github/create-e7-mvp-hardening-issues.sh
#
# Optional:
#   REPO=owner/name bash tools/scripts/github/create-e7-mvp-hardening-issues.sh

REPO="${REPO:-thomascarter613/monad-workspace}"

require_command() {
  local command_name="${1:-}"

  if [[ -z "$command_name" ]]; then
    echo "require_command called without a command name" >&2
    exit 1
  fi

  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "Required command not found: $command_name" >&2
    exit 1
  fi
}

ensure_label() {
  local name="${1:-}"
  local color="${2:-}"
  local description="${3:-}"

  if [[ -z "$name" || -z "$color" || -z "$description" ]]; then
    echo "ensure_label called with missing arguments" >&2
    exit 1
  fi

  gh label create "$name" \
    --repo "$REPO" \
    --color "$color" \
    --description "$description" \
    --force >/dev/null
}

create_issue() {
  local title="${1:-}"
  local body_file="${2:-}"

  if [[ -z "$title" || -z "$body_file" ]]; then
    echo "create_issue called with missing title/body_file" >&2
    exit 1
  fi

  shift 2

  local args=(
    issue create
    --repo "$REPO"
    --title "$title"
    --body-file "$body_file"
  )

  local label
  for label in "$@"; do
    args+=(--label "$label")
  done

  local output
  output="$(gh "${args[@]}")"

  if [[ -z "$output" ]]; then
    echo "gh issue create did not return an issue URL for: $title" >&2
    exit 1
  fi

  echo "$output"
}

issue_number_from_url() {
  local issue_url="${1:-}"

  if [[ -z "$issue_url" ]]; then
    echo "issue_number_from_url called without an issue URL" >&2
    exit 1
  fi

  echo "${issue_url##*/}"
}

require_command gh

echo "Creating labels in $REPO..."

ensure_label "type:epic" "5319e7" "Large outcome composed of multiple work packets."
ensure_label "type:work-packet" "8250df" "Bounded delivery unit with objective, scope, deliverables, verification, and commit."
ensure_label "area:cli" "1d76db" "Command-line interface, command routing, help output, and CLI UX."
ensure_label "area:docs" "1d76db" "Documentation, README, guides, tutorials, and repo-native written artifacts."
ensure_label "area:verification" "1d76db" "Checks, evidence, validation, reporting, and verification workflows."
ensure_label "area:testing" "1d76db" "Unit tests, integration tests, smoke tests, regression tests, and test infrastructure."
ensure_label "area:evolution" "1d76db" "Safe repository changes, generators, migrations, file operations, and apply workflows."
ensure_label "area:release" "1d76db" "Release readiness, versioning, packaging, changelog, and distribution preparation."
ensure_label "area:policy" "1d76db" "Policy, governance, safety boundaries, approvals, and enforcement rules."
ensure_label "priority:p1" "d93f0b" "High priority; important for the current milestone or near-term progress."
ensure_label "status:ready" "0e8a16" "Clear, scoped, and ready to start."
ensure_label "needs-verification" "fbca04" "Requires test, check, review, or evidence before completion."
ensure_label "context-update-required" "5319e7" "Requires Monad context, handoff, or current-state documentation update."
ensure_label "mvp-hardening" "0052cc" "MVP hardening, stabilization, polish, and release-readiness work."

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

EPIC_BODY="$TMPDIR/E7-mvp-hardening.md"

cat > "$EPIC_BODY" <<'BODY'
## Product Area

MVP Hardening

## Objective

Stabilize Monad’s current E0–E6 foundation into a coherent local-first CLI MVP by tightening command behavior, verification, documentation, repo contracts, safety guarantees, and release-readiness without expanding into major new feature areas.

This epic follows the foundation closure decision to choose **Option A — MVP Hardening** before adding more feature breadth.

## User Value

Users should be able to run Monad locally, understand what it does, trust its no-write and dry-run boundaries, read accurate documentation, and see reliable verification results.

This epic turns the initial foundation into a more usable, testable, and reviewable CLI baseline.

## Scope

### In scope

- Foundation closure audit.
- CLI help and command UX normalization.
- CLI smoke test hardening.
- Documentation alignment with implemented behavior.
- Dry-run and no-write guarantee hardening.
- MVP readiness reporting.
- Release-readiness gap identification.
- Verification evidence for the current command surface.

### Out of scope

- New autonomous agent behavior.
- Real model-provider integrations.
- Full MCP server implementation.
- Apply/write evolution commands.
- Remote Git operations.
- Deployment automation.
- Marketplace distribution.
- Enterprise RBAC or SSO.
- Hosted cloud control plane.

## Current Command Surface to Harden

```bash
cargo run -p monad-cli -- --help
cargo run -p monad-cli -- version
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- check
cargo run -p monad-cli -- plan "explain this repository"
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run
````

If the local package name differs, use the actual Cargo package name.

## Expected Work Packets

* WP-E7-001 — Run foundation closure audit
* WP-E7-002 — Normalize CLI help and command UX
* WP-E7-003 — Harden command smoke tests
* WP-E7-004 — Align documentation with implemented behavior
* WP-E7-005 — Harden dry-run and no-write guarantees
* WP-E7-006 — Create MVP readiness report

## Deliverables

* Foundation closure audit results.
* Normalized CLI help and error behavior.
* Smoke tests for current core commands.
* Documentation aligned to actual implementation.
* Dry-run/no-write guarantees documented and tested.
* MVP readiness report.
* Clear list of remaining pre-MVP blockers.

## Verification Strategy

Suggested verification commands:

```bash
git status --short
git log --oneline --max-count=12

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings

cargo run -p monad-cli -- --help
cargo run -p monad-cli -- version
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- check
cargo run -p monad-cli -- plan "explain this repository"
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run

tools/scripts/verify.sh
```

Expected result:

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* Current command surface runs successfully or has documented blockers.
* Dry-run commands do not write repository files.
* Documentation matches actual command behavior.
* MVP readiness gaps are captured as actionable follow-up work.

## Risks / Open Questions

* Documentation may be ahead of implementation.
* CLI help may not match actual command behavior.
* Tests may not cover failure states.
* Package-name mismatch may exist between docs and Cargo package names.
* Dry-run/no-write guarantees need stronger automated proof.
* MVP scope must remain narrow enough to finish.

## Priority

P1 High

## Size

L
BODY

if [[ -n "${E7_EPIC_URL:-}" ]]; then
  EPIC_URL="$E7_EPIC_URL"
  EPIC_NUMBER="$(issue_number_from_url "$EPIC_URL")"
  echo "Using existing E7 epic: $EPIC_URL"
else
  echo "Creating E7 epic..."
  EPIC_URL="$(gh issue create \
    --repo "$REPO" \
    --title "[Epic]: E7 — MVP Hardening" \
    --body-file "$EPIC_BODY" \
    --label "type:epic" \
    --label "area:cli" \
    --label "area:docs" \
    --label "area:verification" \
    --label "area:testing" \
    --label "area:release" \
    --label "priority:p1" \
    --label "status:ready" \
    --label "needs-verification" \
    --label "context-update-required" \
    --label "mvp-hardening")"

  EPIC_NUMBER="$(issue_number_from_url "$EPIC_URL")"

  echo "Created E7 epic: $EPIC_URL"
fi

WP1_BODY="$TMPDIR/WP-E7-001-foundation-closure-audit.md"
cat > "$WP1_BODY" <<'BODY'

## Work Packet ID

WP-E7-001

## Parent Epic ID

E7

## Parent Epic Issue

#$EPIC_NUMBER — $EPIC_URL

## Work Packet Title

Run foundation closure audit

## Product Area

MVP Hardening / Verification

## Objective

Run the full foundation closure verification sweep and record the result so Monad has a factual baseline before further MVP hardening work begins.

This work packet should determine whether E0–E6 are locally coherent, whether all current commands run, and what immediate blockers remain.

## User Value

Users and maintainers need a truthful baseline before calling the foundation complete or moving toward MVP readiness.

This packet prevents the project from building more work on top of unknown failures.

## Scope

### In scope

* Run closure verification commands.
* Record command results.
* Identify blockers.
* Identify warnings or inconsistencies.
* Update the foundation closure report or create an audit appendix.
* Recommend immediate repairs if needed.

### Out of scope

* Major feature development.
* New command implementation.
* New agent behavior.
* Full release packaging.

## Expected Files or Directories Affected

* `docs/project/FOUNDATION-CLOSURE-REPORT.md`
* `docs/project/FOUNDATION-CLOSURE-AUDIT.md` if a separate audit artifact is preferred
* possibly `.monad/context/` if context state needs updating

## Tasks

* [ ] Run full closure command sweep.
* [ ] Record formatting result.
* [ ] Record test result.
* [ ] Record Clippy result.
* [ ] Record CLI smoke test results.
* [ ] Record dry-run command results.
* [ ] Record `tools/scripts/verify.sh` result.
* [ ] Identify blockers.
* [ ] Identify immediate next fixes.
* [ ] Commit as one atomic closure audit commit.

## Verification Commands / Evidence

```bash
git status --short
git log --oneline --max-count=12

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings

cargo run -p monad-cli -- --help
cargo run -p monad-cli -- version
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- check
cargo run -p monad-cli -- plan "explain this repository"
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run

tools/scripts/verify.sh
```

## Expected Result After Verification

* Audit artifact records actual results.
* Any failures are documented.
* No failures are hidden.
* The next hardening packet has a reliable starting point.

## Definition of Done

* [ ] Closure audit exists.
* [ ] Verification commands were run or explicitly documented as blocked.
* [ ] Results are recorded.
* [ ] Blockers are listed.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add docs/project .monad/context
git commit -m "docs(project): record foundation closure audit"
```

## Risks / Blockers / Open Questions

* Some commands may fail due to package-name mismatch.
* Documentation may not match implemented behavior.
* Verification script may require paths not yet updated after E6.

## Priority

P1 High

## Size

S
BODY

WP2_BODY="$TMPDIR/WP-E7-002-cli-help-ux.md"
cat > "$WP2_BODY" <<'BODY'

## Work Packet ID

WP-E7-002

## Parent Epic ID

E7

## Parent Epic Issue

#$EPIC_NUMBER — $EPIC_URL

## Work Packet Title

Normalize CLI help and command UX

## Product Area

CLI UX

## Objective

Normalize Monad CLI help output, command examples, unsupported flag behavior, missing-argument errors, and user-facing messages for the current MVP command surface.

## User Value

Users should be able to discover commands, understand what each command does, and recover from invalid usage without reading the source code.

## Scope

### In scope

* Review `--help` output.
* Ensure current commands are listed.
* Ensure examples work.
* Normalize missing argument errors.
* Normalize unsupported flag errors.
* Ensure dry-run language is clear.
* Keep output conservative and honest.

### Out of scope

* New major commands.
* External provider integrations.
* Shell completion generation unless already trivial.
* Full CLI redesign.

## Expected Files or Directories Affected

* `crates/monad-cli/src/main.rs`
* CLI tests if already present or newly added
* docs that reference command usage

## Tasks

* [ ] Audit current help output.
* [ ] Audit invalid command behavior.
* [ ] Audit missing argument behavior.
* [ ] Audit unsupported flag behavior.
* [ ] Normalize command examples.
* [ ] Ensure no command implies unimplemented behavior.
* [ ] Add tests where practical.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic CLI UX commit.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings

cargo run -p monad-cli -- --help
cargo run -p monad-cli -- plan
cargo run -p monad-cli -- plan "explain this repository"
cargo run -p monad-cli -- evolve verify-baseline
cargo run -p monad-cli -- evolve verify-baseline --dry-run
```

## Expected Result After Verification

* Help output lists current commands accurately.
* Missing arguments produce useful errors.
* Unsupported flags produce useful errors.
* Dry-run-only commands clearly require `--dry-run`.
* No command writes files unintentionally.

## Definition of Done

* [ ] CLI help is accurate.
* [ ] CLI errors are understandable.
* [ ] Examples work.
* [ ] Tests pass.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-cli docs
git commit -m "fix(cli): normalize help and command ux"
```

## Risks / Blockers / Open Questions

* Current CLI parser may be too monolithic and may need later refactoring.
* Package-name examples must match actual Cargo package names.

## Priority

P1 High

## Size

M
BODY

WP3_BODY="$TMPDIR/WP-E7-003-command-smoke-tests.md"
cat > "$WP3_BODY" <<'BODY'

## Work Packet ID

WP-E7-003

## Parent Epic ID

E7

## Parent Epic Issue

#$EPIC_NUMBER — $EPIC_URL

## Work Packet Title

Harden command smoke tests

## Product Area

Testing / Verification

## Objective

Add or strengthen command smoke tests for Monad's current MVP command surface so regressions are caught before release-readiness work.

## User Value

Users need confidence that the main commands run consistently and safely after each change.

## Scope

### In scope

* Smoke tests for help/version.
* Smoke tests for inspect.
* Smoke tests for check.
* Smoke tests for plan.
* Smoke tests for dry-run evolution commands.
* Failure tests for missing arguments where practical.
* Confirm commands do not unexpectedly write files.

### Out of scope

* Full end-to-end test framework.
* Real model provider tests.
* MCP server tests.
* Remote Git tests.
* Deployment tests.

## Expected Files or Directories Affected

* `crates/monad-cli/`
* `crates/monad-core/` if helper test utilities are needed
* `tests/` if integration tests are preferred
* `tools/scripts/verify.sh` if smoke tests are integrated into verification

## Tasks

* [ ] Identify current command test coverage.
* [ ] Add smoke tests for command success paths.
* [ ] Add tests for important failure paths.
* [ ] Add no-write assertions where practical.
* [ ] Ensure tests run in CI/local verification.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic smoke-test commit.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

## Expected Result After Verification

* Core command smoke tests pass.
* Missing argument behavior is covered where practical.
* Dry-run command behavior is covered where practical.
* No real provider or external service is required.

## Definition of Done

* [ ] Smoke tests exist.
* [ ] Tests pass.
* [ ] Clippy passes.
* [ ] No external service is required.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates tests tools
git commit -m "test(cli): harden command smoke coverage"
```

## Risks / Blockers / Open Questions

* Tests must avoid depending on user-specific absolute paths.
* Tests must avoid modifying the real working tree.
* Current CLI parser may need small refactors to become testable.

## Priority

P1 High

## Size

M
BODY

WP4_BODY="$TMPDIR/WP-E7-004-doc-alignment.md"
cat > "$WP4_BODY" <<'BODY'

## Work Packet ID

WP-E7-004

## Parent Epic ID

E7

## Parent Epic Issue

#$EPIC_NUMBER — $EPIC_URL

## Work Packet Title

Align documentation with implemented behavior

## Product Area

Documentation

## Objective

Update Monad documentation so it accurately describes the implemented current command surface, safety boundaries, and MVP hardening state.

## User Value

Users and future contributors should not have to guess which parts of Monad are implemented, aspirational, or planned for later.

## Scope

### In scope

* README alignment.
* Getting-started command path.
* Current command reference.
* Safety boundary clarification.
* Mark aspirational content clearly.
* Align package names in docs.
* Align verification commands.

### Out of scope

* Full public website.
* Marketing copy.
* Deep tutorial series.
* API reference generation unless already available.

## Expected Files or Directories Affected

* `README.md`
* `docs/project/FOUNDATION-CLOSURE-REPORT.md`
* `docs/architecture/`
* `docs/security/`
* `docs/ai/`
* `docs/workflow/`

## Tasks

* [ ] Review README for accuracy.
* [ ] Review command examples.
* [ ] Review safety docs for implemented-vs-future clarity.
* [ ] Review agent/MCP docs for overclaims.
* [ ] Review verification docs.
* [ ] Update inaccurate package names.
* [ ] Add current command reference if missing.
* [ ] Commit as one atomic docs alignment commit.

## Verification Commands / Evidence

```bash
find README.md docs -maxdepth 4 -type f | sort
grep -R "monad-workspace-cli" README.md docs || true
grep -R "cargo run -p monad-cli" README.md docs || true
tools/scripts/verify.sh
```

## Expected Result After Verification

* Documentation matches current implementation.
* Aspirational future behavior is labeled.
* Current CLI examples are accurate.
* Verification commands are consistent.

## Definition of Done

* [ ] README is aligned.
* [ ] Command docs are aligned.
* [ ] Future claims are clearly marked.
* [ ] Verification commands are accurate.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add README.md docs
git commit -m "docs: align mvp behavior and command reference"
```

## Risks / Blockers / Open Questions

* Some docs intentionally define future architecture.
* Avoid deleting strategic docs; label future scope instead.

## Priority

P1 High

## Size

M
BODY

WP5_BODY="$TMPDIR/WP-E7-005-dry-run-no-write.md"
cat > "$WP5_BODY" <<'BODY'

## Work Packet ID

WP-E7-005

## Parent Epic ID

E7

## Parent Epic Issue

#$EPIC_NUMBER — $EPIC_URL

## Work Packet Title

Harden dry-run and no-write guarantees

## Product Area

Evolution / Safety

## Objective

Strengthen the proof that dry-run and planning commands do not write files, mutate Git state, or perform external side effects.

## User Value

Users must be able to trust Monad's dry-run and planning modes before they trust future apply or agent-assisted workflows.

## Scope

### In scope

* Audit dry-run commands.
* Audit `monad plan`.
* Add no-write tests where practical.
* Add documentation notes for dry-run behavior.
* Confirm dry-run wording is explicit.
* Confirm no Git mutation occurs.

### Out of scope

* Implement apply command.
* Implement file writes.
* Implement branch/worktree creation.
* Implement remote side effects.
* Implement autonomous agents.

## Expected Files or Directories Affected

* `crates/monad-core/src/file_ops/`
* `crates/monad-core/src/evolution/`
* `crates/monad-core/src/agents/`
* `crates/monad-cli/src/main.rs`
* tests and docs as appropriate

## Tasks

* [ ] Identify all current dry-run/planning commands.
* [ ] Add no-write assertions where practical.
* [ ] Add tests for conflict/no-op behavior where practical.
* [ ] Ensure dry-run output says no files were written.
* [ ] Ensure plan output says no commands were run.
* [ ] Ensure no Git mutation occurs.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic safety hardening commit.

## Verification Commands / Evidence

```bash
git status --short

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings

cargo run -p monad-cli -- plan "explain this repository"
cargo run -p monad-cli -- evolve verify-baseline --dry-run
cargo run -p monad-cli -- evolve context-baseline --dry-run

git status --short
```

## Expected Result After Verification

* Planning command does not write files.
* Dry-run evolution commands do not write files.
* Git status does not change due to planning/dry-run commands.
* Tests prove no-write behavior where practical.

## Definition of Done

* [ ] No-write guarantees are tested or documented.
* [ ] Dry-run language is clear.
* [ ] Planning language is clear.
* [ ] Tests pass.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates docs
git commit -m "test(evolution): harden dry-run no-write guarantees"
```

## Risks / Blockers / Open Questions

* Some no-write behavior may need integration test harness support.
* Git status checks must avoid false positives from intentional source changes.

## Priority

P1 High

## Size

M
BODY

WP6_BODY="$TMPDIR/WP-E7-006-mvp-readiness-report.md"
cat > "$WP6_BODY" <<'BODY'

## Work Packet ID

WP-E7-006

## Parent Epic ID

E7

## Parent Epic Issue

#$EPIC_NUMBER — $EPIC_URL

## Work Packet Title

Create MVP readiness report

## Product Area

Release Readiness

## Objective

Create a concrete MVP readiness report that summarizes what is ready, what remains blocked, and what must happen before Monad can be treated as an MVP candidate.

## User Value

The project needs a clear release-readiness decision point instead of endlessly adding features.

## Scope

### In scope

* Summarize E7 hardening results.
* Identify MVP-ready features.
* Identify non-MVP future features.
* Identify release blockers.
* Define final MVP cut line.
* Recommend next milestone after MVP hardening.

### Out of scope

* Actual public release.
* Package publishing.
* Installer generation.
* Hosted service launch.
* Marketing launch.

## Expected Files or Directories Affected

* `docs/project/MVP-READINESS-REPORT.md`
* `docs/project/FOUNDATION-CLOSURE-REPORT.md` if status needs updating
* `.monad/context/` if context state needs updating

## Tasks

* [ ] Create MVP readiness report.
* [ ] Summarize current implemented command surface.
* [ ] Summarize verification status.
* [ ] Summarize known blockers.
* [ ] Define MVP cut line.
* [ ] Define post-MVP work recommendations.
* [ ] Commit as one atomic readiness-report commit.

## Verification Commands / Evidence

```bash
find docs/project .monad/context -maxdepth 4 -type f | sort
git status --short
tools/scripts/verify.sh
```

## Expected Result After Verification

* MVP readiness report exists.
* Report clearly distinguishes ready, blocked, and future work.
* Next milestone is clear.
* No code changes are required unless discovered during readiness work.

## Definition of Done

* [ ] MVP readiness report exists.
* [ ] Ready features are listed.
* [ ] Blockers are listed.
* [ ] Future work is listed.
* [ ] MVP cut line is defined.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add docs/project .monad/context
git commit -m "docs(project): add mvp readiness report"
```

## Risks / Blockers / Open Questions

* Report should be honest and not overstate readiness.
* Some E7 blockers may require follow-up work before MVP.

## Priority

P1 High

## Size

S
BODY


python3 - "$EPIC_NUMBER" "$EPIC_URL" \
  "$WP1_BODY" "$WP2_BODY" "$WP3_BODY" "$WP4_BODY" "$WP5_BODY" "$WP6_BODY" <<'PY_REPLACE_EPIC'
from pathlib import Path
import sys

epic_number, epic_url, *body_files = sys.argv[1:]

for body_file in body_files:
    path = Path(body_file)
    text = path.read_text()
    text = text.replace("#$EPIC_NUMBER", f"#{epic_number}")
    text = text.replace("$EPIC_URL", epic_url)
    path.write_text(text)
PY_REPLACE_EPIC

echo "Creating E7 work packets..."

WP1_URL="$(create_issue 
"[Work Packet]: WP-E7-001 — Run foundation closure audit" 
"$WP1_BODY" 
"type:work-packet" 
"area:verification" 
"area:docs" 
"priority:p1" 
"status:ready" 
"needs-verification" 
"context-update-required" 
"mvp-hardening")"

WP2_URL="$(create_issue 
"[Work Packet]: WP-E7-002 — Normalize CLI help and command UX" 
"$WP2_BODY" 
"type:work-packet" 
"area:cli" 
"priority:p1" 
"status:ready" 
"needs-verification" 
"mvp-hardening")"

WP3_URL="$(create_issue 
"[Work Packet]: WP-E7-003 — Harden command smoke tests" 
"$WP3_BODY" 
"type:work-packet" 
"area:testing" 
"area:verification" 
"area:cli" 
"priority:p1" 
"status:ready" 
"needs-verification" 
"mvp-hardening")"

WP4_URL="$(create_issue 
"[Work Packet]: WP-E7-004 — Align documentation with implemented behavior" 
"$WP4_BODY" 
"type:work-packet" 
"area:docs" 
"priority:p1" 
"status:ready" 
"needs-verification" 
"context-update-required" 
"mvp-hardening")"

WP5_URL="$(create_issue 
"[Work Packet]: WP-E7-005 — Harden dry-run and no-write guarantees" 
"$WP5_BODY" 
"type:work-packet" 
"area:evolution" 
"area:policy" 
"area:testing" 
"priority:p1" 
"status:ready" 
"needs-verification" 
"mvp-hardening")"

WP6_URL="$(create_issue 
"[Work Packet]: WP-E7-006 — Create MVP readiness report" 
"$WP6_BODY" 
"type:work-packet" 
"area:release" 
"area:docs" 
"priority:p1" 
"status:ready" 
"needs-verification" 
"context-update-required" 
"mvp-hardening")"

cat <<SUMMARY

Created E7 issue set in $REPO:

Epic:
E7 — $EPIC_URL

Work packets:
WP-E7-001 — $WP1_URL
WP-E7-002 — $WP2_URL
WP-E7-003 — $WP3_URL
WP-E7-004 — $WP4_URL
WP-E7-005 — $WP5_URL
WP-E7-006 — $WP6_URL

Recommended next command:
gh issue view "$(issue_number_from_url "$WP1_URL")" --repo "$REPO"

SUMMARY
