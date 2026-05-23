#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./tools/github/seed-e2-issues.sh OWNER REPO [PROJECT_NUMBER]
#
# Example:
#   ./tools/github/seed-e2-issues.sh thomascarter613 monad-workspace 1
#
# Notes:
# - This creates the E2 epic and E2 work packet issues.
# - It skips issues that already exist with the same exact title.
# - It optionally adds created/existing issues to a GitHub Project.
# - It does not set sub-issue hierarchy or custom project fields automatically.

OWNER="${1:?Missing GitHub owner, e.g. thomascarter613}"
REPO="${2:?Missing GitHub repo name, e.g. monad-workspace}"
PROJECT_NUMBER="${3:-}"

REPO_SLUG="${OWNER}/${REPO}"

echo "Seeding E2 issues into ${REPO_SLUG}"

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
  ensure_label "area:repo-intelligence" "Repository inspection, tool detection, workspace discovery, and project graph work." "1d76db"
  ensure_label "area:cli" "Command-line interface, command routing, help output, and CLI UX." "1d76db"
  ensure_label "area:core" "Monad core runtime, shared domain logic, and foundational engine work." "1d76db"
  ensure_label "priority:p0" "Critical; blocks progress or must be handled immediately." "b60205"
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

cat > "${TMP_DIR}/e2.md" <<'BODY'
## Product Area

Repo Intelligence

## Objective

Teach Monad to inspect a repository and produce a clear, useful description of what it finds: languages, package managers, workspace structure, scripts, toolchains, services, packages, and an initial project graph.

This epic should produce Monad’s first meaningful repository-understanding capability: a user can run a command in a repo and receive a deterministic, explainable summary of the project’s detected structure.

## User Value

This epic is the first major user-facing “magic” of Monad.

A user should be able to run Monad in an unfamiliar repository and quickly understand what kind of project it is, which tools it uses, which manifests matter, and where future work should begin.

For maintainers, consultants, contributors, and AI assistants, E2 creates the foundation for onboarding, repo audits, context generation, verification planning, and later safe-evolution workflows.

## Scope

### In scope

- Repository inspection model.
- Toolchain detection model.
- Package manager detection.
- Workspace/package detection.
- Script/task discovery.
- Initial project graph model.
- Human-readable inspection output.
- JSON output for automation where appropriate.
- Initial graph output foundation.
- Fixture-based tests for supported detection scenarios.

### Out of scope

- Full static analysis.
- Full language-server integration.
- Full dependency impact analysis.
- Deep semantic code understanding.
- Full security audit.
- Full CI optimization.
- AI agent planning.
- Automated repo modification.
- Cloud indexing or hosted repo analysis.

## Expected Work Packets

- WP-E2-001 — Add toolchain detection model
- WP-E2-002 — Detect Node and JavaScript package managers
- WP-E2-003 — Detect Rust Cargo workspaces
- WP-E2-004 — Add inspect command report
- WP-E2-005 — Add basic project graph model
- WP-E2-006 — Add graph output formats

## Deliverables

- Repo inspection domain model.
- Toolchain detection result types.
- Package manager detection.
- Basic Node/Bun/npm/pnpm/yarn detection.
- Basic Cargo workspace detection.
- `monad inspect` command.
- Initial project graph structure.
- Graph output in at least text and JSON.
- Tests for sample fixture repositories.
- Deterministic output suitable for future context generation and verification.

## Verification Strategy

Suggested verification commands:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- inspect
cargo run -p monad-cli -- inspect --format json
```

Expected result:

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `monad inspect` runs successfully in the Monad repo.
* `monad inspect --format json` produces valid machine-readable output if JSON output has been implemented by the end of the epic.
* Monad can detect at least its own Rust workspace structure.
* Monad can detect supported JavaScript package manager markers in fixtures.
* Initial graph-related output is deterministic.

## Risks / Open Questions

* Tool detection can become messy if too many ecosystems are added too early.
* Detection should be conservative and explain uncertainty.
* Fixture repositories must be small but realistic.
* Graph output must remain deterministic.
* We need to avoid pretending to understand more than we actually detect.
* JSON schemas may need refinement after context and verification features mature.
* The first version should prioritize correctness and explainability over breadth.

## Priority

P1 High

## Size

L
BODY

cat > "${TMP_DIR}/wp-e2-001.md" <<'BODY'

## Work Packet ID

WP-E2-001

## Parent Epic ID

E2

## Work Packet Title

Add toolchain detection model

## Product Area

Repo Intelligence

## Objective

Define the core model Monad uses to represent detected languages, runtimes, package managers, manifests, and toolchains.

This work packet should create the types and vocabulary needed for later detection work without yet implementing every ecosystem-specific detector.

## User Value

This work matters because Monad needs a stable way to describe what it finds in a repository.

For users, this eventually enables clear output such as “this repo uses Rust with Cargo and JavaScript with Bun.” For maintainers, it gives future detectors a shared model instead of each command inventing its own representation.

## Scope

### In scope

* Define toolchain-related domain types.
* Represent detected language/ecosystem.
* Represent detected package manager.
* Represent detected manifest path.
* Represent confidence or detection source if kept simple.
* Add tests for constructing and comparing detection results.
* Keep the model small and easy to evolve.

### Out of scope

* Actual Node package manager detection.
* Actual Cargo workspace detection.
* CLI output.
* Project graph construction.
* JSON report design beyond what is immediately useful.
* Deep dependency parsing.

## Expected Files or Directories Affected

* `crates/monad-core/src/lib.rs`
* `crates/monad-core/src/intelligence.rs`
* `crates/monad-core/src/intelligence/toolchain.rs`

## Tasks

* [ ] Create repo intelligence module boundary.
* [ ] Add toolchain detection model types.
* [ ] Add package manager enum or equivalent model.
* [ ] Add language/ecosystem enum or equivalent model.
* [ ] Add manifest/path representation.
* [ ] Add simple tests.
* [ ] Export module from `monad-core`.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic repo-intelligence model commit.

## Deliverables

* Repo intelligence module exists.
* Toolchain detection model exists.
* Future detector work has a shared type vocabulary.
* Tests prove the model can be constructed and used.

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
* Repo intelligence module files exist.
* Toolchain detection model tests pass.
* No ecosystem-specific detector implementation has been added yet unless needed for tests.

## Definition of Done

* [ ] Toolchain detection model exists.
* [ ] Model is exported from `monad-core`.
* [ ] Tests cover basic construction behavior.
* [ ] Formatting passes.
* [ ] Tests pass.
* [ ] Clippy passes with warnings denied.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core
git commit -m "feat(intelligence): add toolchain detection model"
```

## Risks / Blockers / Open Questions

* The model should not overfit to one ecosystem.
* Avoid premature complexity such as full dependency graphs in this packet.
* Detection confidence may be useful, but should remain simple if included.
* Naming should stay understandable for Rust beginners.

## Priority

P1 High

## Size

S
BODY

cat > "${TMP_DIR}/wp-e2-002.md" <<'BODY'

## Work Packet ID

WP-E2-002

## Parent Epic ID

E2

## Work Packet Title

Detect Node and JavaScript package managers

## Product Area

Repo Intelligence

## Objective

Implement conservative detection for JavaScript/TypeScript repository indicators, including `package.json` and common package manager lockfiles for Bun, npm, pnpm, and Yarn.

## User Value

This work matters because JavaScript and TypeScript projects are among the most common repositories Monad will inspect.

Users should be able to run Monad and receive a clear explanation that a repo appears to use Node/JavaScript tooling and which package manager markers were found.

## Scope

### In scope

* Detect `package.json`.
* Detect `bun.lock` or `bun.lockb`.
* Detect `package-lock.json`.
* Detect `pnpm-lock.yaml`.
* Detect `yarn.lock`.
* Return detection results through the shared toolchain model.
* Add fixture-based tests.
* Keep detection filesystem-based and conservative.

### Out of scope

* Parsing dependency trees.
* Running package manager commands.
* Installing dependencies.
* Detecting every JavaScript runtime edge case.
* Monorepo package discovery from workspace fields.
* Deep TypeScript configuration analysis.

## Expected Files or Directories Affected

* `crates/monad-core/src/intelligence/toolchain.rs`
* `crates/monad-core/src/intelligence/detectors.rs`
* `crates/monad-core/src/intelligence/detectors/javascript.rs`
* test fixtures under `crates/monad-core/tests/fixtures/` or equivalent

## Tasks

* [ ] Add JavaScript/Node detector module.
* [ ] Detect `package.json`.
* [ ] Detect Bun lockfile.
* [ ] Detect npm lockfile.
* [ ] Detect pnpm lockfile.
* [ ] Detect Yarn lockfile.
* [ ] Add tests for each package manager marker.
* [ ] Verify deterministic detection results.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic JavaScript detection commit.

## Deliverables

* JavaScript package manager detection exists.
* Fixture tests cover Bun, npm, pnpm, and Yarn markers.
* Detection results use the shared repo intelligence model.
* Output remains conservative and explainable.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* Tests prove `package.json` detection works.
* Tests prove Bun, npm, pnpm, and Yarn lockfile markers are detected.
* No package install or dependency analysis behavior has been added.

## Definition of Done

* [ ] JavaScript detector exists.
* [ ] Bun marker detection exists.
* [ ] npm marker detection exists.
* [ ] pnpm marker detection exists.
* [ ] Yarn marker detection exists.
* [ ] Fixture tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core
git commit -m "feat(intelligence): detect javascript package managers"
```

## Risks / Blockers / Open Questions

* JavaScript tooling has many edge cases; this packet should stay marker-based.
* Multiple lockfiles may exist; Monad should report what it finds rather than guessing too aggressively.
* Workspace/package discovery should be handled later if needed.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e2-003.md" <<'BODY'

## Work Packet ID

WP-E2-003

## Parent Epic ID

E2

## Work Packet Title

Detect Rust Cargo workspaces

## Product Area

Repo Intelligence

## Objective

Implement conservative detection for Rust Cargo projects and Cargo workspaces, including root `Cargo.toml` detection and basic workspace member recognition.

## User Value

This work matters because Monad itself is a Rust project, and Monad should be able to inspect and explain its own repository.

For users, this enables Monad to identify Rust projects, Cargo manifests, and basic workspace structure.

## Scope

### In scope

* Detect `Cargo.toml`.
* Detect whether a manifest appears to define a package.
* Detect whether a manifest appears to define a workspace.
* Detect basic workspace members if feasible without overbuilding.
* Return detection results through the shared toolchain model.
* Add fixture-based tests.
* Keep TOML parsing simple and focused.

### Out of scope

* Full Cargo metadata integration.
* Running `cargo metadata`.
* Resolving every workspace glob edge case.
* Dependency graph analysis.
* Crate feature analysis.
* Publishing/release metadata analysis.

## Expected Files or Directories Affected

* `crates/monad-core/src/intelligence/detectors.rs`
* `crates/monad-core/src/intelligence/detectors/rust.rs`
* `crates/monad-core/src/intelligence/toolchain.rs`
* test fixtures under `crates/monad-core/tests/fixtures/` or equivalent
* possibly `crates/monad-core/Cargo.toml` if TOML parsing dependency is added

## Tasks

* [ ] Add Rust/Cargo detector module.
* [ ] Detect `Cargo.toml`.
* [ ] Detect package manifests.
* [ ] Detect workspace manifests.
* [ ] Detect basic workspace members if feasible.
* [ ] Add fixture tests for package and workspace cases.
* [ ] Verify deterministic detection results.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic Cargo detection commit.

## Deliverables

* Rust/Cargo detection exists.
* Monad can detect its own Cargo workspace.
* Tests cover package and workspace detection.
* Detection uses shared repo intelligence model.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- inspect
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* Cargo package detection tests pass.
* Cargo workspace detection tests pass.
* `monad inspect` either uses the detection directly if implemented by this point or remains ready to use it in the next packet.
* No full Cargo metadata or dependency graph behavior has been added.

## Definition of Done

* [ ] Rust detector exists.
* [ ] Cargo manifest detection exists.
* [ ] Cargo package detection exists.
* [ ] Cargo workspace detection exists.
* [ ] Fixture tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core
git commit -m "feat(intelligence): detect rust cargo workspaces"
```

## Risks / Blockers / Open Questions

* Full Cargo workspace resolution can become complex.
* This packet should avoid invoking `cargo metadata` unless a later decision justifies it.
* TOML parsing should be minimal and well-contained.
* Workspace glob patterns may need later refinement.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e2-004.md" <<'BODY'

## Work Packet ID

WP-E2-004

## Parent Epic ID

E2

## Work Packet Title

Add inspect command report

## Product Area

CLI

## Objective

Add the first `monad inspect` command that displays repository inspection results using the repo intelligence detection model.

This command should provide a clear, human-readable summary of what Monad detects in the current repository.

## User Value

This is the first major visible repo-intelligence command.

Users should be able to run `monad inspect` and quickly understand the project’s detected toolchains, manifests, and basic structure without manually searching through files.

## Scope

### In scope

* Add `inspect` command to CLI.
* Connect CLI command to `monad-core` inspection logic.
* Produce human-readable output.
* Add JSON output if the format option already exists or is straightforward.
* Include detected JavaScript and Rust tooling if prior packets are complete.
* Add tests where practical.

### Out of scope

* Full project graph rendering.
* Full health scoring.
* Recommendations.
* Verification execution.
* Context generation.
* AI explanation.
* Deep static analysis.

## Expected Files or Directories Affected

* `crates/monad-cli/src/commands.rs`
* `crates/monad-cli/src/cli.rs`
* `crates/monad-core/src/intelligence.rs`
* `crates/monad-core/src/intelligence/inspect.rs`
* tests as appropriate

## Tasks

* [ ] Add `inspect` command to CLI command enum.
* [ ] Add command handler.
* [ ] Add core inspection function.
* [ ] Render human-readable output.
* [ ] Add optional format argument if appropriate.
* [ ] Add tests where practical.
* [ ] Verify command runs in the Monad repo.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic inspect-command commit.

## Deliverables

* `monad inspect` command exists.
* Command reports detected repository indicators.
* Command output is understandable.
* CLI remains thin; inspection logic lives in `monad-core`.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- inspect
```

If JSON output is included:

```bash
cargo run -p monad-cli -- inspect --format json
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `cargo run -p monad-cli -- inspect` exits successfully.
* Inspect output identifies the Monad repo as a Rust/Cargo workspace once Cargo detection exists.
* Inspect output reports detected manifests or toolchain markers.
* No recommendations, verification runs, or file modifications are performed.

## Definition of Done

* [ ] `inspect` command exists.
* [ ] Command connects to core inspection logic.
* [ ] Human-readable output works.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] CLI remains thin.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-cli crates/monad-core
git commit -m "feat(cli): add inspect command"
```

## Risks / Blockers / Open Questions

* Output should not overclaim.
* CLI formatting should stay simple until a broader output/rendering strategy exists.
* JSON output can be added now only if it does not distract from the core command.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e2-005.md" <<'BODY'

## Work Packet ID

WP-E2-005

## Parent Epic ID

E2

## Work Packet Title

Add basic project graph model

## Product Area

Repo Intelligence

## Objective

Add Monad’s initial project graph model so detected repository entities and relationships can be represented deterministically.

This should be a basic graph model, not a full dependency analyzer.

## User Value

This work matters because Monad’s future capabilities depend on understanding relationships inside a repository.

The project graph becomes the foundation for impact analysis, context generation, verification planning, architecture boundary checks, and later safe evolution.

## Scope

### In scope

* Define project graph types.
* Represent graph nodes.
* Represent graph edges.
* Represent basic node kinds such as repository, manifest, package, toolchain, or workspace.
* Ensure deterministic ordering for output/tests.
* Add tests for graph construction.

### Out of scope

* Full dependency graph analysis.
* Language-server integration.
* Runtime service topology.
* Visualization UI.
* Advanced graph algorithms.
* Impact analysis.
* Architecture rule enforcement.

## Expected Files or Directories Affected

* `crates/monad-core/src/graph.rs`
* `crates/monad-core/src/graph/model.rs`
* `crates/monad-core/src/lib.rs`
* tests as appropriate

## Tasks

* [ ] Create graph module.
* [ ] Define graph node type.
* [ ] Define graph edge type.
* [ ] Define graph container type.
* [ ] Add deterministic ordering strategy.
* [ ] Add tests for graph construction.
* [ ] Export graph module from `monad-core`.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic graph-model commit.

## Deliverables

* Basic project graph model exists.
* Tests prove graph construction works.
* Future output formats and analysis features have a foundation.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
find crates/monad-core/src -maxdepth 4 -type f | sort
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* Graph module files exist.
* Tests prove graph nodes and edges can be created.
* Output ordering is deterministic where applicable.
* No advanced graph analysis has been added.

## Definition of Done

* [ ] Graph model exists.
* [ ] Node model exists.
* [ ] Edge model exists.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core
git commit -m "feat(graph): add basic project graph model"
```

## Risks / Blockers / Open Questions

* Graph model can become overcomplicated early.
* Keep the first version focused on representation, not analysis.
* Determinism matters for tests, context generation, and future diffs.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e2-006.md" <<'BODY'

## Work Packet ID

WP-E2-006

## Parent Epic ID

E2

## Work Packet Title

Add graph output formats

## Product Area

Repo Intelligence

## Objective

Add initial project graph rendering so Monad can output graph information in user-readable and machine-readable formats.

## User Value

This work matters because a graph is only useful if humans and tools can inspect it.

Users should be able to view a basic project graph in the terminal, while future automation and AI tools should be able to consume structured graph output.

## Scope

### In scope

* Add graph output rendering.
* Support human-readable text output.
* Support JSON output if serialization foundation exists or is straightforward.
* Add `monad graph` command if appropriate.
* Keep output deterministic.
* Add tests for rendering where practical.

### Out of scope

* Full visualization UI.
* Graph database.
* Mermaid/DOT rendering unless trivial.
* Advanced impact analysis.
* Architecture rule validation.
* Interactive graph exploration.

## Expected Files or Directories Affected

* `crates/monad-core/src/graph.rs`
* `crates/monad-core/src/graph/render.rs`
* `crates/monad-cli/src/commands.rs`
* `crates/monad-cli/src/cli.rs`
* tests as appropriate

## Tasks

* [ ] Add graph render module.
* [ ] Add text renderer.
* [ ] Add JSON renderer if appropriate.
* [ ] Add `graph` CLI command if appropriate.
* [ ] Ensure deterministic output.
* [ ] Add tests for rendering.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic graph-output commit.

## Deliverables

* Project graph can be rendered.
* Human-readable graph output exists.
* Machine-readable graph output exists if included.
* CLI command exists if included in scope.
* Output is deterministic.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- graph
```

If JSON output is included:

```bash
cargo run -p monad-cli -- graph --format json
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `monad graph` exits successfully if the command is included.
* Graph output is deterministic.
* JSON graph output is valid if implemented.
* No advanced visualization or impact analysis has been added.

## Definition of Done

* [ ] Graph rendering exists.
* [ ] Text output exists.
* [ ] JSON output exists if included.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-cli crates/monad-core
git commit -m "feat(graph): add graph output formats"
```

## Risks / Blockers / Open Questions

* Output format should not become a permanent API too early unless documented.
* JSON schema may need refinement later.
* Mermaid/DOT support can wait unless needed for immediate docs.

## Priority

P1 High

## Size

M
BODY

create_issue \
"[Epic]: E2 — Repo Intelligence" \
"type:epic,area:repo-intelligence,priority:p1,needs-verification" \
"${TMP_DIR}/e2.md"

create_issue \
"[Work Packet]: WP-E2-001 — Add toolchain detection model" \
"type:work-packet,area:repo-intelligence,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e2-001.md"

create_issue \
"[Work Packet]: WP-E2-002 — Detect Node and JavaScript package managers" \
"type:work-packet,area:repo-intelligence,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e2-002.md"

create_issue \
"[Work Packet]: WP-E2-003 — Detect Rust Cargo workspaces" \
"type:work-packet,area:repo-intelligence,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e2-003.md"

create_issue \
"[Work Packet]: WP-E2-004 — Add inspect command report" \
"type:work-packet,area:cli,area:repo-intelligence,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e2-004.md"

create_issue \
"[Work Packet]: WP-E2-005 — Add basic project graph model" \
"type:work-packet,area:repo-intelligence,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e2-005.md"

create_issue \
"[Work Packet]: WP-E2-006 — Add graph output formats" \
"type:work-packet,area:repo-intelligence,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e2-006.md"

echo
echo "Done seeding E2."
echo
echo "Manual follow-up:"
echo "- Open the E2 epic issue in GitHub."
echo "- Add WP-E2-001 through WP-E2-006 as sub-issues under E2."
echo "- Set project fields if needed:"
echo "  Type, Status, Epic, Product Area, Priority, Size, Work Packet ID."
echo "- Recommended status for E2 and its work packets right now: Deferred or Ready, not Active."
