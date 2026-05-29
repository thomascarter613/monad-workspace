# Contributing to Monad

Monad is currently in post-internal-MVP-candidate stabilization.

Public contributions may be limited until the project reaches a documented public pre-release boundary.

## Ground rules

By contributing, you agree that:

- contributions require review before merge
- issue discussion does not imply roadmap acceptance
- pull requests must stay within their stated scope
- unrelated changes should be split into separate work
- verification evidence is expected
- AI-generated contributions must be disclosed and carefully reviewed

## Development baseline

Before submitting a change, run:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
tools/scripts/verify.sh
```

## Commit style

Use Conventional Commits.

Examples:

```bash
git commit -m "docs(repo): update contribution guide"
git commit -m "chore(repo): harden generated artifact policy"
git commit -m "feat(core): add repository inspection capability"
```

## Security issues

Do not report security vulnerabilities in public issues.

Use the security reporting process in `SECURITY.md`.
