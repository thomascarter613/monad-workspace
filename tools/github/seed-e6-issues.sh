#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   ./tools/github/seed-e6-issues.sh OWNER REPO [PROJECT_NUMBER]
#
# Example:
#   ./tools/github/seed-e6-issues.sh thomascarter613 monad-workspace 1
#
# Notes:
# - This creates the E6 epic and E6 work packet issues.
# - It skips issues that already exist with the same exact title.
# - It optionally adds created/existing issues to a GitHub Project.
# - It does not set sub-issue hierarchy or custom project fields automatically.

OWNER="${1:?Missing GitHub owner, e.g. thomascarter613}"
REPO="${2:?Missing GitHub repo name, e.g. monad-workspace}"
PROJECT_NUMBER="${3:-}"

REPO_SLUG="${OWNER}/${REPO}"

echo "Seeding E6 issues into ${REPO_SLUG}"

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
  ensure_label "area:agents" "AI agents, model-provider integrations, supervised execution, and approvals." "1d76db"
  ensure_label "area:core" "Monad core runtime, shared domain logic, and foundational engine work." "1d76db"
  ensure_label "area:cli" "Command-line interface, command routing, help output, and CLI UX." "1d76db"
  ensure_label "area:verification" "Checks, test orchestration, evidence packets, and verification reports." "1d76db"
  ensure_label "area:evolution" "Safe repository changes, generators, migrations, file operations, and apply workflows." "1d76db"
  ensure_label "area:policy" "Policy, governance, safety boundaries, approvals, and enforcement rules." "1d76db"
  ensure_label "area:integrations" "External tool integrations, protocols, MCP, providers, and interoperability." "1d76db"
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

cat > "${TMP_DIR}/e6.md" <<'BODY'
## Product Area

Agent Supervision

## Objective

Define and implement Monad’s first supervised AI-agent workflow: planning, drafting, verifying, reviewing, and approving AI-assisted changes without giving agents uncontrolled authority.

This epic should establish the foundation for human-in-command agent assistance while keeping repo-native context, verification evidence, approval gates, and auditability at the center.

## User Value

This epic brings Monad closer to the full Software Foundry vision: AI-assisted, evidence-backed, architecture-aware software evolution.

Users should be able to use AI as a supervised engineering crew while retaining final authority over plans, file changes, command execution, verification, and commits.

For maintainers and contributors, E6 creates the foundation for model-provider abstraction, intent-to-plan workflows, draft sandboxes, approval gates, audit logs, and MCP-compatible integration surfaces.

## Scope

### In scope

- Supervised agent workflow standard.
- Model-provider abstraction foundation.
- Basic `monad plan` command.
- Draft sandbox workflow concept.
- Human approval gate model.
- Audit log model.
- Initial MCP integration foundation.
- Safety boundaries for agent-assisted execution.
- Documentation of what agents may and may not do.

### Out of scope

- Fully autonomous coding.
- Unrestricted command execution.
- Unreviewed file writes.
- Automatic production deployment.
- Enterprise cloud control plane.
- Billing.
- Multi-tenant hosted agents.
- Marketplace.
- Provider-specific optimization.
- Long-running background agents.

## Expected Work Packets

- WP-E6-001 — Define supervised agent workflow
- WP-E6-002 — Add model provider abstraction
- WP-E6-003 — Add plan command
- WP-E6-004 — Add draft sandbox workflow
- WP-E6-005 — Add approval gates and audit log
- WP-E6-006 — Add MCP integration foundation

## Deliverables

- Supervised agent workflow documentation.
- Agent execution policy.
- Provider abstraction design.
- Initial `monad plan` command.
- Draft workflow prototype.
- Approval gate model.
- Audit/event log model.
- MCP integration foundation.
- Tests where applicable.
- Clear safety boundaries that keep the human in command.

## Verification Strategy

Suggested verification commands:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-workspace-cli -- plan "explain this repository"
```

Expected result:

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* Agent workflow documentation exists.
* Provider abstraction compiles once implemented.
* `monad plan` produces a structured plan once implemented.
* Approval gates and audit logs are represented before risky operations are permitted.
* No fully autonomous execution has been introduced.
* No unreviewed file writes are performed by agent workflows.
* The system remains provider-agnostic.

## Risks / Open Questions

* Agent features can become too broad too quickly.
* Provider abstraction must avoid lock-in.
* Safety boundaries must be explicit from the beginning.
* Model output must not be treated as verified truth.
* MCP integration should not distract from core local value.
* Draft workflows must not bypass file operation safety rules.
* Approval gates must be understandable and enforceable.
* We need to avoid building a cloud agent platform before the local supervised workflow works.

## Priority

P1 High

## Size

L
BODY

cat > "${TMP_DIR}/wp-e6-001.md" <<'BODY'

## Work Packet ID

WP-E6-001

## Parent Epic ID

E6

## Work Packet Title

Define supervised agent workflow

## Product Area

Agent Supervision

## Objective

Document Monad’s human-in-command supervised agent workflow before implementing agent execution behavior.

This work packet should define how Monad uses AI assistance safely: plan first, draft separately, verify, review, require approval, then apply only through explicit user-controlled steps.

## User Value

This work matters because users will not trust an AI development tool that can change repositories unpredictably.

For users, the supervised workflow makes clear that Monad agents assist rather than silently take over. For maintainers, it creates the safety standard that later agent features must follow.

## Scope

### In scope

* Define supervised agent lifecycle.
* Define agent modes such as explain, plan, draft, verify, repair, review, and apply.
* Define human approval requirements.
* Define what actions are forbidden without explicit approval.
* Define relationship to file operations, verification, context, and evidence packets.
* Document safety principles.

### Out of scope

* Implementing model calls.
* Implementing `monad plan`.
* Implementing file writes.
* Implementing sandbox execution.
* Implementing MCP server.
* Implementing cloud agents.
* Implementing provider integrations.

## Expected Files or Directories Affected

* `docs/architecture/SUPERVISED-AGENT-WORKFLOW.md`
* `docs/security/AGENT-SAFETY-MODEL.md`
* `docs/ai/AGENT-RUNBOOK.md`
* `docs/workflow/APPROVAL-GATES.md`

## Tasks

* [ ] Define supervised agent lifecycle.
* [ ] Define agent modes.
* [ ] Define approval requirements.
* [ ] Define forbidden actions.
* [ ] Define relationship to verification and evidence.
* [ ] Define relationship to safe file operations.
* [ ] Document safety principles.
* [ ] Verify docs are internally consistent.
* [ ] Commit as one atomic agent-workflow documentation commit.

## Deliverables

* Supervised agent workflow is documented.
* Agent safety model is documented.
* Approval expectations are documented.
* Future agent implementation has clear guardrails.

## Verification Commands / Evidence

```bash id="qc3s72"
find docs/architecture docs/security docs/ai docs/workflow -maxdepth 3 -type f | sort
git status --short
```

## Expected Result After Verification

* Supervised agent workflow documentation exists.
* Agent safety model documentation exists.
* Agent runbook exists or is updated.
* Approval-gate documentation exists or is updated.
* No agent execution code has been added yet.
* Repository changes are ready for one atomic documentation commit.

## Definition of Done

* [ ] Supervised agent lifecycle is documented.
* [ ] Agent modes are documented.
* [ ] Approval requirements are documented.
* [ ] Forbidden actions are documented.
* [ ] Verification/evidence relationship is documented.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash id="ngsniu"
git add docs/architecture docs/security docs/ai docs/workflow
git commit -m "docs(agents): define supervised agent workflow"
```

## Risks / Blockers / Open Questions

* Workflow must be strict enough to be safe but not so heavy that it becomes unusable.
* Agent modes may need refinement once implementation begins.
* Approval gates should align with E5 safe file operations and E4 verification evidence.

## Priority

P1 High

## Size

S
BODY

cat > "${TMP_DIR}/wp-e6-002.md" <<'BODY'

## Work Packet ID

WP-E6-002

## Parent Epic ID

E6

## Work Packet Title

Add model provider abstraction

## Product Area

Agent Supervision

## Objective

Add Monad’s initial model-provider abstraction so future AI features can remain provider-agnostic and subscription-agnostic.

This work packet should define the first interface or trait for requesting model-assisted output without binding Monad to one AI vendor.

## User Value

This work matters because Monad should not require one model provider, one API, or one paid subscription.

Users should eventually be able to connect hosted providers, local models, self-hosted services, MCP-compatible tools, or future Monad-managed services without changing Monad’s core workflow.

## Scope

### In scope

* Define provider abstraction type or trait.
* Define model request structure.
* Define model response structure.
* Define provider capability metadata if useful.
* Add a no-op/mock provider for tests.
* Keep provider abstraction small and easy to replace.
* Add tests for mock provider behavior.

### Out of scope

* Real provider API integration.
* API key management.
* Streaming responses.
* Tool calling.
* MCP implementation.
* Cost tracking.
* Model routing.
* Provider marketplace.
* Cloud service.

## Expected Files or Directories Affected

* `crates/monad-core/src/agents.rs`
* `crates/monad-core/src/agents/provider.rs`
* `crates/monad-core/src/agents/model.rs`
* `crates/monad-core/src/lib.rs`
* tests as appropriate

## Tasks

* [ ] Create agents module boundary.
* [ ] Define provider abstraction.
* [ ] Define request type.
* [ ] Define response type.
* [ ] Add mock provider.
* [ ] Add tests for mock provider.
* [ ] Export agents module from `monad-core`.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic provider-abstraction commit.

## Deliverables

* Model-provider abstraction exists.
* Request/response types exist.
* Mock provider exists for tests.
* Tests prove provider abstraction can be used without a real model API.
* No provider lock-in is introduced.

## Verification Commands / Evidence

```bash id="2fhj9r"
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
find crates/monad-core/src -maxdepth 4 -type f | sort
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* Agent/provider module files exist.
* Mock provider tests pass.
* No real external AI API call is made.
* No provider-specific dependency is required.

## Definition of Done

* [ ] Provider abstraction exists.
* [ ] Request model exists.
* [ ] Response model exists.
* [ ] Mock provider exists.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash id="l9t4pe"
git add crates/monad-core
git commit -m "feat(agents): add model provider abstraction"
```

## Risks / Blockers / Open Questions

* Abstraction should not overfit to any one provider.
* Avoid advanced async complexity until justified.
* Streaming and tool calling can wait.
* Provider configuration and secrets should be handled in later work.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e6-003.md" <<'BODY'

## Work Packet ID

WP-E6-003

## Parent Epic ID

E6

## Work Packet Title

Add plan command

## Product Area

Agent Supervision

## Objective

Add the initial `monad plan` command that converts a user intent into a structured, reviewable plan without making repository changes.

The first implementation may use a mock provider or deterministic local planning behavior if real model integration is not yet available.

## User Value

This work matters because planning is the safe first step of AI-assisted development.

Users should be able to express an intent and receive a structured plan that can be reviewed before any files are changed, commands are run, or agents are allowed to act.

## Scope

### In scope

* Add `plan` command to CLI.
* Accept a user intent string.
* Produce a structured plan output.
* Use provider abstraction or deterministic local mock behavior.
* Keep output reviewable.
* Do not write files.
* Add tests where practical.

### Out of scope

* File modification.
* Agent draft execution.
* Real provider API calls unless already available.
* Tool calling.
* Auto-running verification.
* Opening PRs.
* Multi-step autonomous execution.

## Expected Files or Directories Affected

* `crates/monad-cli/src/cli.rs`
* `crates/monad-cli/src/commands.rs`
* `crates/monad-core/src/agents/plan.rs`
* `crates/monad-core/src/agents.rs`
* tests as appropriate

## Tasks

* [ ] Add `plan` command to CLI.
* [ ] Accept intent argument.
* [ ] Define plan output model.
* [ ] Generate deterministic plan using mock/local behavior.
* [ ] Render human-readable plan.
* [ ] Ensure no files are written.
* [ ] Add tests where practical.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic plan-command commit.

## Deliverables

* `monad plan` command exists.
* Command accepts user intent.
* Command produces structured plan output.
* Command does not modify files.
* Plan behavior is testable.

## Verification Commands / Evidence

```bash id="224610"
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-workspace-cli -- plan "explain this repository"
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* `cargo run -p monad-workspace-cli -- plan "explain this repository"` exits successfully.
* Command prints a structured plan.
* No repository files are modified.
* No real model provider is required unless separately configured.

## Definition of Done

* [ ] `plan` command exists.
* [ ] Intent argument is accepted.
* [ ] Structured plan output exists.
* [ ] Command does not write files.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash id="bzb7ye"
git add crates/monad-cli crates/monad-core
git commit -m "feat(cli): add plan command"
```

## Risks / Blockers / Open Questions

* The command should not imply a plan is verified truth.
* Real model integration should not be required for the first command.
* Output should clearly identify assumptions and non-actions.
* Plan command must not become an uncontrolled execution path.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e6-004.md" <<'BODY'

## Work Packet ID

WP-E6-004

## Parent Epic ID

E6

## Work Packet Title

Add draft sandbox workflow

## Product Area

Agent Supervision

## Objective

Define and prototype Monad’s draft sandbox workflow for preparing AI-assisted changes in an isolated, reviewable way before applying them to the main working tree.

This work packet should connect agent planning concepts with safe file operation and worktree safety principles.

## User Value

This work matters because users need a safe place for proposed changes.

An AI-assisted draft should not immediately modify important files without review. The draft workflow should allow proposed work to be generated, inspected, verified, and accepted or discarded.

## Scope

### In scope

* Define draft workflow model.
* Represent draft state.
* Represent draft workspace or sandbox concept.
* Connect draft workflow to safe file operations.
* Document or prototype basic draft behavior.
* Add tests where practical.

### Out of scope

* Full autonomous coding.
* Real AI-generated file modifications.
* Automatic branch creation unless already safe.
* Automatic PR creation.
* Cloud sandboxing.
* Multi-agent execution.
* Long-running background tasks.

## Expected Files or Directories Affected

* `docs/architecture/DRAFT-SANDBOX-WORKFLOW.md`
* `crates/monad-core/src/agents/draft.rs`
* `crates/monad-core/src/agents.rs`
* `crates/monad-core/src/file_ops/`
* tests as appropriate

## Tasks

* [ ] Document draft sandbox workflow.
* [ ] Define draft state model.
* [ ] Define draft operation relationship to file operations.
* [ ] Add prototype model in `monad-core` if appropriate.
* [ ] Add tests where practical.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic draft-sandbox workflow commit.

## Deliverables

* Draft sandbox workflow is documented.
* Draft state model exists if implementation is included.
* Draft workflow is connected conceptually to safe file operations.
* Future agent-generated changes have a safe workflow target.

## Verification Commands / Evidence

```bash id="3kq98t"
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
find docs/architecture crates/monad-core/src -maxdepth 4 -type f | sort
```

## Expected Result After Verification

* Documentation for draft sandbox workflow exists.
* Formatting passes if code changed.
* Tests pass if code changed.
* Clippy passes if code changed.
* Draft workflow does not perform unreviewed file writes.
* No autonomous execution has been added.

## Definition of Done

* [ ] Draft sandbox workflow is documented.
* [ ] Draft state is defined if implemented.
* [ ] Relationship to safe file operations is clear.
* [ ] Tests pass if implementation is included.
* [ ] Formatting passes.
* [ ] Clippy passes if code changed.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash id="wyr6uf"
git add docs/architecture crates/monad-core
git commit -m "feat(agents): add draft sandbox workflow"
```

## Risks / Blockers / Open Questions

* Draft workflow should not bypass E5 file operation safety.
* Worktree and branch strategy may need to be finalized before full implementation.
* Keep the first version as a controlled workflow model rather than an autonomous agent system.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e6-005.md" <<'BODY'

## Work Packet ID

WP-E6-005

## Parent Epic ID

E6

## Work Packet Title

Add approval gates and audit log

## Product Area

Policy / Governance

## Objective

Add Monad’s initial approval gate and audit log model for risky or consequential actions.

This work packet should define how Monad records proposed actions, required approvals, approval decisions, and execution events.

## User Value

This work matters because agent-assisted repository evolution must be accountable.

Users should be able to see what was proposed, who or what approved it, when it was approved, what happened afterward, and what evidence was produced.

For future teams, this becomes the foundation for trust, governance, compliance, review, and enterprise readiness.

## Scope

### In scope

* Define approval gate model.
* Define approval decision model.
* Define audit event model.
* Define basic audit log structure.
* Record proposed action metadata.
* Record approval/rejection metadata.
* Add tests for audit event creation.
* Keep storage simple and repo-native if writing to disk.

### Out of scope

* Enterprise RBAC.
* SSO.
* Multi-user cloud approvals.
* Cryptographic signing.
* Tamper-proof ledger.
* Remote audit storage.
* Compliance certification.
* Full policy engine.

## Expected Files or Directories Affected

* `crates/monad-core/src/policy.rs`
* `crates/monad-core/src/policy/approval.rs`
* `crates/monad-core/src/policy/audit.rs`
* `.monad/reports/` or `.monad/audit/` if disk output is included
* tests as appropriate

## Tasks

* [ ] Create policy/governance module boundary.
* [ ] Define approval gate type.
* [ ] Define approval decision type.
* [ ] Define audit event type.
* [ ] Define audit log model.
* [ ] Add tests for approval and audit event creation.
* [ ] Export policy module from `monad-core`.
* [ ] Verify formatting.
* [ ] Verify tests.
* [ ] Verify Clippy.
* [ ] Commit as one atomic approval/audit commit.

## Deliverables

* Approval gate model exists.
* Audit event model exists.
* Audit log model exists.
* Tests prove approval and audit events can be represented.
* Future risky actions have a governance foundation.

## Verification Commands / Evidence

```bash id="8e2vt5"
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
find crates/monad-core/src .monad -maxdepth 4 -type f | sort
```

## Expected Result After Verification

* Formatting passes.
* Tests pass.
* Clippy passes with warnings denied.
* Policy/governance module files exist.
* Tests prove approval gates can be represented.
* Tests prove audit events can be represented.
* No enterprise RBAC, SSO, or remote audit service has been added.

## Definition of Done

* [ ] Approval gate model exists.
* [ ] Approval decision model exists.
* [ ] Audit event model exists.
* [ ] Tests pass.
* [ ] Formatting passes.
* [ ] Clippy passes.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash id="md2zrr"
git add crates/monad-core .monad
git commit -m "feat(governance): add approval gates and audit log"
```

## Risks / Blockers / Open Questions

* Audit model should not overpromise compliance-grade guarantees yet.
* Approval gates must be enforceable by future workflows.
* Disk output can wait if model-level work is enough for this packet.
* Avoid confusing local audit records with immutable enterprise audit logs.

## Priority

P1 High

## Size

M
BODY

cat > "${TMP_DIR}/wp-e6-006.md" <<'BODY'

## Work Packet ID

WP-E6-006

## Parent Epic ID

E6

## Work Packet Title

Add MCP integration foundation

## Product Area

Agent Supervision

## Objective

Add Monad’s initial MCP integration foundation so selected Monad capabilities can later be exposed to AI tools through a standard tool/resource/prompt interface.

This work packet should establish the boundary and design for MCP compatibility without distracting from Monad’s local CLI value.

## User Value

This work matters because Monad should integrate with the broader AI tooling ecosystem rather than being isolated.

Users should eventually be able to expose repo context, safe tools, verification outputs, and planning capabilities to compatible AI clients while preserving Monad’s safety and approval rules.

## Scope

### In scope

* Document MCP integration strategy.
* Define which Monad capabilities are candidates for MCP exposure.
* Define safety boundaries for MCP tools.
* Add initial module boundary or crate placeholder if appropriate.
* Keep implementation minimal.
* Add tests only if code is included.

### Out of scope

* Full MCP server implementation.
* Tool calling with real agents.
* Remote hosted MCP service.
* Authentication/authorization.
* Streaming protocol implementation.
* Exposing unsafe write operations.
* Provider-specific integrations.
* Marketplace distribution.

## Expected Files or Directories Affected

* `docs/architecture/MCP-INTEGRATION-STRATEGY.md`
* `docs/security/MCP-SAFETY-BOUNDARIES.md`
* `crates/monad-mcp/Cargo.toml` if a crate placeholder is created
* `crates/monad-mcp/src/lib.rs` if a crate placeholder is created
* root `Cargo.toml` if a crate placeholder is created

## Tasks

* [ ] Document MCP integration strategy.
* [ ] Define candidate tools/resources/prompts.
* [ ] Define MCP safety boundaries.
* [ ] Decide whether to create a crate placeholder now.
* [ ] Add crate placeholder if appropriate.
* [ ] Verify formatting if code changed.
* [ ] Verify tests if code changed.
* [ ] Verify Clippy if code changed.
* [ ] Commit as one atomic MCP-foundation commit.

## Deliverables

* MCP integration strategy exists.
* MCP safety boundaries are documented.
* Candidate MCP-exposed Monad capabilities are listed.
* Optional crate placeholder exists if included.
* Future MCP work has a clear boundary.

## Verification Commands / Evidence

```bash id="vivxty"
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
find docs/architecture docs/security crates -maxdepth 4 -type f | sort
```

## Expected Result After Verification

* MCP strategy documentation exists.
* MCP safety boundary documentation exists.
* Formatting passes if code changed.
* Tests pass if code changed.
* Clippy passes if code changed.
* No unsafe write operation is exposed through MCP.
* No full MCP server implementation is required in this packet.

## Definition of Done

* [ ] MCP integration strategy is documented.
* [ ] MCP safety boundaries are documented.
* [ ] Candidate exposed capabilities are listed.
* [ ] Optional crate placeholder exists if included.
* [ ] Formatting passes if code changed.
* [ ] Tests pass if code changed.
* [ ] Clippy passes if code changed.
* [ ] Atomic commit completed.

## Recommended Conventional Commit

```bash id="wxstqo"
git add docs/architecture docs/security crates Cargo.toml
git commit -m "feat(integrations): add mcp foundation"
```

## Risks / Blockers / Open Questions

* MCP work should not distract from Monad’s local-first CLI foundation.
* Do not expose write/apply tools until approval gates and file operation safety are enforced.
* Full MCP server behavior can wait.
* Integration design should remain provider-agnostic.

## Priority

P1 High

## Size

M
BODY

create_issue \
"[Epic]: E6 — Agent Supervision" \
"type:epic,area:agents,priority:p1,needs-verification,context-update-required" \
"${TMP_DIR}/e6.md"

create_issue \
"[Work Packet]: WP-E6-001 — Define supervised agent workflow" \
"type:work-packet,area:agents,area:policy,priority:p1,needs-verification,context-update-required" \
"${TMP_DIR}/wp-e6-001.md"

create_issue \
"[Work Packet]: WP-E6-002 — Add model provider abstraction" \
"type:work-packet,area:agents,area:core,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e6-002.md"

create_issue \
"[Work Packet]: WP-E6-003 — Add plan command" \
"type:work-packet,area:agents,area:cli,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e6-003.md"

create_issue \
"[Work Packet]: WP-E6-004 — Add draft sandbox workflow" \
"type:work-packet,area:agents,area:evolution,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e6-004.md"

create_issue \
"[Work Packet]: WP-E6-005 — Add approval gates and audit log" \
"type:work-packet,area:policy,area:agents,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e6-005.md"

create_issue \
"[Work Packet]: WP-E6-006 — Add MCP integration foundation" \
"type:work-packet,area:integrations,area:agents,priority:p1,needs-verification,context-update-required,rust-learning" \
"${TMP_DIR}/wp-e6-006.md"

echo
echo "Done seeding E6."
echo
echo "Manual follow-up:"
echo "- Open the E6 epic issue in GitHub."
echo "- Add WP-E6-001 through WP-E6-006 as sub-issues under E6."
echo "- Set project fields if needed:"
echo "  Type, Status, Epic, Product Area, Priority, Size, Work Packet ID."
echo "- Recommended status for E6 and its work packets right now: Deferred or Ready, not Active."
