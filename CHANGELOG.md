# Changelog

All notable changes to Monad will be documented in this file.

This project follows a repo-native, governance-grade release discipline. The changelog records meaningful product, architecture, safety, documentation, and verification changes. It does not imply public release unless a tagged release and release record explicitly say so.

## Release policy notes

- `Unreleased` tracks work merged after the latest tag.
- `Internal MVP Candidate` tracks the current pre-public release candidate cut.
- Public release entries must not be added until public distribution is explicitly approved.
- Release claims must stay aligned with `docs/project/MVP-SCOPE-FREEZE.md`.
- Future capabilities must not be described as completed behavior.

## [Unreleased]

### Added

- Release documentation foundation for E8 MVP candidate preparation.
- Initial release notes template under `docs/release/RELEASE-NOTES-TEMPLATE.md`.

### Changed

- None yet.

### Fixed

- None yet.

### Security

- None yet.

### Verification

- None yet.

## [0.1.0-internal-mvp-candidate] - Pending

### Status

This is not a public release.

This section represents the intended internal MVP candidate cut line once E8 release-preparation work completes and verification passes.

### Included candidate capabilities

- Workspace summary command.
- Repository inspection command.
- Workspace check command.
- Repository graph rendering in text, JSON, Mermaid, and DOT formats.
- Repo-native context rendering, generation, verification, and context pack assembly.
- Supervised no-write planning.
- Dry-run evolution previews for verification and context baselines.
- Local verification via formatting, tests, Clippy, and root verification script.

### Explicitly excluded from this candidate

- Public package publishing.
- Installer generation.
- Hosted service launch.
- Autonomous agent execution.
- Apply/write evolution behavior.
- Real model-provider execution by default.
- MCP runtime/server release.
- Enterprise SaaS features.
