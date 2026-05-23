#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./tools/github/seed-e4-issues.sh OWNER REPO [PROJECT_NUMBER]
#
# Example:
#   ./tools/github/seed-e4-issues.sh thomascarter613 monad-workspace 1
#
# Notes:
# - This creates the E4 epic and E4 work packet issues.
# - It skips issues that already exist with the same exact title.
# - It optionally adds created/existing issues to a GitHub Project.
# - It does not set sub-issue hierarchy or custom project fields automatically.

OWNER="${1:?Missing GitHub owner, e.g. thomascarter613}"
REPO="${2:?Missing GitHub repo name, e.g. monad-workspace}"
PROJECT_NUMBER="${3:-}"

REPO_SLUG="${OWNER}/${REPO}"

echo "Seeding E4 issues into ${REPO_SLUG}"

if ! gh auth status >/dev/null 2>&1; then
  echo "GitHub CLI is not authenticated. Run:"
  echo "  gh auth login"
  exit 1
fi

if [[ -n "${PROJECT_NUMBER}" ]]; then
  echo "Project number provided: ${PROJECT_NUMBER}"
  echo "If project add fails, refresh GitHub CLI project scope with:"
  echo "  gh auth refresh -s project"
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "${TMP_DIR}"' EXIT

ensure_label() {
  local name="$1"
  local description="$2"
  local color="$3"

  if gh label list --repo "${REPO_SLUG}" --search "${name}" --json name --jq '.[].name' | grep -Fxq "${name}"; then
    return 0
  fi

  echo "Creating missing label: ${name}"
  gh label create "${name}" \
    --repo "${REPO_SLUG}" \
    --description "${description}" \
    --color "${color}" \
    >/dev/null
}

ensure_labels() {
  ensure_label "type:epic" "Large outcome composed of multiple work packets." "5319e7"
  ensure_label "type:work-packet" "Bounded delivery unit with objective, scope, deliverables, verification, and commit." "8250df"
  ensure_label "area:verification" "Checks, test orchestration, evidence packets, and verification reports." "1d76db"
  ensure_label "area:core" "Monad core runtime, shared domain logic, and foundational engine work." "1d76db"
  ensure_label "area:cli" "Command-line interface, command routing, help output, and CLI UX." "1d76db"
  ensure_label "area:repo-intelligence" "Repository inspection, tool detection, workspace discovery, and project graph work." "1d76db"
  ensure_label "priority:p1" "High priority; important for the current milestone or near-term progress." "d93f0b"
  ensure_label "needs-verification" "Requires test, check, review, or evidence before completion." "fbca04"
  ensure_label "context-update-required" "Requires Monad context, handoff, or current-state documentation update." "5319e7"
  ensure_label "rust-learning" "Introduces or explains Rust concepts as part of implementation." "dea584"
}

find_existing_issue_url() {
  local title="$1"

  gh issue list \
    --repo "${REPO_SLUG}" \
    --state all \
    --search "\"${title}\" in:title" \
    --json title,url \
    --limit 50 \
    --jq ".[] | select(.title == \"${title}\") | .url" \
    | head -n 1
}

add_to_project_if_requested() {
  local url="$1"

  if [[ -z "${PROJECT_NUMBER}" ]]; then
    return 0
  fi

  echo "Adding to GitHub Project ${PROJECT_NUMBER}: ${url}"

  gh project item-add "${PROJECT_NUMBER}" \
    --owner "${OWNER}" \
    --url "${url}" \
    >/dev/null || {
      echo "Warning: could not add ${url} to project ${PROJECT_NUMBER}."
      echo "You may need:"
      echo "  gh auth refresh -s project"
    }
}

create_issue() {
  if [[ "$#" -ne 3 ]]; then
    echo
    echo "Internal script error: create_issue expected 3 arguments but received $#."
    echo "Arguments received:"
    printf '  - %q\n' "$@"
    exit 1
  fi

  local title="${1:-}"
  local labels="${2:-}"
  local body_file="${3:-}"

  echo
  echo "Processing issue: ${title}"

  local existing_url
  existing_url="$(find_existing_issue_url "${title}")"

  if [[ -n "${existing_url}" ]]; then
    echo "Already exists: ${existing_url}"
    add_to_project_if_requested "${existing_url}"
    return 0
  fi

  local url
  url="$(gh issue create \
    --repo "${REPO_SLUG}" \
    --title "${title}" \
    --label "${labels}" \
    --body-file "${body_file}")"

  echo "Created: ${url}"
  add_to_project_if_requested "${url}"
}

ensure_labels

cat > "${TMP_DIR}/e4.md" <<'BODY'
## Product Area

Verification

## Objective

Build Monad’s verification foundation: a system that can define checks, run native commands, collect results, and produce evidence that supports trustworthy repository understanding and safe repository evolution.

This epic should turn Monad from a tool that can inspect a repository into a tool that can begin proving whether a repository, change, or generated improvement is healthy enough to review.

## User Value

This epic matters because Monad’s core promise is not merely to generate or describe software. Monad must help users trust the work being performed.

Users should be able to run Monad and receive clear evidence about what was checked, what passed, what failed, and what action may be needed next.

For maintainers and contributors, E4 establishes the foundation for repeatable quality gates, evidence packets, future repo audits, and later safe-evolution workflows.

## Scope

### In scope

- Check registry model.
- Check result model.
- Check severity/status model.
- Command runner foundation.
- `monad check` command.
- Verification report format.
- Evidence packet format.
- Adapter-specific check discovery foundation.
- JSON output for automation where appropriate.
- Tests for check execution and reporting behavior.

### Out of scope

- Full CI/CD replacement.
- Remote execution.
- Cloud sandboxing.
- Full security scanning suite.
- Full flaky-test detection.
- Deployment gates.
- Autonomous repair.
- Pull request automation.
- Enterprise policy engine.

## Expected Work Packets

- WP-E4-001 — Define check registry and result model
- WP-E4-002 — Add command runner
- WP-E4-003 — Add monad-workspace check command
- WP-E4-004 — Add evidence packet report
- WP-E4-005 — Add adapter-specific checks
- WP-E4-006 — Add JSON verification output

## Deliverables

- Check domain model.
- Check registry.
- Command execution wrapper.
- `monad check` command.
- Human-readable verification summary.
- JSON verification report if implemented by the end of the epic.
- Evidence packet structure.
- Tests for check execution and reporting.
- Clear exit-code behavior for successful and failed checks.

## Verification Strategy

Suggested verification commands:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-workspace-cli -- check
cargo run -p monad-workspace-cli -- check --format json
```

Expected result:

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `monad check` runs successfully once implemented.
* `monad check` reports which checks were run.
* Passing checks and failing checks are distinguishable.
* Exit codes are predictable.
* Verification output is understandable for humans.
* JSON output is valid and machine-readable if implemented by the end of the epic.
* Evidence packet output can be reviewed in Git or terminal output.

## Risks / Open Questions

* Command execution must be safe and transparent.
* Exit-code behavior must be predictable.
* Monad should not hide native tool output in ways that make debugging harder.
* Checks should not become too magical.
* Adapter-specific checks must be explainable.
* The first verification engine should stay local-first and simple.
* We must avoid implying that a passing check proves more than it actually proves.
* Evidence packet design may need refinement after evolution and agent workflows mature.

## Priority

P1 High

## Size

L
BODY

cat > "${TMP_DIR}/wp-e4-001.md" <<'BODY'

## Work Packet ID

WP-E4-001

## Parent Epic ID

E4

## Work Packet Title

Define check registry and result model

## Product Area

Verification

## Objective

Define Monad’s foundational verification model: checks, check groups, check statuses, check severity, check results, and a registry for available checks.

This work packet should create the vocabulary Monad uses to describe verification work before command execution is added.

## User Value

This work matters because Monad needs a consistent way to explain what it checked and what happened.

Users should eventually see clear verification output such as:

* which checks ran;
* which checks passed;
* which checks failed;
* which checks were skipped;
* what each result means;
* what the user may need to do next.

For maintainers, this creates a reusable verification foundation for `monad check`, context verification, adapter-specific checks, repo audits, and evidence packets.

## Scope

### In scope

* Define check identifier model.
* Define check metadata model.
* Define check status/outcome model.
* Define check severity if useful.
* Define check result model.
* Define basic check registry.
* Add tests for constructing check definitions and results.
* Keep the model small and beginner-readable.

### Out of scope

* Running external commands.
* `monad check` CLI implementation.
* JSON report rendering.
* Evidence packet rendering.
* Adapter-specific checks.
* CI integration.
* Policy engine.
* Auto-repair.

## Expected Files or Directories Affected

* `crates/monad-core/src/lib.rs`
* `crates/monad-core/src/checks.rs`
* `crates/monad-core/src/checks/model.rs`
* `crates/monad-core/src/checks/registry.rs`

## Tasks

* [ ] Create checks module boundary.
* [ ] Define check metadata type.
* [ ] Define check outcome/status type.
* [ ] Define check result type.
* [ ] Define basic check registry type.
* [ ] Add tests for check registration and result creation.
* [ ] Export checks module from `monad-core`.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic check-model commit.

## Deliverables

* Check registry model exists.
* Check result model exists.
* Check status/outcome model exists.
* Tests prove basic check registration and result construction.
* Future verification commands have a stable foundation.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
find crates/monad-core/src -maxdepth 4 -type f | sort
```

## Expected Result After Verification

* `cargo fmt --check` exits successfully.
* `cargo test` exits successfully.
* `cargo clippy --all-targets --all-features -- -D warnings` exits successfully.
* Checks module files exist.
* Tests prove check definitions can be created.
* Tests prove check results can be created.
* No external command execution behavior has been added yet.

## Definition of Done

* [ ] Check registry model exists.
* [ ] Check result model exists.
* [ ] Check status/outcome model exists.
* [ ] Tests cover basic model behavior.
* [ ] Formatting passes.
* [ ] Tests pass.
* [ ] Clippy passes with warnings denied.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core
git commit -m "feat(verification): add check registry model"
```

## Risks / Blockers / Open Questions

* The model should stay small until real checks create concrete needs.
* Avoid building a full policy engine inside the check model.
* Check names and IDs should be deterministic.
* The model should work for both human-readable and future machine-readable reports.

## Priority

P1 High

## Size

S
BODY

cat > "${TMP_DIR}/wp-e4-002.md" <<'BODY'

## Work Packet ID

WP-E4-002

## Parent Epic ID

E4

## Work Packet Title

Add command runner

## Product Area

Verification

## Objective

Add a safe, transparent command runner foundation that Monad can use to coordinate native tools such as `cargo fmt`, `cargo test`, package manager scripts, linters, and future verification commands.

This work packet should create the basic execution abstraction without attempting to replace native tools.

## User Value

This work matters because Monad’s verification value comes from coordinating trusted native tools and reporting their results clearly.

Users should be able to see what command Monad ran, where it ran, whether it succeeded, what exit code it returned, and enough output to debug failures.

## Scope

### In scope

* Define command specification type.
* Define command output/result type.
* Run external commands locally.
* Capture exit status.
* Capture stdout and stderr.
* Support working directory selection.
* Add tests for simple command execution where practical.
* Keep behavior transparent and explainable.

### Out of scope

* Sandboxed execution.
* Network restrictions.
* Secret redaction.
* Long-running daemon execution.
* Remote execution.
* Interactive PTY support.
* Streaming output UI.
* Retry/repair behavior.
* Agent-controlled command execution.

## Expected Files or Directories Affected

* `crates/monad-core/src/lib.rs`
* `crates/monad-core/src/exec.rs`
* `crates/monad-core/src/exec/command.rs`
* `crates/monad-core/src/exec/result.rs`

## Tasks

* [ ] Create exec module boundary.
* [ ] Define command specification type.
* [ ] Define command result type.
* [ ] Implement local command execution.
* [ ] Capture stdout.
* [ ] Capture stderr.
* [ ] Capture exit code/status.
* [ ] Add tests for a simple successful command.
* [ ] Add tests for a simple failing command if practical.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic command-runner commit.

## Deliverables

* Command runner exists.
* Command result captures status, stdout, and stderr.
* Working directory can be specified.
* Tests prove basic command execution behavior.
* Future checks can use the runner.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
find crates/monad-core/src -maxdepth 4 -type f | sort
```

## Expected Result After Verification

* `cargo fmt --check` exits successfully.
* `cargo test` exits successfully.
* `cargo clippy --all-targets --all-features -- -D warnings` exits successfully.
* Exec module files exist.
* Tests prove successful command execution works.
* Tests prove failed command behavior is represented clearly if included.
* No sandboxing, agent execution, or auto-repair behavior has been added yet.

## Definition of Done

* [ ] Command specification type exists.
* [ ] Command result type exists.
* [ ] Local command runner exists.
* [ ] Exit status is captured.
* [ ] Stdout is captured.
* [ ] Stderr is captured.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core
git commit -m "feat(exec): add command runner"
```

## Risks / Blockers / Open Questions

* Command execution is trust-sensitive and must remain transparent.
* We should not hide native tool failures.
* The first version should not attempt sandboxing.
* Cross-platform behavior may need refinement later.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e4-003.md" <<'BODY'

## Work Packet ID

WP-E4-003

## Parent Epic ID

E4

## Work Packet Title

Add monad-workspace check command

## Product Area

CLI

## Objective

Add the first `monad check` command that runs a small set of verification checks and reports the result to the user.

This work packet should connect the verification model and command runner to a user-facing CLI command.

## User Value

This work matters because `monad check` is the first clear trust-building command.

Users should be able to ask Monad to verify the repository and receive a clear pass/fail summary instead of manually remembering every native command.

## Scope

### In scope

* Add `check` command to the CLI.
* Connect command to core verification logic.
* Run an initial small check set.
* Report human-readable results.
* Return appropriate process exit code.
* Keep CLI thin and core-owned.
* Add tests where practical.

### Out of scope

* Full adapter-specific check discovery.
* Full JSON report.
* Evidence packet output.
* Full CI integration.
* Auto-fixing.
* Agent repair loops.
* Remote execution.
* Security scanning suite.

## Expected Files or Directories Affected

* `crates/monad-cli/src/cli.rs`
* `crates/monad-cli/src/commands.rs`
* `crates/monad-core/src/checks.rs`
* `crates/monad-core/src/checks/run.rs`
* `crates/monad-core/src/exec.rs`

## Tasks

* [ ] Add `check` command to CLI.
* [ ] Add command handler.
* [ ] Add core check runner function.
* [ ] Run initial checks using existing check model/command runner.
* [ ] Render human-readable output.
* [ ] Return success exit code when checks pass.
* [ ] Return failure exit code when required checks fail.
* [ ] Add tests where practical.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic check-command commit.

## Deliverables

* `monad check` command exists.
* Command runs initial verification checks.
* Command prints clear summary output.
* Exit-code behavior is predictable.
* CLI remains thin.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-workspace-cli -- check
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `cargo run -p monad-workspace-cli -- check` exits successfully when configured checks pass.
* Check output lists or summarizes the checks that ran.
* Required check failures produce non-zero exit behavior once failure scenarios are implemented.
* No evidence packet or JSON output is required in this packet.

## Definition of Done

* [ ] `check` command exists.
* [ ] Command connects to core verification logic.
* [ ] Human-readable output works.
* [ ] Exit-code behavior is defined.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-cli crates/monad-core
git commit -m "feat(cli): add check command"
```

## Risks / Blockers / Open Questions

* Initial checks should be useful but not overbroad.
* Exit-code behavior must be clear.
* CLI output should be simple and readable.
* Avoid turning `monad check` into a full CI system too early.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e4-004.md" <<'BODY'

## Work Packet ID

WP-E4-004

## Parent Epic ID

E4

## Work Packet Title

Add evidence packet report

## Product Area

Verification

## Objective

Add a reviewable evidence packet report that summarizes verification activity, check results, command outcomes, and remaining risks or failures.

This work packet should establish Monad’s first trust-through-evidence artifact.

## User Value

This work matters because users should not have to trust Monad blindly.

An evidence packet gives users, maintainers, reviewers, and future AI assistants a durable explanation of what was checked, what passed, what failed, and what remains unresolved.

## Scope

### In scope

* Define evidence packet structure.
* Include check results.
* Include command execution summaries.
* Include timestamp or generated metadata if appropriate.
* Include pass/fail summary.
* Include failed check details.
* Generate human-readable Markdown report.
* Add tests where practical.

### Out of scope

* Full JSON evidence schema.
* Signed attestations.
* SLSA provenance.
* SBOM generation.
* Cloud report storage.
* PR comment automation.
* Compliance reporting.
* Full audit log.

## Expected Files or Directories Affected

* `crates/monad-core/src/checks/evidence.rs`
* `crates/monad-core/src/checks/report.rs`
* `.monad/reports/` if reports are written to disk
* tests as appropriate

## Tasks

* [ ] Define evidence packet structure.
* [ ] Add Markdown rendering for evidence packet.
* [ ] Include check summary.
* [ ] Include failed check details.
* [ ] Include command summary where available.
* [ ] Decide report output path if writing to disk.
* [ ] Add tests for report rendering.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic evidence-report commit.

## Deliverables

* Evidence packet model exists.
* Human-readable evidence report can be generated.
* Report includes check outcomes.
* Report is reviewable and diff-friendly.
* Tests cover rendering or generation behavior.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-workspace-cli -- check
find .monad/reports -maxdepth 3 -type f | sort
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* Evidence packet code exists.
* Evidence report is generated if disk output is included.
* Report clearly summarizes passed and failed checks.
* Report is Markdown or otherwise human-readable.
* No signed provenance or compliance-grade attestation has been added yet.

## Definition of Done

* [ ] Evidence packet structure exists.
* [ ] Evidence report rendering exists.
* [ ] Check outcomes appear in the report.
* [ ] Failed checks are represented clearly.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core crates/monad-cli .monad/reports
git commit -m "feat(verification): add evidence packet report"
```

## Risks / Blockers / Open Questions

* Evidence reports should be concise enough to review.
* Generated metadata should not create unnecessary noisy diffs.
* We may need separate human-readable and machine-readable evidence later.
* Report path and retention policy may need refinement.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e4-005.md" <<'BODY'

## Work Packet ID

WP-E4-005

## Parent Epic ID

E4

## Work Packet Title

Add adapter-specific checks

## Product Area

Verification

## Objective

Add initial adapter-specific verification checks based on detected repository tooling, such as Rust/Cargo checks for Rust projects and package-manager checks for JavaScript projects.

This work packet should make verification more useful by connecting repo intelligence to check selection.

## User Value

This work matters because generic checks are not enough. Users expect Monad to understand what kind of repo it is working in and select relevant verification steps.

For example, a Rust repo should naturally run Rust checks, and a JavaScript repo should detect relevant package manager scripts or baseline commands.

## Scope

### In scope

* Connect repo intelligence/toolchain detection to check selection.
* Add initial Rust/Cargo checks.
* Add initial JavaScript/package manager checks if safe and clearly detectable.
* Skip unavailable checks clearly.
* Report why a check was selected or skipped where practical.
* Add fixture-based tests where practical.

### Out of scope

* Full ecosystem adapter framework.
* Every language ecosystem.
* Installing dependencies.
* Running destructive commands.
* Auto-fixing.
* Deep CI parity.
* Full dependency audit.

## Expected Files or Directories Affected

* `crates/monad-core/src/checks/adapters.rs`
* `crates/monad-core/src/checks/adapters/rust.rs`
* `crates/monad-core/src/checks/adapters/javascript.rs`
* `crates/monad-core/src/intelligence/`
* tests as appropriate

## Tasks

* [ ] Add adapter-specific check module boundary.
* [ ] Add Rust/Cargo check selection.
* [ ] Add JavaScript/package-manager check selection if feasible.
* [ ] Add skip behavior for unavailable checks.
* [ ] Connect detected toolchains to check registry.
* [ ] Add tests for selected and skipped checks.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic adapter-checks commit.

## Deliverables

* Adapter-specific checks exist.
* Rust/Cargo checks can be selected for Rust repos.
* JavaScript checks can be selected for supported JS repos if included.
* Skipped checks are represented clearly.
* Tests cover selection behavior.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-workspace-cli -- check
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `monad check` selects checks based on detected repo tooling.
* Rust/Cargo checks are selected in the Monad repo.
* Missing or unsupported checks are skipped or reported clearly.
* No package installation or destructive command behavior occurs.

## Definition of Done

* [ ] Adapter-specific check selection exists.
* [ ] Rust/Cargo checks are selected where appropriate.
* [ ] JavaScript checks are selected if included.
* [ ] Skip behavior is clear.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core crates/monad-cli
git commit -m "feat(verification): add adapter-specific checks"
```

## Risks / Blockers / Open Questions

* Adapter-specific checks can grow quickly; keep the first version small.
* Avoid running commands that modify files.
* JavaScript package manager behavior varies; detection should remain conservative.
* Check selection should be explainable.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e4-006.md" <<'BODY'

## Work Packet ID

WP-E4-006

## Parent Epic ID

E4

## Work Packet Title

Add JSON verification output

## Product Area

Verification

## Objective

Add machine-readable JSON output for verification results so Monad checks can be consumed by future automation, GitHub Actions, dashboards, agents, and context-generation workflows.

## User Value

This work matters because Monad should be useful both to humans and to tools.

Human-readable output helps developers in the terminal. JSON output enables automation, CI integration, future UI surfaces, and AI-assisted workflows that need structured evidence.

## Scope

### In scope

* Add serializable verification result model if not already present.
* Add `--format json` support to `monad check`.
* Output valid JSON.
* Include check IDs, names, statuses, messages, and command summaries where available.
* Add tests for JSON rendering or serialization.
* Keep schema simple and stable enough for near-term use.

### Out of scope

* Full public JSON schema commitment.
* OpenAPI contract.
* Hosted dashboard ingestion.
* Signed attestations.
* SBOM/provenance formats.
* SARIF output.
* JUnit XML output.
* GitHub Checks API integration.

## Expected Files or Directories Affected

* `crates/monad-core/src/checks/report.rs`
* `crates/monad-core/src/checks/json.rs`
* `crates/monad-cli/src/cli.rs`
* `crates/monad-cli/src/commands.rs`
* `crates/monad-core/Cargo.toml` if serialization dependencies are added
* tests as appropriate

## Tasks

* [ ] Add output format option if not already present.
* [ ] Add JSON verification report model.
* [ ] Add JSON rendering/serialization.
* [ ] Wire `monad check --format json`.
* [ ] Add tests for valid JSON output.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic JSON-output commit.

## Deliverables

* `monad check --format json` works.
* JSON output includes check result data.
* JSON output is valid.
* Tests cover JSON report behavior.
* Machine-readable verification output exists for future automation.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-workspace-cli -- check --format json
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `cargo run -p monad-workspace-cli -- check --format json` exits successfully when checks pass.
* The command prints valid JSON.
* JSON includes check identifiers, statuses, and messages.
* Human-readable output remains available as the default or explicit text format.
* No hosted reporting or external service integration has been added.

## Definition of Done

* [ ] JSON output option exists.
* [ ] JSON verification report is valid.
* [ ] Tests cover JSON output.
* [ ] Human-readable output still works.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-cli crates/monad-core
git commit -m "feat(verification): add json report output"
```

## Risks / Blockers / Open Questions

* JSON structure should be useful but not overcommitted as a permanent public API too early.
* Adding serialization dependencies should be justified.
* Output should avoid nondeterministic fields unless clearly needed.
* Future report formats such as SARIF or JUnit can wait.

## Priority

P1 High

## Size

M
BODY

create_issue \
"[Epic]: E4 — Verification Engine" \
"type:epic,area:verification,priority:p1,needs-verification,context-update-required" \
"${TMP_DIR}/e4.md"

create_issue \
"[Work Packet]: WP-E4-001 — Define check registry and result model" \
"type:work-packet,area:verification,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e4-001.md"

create_issue \
"[Work Packet]: WP-E4-002 — Add command runner" \
"type:work-packet,area:verification,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e4-002.md"

create_issue \
"[Work Packet]: WP-E4-003 — Add monad-workspace check command" \
"type:work-packet,area:verification,area:cli,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e4-003.md"

create_issue \
"[Work Packet]: WP-E4-004 — Add evidence packet report" \
"type:work-packet,area:verification,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e4-004.md"

create_issue \
"[Work Packet]: WP-E4-005 — Add adapter-specific checks" \
"type:work-packet,area:verification,area:repo-intelligence,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e4-005.md"

create_issue \
"[Work Packet]: WP-E4-006 — Add JSON verification output" \
"type:work-packet,area:verification,area:cli,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e4-006.md"

echo
echo "Done seeding E4."
echo
echo "Manual follow-up:"
echo "- Open the E4 epic issue in GitHub."
echo "- Add WP-E4-001 through WP-E4-006 as sub-issues under E4."
echo "- Set project fields if needed:"
echo "  Type, Status, Epic, Product Area, Priority, Size, Work Packet ID."
echo "- Recommended status for E4 and its work packets right now: Deferred or Ready, not Active."
