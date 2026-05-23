#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./tools/github/seed-e3-issues.sh OWNER REPO [PROJECT_NUMBER]
#
# Example:
#   ./tools/github/seed-e3-issues.sh thomascarter613 monad-workspace 1
#
# Notes:
# - This creates the E3 epic and E3 work packet issues.
# - It skips issues that already exist with the same exact title.
# - It optionally adds created/existing issues to a GitHub Project.
# - It does not set sub-issue hierarchy or custom project fields automatically.

OWNER="${1:?Missing GitHub owner, e.g. thomascarter613}"
REPO="${2:?Missing GitHub repo name, e.g. monad-workspace}"
PROJECT_NUMBER="${3:-}"

REPO_SLUG="${OWNER}/${REPO}"

echo "Seeding E3 issues into ${REPO_SLUG}"

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
  ensure_label "area:context-bridge" "Context handoff, AI-readable state, bootstrap prompts, and session continuity." "1d76db"
  ensure_label "area:docs" "Documentation architecture, standards, guides, and repo-native knowledge." "1d76db"
  ensure_label "area:core" "Monad core runtime, shared domain logic, and foundational engine work." "1d76db"
  ensure_label "area:cli" "Command-line interface, command routing, help output, and CLI UX." "1d76db"
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

cat > "${TMP_DIR}/e3.md" <<'BODY'
## Product Area

Context Bridge

## Objective

Implement Monad’s repo-native context bridge: a simple but useful system for generating current-state files, fresh-chat handoffs, bootstrap prompts, session chronicles, and context packs from repository state.

This epic should turn Monad’s early Charon-style handoff foundation into an actual working capability that helps humans and AI assistants resume work from repo-resident context instead of fragile chat history.

## User Value

This epic is central to Monad’s identity.

Users should be able to keep project context durable, reviewable, and close to the code. Maintainers and AI assistants should be able to understand the current project state, recent decisions, active work, next steps, and handoff instructions by reading repo files.

For Monad itself, this epic makes the repo a living demonstration of the product’s philosophy: the repository should be able to explain itself well enough for work to continue across sessions, people, and tools.

## Scope

### In scope

- Context artifact standards.
- Current-state generation.
- Handoff generation.
- Bootstrap prompt generation.
- Session chronicle structure.
- Context pack assembly.
- Basic validation that required context files exist.
- Repo-native context storage under `docs/ai`, `docs/context`, and `.monad/context`.
- Clear separation between human-authored context and generated/reviewable context.

### Out of scope

- Vector database integration.
- Semantic retrieval.
- Multi-agent memory system.
- Cloud-hosted context service.
- Long-running automated context updates.
- Full MCP integration.
- Complex summarization pipelines.
- Autonomous agent memory.

## Expected Work Packets

- WP-E3-001 — Define context artifact schemas
- WP-E3-002 — Implement current-state generator
- WP-E3-003 — Implement handoff generator
- WP-E3-004 — Implement context pack assembler
- WP-E3-005 — Implement bootstrap prompt generator
- WP-E3-006 — Add context verification checks

## Deliverables

- Context artifact standards.
- Generated `.monad/context/current-state.md`.
- Generated `.monad/context/latest-handoff.md`.
- Generated fresh-chat handoff.
- Generated bootstrap prompt.
- Basic context pack output.
- Context verification checks.
- Tests for context generation.
- Documentation explaining what each context artifact means and when it should be updated.

## Verification Strategy

Suggested verification commands:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- context generate
find docs/ai docs/context .monad/context -maxdepth 4 -type f | sort
```

Expected result:

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `monad context generate` exits successfully once implemented.
* Required context files exist.
* Generated context artifacts are deterministic enough to review.
* A new session can read the bootstrap/handoff/current-state files and understand the current project state.
* Generated context distinguishes known facts from assumptions or inferred next steps.

## Risks / Open Questions

* Context files may become stale if not updated consistently.
* Generated context must distinguish fact from inference.
* The system should remain simple before adding semantic retrieval.
* We need clear boundaries between user-authored docs and generated state.
* Context files should not become noisy or too large.
* Generated files should be reviewable and diff-friendly.
* Context generation should not depend on one AI provider.

## Priority

P1 High

## Size

L
BODY

cat > "${TMP_DIR}/wp-e3-001.md" <<'BODY'

## Work Packet ID

WP-E3-001

## Parent Epic ID

E3

## Work Packet Title

Define context artifact schemas

## Product Area

Context Bridge

## Objective

Define the expected structure and purpose of Monad’s context artifacts before implementing automated context generation.

This work packet should establish the first schema/standard for current-state files, latest handoffs, session chronicles, bootstrap prompts, and context packs.

## User Value

This work matters because context handoff is one of Monad’s core differentiators.

Users and AI assistants need predictable context files. Without a clear artifact standard, handoffs become inconsistent, stale, noisy, or hard to trust.

This packet creates the rules for what context artifacts should contain, how they should be organized, and how they should distinguish durable facts from temporary notes.

## Scope

### In scope

* Define current-state artifact structure.
* Define latest-handoff artifact structure.
* Define session-chronicle structure.
* Define decisions/context decision log structure.
* Define bootstrap prompt structure.
* Define context pack structure.
* Document human-authored versus generated/reviewable context.
* Keep schemas Markdown-first and easy to understand.

### Out of scope

* Code generation implementation.
* CLI command implementation.
* Vector database schemas.
* Semantic retrieval.
* Full MCP tool schemas.
* Cloud storage model.
* Automated stale-context detection.

## Expected Files or Directories Affected

* `docs/context/CONTEXT-ARTIFACT-SCHEMAS.md`
* `docs/context/CONTEXT-PACK-STANDARD.md`
* `docs/context/HANDOFF-STANDARD.md`
* `docs/ai/README.md`
* `.monad/context/README.md` if present or needed

## Tasks

* [ ] Define current-state artifact fields.
* [ ] Define latest-handoff artifact fields.
* [ ] Define session-chronicle artifact fields.
* [ ] Define decisions artifact fields.
* [ ] Define bootstrap prompt artifact fields.
* [ ] Define context pack artifact fields.
* [ ] Document generated versus human-authored context boundaries.
* [ ] Verify the schema docs are internally consistent.
* [ ] Commit as one atomic context-schema commit.

## Deliverables

* Context artifact schema documentation exists.
* Each context artifact has a clear purpose.
* Each context artifact has expected sections.
* Future generators have a stable target format.
* Future handoffs have a consistent standard.

## Verification Commands / Evidence

```bash
find docs/context docs/ai .monad/context -maxdepth 4 -type f | sort
git status --short
```

## Expected Result After Verification

* Context schema documentation files exist.
* `HANDOFF-STANDARD.md` exists and remains aligned with the new schema docs.
* The expected context artifacts are named and described.
* No generator implementation code has been added yet.
* Repository changes are ready for one atomic documentation commit.

## Definition of Done

* [ ] Current-state schema is documented.
* [ ] Handoff schema is documented.
* [ ] Session chronicle schema is documented.
* [ ] Decisions/context log schema is documented.
* [ ] Bootstrap prompt schema is documented.
* [ ] Context pack schema is documented.
* [ ] Human-authored versus generated context boundary is documented.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add docs/context docs/ai .monad/context
git commit -m "docs(context): define context artifact schemas"
```

## Risks / Blockers / Open Questions

* Schema docs should not become too abstract.
* Context artifacts should stay Markdown-first for readability.
* We should avoid designing a complex context database before the simple repo-native files prove useful.
* These schemas may need refinement once generators are implemented.

## Priority

P1 High

## Size

S
BODY

cat > "${TMP_DIR}/wp-e3-002.md" <<'BODY'

## Work Packet ID

WP-E3-002

## Parent Epic ID

E3

## Work Packet Title

Implement current-state generator

## Product Area

Context Bridge

## Objective

Implement a `current-state` generator that writes or updates a repo-native current-state artifact describing Monad’s present project state.

This work packet should create the first executable context-generation behavior.

## User Value

This work matters because a new human or AI assistant should be able to understand the project’s current state without reading the entire repository or relying on prior chat history.

The current-state artifact should answer:

* What is this project?
* What epic/work packet is active?
* What has recently been completed?
* What is next?
* What should a new session read first?

## Scope

### In scope

* Add a current-state generation function in `monad-core`.
* Add or prepare a CLI path for generating current-state context.
* Generate `.monad/context/current-state.md`.
* Include project name, active epic/work packet, completed/recent work where available, and next recommended step.
* Keep generation deterministic and reviewable.
* Add tests where practical.

### Out of scope

* Full handoff generation.
* Full context pack assembly.
* AI summarization.
* Vector search.
* GitHub API integration.
* Automated issue/project synchronization.
* Complex parsing of all docs.

## Expected Files or Directories Affected

* `crates/monad-core/src/context.rs`
* `crates/monad-core/src/context/current_state.rs`
* `crates/monad-cli/src/commands.rs`
* `.monad/context/current-state.md`
* tests as appropriate

## Tasks

* [ ] Add context module boundary in `monad-core`.
* [ ] Add current-state generator type/function.
* [ ] Define current-state output structure.
* [ ] Write generated artifact to `.monad/context/current-state.md`.
* [ ] Add CLI entrypoint if appropriate.
* [ ] Add tests for deterministic generation.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic current-state generator commit.

## Deliverables

* Current-state generator exists.
* `.monad/context/current-state.md` can be generated or updated.
* Generated current-state artifact is readable.
* Tests cover generation behavior where practical.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- context generate current-state
cat .monad/context/current-state.md
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `cargo run -p monad-cli -- context generate current-state` exits successfully once command routing exists.
* `.monad/context/current-state.md` exists.
* The current-state file identifies Monad, the current active work if available, and the next recommended step.
* Generated content is reviewable and does not overwrite unrelated human notes.

## Definition of Done

* [ ] Current-state generator exists.
* [ ] Current-state artifact can be written.
* [ ] Output is readable.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core crates/monad-cli .monad/context
git commit -m "feat(context): generate current state artifact"
```

## Risks / Blockers / Open Questions

* Generator should avoid overclaiming project status.
* Generated files should be deterministic and diff-friendly.
* We need to decide how much source data to read in the first version.
* The first generator can be simple and improved later.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e3-003.md" <<'BODY'

## Work Packet ID

WP-E3-003

## Parent Epic ID

E3

## Work Packet Title

Implement handoff generator

## Product Area

Context Bridge

## Objective

Implement a handoff generator that creates or updates Monad’s latest handoff artifact for continuing work in a future session.

The handoff should tell a new human or AI assistant exactly where the project stands and what to do next.

## User Value

This work matters because Monad development often spans multiple sessions. Without durable handoffs, context gets lost, repeated, or distorted.

For future Monad users, this becomes a core product capability: repositories can preserve enough context to resume work safely.

## Scope

### In scope

* Add handoff generation function in `monad-core`.
* Generate `.monad/context/latest-handoff.md`.
* Include current epic, active work packet, recent changes, verification status, next step, and reading order.
* Keep generated handoff concise and reviewable.
* Add tests where practical.

### Out of scope

* Full context pack assembly.
* Full session chronicle generation.
* AI summarization.
* GitHub issue synchronization.
* Model-provider integration.
* Autonomous task continuation.

## Expected Files or Directories Affected

* `crates/monad-core/src/context/handoff.rs`
* `crates/monad-core/src/context.rs`
* `crates/monad-cli/src/commands.rs`
* `.monad/context/latest-handoff.md`
* tests as appropriate

## Tasks

* [ ] Add handoff generator module.
* [ ] Define handoff output sections.
* [ ] Write generated artifact to `.monad/context/latest-handoff.md`.
* [ ] Add CLI entrypoint if appropriate.
* [ ] Add tests for deterministic handoff output.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic handoff-generator commit.

## Deliverables

* Handoff generator exists.
* `.monad/context/latest-handoff.md` can be generated or updated.
* Handoff contains enough information to resume work.
* Output is concise and reviewable.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- context generate handoff
cat .monad/context/latest-handoff.md
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `cargo run -p monad-cli -- context generate handoff` exits successfully once command routing exists.
* `.monad/context/latest-handoff.md` exists.
* Handoff content includes current status, active work, recent progress, and next action.
* Generated output is deterministic enough to review in Git.

## Definition of Done

* [ ] Handoff generator exists.
* [ ] Latest handoff artifact can be written.
* [ ] Output is readable and actionable.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core crates/monad-cli .monad/context
git commit -m "feat(context): generate handoff artifact"
```

## Risks / Blockers / Open Questions

* Handoff should stay concise.
* Generated handoff should not replace human judgment.
* We need a clear source of truth for active work.
* Future versions may read GitHub issues or project fields, but this packet should stay repo-native.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e3-004.md" <<'BODY'

## Work Packet ID

WP-E3-004

## Parent Epic ID

E3

## Work Packet Title

Implement context pack assembler

## Product Area

Context Bridge

## Objective

Implement a context pack assembler that gathers selected repo-native context artifacts into a compact, ordered bundle for humans or AI assistants.

The first version should be simple, deterministic, and Markdown-first.

## User Value

This work matters because AI assistants and new contributors do not need every file in the repo at once. They need the right context in the right order.

A context pack helps reduce confusion, repeated explanations, and context-window waste.

## Scope

### In scope

* Define context pack assembly rules.
* Select key files such as README, product charter, current state, latest handoff, active work packet, and relevant workflow docs.
* Generate a context pack file.
* Preserve source references/headings.
* Keep output deterministic and reviewable.
* Add tests where practical.

### Out of scope

* Vector retrieval.
* Semantic ranking.
* Token budgeting.
* Model-specific prompt optimization.
* Cloud context service.
* Dynamic GitHub issue scraping.
* Full document summarization.

## Expected Files or Directories Affected

* `crates/monad-core/src/context/pack.rs`
* `crates/monad-core/src/context.rs`
* `crates/monad-cli/src/commands.rs`
* `.monad/context/context-pack.md` or `.monad/context/latest-context-pack.md`
* tests as appropriate

## Tasks

* [ ] Add context pack module.
* [ ] Define default file inclusion order.
* [ ] Assemble selected files into one output.
* [ ] Add source file markers or headings.
* [ ] Add CLI entrypoint if appropriate.
* [ ] Add tests for deterministic assembly.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic context-pack commit.

## Deliverables

* Context pack assembler exists.
* Context pack output can be generated.
* Output includes selected canonical context files in a predictable order.
* Future AI handoffs have a compact source bundle.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- context pack
cat .monad/context/latest-context-pack.md
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `cargo run -p monad-cli -- context pack` exits successfully once command routing exists.
* Context pack file exists.
* Context pack includes ordered source sections.
* Missing optional files are handled clearly rather than causing confusing failure.

## Definition of Done

* [ ] Context pack assembler exists.
* [ ] Default file order is defined.
* [ ] Context pack can be generated.
* [ ] Output is deterministic and readable.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core crates/monad-cli .monad/context
git commit -m "feat(context): assemble context packs"
```

## Risks / Blockers / Open Questions

* Context packs can become too large.
* Missing files should be reported clearly.
* The first version should avoid smart ranking and focus on deterministic assembly.
* Later versions may need token budgets and profile-specific packs.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e3-005.md" <<'BODY'

## Work Packet ID

WP-E3-005

## Parent Epic ID

E3

## Work Packet Title

Implement bootstrap prompt generator

## Product Area

Context Bridge

## Objective

Implement a bootstrap prompt generator that produces a fresh-chat prompt for continuing Monad work from repo-resident context.

The prompt should tell an AI assistant what to read first, what project rules to follow, and how to continue without relying on hidden chat memory.

## User Value

This work matters because Monad’s development workflow depends on reliable context handoffs.

For future users, this feature helps any AI-capable workflow start from repo truth rather than stale memory, scattered notes, or vague instructions.

## Scope

### In scope

* Generate `docs/ai/BOOTSTRAP-PROMPT.md` or a generated equivalent.
* Include required reading order.
* Include project identity summary.
* Include workflow rules.
* Include instruction to use repo files as source of truth.
* Include active handoff/current-state references.
* Add tests where practical.

### Out of scope

* Model-specific prompt tuning.
* Provider-specific API calls.
* Automatic prompt submission.
* Vector retrieval.
* Agent execution.
* Multi-agent orchestration.

## Expected Files or Directories Affected

* `crates/monad-core/src/context/bootstrap.rs`
* `crates/monad-core/src/context.rs`
* `crates/monad-cli/src/commands.rs`
* `docs/ai/BOOTSTRAP-PROMPT.md`
* tests as appropriate

## Tasks

* [ ] Add bootstrap prompt generator module.
* [ ] Define bootstrap prompt sections.
* [ ] Include required reading order.
* [ ] Include project source-of-truth rules.
* [ ] Include workflow rules.
* [ ] Add CLI entrypoint if appropriate.
* [ ] Add tests for deterministic output.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic bootstrap-generator commit.

## Deliverables

* Bootstrap prompt generator exists.
* Bootstrap prompt can be generated or updated.
* Prompt provides a clear reading order and continuation protocol.
* Prompt reinforces repo-native truth.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- context generate bootstrap
cat docs/ai/BOOTSTRAP-PROMPT.md
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `cargo run -p monad-cli -- context generate bootstrap` exits successfully once command routing exists.
* `docs/ai/BOOTSTRAP-PROMPT.md` exists.
* Bootstrap prompt includes reading order, source-of-truth rules, current-state references, and handoff instructions.

## Definition of Done

* [ ] Bootstrap prompt generator exists.
* [ ] Bootstrap prompt can be written.
* [ ] Reading order is included.
* [ ] Repo source-of-truth rule is included.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core crates/monad-cli docs/ai
git commit -m "feat(context): generate bootstrap prompt"
```

## Risks / Blockers / Open Questions

* Prompt should be concise enough to be usable.
* Prompt should not embed stale implementation details.
* Provider-specific prompting should wait.
* Generated prompt should stay aligned with context artifact standards.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e3-006.md" <<'BODY'

## Work Packet ID

WP-E3-006

## Parent Epic ID

E3

## Work Packet Title

Add context verification checks

## Product Area

Context Bridge

## Objective

Add basic checks that verify required Monad context files exist and meet minimal structural expectations.

This work packet should create the first guardrail against context rot.

## User Value

This work matters because context files are only useful if they exist, are findable, and are maintained.

Users and AI assistants should be able to trust that Monad’s context bridge has the basic artifacts required for handoff and continuation.

## Scope

### In scope

* Define required context files.
* Check for missing context files.
* Check for minimal required headings or sections if practical.
* Report clear diagnostics.
* Integrate checks with existing diagnostic model or verification foundation if available.
* Add tests for missing and present context artifacts.

### Out of scope

* Full semantic validation.
* Staleness detection based on GitHub issues.
* AI-based summarization validation.
* Vector index validation.
* Cloud synchronization.
* Full verification engine integration if E4 has not been implemented yet.

## Expected Files or Directories Affected

* `crates/monad-core/src/context/verify.rs`
* `crates/monad-core/src/context.rs`
* `crates/monad-cli/src/commands.rs`
* tests as appropriate

## Tasks

* [ ] Define required context artifact list.
* [ ] Add context verification function.
* [ ] Report missing files clearly.
* [ ] Add minimal structure checks where practical.
* [ ] Add CLI entrypoint if appropriate.
* [ ] Add tests for success and failure cases.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic context-verification commit.

## Deliverables

* Context verification checks exist.
* Missing context artifacts produce clear diagnostics.
* Tests prove verification behavior.
* Context bridge has a baseline quality gate.

## Verification Commands / Evidence

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- context verify
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `cargo run -p monad-cli -- context verify` exits successfully once command routing exists.
* Required context files are checked.
* Missing files produce understandable diagnostics.
* Existing valid context files pass basic verification.

## Definition of Done

* [ ] Required context artifact list exists.
* [ ] Context verification function exists.
* [ ] Missing files are reported clearly.
* [ ] Tests cover success and failure cases.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash
git add crates/monad-core crates/monad-cli
git commit -m "feat(context): add context verification checks"
```

## Risks / Blockers / Open Questions

* Verification should not be too strict before context formats stabilize.
* Missing optional files should be distinguished from missing required files.
* If E4 verification engine does not exist yet, this packet should keep checks simple and local.

## Priority

P1 High

## Size

M
BODY

create_issue \
"[Epic]: E3 — Context Bridge" \
"type:epic,area:context-bridge,priority:p1,needs-verification,context-update-required" \
"${TMP_DIR}/e3.md"

create_issue \
"[Work Packet]: WP-E3-001 — Define context artifact schemas" \
"type:work-packet,area:context-bridge,area:docs,priority:p1,needs-verification,context-update-required" \
"${TMP_DIR}/wp-e3-001.md"

create_issue \
"[Work Packet]: WP-E3-002 — Implement current-state generator" \
"type:work-packet,area:context-bridge,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e3-002.md"

create_issue \
"[Work Packet]: WP-E3-003 — Implement handoff generator" \
"type:work-packet,area:context-bridge,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e3-003.md"

create_issue \
"[Work Packet]: WP-E3-004 — Implement context pack assembler" \
"type:work-packet,area:context-bridge,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e3-004.md"

create_issue \
"[Work Packet]: WP-E3-005 — Implement bootstrap prompt generator" \
"type:work-packet,area:context-bridge,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e3-005.md"

create_issue \
"[Work Packet]: WP-E3-006 — Add context verification checks" \
"type:work-packet,area:context-bridge,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e3-006.md"

echo
echo "Done seeding E3."
echo
echo "Manual follow-up:"
echo "- Open the E3 epic issue in GitHub."
echo "- Add WP-E3-001 through WP-E3-006 as sub-issues under E3."
echo "- Set project fields if needed:"
echo "  Type, Status, Epic, Product Area, Priority, Size, Work Packet ID."
echo "- Recommended status for E3 and its work packets right now: Deferred or Ready, not Active."
