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
    --limit 200 \
    | awk -v wanted="$title" -F '\t' '
        $2 == wanted {
          gsub(/^#/, "", $1)
          print $1
          exit
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
  local scope_in="$5"
  local scope_out="$6"
  local work_packets="$7"
  local definition_of_done="$8"

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
- [ ] Verify formatting.
- [ ] Verify tests.
- [ ] Verify clippy.
- [ ] Verify root verification.
- [ ] Commit as one atomic stabilization commit.
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

Medium
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

echo "Seeding issues in repo: $REPO"
echo

E10_SCOPE_IN="$(cat <<'EOF'
- Audit README and public claims against implemented capability.
- Convert the public pre-release checklist into pass/fail evidence.
- Decide source-only versus packaged pre-release posture.
- Draft public pre-release notes.
- Run final public pre-release verification audit.
- Decide whether to cut the first public pre-release tag.
EOF
)"

E10_SCOPE_OUT="$(cat <<'EOF'
- Hosted launch.
- SaaS launch.
- Installer generation unless explicitly approved.
- Crates.io publishing unless explicitly approved.
- Autonomous agent execution.
- Apply/write evolution behavior.
- Plugin marketplace.
- Enterprise feature launch.
EOF
)"

E10_WORK_PACKETS="$(cat <<'EOF'
- WP-E10-001 — Audit README and public claims against implemented capability
- WP-E10-002 — Convert public pre-release checklist into pass/fail evidence
- WP-E10-003 — Decide source-only versus packaged pre-release posture
- WP-E10-004 — Draft public pre-release notes
- WP-E10-005 — Run final public pre-release verification audit
- WP-E10-006 — Decide and cut first public pre-release tag, if approved
EOF
)"

E10_DOD="$(cat <<'EOF'
- Public-facing claims match implemented behavior.
- Public pre-release checklist has pass/fail evidence.
- Source-only/package/installer posture is explicit.
- Public pre-release notes exist.
- Final public pre-release verification audit passes or records blockers.
- The first public pre-release is either approved and tagged or explicitly deferred.
EOF
)"

E10_NUMBER="$(create_epic \
  "E10" \
  "Public Pre-Release Hardening and Boundary Enforcement" \
  "Public Pre-Release / Release Governance" \
  "Harden Monad for a responsible first public pre-release by enforcing the public pre-release boundary, auditing public claims, producing release evidence, and deciding whether a verified public pre-release tag should be cut." \
  "$E10_SCOPE_IN" \
  "$E10_SCOPE_OUT" \
  "$E10_WORK_PACKETS" \
  "$E10_DOD"
)"

echo "E10 issue: #$E10_NUMBER"

WP_E10_001="$(create_work_packet \
  "E10" \
  "$E10_NUMBER" \
  "WP-E10-001" \
  "Audit README and public claims against implemented capability" \
  "Documentation / Public Readiness" \
  "Audit the README and public-facing repository claims so Monad does not advertise capabilities that are not implemented in the current binary." \
  "- Review README claims against implemented CLI commands.
- Identify overclaims, underclaims, and ambiguous claims.
- Update README or create an audit document if needed.
- Preserve an honest distinction between current capability and future roadmap." \
  "- Implementing missing capabilities.
- Publishing a public release.
- Expanding MVP scope." \
  "- \`README.md\`
- \`docs/release/README-PUBLIC-CLAIMS-AUDIT.md\` if needed
- \`docs/release/PUBLIC-PRERELEASE-CHECKLIST.md\` if needed" \
  "README/public claims are accurate and do not imply unimplemented init, package publishing, autonomous agents, apply/write evolution, hosted service, installer, or plugin marketplace capabilities." \
  "docs(readme): audit public capability claims"
)"

WP_E10_002="$(create_work_packet \
  "E10" \
  "$E10_NUMBER" \
  "WP-E10-002" \
  "Convert public pre-release checklist into pass/fail evidence" \
  "Release Governance / Verification" \
  "Turn the public pre-release checklist into a concrete evidence record with pass/fail/deferred status for each gate." \
  "- Review the public pre-release checklist.
- Add evidence status for each gate.
- Identify unresolved blockers.
- Mark deferrals explicitly." \
  "- Cutting the release.
- Implementing deferred capabilities.
- Hiding unresolved blockers." \
  "- \`docs/release/PUBLIC-PRERELEASE-CHECKLIST.md\`
- \`docs/release/PUBLIC-PRERELEASE-EVIDENCE.md\`" \
  "Every public pre-release gate has visible evidence, blocker, or deferral status." \
  "docs(release): add public prerelease evidence"
)"

WP_E10_003="$(create_work_packet \
  "E10" \
  "$E10_NUMBER" \
  "WP-E10-003" \
  "Decide source-only versus packaged pre-release posture" \
  "Release Strategy" \
  "Decide whether the first public pre-release is source-only, binary-artifact-based, package-published, or explicitly deferred." \
  "- Compare source-only, binary artifact, installer, and package publication options.
- Decide what the first public pre-release will and will not provide.
- Record the decision in release docs." \
  "- Publishing packages.
- Generating installers.
- Cutting a tag before verification passes." \
  "- \`docs/release/PUBLIC-PRERELEASE-DISTRIBUTION-DECISION.md\`
- \`docs/release/PUBLIC-PRERELEASE-CHECKLIST.md\` if needed" \
  "The first public pre-release distribution posture is explicit and bounded." \
  "docs(release): decide prerelease distribution posture"
)"

WP_E10_004="$(create_work_packet \
  "E10" \
  "$E10_NUMBER" \
  "WP-E10-004" \
  "Draft public pre-release notes" \
  "Release Documentation" \
  "Draft honest public pre-release notes that describe implemented capabilities, known limitations, deferred capabilities, and verification evidence." \
  "- Create pre-release notes.
- Distinguish implemented, limited, deferred, and not implemented features.
- Include verification commands and evidence expectations." \
  "- Publishing the release.
- Claiming production readiness.
- Claiming autonomous execution or write/apply evolution." \
  "- \`docs/release/PUBLIC-PRERELEASE-NOTES.md\`
- \`CHANGELOG.md\` if needed" \
  "Public pre-release notes are accurate, bounded, and ready for final verification review." \
  "docs(release): draft public prerelease notes"
)"

WP_E10_005="$(create_work_packet \
  "E10" \
  "$E10_NUMBER" \
  "WP-E10-005" \
  "Run final public pre-release verification audit" \
  "Release Verification" \
  "Run and record final verification evidence for the public pre-release candidate." \
  "- Run full local verification.
- Check generated context.
- Check binary build path.
- Record pass/fail evidence.
- Identify blockers honestly." \
  "- Cutting the release tag.
- Publishing packages.
- Ignoring failed checks." \
  "- \`docs/release/PUBLIC-PRERELEASE-VERIFICATION-AUDIT.md\`
- \`.monad/context/\` if regenerated" \
  "Final public pre-release verification evidence exists and records whether the candidate is releasable." \
  "docs(release): add public prerelease verification audit"
)"

WP_E10_006="$(create_work_packet \
  "E10" \
  "$E10_NUMBER" \
  "WP-E10-006" \
  "Decide and cut first public pre-release tag, if approved" \
  "Release Decision" \
  "Make the final go/no-go decision for the first public pre-release and cut the tag only if approved by evidence." \
  "- Review all E10 evidence.
- Decide go/no-go.
- If approved, cut the first public pre-release tag.
- If not approved, document blockers and defer." \
  "- Publishing packages unless separately approved.
- Hosted launch.
- Installer generation unless separately approved." \
  "- \`docs/release/PUBLIC-PRERELEASE-GO-NOGO.md\`
- Git tag if approved" \
  "The first public pre-release is either tagged from a verified commit or explicitly deferred with blockers recorded." \
  "docs(release): decide public prerelease go no-go"
)"

E10_COMMENT="$(cat <<EOF
## Child Work Packets

- #${WP_E10_001} — WP-E10-001 — Audit README and public claims against implemented capability
- #${WP_E10_002} — WP-E10-002 — Convert public pre-release checklist into pass/fail evidence
- #${WP_E10_003} — WP-E10-003 — Decide source-only versus packaged pre-release posture
- #${WP_E10_004} — WP-E10-004 — Draft public pre-release notes
- #${WP_E10_005} — WP-E10-005 — Run final public pre-release verification audit
- #${WP_E10_006} — WP-E10-006 — Decide and cut first public pre-release tag, if approved
EOF
)"

add_epic_child_index_comment "$E10_NUMBER" "$E10_COMMENT"

E11_SCOPE_IN="$(cat <<'EOF'
- Define `monad init` UX and safety contract.
- Add init dry-run planning.
- Add minimal embedded scaffold templates.
- Add guarded init write path.
- Add basic and/or polyglot-minimal preset.
- Add init smoke tests and verification evidence.
EOF
)"

E11_SCOPE_OUT="$(cat <<'EOF'
- Full enterprise-grade monorepo generation.
- Arbitrary app/service generators.
- Destructive overwrites.
- Apply/write evolution beyond approved init files.
- Package publishing.
- Hosted/SaaS functionality.
EOF
)"

E11_WORK_PACKETS="$(cat <<'EOF'
- WP-E11-001 — Define `monad init` UX and safety contract
- WP-E11-002 — Add init dry-run plan
- WP-E11-003 — Add minimal embedded scaffold templates
- WP-E11-004 — Add guarded init write path
- WP-E11-005 — Add basic/polyglot-minimal preset
- WP-E11-006 — Add init smoke tests and verification evidence
EOF
)"

E11_DOD="$(cat <<'EOF'
- `monad init --dry-run` previews the exact files that would be created.
- `monad init` can create a minimal Monad-managed repository skeleton.
- Init refuses unsafe overwrites.
- Init output is deterministic and test-covered.
- Generated skeleton includes `monad.toml`, `README.md`, `docs/`, `work/`, `.monad/context/`, verification baseline, and optional CI baseline.
EOF
)"

E11_NUMBER="$(create_epic \
  "E11" \
  "Init Command and Monorepo Scaffold Foundation" \
  "Init / Scaffolding / Monorepo Foundation" \
  "Implement the first safe, minimal `monad init` capability so Monad can initialize a new Monad-managed monorepo skeleton without overwriting existing work." \
  "$E11_SCOPE_IN" \
  "$E11_SCOPE_OUT" \
  "$E11_WORK_PACKETS" \
  "$E11_DOD"
)"

echo "E11 issue: #$E11_NUMBER"

WP_E11_001="$(create_work_packet \
  "E11" \
  "$E11_NUMBER" \
  "WP-E11-001" \
  "Define \`monad init\` UX and safety contract" \
  "Init / Product Design" \
  "Define the command-line UX, safety rules, generated file set, and initial presets for `monad init`." \
  "- Define `monad init` command shape.
- Define flags such as `--dry-run` and `--preset`.
- Define no-overwrite safety behavior.
- Document generated file baseline." \
  "- Implementing file writes.
- Adding full template engine.
- Generating complex app/service structures." \
  "- \`docs/commands/INIT.md\` or equivalent
- \`docs/product/INIT-SAFETY-CONTRACT.md\` if needed" \
  "The init command contract is documented before implementation." \
  "docs(init): define init safety contract"
)"

WP_E11_002="$(create_work_packet \
  "E11" \
  "$E11_NUMBER" \
  "WP-E11-002" \
  "Add init dry-run plan" \
  "Init / CLI" \
  "Implement `monad init --dry-run` so users can preview generated files before anything is written." \
  "- Add CLI parse path for `init`.
- Add dry-run plan model.
- Render planned file operations.
- Ensure no files are written in dry-run mode." \
  "- Writing files.
- Overwrite behavior.
- Complex presets." \
  "- \`crates/monad-cli/src/main.rs\`
- \`crates/monad-core/src/init.rs\`
- tests" \
  "`monad init --dry-run` renders a deterministic plan and writes nothing." \
  "feat(init): add dry-run init plan"
)"

WP_E11_003="$(create_work_packet \
  "E11" \
  "$E11_NUMBER" \
  "WP-E11-003" \
  "Add minimal embedded scaffold templates" \
  "Init / Templates" \
  "Add the minimal embedded templates required for a Monad-managed repository skeleton." \
  "- Add templates for `monad.toml`, README, docs stub, work directories, context directories, verification script, and optional CI file.
- Keep templates deterministic and small." \
  "- Full app/service generators.
- External template registry.
- Plugin templates." \
  "- \`crates/monad-core/src/templates/\`
- \`crates/monad-core/src/init/\`
- tests" \
  "Monad has embedded minimal templates suitable for init dry-run and later guarded writes." \
  "feat(init): add minimal scaffold templates"
)"

WP_E11_004="$(create_work_packet \
  "E11" \
  "$E11_NUMBER" \
  "WP-E11-004" \
  "Add guarded init write path" \
  "Init / File Operations" \
  "Implement guarded `monad init` file creation with strict no-overwrite behavior." \
  "- Write only planned init files.
- Refuse to overwrite existing files.
- Report conflicts clearly.
- Keep behavior deterministic." \
  "- Destructive overwrites.
- Apply/write evolution outside init.
- Git branch automation." \
  "- \`crates/monad-core/src/init/\`
- \`crates/monad-cli/src/main.rs\`
- tests" \
  "`monad init` creates a minimal skeleton in an empty repo and refuses unsafe overwrites." \
  "feat(init): add guarded init write path"
)"

WP_E11_005="$(create_work_packet \
  "E11" \
  "$E11_NUMBER" \
  "WP-E11-005" \
  "Add basic/polyglot-minimal preset" \
  "Init / Presets" \
  "Add the first bounded init preset for a minimal Monad-managed monorepo." \
  "- Define `basic` and/or `polyglot-minimal`.
- Keep preset output small and reviewable.
- Include only first MVP-safe files." \
  "- Full enterprise monorepo scaffold.
- Language-specific generators beyond skeleton directories.
- Package-manager orchestration." \
  "- \`crates/monad-core/src/init/\`
- docs
- tests" \
  "A first MVP-safe init preset exists and is documented." \
  "feat(init): add minimal init preset"
)"

WP_E11_006="$(create_work_packet \
  "E11" \
  "$E11_NUMBER" \
  "WP-E11-006" \
  "Add init smoke tests and verification evidence" \
  "Init / Verification" \
  "Add tests and verification evidence proving init dry-run and guarded init behavior." \
  "- Add CLI smoke tests for init help, dry-run, and guarded write behavior.
- Add no-overwrite tests.
- Add verification evidence docs if needed." \
  "- Public release tagging.
- Complex preset expansion." \
  "- CLI smoke tests
- core tests
- \`docs/release/\` evidence if needed" \
  "Init behavior is covered by tests and root verification passes." \
  "test(init): add init smoke tests"
)"

E11_COMMENT="$(cat <<EOF
## Child Work Packets

- #${WP_E11_001} — WP-E11-001 — Define \`monad init\` UX and safety contract
- #${WP_E11_002} — WP-E11-002 — Add init dry-run plan
- #${WP_E11_003} — WP-E11-003 — Add minimal embedded scaffold templates
- #${WP_E11_004} — WP-E11-004 — Add guarded init write path
- #${WP_E11_005} — WP-E11-005 — Add basic/polyglot-minimal preset
- #${WP_E11_006} — WP-E11-006 — Add init smoke tests and verification evidence
EOF
)"

add_epic_child_index_comment "$E11_NUMBER" "$E11_COMMENT"

echo
echo "Created/confirmed roadmap issues:"
echo "E10 #$E10_NUMBER"
echo "  WP-E10-001 #$WP_E10_001"
echo "  WP-E10-002 #$WP_E10_002"
echo "  WP-E10-003 #$WP_E10_003"
echo "  WP-E10-004 #$WP_E10_004"
echo "  WP-E10-005 #$WP_E10_005"
echo "  WP-E10-006 #$WP_E10_006"
echo "E11 #$E11_NUMBER"
echo "  WP-E11-001 #$WP_E11_001"
echo "  WP-E11-002 #$WP_E11_002"
echo "  WP-E11-003 #$WP_E11_003"
echo "  WP-E11-004 #$WP_E11_004"
echo "  WP-E11-005 #$WP_E11_005"
echo "  WP-E11-006 #$WP_E11_006"
