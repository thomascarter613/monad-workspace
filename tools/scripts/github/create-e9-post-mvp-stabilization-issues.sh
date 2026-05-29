#!/usr/bin/env bash
set -euo pipefail

REPO="thomascarter613/monad-workspace"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

ensure_label() {
  local name="$1"
  local color="$2"
  local description="$3"

  if gh label list --repo "$REPO" --search "$name" --json name --jq '.[].name' | grep -Fxq "$name"; then
    gh label edit "$name" \
      --repo "$REPO" \
      --color "$color" \
      --description "$description" >/dev/null
  else
    gh label create "$name" \
      --repo "$REPO" \
      --color "$color" \
      --description "$description" >/dev/null
  fi
}

existing_issue_url() {
  local title="$1"

  gh issue list \
    --repo "$REPO" \
    --state all \
    --search "$title in:title" \
    --json title,url \
    --jq ".[] | select(.title == \"$title\") | .url" \
    | head -n 1
}

create_issue_if_missing() {
  local title="$1"
  local labels="$2"
  local body_file="$3"

  local existing
  existing="$(existing_issue_url "$title")"

  if [[ -n "$existing" ]]; then
    echo "Exists: $title" >&2
    echo "$existing"
    return 0
  fi

  echo "Creating: $title" >&2

  gh issue create \
    --repo "$REPO" \
    --title "$title" \
    --label "$labels" \
    --body-file "$body_file"
}

issue_number_from_url() {
  local url="$1"
  sed -E 's#.*/issues/([0-9]+).*#\1#' <<<"$url"
}

write_wp_body() {
  if [[ "$#" -ne 12 ]]; then
    echo "write_wp_body expected 12 arguments, got $#." >&2
    exit 2
  fi

  local file="$1"
  local wp_id="$2"
  local title="$3"
  local product_area="$4"
  local objective="$5"
  local in_scope="$6"
  local out_scope="$7"
  local expected_files="$8"
  local verification="$9"
  local commit_msg="${10}"
  local epic_number="${11}"
  local epic_url="${12}"

  cat > "$file" <<EOF_BODY
## Work Packet ID

${wp_id}

## Parent Epic ID

E9

## Parent Epic Issue

#${epic_number} — ${epic_url}

## Work Packet Title

${title}

## Product Area

${product_area}

## Objective

${objective}

## Scope

### In scope

${in_scope}

### Out of scope

${out_scope}

## Expected Files or Directories Affected

${expected_files}

## Tasks

- [ ] Confirm current repository state.
- [ ] Implement the scoped work.
- [ ] Verify formatting.
- [ ] Verify tests.
- [ ] Verify Clippy.
- [ ] Run root verification.
- [ ] Record evidence or update documentation where appropriate.
- [ ] Commit as one atomic stabilization commit.

## Verification Commands / Evidence

\`\`\`bash
${verification}
\`\`\`

## Expected Result After Verification

- Formatting passes where applicable.
- Tests pass where applicable.
- Clippy passes where applicable.
- Root verification passes where applicable.
- The work packet outcome is visible and reviewable.
- No unrelated feature expansion is introduced.

## Definition of Done

- [ ] Scoped work is complete.
- [ ] Verification is complete or blockers are documented.
- [ ] Documentation is updated where needed.
- [ ] Atomic commit completed.

## Recommended Conventional Commit

\`\`\`bash
git commit -m "${commit_msg}"
\`\`\`

## Risks / Blockers / Open Questions

- Keep this work focused on post-MVP candidate stabilization.
- Do not expand public release scope prematurely.
- Record blockers honestly rather than hiding failures.

## Priority

P1 High

## Size

M
EOF_BODY
}

echo "==> Ensuring E9 labels exist"

ensure_label "epic:e9" "5319e7" "Epic E9: post-MVP candidate stabilization and public-readiness gap closure."
ensure_label "post-mvp" "0052cc" "Work after the internal MVP candidate cut."
ensure_label "stabilization" "0e8a16" "Stabilization, hardening, cleanup, and reliability work."
ensure_label "public-readiness" "d93f0b" "Work required before public pre-release or public distribution."
ensure_label "area:release" "1d76db" "Release readiness, versioning, packaging, changelog, and distribution preparation."
ensure_label "area:docs" "1d76db" "Documentation, README, guides, tutorials, and repo-native written artifacts."
ensure_label "area:verification" "1d76db" "Checks, evidence, validation, reporting, and verification workflows."
ensure_label "area:repo-hygiene" "1d76db" "Repository hygiene, ignore policies, generated artifacts, licensing, and contribution readiness."
ensure_label "area:context" "1d76db" "Repo-native context, handoff, bootstrap, and AI-readable project state."
ensure_label "type:epic" "5319e7" "Large body of work composed of multiple work packets."
ensure_label "type:work-packet" "8250df" "Bounded delivery unit with objective, scope, deliverables, verification, and commit."
ensure_label "priority:p1" "d93f0b" "High priority; important for the current milestone or near-term progress."
ensure_label "status:ready" "0e8a16" "Clear, scoped, and ready to start."
ensure_label "needs-verification" "fbca04" "Requires test, check, review, or evidence before completion."
ensure_label "context-update-required" "5319e7" "Requires Monad context, handoff, or current-state documentation update."

echo "==> Creating or reusing E9 epic"

epic_body="$tmpdir/e9-epic.md"
cat > "$epic_body" <<'EOF_BODY'
## Epic ID

E9

## Epic Title

Post-MVP Candidate Stabilization and Public-Readiness Gap Closure

## Product Area

Stabilization / Public Readiness

## Objective

Stabilize Monad after the internal MVP candidate cut, identify gaps between the internal candidate and a future public pre-release, and close repo-hygiene, verification, documentation, context, and release-readiness risks without prematurely expanding public release scope.

## Scope

### In scope

- Audit MVP candidate gaps against public-readiness criteria.
- Harden generated artifact and ignore policies.
- Stabilize context-generation freshness and release metadata.
- Add public pre-release readiness checklist.
- Review licensing, contribution, and repository hygiene.
- Decide the first public pre-release boundary.

### Out of scope

- Public release.
- Crates.io publishing.
- Installer generation.
- Hosted service launch.
- Marketing launch.
- Autonomous agent execution.
- Apply/write evolution behavior.
- MCP server release.
- Enterprise SaaS features.

## Work Packets

- WP-E9-001 — Audit MVP candidate gaps against public-readiness criteria
- WP-E9-002 — Harden generated artifact and ignore policies
- WP-E9-003 — Stabilize context-generation freshness and release metadata
- WP-E9-004 — Add public pre-release readiness checklist
- WP-E9-005 — Review licensing, contribution, and repository hygiene
- WP-E9-006 — Decide first public pre-release boundary

## Definition of Done

- All E9 work packets are complete.
- Public-readiness gaps are recorded.
- Generated artifact policy is hardened.
- Context freshness policy is stabilized.
- Public pre-release checklist exists.
- Licensing and contribution hygiene are reviewed.
- First public pre-release boundary is explicitly decided or deferred.

## Priority

P1 High
EOF_BODY

epic_title="[Epic]: E9 — Post-MVP Candidate Stabilization and Public-Readiness Gap Closure"
epic_url="$(create_issue_if_missing \
  "$epic_title" \
  "type:epic,epic:e9,post-mvp,stabilization,public-readiness,area:release,priority:p1,status:ready,needs-verification" \
  "$epic_body")"

epic_url="$(tail -n 1 <<<"$epic_url")"
epic_number="$(issue_number_from_url "$epic_url")"

echo "E9 epic: #$epic_number — $epic_url"

echo "==> Creating or reusing E9 work packets"

wp1="$tmpdir/wp-e9-001.md"
write_wp_body \
  "$wp1" \
  "WP-E9-001" \
  "Audit MVP candidate gaps against public-readiness criteria" \
  "Public Readiness / Product Gap Analysis" \
  "Create a gap audit comparing the internal MVP candidate against what would be required for a responsible first public pre-release." \
  "- Define public-readiness criteria.
- Compare current MVP candidate capabilities against those criteria.
- Identify blockers.
- Identify deferred capabilities.
- Create a prioritized gap table.
- Avoid implementing fixes in this packet unless they are tiny documentation corrections." \
  "- Public release.
- Package publishing.
- Installer generation.
- Hosted launch.
- Major feature implementation." \
  "- \`docs/release/PUBLIC-READINESS-GAP-AUDIT.md\`
- \`docs/release/E9-STABILIZATION-PLAN.md\`
- \`.monad/context/\` if context state needs updating" \
  "git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh" \
  "docs(release): audit public readiness gaps" \
  "$epic_number" \
  "$epic_url"

create_issue_if_missing \
  "[Work Packet]: WP-E9-001 — Audit MVP candidate gaps against public-readiness criteria" \
  "type:work-packet,epic:e9,post-mvp,public-readiness,area:release,area:docs,priority:p1,status:ready,needs-verification,context-update-required" \
  "$wp1"

wp2="$tmpdir/wp-e9-002.md"
write_wp_body \
  "$wp2" \
  "WP-E9-002" \
  "Harden generated artifact and ignore policies" \
  "Repository Hygiene / Verification" \
  "Harden ignore rules and verifier behavior so generated artifacts, vendor folders, reports, and imported dumps do not destabilize first-party documentation and verification gates." \
  "- Review .gitignore.
- Review generated artifact locations.
- Review Markdown frontmatter checker exclusions.
- Review context/report generated output behavior.
- Document first-party versus generated artifact policy.
- Add or update verification for generated-artifact boundaries." \
  "- Deleting useful imported records without review.
- Broadly disabling first-party documentation verification.
- Public packaging work." \
  "- \`.gitignore\`
- \`tools/scripts/check-markdown-frontmatter.py\`
- \`tools/scripts/verify.sh\`
- \`docs/repository/GENERATED-ARTIFACT-POLICY.md\`
- \`.artifacts/\` only if intentional evidence is retained" \
  "git status --short
python3 tools/scripts/check-markdown-frontmatter.py
tools/scripts/verify.sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings" \
  "chore(repo): harden generated artifact policy" \
  "$epic_number" \
  "$epic_url"

create_issue_if_missing \
  "[Work Packet]: WP-E9-002 — Harden generated artifact and ignore policies" \
  "type:work-packet,epic:e9,post-mvp,stabilization,area:repo-hygiene,area:verification,priority:p1,status:ready,needs-verification" \
  "$wp2"

wp3="$tmpdir/wp-e9-003.md"
write_wp_body \
  "$wp3" \
  "WP-E9-003" \
  "Stabilize context-generation freshness and release metadata" \
  "Context / Release Metadata" \
  "Stabilize repo-native context generation so current-state, handoff, context pack, decision log, and release metadata remain accurate after the MVP candidate cut." \
  "- Review context artifacts for stale epic references.
- Review context checker requirements.
- Document context freshness policy.
- Ensure E8/E9 release state is discoverable.
- Update handoff/current-state generation outputs if needed." \
  "- New AI provider behavior.
- Autonomous context mutation.
- Remote context storage.
- Major context engine redesign." \
  "- \`.monad/context/current-state.md\`
- \`.monad/context/latest-handoff.md\`
- \`.monad/context/latest-context-pack.md\`
- \`.monad/context/decision-log.md\`
- \`docs/context/CONTEXT-FRESHNESS-POLICY.md\`
- \`tools/scripts/check-context-records.py\` if needed" \
  "cargo run -p monad-cli -- context generate current-state
cargo run -p monad-cli -- context generate handoff
cargo run -p monad-cli -- context pack
python3 tools/scripts/check-context-records.py
tools/scripts/verify.sh" \
  "docs(context): stabilize release context freshness" \
  "$epic_number" \
  "$epic_url"

create_issue_if_missing \
  "[Work Packet]: WP-E9-003 — Stabilize context-generation freshness and release metadata" \
  "type:work-packet,epic:e9,post-mvp,stabilization,area:context,area:release,priority:p1,status:ready,needs-verification,context-update-required" \
  "$wp3"

wp4="$tmpdir/wp-e9-004.md"
write_wp_body \
  "$wp4" \
  "WP-E9-004" \
  "Add public pre-release readiness checklist" \
  "Public Readiness / Release Governance" \
  "Create a public pre-release readiness checklist that defines what must be true before Monad can move from internal MVP candidate to public pre-release." \
  "- Define public pre-release gates.
- Include build/install expectations.
- Include documentation expectations.
- Include verification expectations.
- Include licensing/contribution/repository hygiene gates.
- Include explicit no-go criteria." \
  "- Actually publishing a public pre-release.
- Package registry configuration.
- Installer generation.
- Marketing launch." \
  "- \`docs/release/PUBLIC-PRERELEASE-CHECKLIST.md\`
- \`docs/release/README.md\`
- \`docs/project/MVP-SCOPE-FREEZE.md\` if cross-reference is needed" \
  "git status --short
grep -R \"public pre-release\" docs/release docs/project || true
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh" \
  "docs(release): add public prerelease checklist" \
  "$epic_number" \
  "$epic_url"

create_issue_if_missing \
  "[Work Packet]: WP-E9-004 — Add public pre-release readiness checklist" \
  "type:work-packet,epic:e9,post-mvp,public-readiness,area:release,area:docs,priority:p1,status:ready,needs-verification" \
  "$wp4"

wp5="$tmpdir/wp-e9-005.md"
write_wp_body \
  "$wp5" \
  "WP-E9-005" \
  "Review licensing, contribution, and repository hygiene" \
  "Repository Hygiene / Open Source Readiness" \
  "Review and document the repository hygiene requirements needed before a future public pre-release, including license, contribution policy, code of conduct decision, security policy, and issue/PR hygiene." \
  "- Review license status.
- Review contribution policy status.
- Review security policy status.
- Review code of conduct decision.
- Review issue/PR template readiness.
- Create repository hygiene report." \
  "- Legal advice.
- Public release.
- Organization-level policy creation.
- Package publishing." \
  "- \`LICENSE\` if needed
- \`CONTRIBUTING.md\` if needed
- \`SECURITY.md\` if needed
- \`CODE_OF_CONDUCT.md\` if explicitly chosen
- \`docs/repository/REPOSITORY-HYGIENE-REVIEW.md\`
- \`.github/\` templates if needed" \
  "git status --short
find .github -maxdepth 3 -type f | sort 2>/dev/null || true
test -f LICENSE || true
test -f CONTRIBUTING.md || true
test -f SECURITY.md || true
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh" \
  "docs(repo): review public readiness hygiene" \
  "$epic_number" \
  "$epic_url"

create_issue_if_missing \
  "[Work Packet]: WP-E9-005 — Review licensing, contribution, and repository hygiene" \
  "type:work-packet,epic:e9,post-mvp,public-readiness,area:repo-hygiene,area:docs,priority:p1,status:ready,needs-verification" \
  "$wp5"

wp6="$tmpdir/wp-e9-006.md"
write_wp_body \
  "$wp6" \
  "WP-E9-006" \
  "Decide first public pre-release boundary" \
  "Product / Release Decision" \
  "Make an explicit go/no-go decision on the first public pre-release boundary, including what remains internal, what can be public, and what must be deferred." \
  "- Review E9 gap audit.
- Review public pre-release checklist.
- Review repository hygiene review.
- Decide public pre-release boundary.
- Record decision as a release decision and/or ADR.
- Create E9 closeout note." \
  "- Publishing the release.
- Creating public tag.
- Package publication.
- Installer generation.
- Hosted launch." \
  "- \`docs/release/FIRST-PUBLIC-PRERELEASE-BOUNDARY.md\`
- \`docs/release/E9-CLOSEOUT.md\`
- \`docs/adr/\` if an ADR is needed
- \`.monad/context/\` if context state needs updating" \
  "git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
git status --short" \
  "docs(release): decide first public prerelease boundary" \
  "$epic_number" \
  "$epic_url"

create_issue_if_missing \
  "[Work Packet]: WP-E9-006 — Decide first public pre-release boundary" \
  "type:work-packet,epic:e9,post-mvp,public-readiness,area:release,area:docs,priority:p1,status:ready,needs-verification,context-update-required" \
  "$wp6"

echo
echo "E9 issue creation complete."
