#!/usr/bin/env bash
# create_remaining_monad_issues_e20_e45.sh
#
# Creates the remaining Monad roadmap GitHub issues and subissues for:
#   E20 / WP-E20-001 through E45 / WP-E45-006
#
# Defaults to DRY RUN mode.
#
# Usage:
#   bash create_remaining_monad_issues_e20_e45.sh
#   bash create_remaining_monad_issues_e20_e45.sh --apply
#   bash create_remaining_monad_issues_e20_e45.sh --apply --repo thomascarter613/monad-workspace
#
# Requirements:
#   - gh CLI installed and authenticated
#   - python3 installed
#   - repository write permission
#
# Notes:
#   - The script is intentionally idempotent by exact issue title.
#   - If an issue already exists, it reuses it instead of creating a duplicate.
#   - GitHub subissue linking is performed with the GraphQL addSubIssue mutation.
#   - If subissue linking is unavailable for the repo/account, the issues are still created.

set -Eeuo pipefail
IFS=$'\n\t'

REPO="${MONAD_REPO:-thomascarter613/monad-workspace}"
DRY_RUN="${DRY_RUN:-1}"
LINK_SUBISSUES="${LINK_SUBISSUES:-1}"
CREATE_LABELS="${CREATE_LABELS:-1}"
MILESTONE="${MILESTONE:-}"
PROJECT="${PROJECT:-}"

CREATED_EPICS=0
REUSED_EPICS=0
CREATED_WORKPACKETS=0
REUSED_WORKPACKETS=0
LINKED_SUBISSUES=0
SKIPPED_LINKS=0
FAILED_LINKS=0

TMP_DIR=""
ISSUE_CACHE=""

usage() {
  cat <<'USAGE'
Create Monad GitHub issues/subissues for E20 through E45.

Default mode is dry-run. Use --apply to create/link issues.

Usage:
  bash create_remaining_monad_issues_e20_e45.sh [options]

Options:
  --apply              Actually create issues and link subissues.
  --dry-run            Print planned actions without creating issues. Default.
  --repo OWNER/REPO    Target repository. Default: thomascarter613/monad-workspace
  --no-subissues       Create/reuse issues but skip GitHub subissue linking.
  --no-labels          Do not create labels and do not apply labels to issues.
  --milestone NAME     Add created issues to an existing milestone.
  --project TITLE      Add created issues to an existing GitHub project by title.
  -h, --help           Show this help.

Environment alternatives:
  MONAD_REPO=OWNER/REPO
  DRY_RUN=0
  LINK_SUBISSUES=0
  CREATE_LABELS=0
  MILESTONE='v1 MVP'
  PROJECT='Monad Roadmap'

Examples:
  bash create_remaining_monad_issues_e20_e45.sh

  bash create_remaining_monad_issues_e20_e45.sh --apply

  bash create_remaining_monad_issues_e20_e45.sh \
    --apply \
    --repo thomascarter613/monad-workspace \
    --project 'Monad Roadmap'
USAGE
}

log() {
  printf '%s\n' "$*" >&2
}

warn() {
  printf 'WARN: %s\n' "$*" >&2
}

die() {
  printf 'ERROR: %s\n' "$*" >&2
  exit 1
}

parse_args() {
  while (($#)); do
    case "$1" in
      --apply)
        DRY_RUN=0
        shift
        ;;
      --dry-run)
        DRY_RUN=1
        shift
        ;;
      --repo)
        [[ $# -ge 2 ]] || die "--repo requires OWNER/REPO"
        REPO="$2"
        shift 2
        ;;
      --no-subissues)
        LINK_SUBISSUES=0
        shift
        ;;
      --no-labels)
        CREATE_LABELS=0
        shift
        ;;
      --milestone)
        [[ $# -ge 2 ]] || die "--milestone requires a milestone name"
        MILESTONE="$2"
        shift 2
        ;;
      --project)
        [[ $# -ge 2 ]] || die "--project requires a project title"
        PROJECT="$2"
        shift 2
        ;;
      -h|--help)
        usage
        exit 0
        ;;
      *)
        die "Unknown argument: $1"
        ;;
    esac
  done
}

require_tools() {
  command -v gh >/dev/null 2>&1 || die "GitHub CLI 'gh' is required."
  command -v python3 >/dev/null 2>&1 || die "python3 is required."
}

check_github_access() {
  gh auth status --hostname github.com >/dev/null 2>&1 || {
    die "gh is not authenticated. Run: gh auth login"
  }

  gh repo view "$REPO" --json nameWithOwner --jq '.nameWithOwner' >/dev/null 2>&1 || {
    die "Cannot access repository '$REPO'. Check repo name and permissions."
  }
}

setup_temp() {
  TMP_DIR="$(mktemp -d)"
  ISSUE_CACHE="$TMP_DIR/issues.tsv"
  trap 'rm -rf "$TMP_DIR"' EXIT
}

emit_epics() {
  cat <<'TSV'
E20	Patch Planning and Supervised Apply Foundation
E21	Work Packet Execution Workflow Foundation
E22	Repo Contract Schema and Validation Foundation
E23	Language Adapter Foundation
E24	LSP and Static Analysis Foundation
E25	Dependency Graph and Impact Analysis Foundation
E26	Test Intelligence and Verification Planning Foundation
E27	Build Cache and Incremental Execution Foundation
E28	Local Artifact and Report Store Foundation
E29	Template Registry and Preset Evolution Foundation
E30	Plugin and Extension System Foundation
E31	MCP and External Tool Integration Foundation
E32	Local AI Retrieval and Vector Memory Foundation
E33	Agent Workflow Sandbox Foundation
E34	Interactive Workbench / TUI Foundation
E35	Web Workbench Foundation
E36	GitHub Integration and PR Workflow Foundation
E37	Remote Execution and CI Parity Foundation
E38	Release Channel and Installer Ecosystem Foundation
E39	Security and Supply Chain Hardening Foundation
E40	Governance and Compliance Evidence Foundation
E41	Team and Multi-User Workflow Foundation
E42	Cloud/Hosted Control Plane Exploration
E43	Enterprise Policy and Organization Standards Foundation
E44	Marketplace and Community Template Ecosystem
E45	Self-Seeding Monad Evolution Foundation
TSV
}

emit_workpackets() {
  cat <<'TSV'
E20	WP-E20-001	Define patch planning and supervised apply contract
E20	WP-E20-002	Add change-set and patch diff model
E20	WP-E20-003	Add patch dry-run plan output
E20	WP-E20-004	Add patch validation and conflict checks
E20	WP-E20-005	Add supervised apply under approval gates
E20	WP-E20-006	Add patch evidence reports and smoke tests
E21	WP-E21-001	Define work-packet execution model
E21	WP-E21-002	Add work-packet metadata parser
E21	WP-E21-003	Add work-packet implementation plan generator
E21	WP-E21-004	Add verification and evidence checklist automation
E21	WP-E21-005	Add closeout and handoff record generation
E21	WP-E21-006	Add work-packet workflow smoke tests
E22	WP-E22-001	Define repository contract schema boundary
E22	WP-E22-002	Add `monad.toml` schema validation
E22	WP-E22-003	Add `monad.lock` / generated state model
E22	WP-E22-004	Add schema migration planning model
E22	WP-E22-005	Add contract validation fixtures
E22	WP-E22-006	Add contract validation reports and smoke tests
E23	WP-E23-001	Define language adapter interface contract
E23	WP-E23-002	Add Rust adapter foundation
E23	WP-E23-003	Add Node/Bun adapter foundation
E23	WP-E23-004	Add Python adapter foundation
E23	WP-E23-005	Add Go and Java adapter foundations
E23	WP-E23-006	Add adapter registry tests and documentation
E24	WP-E24-001	Define static-analysis and symbol model
E24	WP-E24-002	Add parser abstraction foundation
E24	WP-E24-003	Add LSP discovery and capability model
E24	WP-E24-004	Add symbol extraction proof of concept
E24	WP-E24-005	Add source map and ownership metadata
E24	WP-E24-006	Add static-analysis report tests
E25	WP-E25-001	Define dependency and impact graph model
E25	WP-E25-002	Add component dependency edge detection
E25	WP-E25-003	Add task-to-component graph linkage
E25	WP-E25-004	Add changed-file impact analysis
E25	WP-E25-005	Add impacted verification recommendation output
E25	WP-E25-006	Add graph/impact fixtures and smoke tests
E26	WP-E26-001	Define test intelligence model
E26	WP-E26-002	Discover test commands from manifests
E26	WP-E26-003	Map tests to components/packages
E26	WP-E26-004	Generate targeted verification plans
E26	WP-E26-005	Add verification confidence/evidence model
E26	WP-E26-006	Add verification planner smoke tests
E27	WP-E27-001	Define cache and incremental execution contract
E27	WP-E27-002	Add task fingerprint model
E27	WP-E27-003	Add local execution metadata store
E27	WP-E27-004	Add cache-aware dry-run planning
E27	WP-E27-005	Add incremental execution proof of concept
E27	WP-E27-006	Add cache evidence and invalidation tests
E28	WP-E28-001	Define `.monad/reports` and `.monad/artifacts` contract
E28	WP-E28-002	Add report metadata schema
E28	WP-E28-003	Add artifact metadata schema
E28	WP-E28-004	Add report writing and retention policy
E28	WP-E28-005	Add report index and lookup command foundation
E28	WP-E28-006	Add artifact/report store smoke tests
E29	WP-E29-001	Define template registry evolution contract
E29	WP-E29-002	Add template metadata schema
E29	WP-E29-003	Add preset metadata schema
E29	WP-E29-004	Add template compatibility validation
E29	WP-E29-005	Add preset upgrade planning
E29	WP-E29-006	Add template registry fixtures and tests
E30	WP-E30-001	Define plugin boundary and trust model
E30	WP-E30-002	Add plugin manifest schema
E30	WP-E30-003	Add extension point registry foundation
E30	WP-E30-004	Add adapter/plugin loading plan model
E30	WP-E30-005	Add disabled-by-default plugin safety checks
E30	WP-E30-006	Add plugin contract tests and documentation
E31	WP-E31-001	Define MCP integration boundary
E31	WP-E31-002	Add MCP tool capability model
E31	WP-E31-003	Add MCP context export proof of concept
E31	WP-E31-004	Add external tool invocation policy checks
E31	WP-E31-005	Add MCP/local tool documentation
E31	WP-E31-006	Add MCP integration smoke tests
E32	WP-E32-001	Define local retrieval and vector memory contract
E32	WP-E32-002	Add document chunking model
E32	WP-E32-003	Add embedding provider abstraction
E32	WP-E32-004	Add local index storage proof of concept
E32	WP-E32-005	Add retrieval query and context assembly model
E32	WP-E32-006	Add retrieval fixtures and smoke tests
E33	WP-E33-001	Define local sandbox and agent action boundary
E33	WP-E33-002	Add sandbox workspace model
E33	WP-E33-003	Add isolated draft operation planner
E33	WP-E33-004	Add sandbox verification command path
E33	WP-E33-005	Add sandbox promotion/approval model
E33	WP-E33-006	Add sandbox safety tests
E34	WP-E34-001	Define TUI navigation model
E34	WP-E34-002	Add TUI shell proof of concept
E34	WP-E34-003	Add issue/work-packet view
E34	WP-E34-004	Add plan/report/context viewer
E34	WP-E34-005	Add approval review screen foundation
E34	WP-E34-006	Add TUI smoke tests
E35	WP-E35-001	Define local web workbench architecture
E35	WP-E35-002	Add local server/API foundation
E35	WP-E35-003	Add repository graph view
E35	WP-E35-004	Add work-packet and report views
E35	WP-E35-005	Add approval/context viewer foundation
E35	WP-E35-006	Add web workbench smoke tests
E36	WP-E36-001	Define GitHub integration boundary
E36	WP-E36-002	Add GitHub issue sync/export model
E36	WP-E36-003	Add branch and PR planning model
E36	WP-E36-004	Add PR description and review-pack generation
E36	WP-E36-005	Add issue closeout/evidence helpers
E36	WP-E36-006	Add GitHub workflow smoke tests
E37	WP-E37-001	Define CI parity and remote execution contract
E37	WP-E37-002	Add CI environment detection model
E37	WP-E37-003	Add local-vs-CI command mapping
E37	WP-E37-004	Add reproducibility evidence reports
E37	WP-E37-005	Add remote-runner planning model
E37	WP-E37-006	Add CI parity tests and documentation
E38	WP-E38-001	Define release channel strategy
E38	WP-E38-002	Add multi-platform build matrix
E38	WP-E38-003	Add installer/package strategy docs
E38	WP-E38-004	Add checksum/signing preparation
E38	WP-E38-005	Add GitHub release automation hardening
E38	WP-E38-006	Add release-channel verification tests
E39	WP-E39-001	Define security and supply-chain baseline
E39	WP-E39-002	Add dependency audit integration
E39	WP-E39-003	Add secret/check hygiene foundation
E39	WP-E39-004	Add SBOM/provenance preparation
E39	WP-E39-005	Add signing/attestation preparation
E39	WP-E39-006	Add security evidence reports and tests
E40	WP-E40-001	Define governance evidence model
E40	WP-E40-002	Add ADR traceability checks
E40	WP-E40-003	Add requirement-to-work-packet traceability
E40	WP-E40-004	Add release attestation evidence model
E40	WP-E40-005	Add audit trail report generation
E40	WP-E40-006	Add governance evidence smoke tests
E41	WP-E41-001	Define team workflow and role model
E41	WP-E41-002	Add assignment/reviewer metadata model
E41	WP-E41-003	Add shared approval workflow foundation
E41	WP-E41-004	Add collaboration evidence records
E41	WP-E41-005	Add contributor handoff/report workflow
E41	WP-E41-006	Add team workflow smoke tests
E42	WP-E42-001	Define hosted control-plane exploration boundary
E42	WP-E42-002	Add local-first/cloud-optional architecture note
E42	WP-E42-003	Add hosted sync/use-case analysis
E42	WP-E42-004	Add tenant/org model exploration
E42	WP-E42-005	Add cost/security/risk analysis
E42	WP-E42-006	Add hosted-control-plane go/no-go report
E43	WP-E43-001	Define organization standards pack model
E43	WP-E43-002	Add org-level policy schema draft
E43	WP-E43-003	Add reusable repo baseline pack model
E43	WP-E43-004	Add enterprise verification profile model
E43	WP-E43-005	Add org template/preset governance model
E43	WP-E43-006	Add enterprise standards smoke tests
E44	WP-E44-001	Define marketplace trust and publishing model
E44	WP-E44-002	Add community template metadata schema
E44	WP-E44-003	Add template validation and signing requirements
E44	WP-E44-004	Add adapter/preset discovery model
E44	WP-E44-005	Add marketplace contribution workflow docs
E44	WP-E44-006	Add marketplace safety tests
E45	WP-E45-001	Define Monad self-seeding architecture
E45	WP-E45-002	Add canonical repo blueprint manifest
E45	WP-E45-003	Add self-regeneration planning model
E45	WP-E45-004	Add self-verification and drift detection
E45	WP-E45-005	Add self-upgrade/evolution evidence workflow
E45	WP-E45-006	Add self-seeding closeout and roadmap reset report
TSV
}

epic_title_for_key() {
  local key="$1"
  emit_epics | awk -F '\t' -v k="$key" '$1 == k { print $2; exit }'
}

refresh_issue_cache() {
  log "Refreshing issue cache for $REPO..."
  gh issue list \
    --repo "$REPO" \
    --state all \
    --limit 1000 \
    --json number,title \
    | python3 -c '
import json
import sys

for item in json.load(sys.stdin):
    print(f"{item["number"]}\t{item["title"]}")
' > "$ISSUE_CACHE"
}

issue_number_by_exact_title() {
  local title="$1"

  python3 - "$title" "$ISSUE_CACHE" <<'PY'
import sys

target_title = sys.argv[1]
cache_path = sys.argv[2]

try:
    with open(cache_path, "r", encoding="utf-8") as handle:
        for line in handle:
            line = line.rstrip("\n")
            if not line:
                continue
            number, title = line.split("\t", 1)
            if title == target_title:
                print(number)
                raise SystemExit(0)
except FileNotFoundError:
    pass
PY
}

append_issue_cache() {
  local number="$1"
  local title="$2"
  printf '%s\t%s\n' "$number" "$title" >> "$ISSUE_CACHE"
}

ensure_label() {
  local name="$1"
  local description="$2"
  local color="$3"

  [[ "$CREATE_LABELS" == "1" ]] || return 0

  if gh label list --repo "$REPO" --limit 1000 --json name --jq '.[].name' | grep -Fxq "$name"; then
    log "Label exists: $name"
    return 0
  fi

  if [[ "$DRY_RUN" == "1" ]]; then
    log "[dry-run] Would create label: $name"
    return 0
  fi

  log "Creating label: $name"
  gh label create "$name" \
    --repo "$REPO" \
    --description "$description" \
    --color "$color" >/dev/null
}

ensure_labels() {
  ensure_label "epic" "Large roadmap-level body of work." "7057ff"
  ensure_label "work-packet" "Atomic implementation packet under an epic." "1d76db"
  ensure_label "roadmap" "Part of the Monad roadmap." "0e8a16"
  ensure_label "monad" "Monad project issue." "5319e7"
  ensure_label "automation-created" "Created by issue automation." "c5def5"
}

issue_create_args_for_labels() {
  local labels_csv="$1"
  local args=()

  [[ "$CREATE_LABELS" == "1" ]] || {
    printf '%s\0' "${args[@]}"
    return 0
  }

  IFS=',' read -r -a labels <<< "$labels_csv"
  for label in "${labels[@]}"; do
    args+=(--label "$label")
  done

  printf '%s\0' "${args[@]}"
}

write_epic_body() {
  local epic_key="$1"
  local epic_title="$2"

  cat <<EOF
## Product Area

Monad roadmap / ${epic_title}

## Objective

Create the ${epic_key} foundation for **${epic_title}** as part of the planned Monad roadmap sequence.

## User Value

This epic advances Monad from a manually coordinated polyglot repository runtime toward a governance-grade, AI-ready, locally executable, supervised software-delivery system.

## Scope

This epic includes the six work packets listed below. Each work packet should be implemented as an independently reviewable slice with documentation, tests, verification evidence, and a closeout handoff.

## Work Packets

EOF

  emit_workpackets | while IFS=$'\t' read -r wp_epic wp_id wp_title; do
    if [[ "$wp_epic" == "$epic_key" ]]; then
      printf -- '- [ ] **%s** — %s\n' "$wp_id" "$wp_title"
    fi
  done

  cat <<'EOF'

## Deliverables

- Repo-resident documentation updates where architecture, policy, or workflow behavior changes.
- Implementation code where applicable.
- Fixtures and tests where applicable.
- Verification commands and expected output.
- Evidence records sufficient to support work-packet closeout.
- Handoff notes for the next epic or work packet.

## Verification

At minimum, complete the applicable verification path for the affected area:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `tools/scripts/verify.sh` if present and applicable

## Expected Results After Verification

- The repository remains formatted and testable.
- The implemented slice does not regress previously completed epics.
- New behavior is documented and covered by targeted tests or smoke tests.
- Generated reports, evidence, or handoff artifacts are deterministic where applicable.

## Closeout Checklist

- [ ] All child work packets are completed or explicitly deferred.
- [ ] Verification evidence is attached or linked.
- [ ] Documentation has been updated.
- [ ] Follow-on risks or deferred items are captured.
- [ ] Epic closeout note is recorded.

## Priority

Planned roadmap

## Size

Epic
EOF
}

write_workpacket_body() {
  local epic_key="$1"
  local epic_title="$2"
  local wp_id="$3"
  local wp_title="$4"
  local parent_number="${5:-}"

  cat <<EOF
## Product Area

${epic_title}

## Objective

Complete **${wp_id} — ${wp_title}** as an atomic, reviewable work packet under **${epic_key} — ${epic_title}**.

EOF

  if [[ -n "$parent_number" ]]; then
    cat <<EOF
## Parent Epic

#${parent_number}

EOF
  else
    cat <<EOF
## Parent Epic

${epic_key} — ${epic_title}

EOF
  fi

  cat <<'EOF'
## Scope

This work packet should deliver the smallest coherent slice that satisfies the title and supports the parent epic without introducing avoidable architectural drift.

## Deliverables

- Design notes or documentation updates where the behavior changes architecture, workflow, policy, schema, or user-facing expectations.
- Implementation code where applicable.
- Tests, fixtures, or smoke coverage appropriate to the slice.
- Verification evidence showing the expected result.
- Handoff or closeout notes identifying what changed and what remains.

## Suggested Implementation Standard

- Preserve Monad's local-first, single-binary-first, native-tool-coordinating architecture.
- Prefer deterministic output.
- Keep domain logic in `monad-core` unless the behavior is CLI-specific.
- Keep CLI behavior thin and orchestration-oriented.
- Avoid introducing service dependencies unless the work packet explicitly calls for them.
- Do not add Bazel, Pants, Buck2, or Nx as required dependencies.
- Treat documentation, verification, and governance evidence as first-class deliverables.

## Verification

Run the narrowest useful verification first, then the full workspace checks when the slice is ready:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `tools/scripts/verify.sh` if present and applicable

## Expected Results After Verification

- Formatting passes.
- Clippy passes without warnings.
- Workspace tests pass.
- The new slice has targeted evidence that demonstrates the expected behavior.
- Existing Monad workflow assumptions remain intact.

## Closeout Checklist

- [ ] Implementation complete.
- [ ] Documentation updated.
- [ ] Tests or smoke checks added/updated.
- [ ] Verification commands run.
- [ ] Evidence or report output captured where applicable.
- [ ] Follow-up work noted.
- [ ] Work packet can be closed without blocking the next packet.

## Priority

Planned roadmap

## Size

M
EOF
}

create_or_reuse_issue() {
  local kind="$1"
  local title="$2"
  local labels_csv="$3"
  local body_file="$4"

  local existing_number
  existing_number="$(issue_number_by_exact_title "$title" || true)"

  if [[ -n "$existing_number" ]]; then
    log "Reusing existing $kind issue #$existing_number: $title"
    printf '%s\n' "$existing_number"
    return 0
  fi

  if [[ "$DRY_RUN" == "1" ]]; then
    log "[dry-run] Would create $kind issue: $title"
    return 0
  fi

  local label_args=()
  if [[ "$CREATE_LABELS" == "1" ]]; then
    IFS=',' read -r -a labels <<< "$labels_csv"
    for label in "${labels[@]}"; do
      label_args+=(--label "$label")
    done
  fi

  local optional_args=()
  if [[ -n "$MILESTONE" ]]; then
    optional_args+=(--milestone "$MILESTONE")
  fi
  if [[ -n "$PROJECT" ]]; then
    optional_args+=(--project "$PROJECT")
  fi

  log "Creating $kind issue: $title"

  local url
  url="$(
    gh issue create \
      --repo "$REPO" \
      --title "$title" \
      --body-file "$body_file" \
      "${label_args[@]}" \
      "${optional_args[@]}"
  )"

  local number="${url##*/}"
  [[ "$number" =~ ^[0-9]+$ ]] || die "Could not parse issue number from gh output: $url"

  append_issue_cache "$number" "$title"
  log "Created $kind issue #$number: $title"
  printf '%s\n' "$number"
}

issue_node_id() {
  local number="$1"

  gh issue view "$number" \
    --repo "$REPO" \
    --json id \
    --jq '.id'
}

is_subissue_already_linked() {
  local parent_number="$1"
  local child_number="$2"

  local subissue_numbers
  if ! subissue_numbers="$(
    gh api \
      -H "X-GitHub-Api-Version: 2022-11-28" \
      "/repos/${REPO}/issues/${parent_number}/sub_issues" \
      --jq '.[].number' 2>/dev/null
  )"; then
    return 1
  fi

  grep -Fxq "$child_number" <<< "$subissue_numbers"
}

link_subissue() {
  local parent_number="$1"
  local child_number="$2"
  local child_title="$3"

  [[ "$LINK_SUBISSUES" == "1" ]] || {
    ((SKIPPED_LINKS += 1))
    return 0
  }

  if [[ -z "$parent_number" || -z "$child_number" ]]; then
    log "[dry-run] Would link subissue: parent=${parent_number:-unknown} child=${child_title}"
    ((SKIPPED_LINKS += 1))
    return 0
  fi

  if is_subissue_already_linked "$parent_number" "$child_number"; then
    log "Subissue already linked: #$child_number under #$parent_number"
    ((SKIPPED_LINKS += 1))
    return 0
  fi

  if [[ "$DRY_RUN" == "1" ]]; then
    log "[dry-run] Would link #$child_number as subissue under #$parent_number"
    ((SKIPPED_LINKS += 1))
    return 0
  fi

  local parent_id
  local child_id
  parent_id="$(issue_node_id "$parent_number")"
  child_id="$(issue_node_id "$child_number")"

  log "Linking #$child_number as subissue under #$parent_number"

  if gh api graphql \
    -H "GraphQL-Features: sub_issues" \
    -f parentIssueId="$parent_id" \
    -f childIssueId="$child_id" \
    -f query='
mutation($parentIssueId: ID!, $childIssueId: ID!) {
  addSubIssue(input: { issueId: $parentIssueId, subIssueId: $childIssueId }) {
    issue {
      number
      title
    }
    subIssue {
      number
      title
    }
  }
}
' >/dev/null; then
    ((LINKED_SUBISSUES += 1))
  else
    warn "Failed to link #$child_number as a subissue of #$parent_number. The issue itself still exists."
    warn "This usually means GitHub subissues are unavailable, disabled, or the token lacks access to the preview."
    ((FAILED_LINKS += 1))
  fi
}

main() {
  parse_args "$@"
  require_tools
  setup_temp
  check_github_access

  log "Repository: $REPO"
  if [[ "$DRY_RUN" == "1" ]]; then
    log "Mode: dry-run"
  else
    log "Mode: apply"
  fi

  refresh_issue_cache
  ensure_labels

  declare -A EPIC_NUMBERS

  log ""
  log "Creating/reusing epics E20 through E45..."

  while IFS=$'\t' read -r epic_key epic_title; do
    [[ -n "$epic_key" ]] || continue

    epic_issue_title="${epic_key} — ${epic_title}"
    body_file="$TMP_DIR/${epic_key}.md"
    write_epic_body "$epic_key" "$epic_title" > "$body_file"

    existing_number="$(issue_number_by_exact_title "$epic_issue_title" || true)"
    if [[ -n "$existing_number" ]]; then
      ((REUSED_EPICS += 1))
    else
      if [[ "$DRY_RUN" == "0" ]]; then
        ((CREATED_EPICS += 1))
      fi
    fi

    epic_number="$(create_or_reuse_issue "epic" "$epic_issue_title" "epic,roadmap,monad,automation-created" "$body_file")"
    EPIC_NUMBERS["$epic_key"]="$epic_number"
  done < <(emit_epics)

  log ""
  log "Creating/reusing work packet issues and linking subissues..."

  while IFS=$'\t' read -r epic_key wp_id wp_title; do
    [[ -n "$epic_key" ]] || continue

    epic_title="$(epic_title_for_key "$epic_key")"
    parent_number="${EPIC_NUMBERS[$epic_key]:-}"
    wp_issue_title="${wp_id} — ${wp_title}"
    body_file="$TMP_DIR/${wp_id}.md"
    write_workpacket_body "$epic_key" "$epic_title" "$wp_id" "$wp_title" "$parent_number" > "$body_file"

    existing_number="$(issue_number_by_exact_title "$wp_issue_title" || true)"
    if [[ -n "$existing_number" ]]; then
      ((REUSED_WORKPACKETS += 1))
    else
      if [[ "$DRY_RUN" == "0" ]]; then
        ((CREATED_WORKPACKETS += 1))
      fi
    fi

    wp_number="$(create_or_reuse_issue "work-packet" "$wp_issue_title" "work-packet,roadmap,monad,automation-created" "$body_file")"
    link_subissue "$parent_number" "$wp_number" "$wp_issue_title"
  done < <(emit_workpackets)

  log ""
  log "Done."
  log "Summary:"
  log "  Repository: $REPO"
  log "  Mode: $([[ "$DRY_RUN" == "1" ]] && printf 'dry-run' || printf 'apply')"
  log "  Created epics: $CREATED_EPICS"
  log "  Reused epics: $REUSED_EPICS"
  log "  Created work packets: $CREATED_WORKPACKETS"
  log "  Reused work packets: $REUSED_WORKPACKETS"
  log "  Linked subissues: $LINKED_SUBISSUES"
  log "  Skipped links: $SKIPPED_LINKS"
  log "  Failed links: $FAILED_LINKS"

  if [[ "$DRY_RUN" == "1" ]]; then
    log ""
    log "Dry run complete. To create the issues, run:"
    log "  bash $0 --apply"
  fi
}

main "$@"
