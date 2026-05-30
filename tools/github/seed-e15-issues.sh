#!/usr/bin/env bash
set -euo pipefail

REPO="${REPO:-thomascarter613/monad-workspace}"

require_command() {
  local command_name="$1"

  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "Missing required command: $command_name" >&2
    exit 1
  fi
}

issue_number_by_exact_title() {
  local title="$1"

  gh issue list \
    --repo "$REPO" \
    --state all \
    --search "\"$title\" in:title" \
    --limit 300 \
    | awk -v wanted="$title" -F '\t' '
        {
          for (i = 1; i <= NF; i++) {
            if ($i == wanted) {
              number = $1
              gsub(/^#/, "", number)
              print number
              exit
            }
          }
        }
      '
}

create_issue_once() {
  local title="$1"
  local body_file="$2"

  local existing_number
  existing_number="$(issue_number_by_exact_title "$title")"

  if [[ -n "$existing_number" ]]; then
    echo "$existing_number"
    return 0
  fi

  local issue_url
  issue_url="$(
    gh issue create \
      --repo "$REPO" \
      --title "$title" \
      --body-file "$body_file"
  )"

  echo "$issue_url" | sed -E 's#.*/issues/([0-9]+).*#\1#'
}

make_body_file() {
  local file
  file="$(mktemp)"
  cat > "$file"
  echo "$file"
}

create_epic() {
  local epic_id="$1"
  local epic_title="$2"
  local product_area="$3"
  local objective="$4"
  local user_value="$5"
  local scope_in="$6"
  local scope_out="$7"
  local work_packets="$8"
  local definition_of_done="$9"

  local title="[Epic]: ${epic_id} — ${epic_title}"

  local body_file
  body_file="$(make_body_file <<EOF
## Epic ID

${epic_id}

## Epic Title

${epic_title}

## Product Area

${product_area}

## Objective

${objective}

## User Value

${user_value}

## Scope

### In scope

${scope_in}

### Out of scope

${scope_out}

## Work Packets

${work_packets}

## Definition of Done

${definition_of_done}

## Verification Strategy

\`\`\`bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
\`\`\`

## Priority

High

## Size

Large
EOF
)"

  local issue_number
  issue_number="$(create_issue_once "$title" "$body_file")"
  rm -f "$body_file"

  echo "$issue_number"
}

create_work_packet() {
  local parent_epic_id="$1"
  local parent_issue_number="$2"
  local work_packet_id="$3"
  local work_packet_title="$4"
  local product_area="$5"
  local objective="$6"
  local scope_in="$7"
  local scope_out="$8"
  local expected_files="$9"
  local expected_result="${10}"
  local commit_message="${11}"
  local size="${12:-Medium}"

  local title="[Work Packet]: ${work_packet_id} — ${work_packet_title}"

  local body_file
  body_file="$(make_body_file <<EOF
## Work Packet ID

${work_packet_id}

## Parent Epic ID

${parent_epic_id}

## Parent Epic Issue

#${parent_issue_number}

## Work Packet Title

${work_packet_title}

## Product Area

${product_area}

## Objective

${objective}

## Scope

### In scope

${scope_in}

### Out of scope

${scope_out}

## Expected Files or Directories Affected

${expected_files}

## Tasks

- [ ] Confirm current repository state.
- [ ] Implement the scoped work.
- [ ] Add or update tests.
- [ ] Update documentation if behavior changes.
- [ ] Verify formatting.
- [ ] Verify tests.
- [ ] Verify clippy.
- [ ] Verify root verification.
- [ ] Commit as one atomic Conventional Commit.
- [ ] Close this work packet with verification evidence.

## Verification Commands / Evidence

\`\`\`bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
\`\`\`

## Expected Result

${expected_result}

## Commit Message

\`\`\`bash
git commit -m "${commit_message}"
\`\`\`

## Priority

High

## Size

${size}
EOF
)"

  local issue_number
  issue_number="$(create_issue_once "$title" "$body_file")"
  rm -f "$body_file"

  echo "$issue_number"
}

add_epic_child_index_comment() {
  local epic_issue_number="$1"
  local comment_body="$2"

  gh issue comment "$epic_issue_number" \
    --repo "$REPO" \
    --body "$comment_body"
}

require_command gh

if ! gh auth status >/dev/null 2>&1; then
  echo "GitHub CLI is not authenticated. Run: gh auth login" >&2
  exit 1
fi

echo "Seeding E15 issues in repo: $REPO"
echo

E15_SCOPE_IN="$(cat <<'EOF'
- Define `monad doctor` diagnostic contract.
- Detect required and optional local tools.
- Diagnose Rust/Cargo readiness.
- Diagnose Git/repository readiness.
- Diagnose supported ecosystem tool availability.
- Diagnose Monad repo contract/context/readiness status.
- Produce text and JSON doctor reports.
- Add smoke tests and fixture tests.
EOF
)"

E15_SCOPE_OUT="$(cat <<'EOF'
- Installing missing tools automatically.
- Mutating system configuration.
- Editing user shell profiles.
- Running package-manager install commands.
- Remote/cloud diagnostics.
- Telemetry upload.
- Full enterprise environment management.
EOF
)"

E15_WORK_PACKETS="$(cat <<'EOF'
- WP-E15-001 — Define `monad doctor` diagnostic contract
- WP-E15-002 — Add local tool detection foundation
- WP-E15-003 — Add Rust, Git, and repository readiness diagnostics
- WP-E15-004 — Add ecosystem diagnostics for Node/Bun/Python/Go/Java
- WP-E15-005 — Add Monad context and repo contract diagnostics
- WP-E15-006 — Add doctor report output and smoke tests
EOF
)"

E15_DOD="$(cat <<'EOF'
- `monad doctor` has a documented diagnostic contract.
- Monad can detect key local tools without modifying the environment.
- Monad reports Rust/Cargo and Git readiness clearly.
- Monad reports supported ecosystem tool availability clearly.
- Monad reports context/repo-contract readiness clearly.
- Doctor output is available in human-readable form and JSON where feasible.
- Root verification passes.
EOF
)"

E15_USER_VALUE="$(cat <<'EOF'
Users need one clear command that explains whether their machine and repository are ready for Monad. Instead of failing mysteriously because Rust, Git, Bun, Python, Go, Java, context files, or manifests are missing or stale, Monad should produce an actionable diagnostic report.

This lowers friction for first-time users and gives the maintainer a practical support tool before the public MVP.
EOF
)"

E15_NUMBER="$(create_epic \
  "E15" \
  "Doctor and Environment Diagnostics Foundation" \
  "Diagnostics / Environment Readiness / Developer Experience" \
  "Implement the first MVP-safe `monad doctor` foundation so Monad can diagnose local environment readiness, repository health, supported native tools, and Monad context/contract status without mutating the system." \
  "$E15_USER_VALUE" \
  "$E15_SCOPE_IN" \
  "$E15_SCOPE_OUT" \
  "$E15_WORK_PACKETS" \
  "$E15_DOD"
)"

echo "E15 issue: #$E15_NUMBER"

WP_E15_001="$(create_work_packet \
  "E15" \
  "$E15_NUMBER" \
  "WP-E15-001" \
  "Define \`monad doctor\` diagnostic contract" \
  "Diagnostics / Product Design" \
  "Define the diagnostic scope, severity model, output model, and non-mutating safety contract for `monad doctor`." \
  "- Define diagnostic categories.
- Define severity levels such as pass, warn, fail, info, skipped.
- Define what doctor may inspect.
- Define what doctor must not mutate.
- Document examples." \
  "- Implementing detection.
- Installing missing tools.
- Mutating user environment.
- Telemetry upload." \
  "- \`docs/commands/DOCTOR.md\`
- \`docs/architecture/DIAGNOSTICS-MODEL.md\`
- ADR if needed" \
  "The doctor command has a clear non-mutating diagnostic contract before implementation begins." \
  "docs(doctor): define diagnostics contract" \
  "Medium"
)"

WP_E15_002="$(create_work_packet \
  "E15" \
  "$E15_NUMBER" \
  "WP-E15-002" \
  "Add local tool detection foundation" \
  "Core Runtime / Diagnostics" \
  "Add a reusable local tool detection foundation for checking whether expected commands exist and reporting their versions when available." \
  "- Add command-exists detection.
- Add version probing abstraction.
- Add timeout/bounded execution behavior.
- Add deterministic diagnostic results.
- Add fixture/unit tests." \
  "- Installing tools.
- Running package-manager install commands.
- Mutating PATH.
- Editing shell profiles." \
  "- \`crates/monad-core/src/doctor.rs\`
- \`crates/monad-core/src/doctor/\`
- \`crates/monad-core/src/exec/\` if reused
- tests" \
  "Monad can safely detect local tool availability and versions without mutating the environment." \
  "feat(doctor): add local tool detection foundation" \
  "Medium"
)"

WP_E15_003="$(create_work_packet \
  "E15" \
  "$E15_NUMBER" \
  "WP-E15-003" \
  "Add Rust, Git, and repository readiness diagnostics" \
  "Diagnostics / Core Tooling" \
  "Add diagnostics for the core tools and repo conditions Monad needs in order to operate reliably." \
  "- Diagnose Git availability.
- Diagnose whether current directory is a Git repository.
- Diagnose clean/dirty working tree state where feasible.
- Diagnose Rust/Cargo availability.
- Diagnose `monad.toml` presence and parseability.
- Add tests." \
  "- Enforcing all checks as hard failures.
- Modifying Git state.
- Installing Rust or Git.
- Running long builds." \
  "- \`crates/monad-core/src/doctor/\`
- \`crates/monad-cli/src/main.rs\`
- tests" \
  "`monad doctor` can report core Git, Rust, Cargo, and Monad manifest readiness." \
  "feat(doctor): add core readiness diagnostics" \
  "Large"
)"

WP_E15_004="$(create_work_packet \
  "E15" \
  "$E15_NUMBER" \
  "WP-E15-004" \
  "Add ecosystem diagnostics for Node/Bun/Python/Go/Java" \
  "Diagnostics / Polyglot Toolchains" \
  "Add optional ecosystem diagnostics for the major polyglot tools Monad expects to coordinate." \
  "- Detect Node.
- Detect Bun.
- Detect npm/pnpm/yarn where feasible.
- Detect Python.
- Detect Go.
- Detect Java.
- Report missing optional tools as warnings unless required by repo state.
- Add tests with mocked/probed command behavior." \
  "- Installing tools.
- Managing tool versions.
- Running package installs.
- Full language-server diagnostics." \
  "- \`crates/monad-core/src/doctor/\`
- \`crates/monad-core/src/adapters/\` if reused
- tests" \
  "`monad doctor` can report supported ecosystem tool availability in a bounded, non-mutating way." \
  "feat(doctor): add polyglot tool diagnostics" \
  "Large"
)"

WP_E15_005="$(create_work_packet \
  "E15" \
  "$E15_NUMBER" \
  "WP-E15-005" \
  "Add Monad context and repo contract diagnostics" \
  "Diagnostics / Context / Repository Contract" \
  "Add diagnostics for Monad-specific context files, generated artifacts, and repository contract expectations." \
  "- Check `.monad/context/` presence.
- Check generated context artifact freshness where feasible.
- Check `work/epics/` active epic status consistency.
- Check generated artifact ignore policy markers where feasible.
- Check repo contract/sync status if E14 outputs exist.
- Add tests." \
  "- Rewriting context automatically.
- Editing epic status automatically.
- Performing sync writes.
- Full semantic freshness analysis." \
  "- \`crates/monad-core/src/doctor/\`
- \`crates/monad-core/src/context/\`
- \`crates/monad-core/src/sync/\` if reused
- tests" \
  "`monad doctor` can identify common Monad-specific repository readiness problems before they break context or release workflows." \
  "feat(doctor): add monad context diagnostics" \
  "Medium"
)"

WP_E15_006="$(create_work_packet \
  "E15" \
  "$E15_NUMBER" \
  "WP-E15-006" \
  "Add doctor report output and smoke tests" \
  "CLI / Verification / Evidence" \
  "Add user-facing `monad doctor` output, optional JSON output, and smoke tests proving the command works." \
  "- Add CLI parse path for `monad doctor`.
- Add text report renderer.
- Add JSON report renderer if feasible.
- Add CLI smoke tests.
- Document examples.
- Include clear remediation hints." \
  "- Hosted reports.
- Telemetry.
- Automatic repair.
- Interactive prompts." \
  "- \`crates/monad-cli/src/main.rs\`
- \`crates/monad-core/src/doctor/\`
- CLI smoke tests
- \`docs/commands/DOCTOR.md\`" \
  "`monad doctor` produces actionable reports and is covered by smoke tests." \
  "test(doctor): add doctor report smoke tests" \
  "Medium"
)"

E15_COMMENT="$(cat <<EOF
## Child Work Packets

- #${WP_E15_001} — WP-E15-001 — Define \`monad doctor\` diagnostic contract
- #${WP_E15_002} — WP-E15-002 — Add local tool detection foundation
- #${WP_E15_003} — WP-E15-003 — Add Rust, Git, and repository readiness diagnostics
- #${WP_E15_004} — WP-E15-004 — Add ecosystem diagnostics for Node/Bun/Python/Go/Java
- #${WP_E15_005} — WP-E15-005 — Add Monad context and repo contract diagnostics
- #${WP_E15_006} — WP-E15-006 — Add doctor report output and smoke tests
EOF
)"

add_epic_child_index_comment "$E15_NUMBER" "$E15_COMMENT"

echo
echo "Created/confirmed E15 roadmap issues:"
echo "E15 #$E15_NUMBER"
echo "  WP-E15-001 #$WP_E15_001"
echo "  WP-E15-002 #$WP_E15_002"
echo "  WP-E15-003 #$WP_E15_003"
echo "  WP-E15-004 #$WP_E15_004"
echo "  WP-E15-005 #$WP_E15_005"
echo "  WP-E15-006 #$WP_E15_006"
