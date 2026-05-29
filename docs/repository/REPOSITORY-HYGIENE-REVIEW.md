---
title: Repository Hygiene Review
description: Public-readiness repository hygiene review for Monad.
status: draft
version: 0.1.0
created: 2026-05-29
updated: 2026-05-29
owner: Thomas Carter
project: Monad
phase: Post-MVP Candidate Stabilization
epic: E9
work_packet: WP-E9-005
---

# Repository Hygiene Review

## 1. Purpose

This document reviews Monad's repository hygiene posture before any future public pre-release.

This review does not authorize a public release. It identifies what must exist, what is already present, what is missing, and what may be intentionally deferred.

## 2. Public-readiness summary

Current decision:

```text
NO-GO for public pre-release until repository hygiene gates are resolved or formally deferred.
```

## 3. Required public-facing hygiene areas

| Area                      | Required before public pre-release                                      | Current status                | Required action                                                     |
| ------------------------- | ----------------------------------------------------------------------- | ----------------------------- | ------------------------------------------------------------------- |
| License                   | Repository license must be present and intentional.                     | Pending local inspection      | Add or confirm license.                                             |
| Contribution policy       | Contribution expectations must be documented.                           | Pending local inspection      | Add `CONTRIBUTING.md` or formally defer outside public pre-release. |
| Security policy           | Vulnerability reporting path must be documented.                        | Pending local inspection      | Add `SECURITY.md`.                                                  |
| Code of conduct           | Public community conduct expectations must be decided.                  | Pending decision              | Add `CODE_OF_CONDUCT.md` or explicitly defer.                       |
| Issue templates           | Public issue intake should be structured.                               | Pending local inspection      | Add issue templates or defer.                                       |
| PR template               | Public contribution review expectations should be structured.           | Pending local inspection      | Add PR template or defer.                                           |
| README accuracy           | README must describe implemented behavior only.                         | Pending review                | Review before public pre-release.                                   |
| Generated artifact policy | Generated/imported artifacts must not create false dependency surfaces. | In progress / addressed in E9 | Keep policy enforced.                                               |
| Dependency security       | No unresolved high/critical alerts without documented risk acceptance.  | Cleared as of E9 triage       | Recheck before public pre-release.                                  |

## 4. Recommended minimum public pre-release hygiene set

Before the first public pre-release, Monad should have:

* `LICENSE`
* `CONTRIBUTING.md`
* `SECURITY.md`
* `.github/ISSUE_TEMPLATE/bug_report.md`
* `.github/ISSUE_TEMPLATE/feature_request.md`
* `.github/pull_request_template.md`
* `docs/repository/GENERATED-ARTIFACT-POLICY.md`
* `docs/release/PUBLIC-PRERELEASE-CHECKLIST.md`
* accurate `README.md`

## 5. License decision

The repository should not proceed to public pre-release without an explicit license.

Recommended default for Monad, unless changed by project policy:

```text
Apache License 2.0
```

Rationale:

* compatible with open-source infrastructure tooling
* explicit patent grant
* common for developer tools and platform tooling
* more protective than MIT for a serious tool that may grow into commercial or ecosystem use

This is a project decision, not legal advice.

## 6. Contribution policy decision

Monad should include a contribution policy before public pre-release.

Minimum contribution policy should state:

* public contributions are welcome only after the project owner is ready to accept them
* all contributions require review
* issue discussion does not imply roadmap acceptance
* AI-generated contributions must be disclosed and reviewed carefully
* security issues must not be filed publicly

## 7. Security policy decision

Monad should include a security policy before public pre-release.

Minimum security policy should state:

* do not disclose vulnerabilities publicly before coordination
* report vulnerabilities by a designated private channel
* supported versions are currently limited to internal/pre-release code
* high/critical security issues block public release unless risk-accepted

## 8. Code of conduct decision

A code of conduct is recommended before any broader public/community launch.

For a first technical pre-release, it may be acceptable to defer if the project is not yet accepting community contributions. If deferred, the README or contribution guide should clearly state that public contribution/community process is not yet open.

## 9. Issue and PR template decision

Issue and PR templates are recommended before public pre-release if the repository is public and users may open issues.

Minimum issue templates:

* bug report
* feature request

Minimum PR template:

* summary
* verification
* scope/risks
* checklist confirming no unrelated expansion

## 10. Current no-go items

Monad should not proceed to public pre-release until the following are resolved or formally deferred:

* license status confirmed
* contribution policy exists or contribution intake is explicitly closed
* security reporting policy exists
* README reviewed for capability accuracy
* issue/PR template readiness decided
* public pre-release boundary decided in WP-E9-006

## 11. Verification evidence

Run:

```bash
find .github -maxdepth 4 -type f | sort 2>/dev/null || true

for file in LICENSE LICENSE.md COPYING CONTRIBUTING.md SECURITY.md CODE_OF_CONDUCT.md README.md; do
  if [ -f "$file" ]; then
    echo "FOUND $file"
  else
    echo "MISSING $file"
  fi
done

cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

