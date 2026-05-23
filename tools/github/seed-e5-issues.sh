#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./tools/github/seed-e5-issues.sh OWNER REPO [PROJECT_NUMBER]
#
# Example:
#   ./tools/github/seed-e5-issues.sh thomascarter613 monad-workspace 1
#
# Notes:
# - This creates the E5 epic and E5 work packet issues.
# - It skips issues that already exist with the same exact title.
# - It optionally adds created/existing issues to a GitHub Project.
# - It does not set sub-issue hierarchy or custom project fields automatically.

OWNER="${1:?Missing GitHub owner, e.g. thomascarter613}"
REPO="${2:?Missing GitHub repo name, e.g. monad-workspace}"
PROJECT_NUMBER="${3:-}"

REPO_SLUG="${OWNER}/${REPO}"

echo "Seeding E5 issues into ${REPO_SLUG}"

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
  ensure_label "area:evolution" "Safe repository changes, generators, migrations, file operations, and apply workflows." "1d76db"
  ensure_label "area:verification" "Checks, test orchestration, evidence packets, and verification reports." "1d76db"
  ensure_label "area:context-bridge" "Context handoff, AI-readable state, bootstrap prompts, and session continuity." "1d76db"
  ensure_label "area:core" "Monad core runtime, shared domain logic, and foundational engine work." "1d76db"
  ensure_label "area:cli" "Command-line interface, command routing, help output, and CLI UX." "1d76db"
  ensure_label "priority:p1" "High priority; important for the current milestone or near-term progress." "d93f0b"
  ensure_label "needs-verification" "Requires test, check, review, or evidence before completionHigh priority; important for the current milestone or near-term progress." "d93f0b"
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

cat > "${TMP_DIR}/e5.md" <<'BODY'
## Product Area

Evolution Engine

## Objective

Create Monad’s safe repository evolution foundation: file operation planning, dry-run support, diff previews, template registry foundation, and initial `monad evolve` workflows that improve a repository without surprising the user.

This epic should establish Monad’s first ability to propose and prepare repository changes in a controlled, reviewable, and verification-oriented way.

## User Value

This epic is where Monad begins to fulfill the Software Foundry promise.

Users should be able to ask Monad to improve a repository and receive a planned, inspectable, reversible, and verifiable change rather than an uncontrolled rewrite.

The key value is not raw generation. The key value is safe, evidence-backed repository evolution that keeps the human in command.

For maintainers and contributors, E5 creates the foundation for future generators, migrations, templates, repo upgrades, baseline hardening commands, and eventually supervised agent-assisted changes.

## Scope

### In scope

- Safe file operation model.
- Planned file creates, updates, deletes, and skips.
- Conflict behavior.
- Dry-run mode.
- Diff or preview summaries.
- Template registry foundation.
- Initial `monad evolve` command group.
- Initial baseline evolution commands.
- Verification after generated changes where feasible.
- Worktree and branch safety strategy.

### Out of scope

- Fully autonomous agents.
- Automatic PR creation.
- Production deployment.
- Complex semantic refactors.
- Cloud sandboxing.
- Marketplace/plugin distribution.
- Billing.
- Multi-tenant execution.
- Unreviewed destructive changes.
- Large-scale migration framework.

## Expected Work Packets

- WP-E5-001 — Define safe file operation model
- WP-E5-002 — Add dry-run and diff planner
- WP-E5-003 — Add template registry foundation
- WP-E5-004 — Add evolve verify-baseline command
- WP-E5-005 — Add evolve context-baseline command
- WP-E5-006 — Add worktree and branch safety strategy

## Deliverables

- Safe file operation domain model.
- Dry-run planner.
- Diff/evidence preview.
- Template registry foundation.
- Initial `monad evolve` command group.
- Baseline evolution command for verification setup.
- Baseline evolution command for context bridge setup.
- Worktree/branch safety strategy.
- Tests for file operation planning and dry-run behavior.
- Clear human-readable output describing proposed changes before writes occur.

## Verification Strategy

Suggested verification commands:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-workspace-cli -- evolve verify-baseline --dry-run
cargo run -p monad-workspace-cli -- evolve context-baseline --dry-run
```

Expected result:

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* Dry-run evolution commands exit successfully once implemented.
* Dry-run output shows planned file operations before any writes occur.
* Actual apply behavior is gated and reviewable.
* File operation tests prove creates, updates, skips, and conflicts are handled predictably.
* No fully autonomous agent execution has been added.
* No unreviewed destructive behavior has been added.

## Risks / Open Questions

* File writes are trust-critical and must be conservative.
* Dry-run and actual apply behavior must match.
* Templates must be versioned or traceable.
* The user must understand every proposed change.
* Worktree/branch strategy may require ADR-level decision.
* We need to avoid building a full generator ecosystem before the safe operation model is proven.
* Early evolution commands should improve baselines, not perform complex app rewrites.

## Priority

P1 High

## Size

L
BODY

cat > "${TMP_DIR}/wp-e5-001.md" <<'BODY'

## Work Packet ID

WP-E5-001

## Parent Epic ID

E5

## Work Packet Title

Define safe file operation model

## Product Area

Evolution Engine

## Objective

Define Monad’s safe file operation model for planned repository changes.

This work packet should create the core vocabulary for representing proposed creates, updates, deletes, skips, conflicts, and no-op operations before any evolution command writes files.

## User Value

This work matters because Monad’s evolution features will only be trusted if users can understand exactly what will happen before files are changed.

For users, this means Monad can say:

* this file would be created;
* this file would be updated;
* this file already exists and would be skipped;
* this file conflicts with the requested operation;
* this operation is unsafe without approval.

For maintainers, this model becomes the foundation for templates, dry-run planning, diff previews, baseline generators, and future supervised agent workflows.

## Scope

### In scope

* Define file operation type.
* Define operation target path.
* Define planned create operation.
* Define planned update operation.
* Define planned delete operation if needed, but keep deletion conservative.
* Define skipped/no-op operation.
* Define conflict representation.
* Define operation summary/result model.
* Add tests for constructing planned operations.
* Keep model deterministic and reviewable.

### Out of scope

* Actually writing files.
* Diff generation.
* Template registry.
* CLI command implementation.
* Worktree creation.
* Branch management.
* Agent-generated changes.
* Complex patch application.
* Binary file operations.

## Expected Files or Directories Affected

* `crates/monad-core/src/lib.rs`
* `crates/monad-core/src/file_ops.rs`
* `crates/monad-core/src/file_ops/model.rs`
* `crates/monad-core/src/file_ops/plan.rs`

## Tasks

* [ ] Create file operations module boundary.
* [ ] Define planned file operation enum or equivalent model.
* [ ] Define operation target path representation.
* [ ] Define create/update/delete/skip/conflict variants.
* [ ] Define operation summary/result type.
* [ ] Add tests for operation construction.
* [ ] Export module from `monad-core`.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic file-operation-model commit.

## Deliverables

* Safe file operation model exists.
* Planned file operations can be represented before writes occur.
* Conflict and skip behavior can be represented.
* Tests prove basic operation construction behavior.
* Future dry-run and template work has a stable foundation.

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
* File operation module files exist.
* Tests prove planned operations can be constructed.
* Tests prove skip/conflict states can be represented if included.
* No actual file writing behavior has been added yet.

## Definition of Done

* [ ] File operation model exists.
* [ ] Create operation can be represented.
* [ ] Update operation can be represented.
* [ ] Skip/no-op operation can be represented.
* [ ] Conflict can be represented.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core
git commit -m "feat(evolution): add safe file operation model"
```

## Risks / Blockers / Open Questions

* Avoid implementing writes in this packet.
* Delete operations should be treated with caution.
* Model should stay small until dry-run and template behavior prove what else is needed.
* Paths should be handled with filesystem-safe types, not plain strings where avoidable.

## Priority

P1 High

## Size

S
BODY

cat > "${TMP_DIR}/wp-e5-002.md" <<'BODY'

## Work Packet ID

WP-E5-002

## Parent Epic ID

E5

## Work Packet Title

Add dry-run and diff planner

## Product Area

Evolution Engine

## Objective

Add a dry-run planner that can evaluate planned file operations and show what would happen before any repository files are written.

This work packet should make proposed changes inspectable and reviewable.

## User Value

This work matters because developers will not trust a repository evolution tool that changes files without preview.

Dry-run behavior allows Monad to show the user a safe preview:

* which files would be created;
* which files would be updated;
* which files would be skipped;
* which files would conflict;
* whether applying the plan appears safe.

For future users, this becomes a key trust feature of Monad.

## Scope

### In scope

* Add dry-run planning function.
* Evaluate planned operations against the filesystem.
* Detect whether target files already exist.
* Detect create conflicts.
* Detect update/no-op behavior where practical.
* Produce a human-readable plan summary.
* Add tests using temporary directories.
* Keep behavior deterministic.

### Out of scope

* Full text diff generation if too large for this packet.
* Actual file writes.
* Template registry.
* Worktree creation.
* Git branch management.
* Agent-generated file changes.
* Patch application.
* Binary file diffing.

## Expected Files or Directories Affected

* `crates/monad-core/src/file_ops.rs`
* `crates/monad-core/src/file_ops/plan.rs`
* `crates/monad-core/src/file_ops/dry_run.rs`
* `crates/monad-core/src/file_ops/report.rs`
* tests as appropriate

## Tasks

* [ ] Add dry-run planner module.
* [ ] Evaluate create operations against existing files.
* [ ] Evaluate update operations against existing files.
* [ ] Detect conflict conditions.
* [ ] Produce plan summary.
* [ ] Add tests for create, skip, and conflict behavior.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic dry-run planner commit.

## Deliverables

* Dry-run planner exists.
* Planned file operations can be evaluated before writes.
* Summary output explains what would happen.
* Tests prove dry-run behavior for basic scenarios.

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
* Dry-run planner tests pass.
* Tests prove existing-file conflicts are detected.
* Tests prove planned creates can be previewed.
* Tests prove skipped/no-op operations can be represented if included.
* No actual file writing behavior is required in this packet.

## Definition of Done

* [ ] Dry-run planner exists.
* [ ] Existing file conflicts are detected.
* [ ] Plan summary exists.
* [ ] Tests cover basic dry-run behavior.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core
git commit -m "feat(evolution): add dry-run diff planner"
```

## Risks / Blockers / Open Questions

* Dry-run behavior must match future apply behavior.
* Do not overbuild full diffing before basic operation planning is proven.
* Clear conflict messages matter more than clever output.
* Temporary-directory tests should avoid relying on the developer’s machine state.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e5-003.md" <<'BODY'

## Work Packet ID

WP-E5-003

## Parent Epic ID

E5

## Work Packet Title

Add template registry foundation

## Product Area

Evolution Engine

## Objective

Add Monad’s initial template registry foundation so future evolution commands can use known, versionable templates to create or update repository files consistently.

This work packet should define the registry concept without building a full marketplace or plugin system.

## User Value

This work matters because safe repository evolution needs repeatable, understandable source material.

Users should eventually be able to run baseline commands and know that Monad is applying known templates rather than generating unpredictable files from scratch.

For maintainers, the registry creates a place to organize reusable templates for verification baselines, context baselines, docs baselines, issue forms, and future repository improvements.

## Scope

### In scope

* Define template metadata model.
* Define template identifier.
* Define template source or embedded template representation.
* Define template registry type.
* Add basic lookup by template ID.
* Add tests for registering and retrieving templates.
* Keep templates local/embedded for now.

### Out of scope

* Remote template marketplace.
* Plugin installation.
* Template signing.
* Version negotiation.
* Complex templating engine.
* User-authored template packs.
* Billing or distribution.

## Expected Files or Directories Affected

* `crates/monad-core/src/templates.rs`
* `crates/monad-core/src/templates/model.rs`
* `crates/monad-core/src/templates/registry.rs`
* `crates/monad-core/src/lib.rs`
* possibly `templates/` or embedded template fixtures if needed

## Tasks

* [ ] Create templates module boundary.
* [ ] Define template metadata type.
* [ ] Define template ID type.
* [ ] Define template registry type.
* [ ] Add lookup behavior.
* [ ] Add tests for template registration and retrieval.
* [ ] Export templates module from `monad-core`.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic template-registry commit.

## Deliverables

* Template registry foundation exists.
* Template metadata can be represented.
* Templates can be registered and looked up.
* Tests prove basic registry behavior.
* Future evolution commands have a template foundation.

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
* Template module files exist.
* Tests prove templates can be registered or retrieved.
* No remote marketplace, plugin loading, or complex templating engine has been added.

## Definition of Done

* [ ] Template metadata model exists.
* [ ] Template registry exists.
* [ ] Template lookup works.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core
git commit -m "feat(templates): add template registry foundation"
```

## Risks / Blockers / Open Questions

* Template registry should stay simple.
* Avoid committing to marketplace architecture too early.
* Template versioning may be needed later, but the first model should not overbuild it.
* Embedded templates are likely enough for MVP.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e5-004.md" <<'BODY'

## Work Packet ID

WP-E5-004

## Parent Epic ID

E5

## Work Packet Title

Add evolve verify-baseline command

## Product Area

Evolution Engine

## Objective

Add an initial `monad evolve verify-baseline` command that can plan a verification baseline improvement for a repository.

The first version should support dry-run behavior and show which verification-related files or changes would be created.

## User Value

This work matters because one of Monad’s most useful early workflows is helping repositories become more verifiable.

Users should be able to ask Monad to prepare a baseline verification setup and see the proposed changes before anything is written.

This becomes an early example of safe, reviewable repository evolution.

## Scope

### In scope

* Add `evolve` command group if not already present.
* Add `verify-baseline` subcommand.
* Connect command to file operation planning.
* Use template registry if available.
* Support `--dry-run`.
* Show planned verification-baseline file operations.
* Add tests where practical.

### Out of scope

* Full CI implementation.
* GitHub Actions generation if too broad.
* Applying changes without review.
* Auto-committing.
* Auto-opening PRs.
* Full verification engine replacement.
* Complex repository-specific customization.

## Expected Files or Directories Affected

* `crates/monad-cli/src/cli.rs`
* `crates/monad-cli/src/commands.rs`
* `crates/monad-core/src/evolution.rs`
* `crates/monad-core/src/evolution/verify_baseline.rs`
* `crates/monad-core/src/file_ops/`
* `crates/monad-core/src/templates/`
* tests as appropriate

## Tasks

* [ ] Add `evolve` command group.
* [ ] Add `verify-baseline` subcommand.
* [ ] Add `--dry-run` option.
* [ ] Define initial verification baseline plan.
* [ ] Render planned file operations.
* [ ] Use safe file operation model.
* [ ] Add tests where practical.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic verify-baseline evolution commit.

## Deliverables

* `monad evolve verify-baseline --dry-run` exists.
* Command shows planned verification baseline changes.
* No files are written in dry-run mode.
* Output is understandable and reviewable.
* CLI remains a thin wrapper over core evolution logic.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-workspace-cli -- evolve verify-baseline --dry-run
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `cargo run -p monad-workspace-cli -- evolve verify-baseline --dry-run` exits successfully.
* Dry-run output lists planned verification baseline operations.
* Dry-run mode does not write files.
* No auto-commit, PR creation, or autonomous apply behavior has been added.

## Definition of Done

* [ ] `evolve` command group exists.
* [ ] `verify-baseline` subcommand exists.
* [ ] `--dry-run` works.
* [ ] Planned file operations are displayed.
* [ ] No writes occur during dry-run.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-cli crates/monad-core
git commit -m "feat(evolution): add verify baseline evolution"
```

## Risks / Blockers / Open Questions

* Baseline generation should not be too ambitious in the first version.
* The command should clearly distinguish plan from apply.
* The first version may only generate a minimal verification baseline.
* Applying changes can be added after dry-run behavior is trusted.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e5-005.md" <<'BODY'

## Work Packet ID

WP-E5-005

## Parent Epic ID

E5

## Work Packet Title

Add evolve context-baseline command

## Product Area

Evolution Engine

## Objective

Add an initial `monad evolve context-baseline` command that can plan a context bridge baseline for a repository.

The first version should support dry-run behavior and show which context-related files would be created or updated.

## User Value

This work matters because Monad’s context bridge should not only exist in Monad’s own repo. Monad should eventually help other repositories become AI-readable and handoff-ready.

Users should be able to ask Monad to prepare a context baseline and review the proposed files before applying anything.

## Scope

### In scope

* Add `context-baseline` subcommand under `evolve`.
* Support `--dry-run`.
* Plan creation of core context files.
* Use template registry if available.
* Use safe file operation model.
* Show planned file operations.
* Add tests where practical.

### Out of scope

* Full context generator implementation.
* AI summarization.
* Vector database setup.
* Provider-specific prompt optimization.
* Automatic GitHub issue integration.
* Auto-committing.
* Auto-opening PRs.

## Expected Files or Directories Affected

* `crates/monad-cli/src/cli.rs`
* `crates/monad-cli/src/commands.rs`
* `crates/monad-core/src/evolution.rs`
* `crates/monad-core/src/evolution/context_baseline.rs`
* `crates/monad-core/src/file_ops/`
* `crates/monad-core/src/templates/`
* tests as appropriate

## Tasks

* [ ] Add `context-baseline` subcommand.
* [ ] Add `--dry-run` option.
* [ ] Define initial context baseline plan.
* [ ] Plan context files under `docs/ai`, `docs/context`, or `.monad/context` as appropriate.
* [ ] Render planned file operations.
* [ ] Ensure dry-run does not write files.
* [ ] Add tests where practical.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic context-baseline evolution commit.

## Deliverables

* `monad evolve context-baseline --dry-run` exists.
* Command shows planned context baseline operations.
* No files are written in dry-run mode.
* Output is understandable and reviewable.
* Command demonstrates Monad’s repo-native context philosophy.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-workspace-cli -- evolve context-baseline --dry-run
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `cargo run -p monad-workspace-cli -- evolve context-baseline --dry-run` exits successfully.
* Dry-run output lists planned context baseline operations.
* Dry-run mode does not write files.
* No AI summarization, remote service, auto-commit, or PR creation behavior has been added.

## Definition of Done

* [ ] `context-baseline` subcommand exists.
* [ ] `--dry-run` works.
* [ ] Planned context baseline operations are displayed.
* [ ] No writes occur during dry-run.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-cli crates/monad-core
git commit -m "feat(evolution): add context baseline evolution"
```

## Risks / Blockers / Open Questions

* Context baseline should not be too large in the first version.
* Avoid overwriting existing docs without clear conflict behavior.
* Generated baseline should align with E3 context artifact standards.
* Applying changes should remain gated and reviewable.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e5-006.md" <<'BODY'

## Work Packet ID

WP-E5-006

## Parent Epic ID

E5

## Work Packet Title

Add worktree and branch safety strategy

## Product Area

Evolution Engine

## Objective

Define and prototype Monad’s worktree and branch safety strategy for repository evolution workflows.

This work packet should establish how Monad avoids making risky changes directly in a user’s working tree.

## User Value

This work matters because safe evolution requires isolation.

Users should have confidence that Monad can prepare changes in a controlled branch or worktree, allow review, run verification, and avoid damaging the current working state.

For maintainers, this strategy becomes foundational for future supervised agent workflows and safe apply behavior.

## Scope

### In scope

* Document worktree/branch safety strategy.
* Define when Monad should require a clean working tree.
* Define when Monad should recommend or create a branch/worktree.
* Prototype basic Git state checks if appropriate.
* Add tests where practical.
* Keep behavior conservative.

### Out of scope

* Full Git workflow automation.
* Automatic pushes.
* Automatic PR creation.
* Complex merge handling.
* Remote repository operations.
* Multi-agent branch orchestration.
* Deployment workflows.

## Expected Files or Directories Affected

* `docs/architecture/WORKTREE-SAFETY-STRATEGY.md`
* `crates/monad-core/src/git.rs` if implementation is included
* `crates/monad-core/src/git/status.rs` if implementation is included
* `crates/monad-core/src/evolution/`
* tests as appropriate

## Tasks

* [ ] Document worktree/branch safety rules.
* [ ] Define clean working tree expectations.
* [ ] Define branch/worktree recommendation behavior.
* [ ] Define unsafe-state diagnostics.
* [ ] Prototype Git state detection if appropriate.
* [ ] Add tests if implementation is included.
* [ ] Verify formatting.
* [ ] Verify tests if applicable.
* [ ] Verify Clippy if applicable.
* [ ] Commit as one atomic worktree-safety commit.

## Deliverables

* Worktree/branch safety strategy is documented.
* Unsafe working tree conditions are defined.
* Future evolution commands have safety rules.
* Optional basic Git state detection exists if included.
* Strategy is ready to be refined into an ADR if needed.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
find docs/architecture crates/monad-core/src -maxdepth 4 -type f | sort
```

## Expected Result After Verification

* Documentation exists for worktree/branch safety strategy.
* Formatting passes if code changed.
* Tests pass if code changed.
* Clippy passes if code changed.
* The strategy explains when Monad should avoid direct writes.
* The strategy explains when branch/worktree isolation should be used.
* No automatic push, PR, or remote Git behavior has been added.

## Definition of Done

* [ ] Worktree safety strategy is documented.
* [ ] Clean working tree expectations are defined.
* [ ] Branch/worktree isolation rules are defined.
* [ ] Unsafe states are documented.
* [ ] Tests pass if implementation is included.
* [ ] Formatting passes.
* [ ] Clippy passes if code changed.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add docs/architecture crates/monad-core
git commit -m "feat(evolution): add worktree safety strategy"
```

## Risks / Blockers / Open Questions

* This may require an ADR before full implementation.
* Git behavior can be complex across platforms and repository states.
* The first implementation should be conservative.
* Worktree creation should not surprise the user.
* Branch naming and cleanup strategy may need separate work later.

## Priority

P1 High

## Size

M
BODY

create_issue \
"[Epic]: E5 — Evolution Engine" \
"type:epic,area:evolution,priority:p1,needs-verification,context-update-required" \
"${TMP_DIR}/e5.md"

create_issue \
"[Work Packet]: WP-E5-001 — Define safe file operation model" \
"type:work-packet,area:evolution,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e5-001.md"

create_issue \
"[Work Packet]: WP-E5-002 — Add dry-run and diff planner" \
"type:work-packet,area:evolution,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e5-002.md"

create_issue \
"[Work Packet]: WP-E5-003 — Add template registry foundation" \
"type:work-packet,area:evolution,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e5-003.md"

create_issue \
"[Work Packet]: WP-E5-004 — Add evolve verify-baseline command" \
"type:work-packet,area:evolution,area:verification,area:cli,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e5-004.md"

create_issue \
"[Work Packet]: WP-E5-005 — Add evolve context-baseline command" \
"type:work-packet,area:evolution,area:context-bridge,area:cli,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e5-005.md"

create_issue \
"[Work Packet]: WP-E5-006 — Add worktree and branch safety strategy" \
"type:work-packet,area:evolution,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e5-006.md"

echo
echo "Done seeding E5."
echo
echo "Manual follow-up:"
echo "- Open the E5 epic issue in GitHub."
echo "- Add WP-E5-001 through WP-E5-006 as sub-issues under E5."
echo "- Set project fields if needed:"
echo "  Type, Status, Epic, Product Area, Priority, Size, Work Packet ID."
echo "- Recommended status for E5 and its work packets right now: Deferred or Ready, not Active."
