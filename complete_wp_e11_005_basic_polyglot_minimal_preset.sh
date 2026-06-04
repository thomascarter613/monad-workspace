#!/usr/bin/env bash
set -euo pipefail

# Complete WP-E11-005 — Add basic/polyglot-minimal preset hardening.
#
# This packet keeps the guarded init safety model intact while making the
# preset UX clearer.
#
# No new destructive behavior is added.

mkdir -p docs/commands
mkdir -p work/deliverables/E11

python3 - <<'PY'
from pathlib import Path

init_path = Path("crates/monad-core/src/init.rs")
main_path = Path("crates/monad-cli/src/main.rs")

init = init_path.read_text(encoding="utf-8")

init = init.replace(
    '            "minimal" => Ok(Self::Minimal),\n            "polyglot-minimal" => Ok(Self::PolyglotMinimal),',
    '            "basic" | "minimal" => Ok(Self::Minimal),\n            "polyglot-minimal" => Ok(Self::PolyglotMinimal),',
)

init = init.replace(
    "unsupported init preset `{other}`; supported presets: minimal, polyglot-minimal",
    "unsupported init preset `{other}`; supported presets: basic, minimal, polyglot-minimal",
)

if "fn init_preset_parses_basic_alias" not in init:
    anchor = "    #[test]\n    fn init_preset_rejects_unknown_values() {"
    test = '''    #[test]
    fn init_preset_parses_basic_alias() -> MonadResult<()> {
        assert_eq!(InitPreset::parse("basic")?, InitPreset::Minimal);
        assert_eq!(InitPreset::parse("minimal")?, InitPreset::Minimal);

        Ok(())
    }

'''
    if anchor not in init:
        raise SystemExit("Could not find init test insertion anchor")
    init = init.replace(anchor, test + anchor, 1)

init_path.write_text(init, encoding="utf-8")

main = main_path.read_text(encoding="utf-8")

basic_help_line = '        "  init --preset=basic --dry-run             Preview the basic repository scaffold",\n'
if basic_help_line not in main:
    dry_run_line = '        "  init --dry-run                            Preview repository initialization plan",\n'
    if dry_run_line in main:
        main = main.replace(dry_run_line, dry_run_line + basic_help_line, 1)

if "fn init_basic_preset_alias_parses" not in main:
    anchor = "    #[test]\n    fn info_command_parses_text_and_json_formats() {"
    test = '''    #[test]
    fn init_basic_preset_alias_parses() {
        assert_eq!(
            parse_arguments(&["monad", "init", "--dry-run", "--preset=basic"])
                .expect("init basic preset should parse"),
            CliCommand::Init {
                dry_run: true,
                yes: false,
                preset: InitPreset::Minimal,
                project_name: None,
            }
        );
    }

'''
    if anchor not in main:
        raise SystemExit("Could not find CLI test insertion anchor")
    main = main.replace(anchor, test + anchor, 1)

if 'assert!(text.contains("init --preset=basic --dry-run"));' not in main:
    main = main.replace(
        '        assert!(text.contains("init --dry-run"));\n',
        '        assert!(text.contains("init --dry-run"));\n'
        '        assert!(text.contains("init --preset=basic --dry-run"));\n',
        1,
    )

main_path.write_text(main, encoding="utf-8")
PY

cat > docs/commands/INIT-PRESETS.md <<'EOF'
---
title: "monad init Presets"
document_type: "command-reference"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E11
work_packet: WP-E11-005
tags:
  - monad
  - command
  - init
  - presets
  - scaffold
related:
  - docs/commands/INIT.md
  - crates/monad-core/src/init.rs
  - crates/monad-core/src/templates/registry.rs
---

# `monad init` Presets

## Status

Accepted.

## Work Packet

WP-E11-005 — Add basic/polyglot-minimal preset.

## Purpose

This document records the initial `monad init` preset model.

The preset model is intentionally small and conservative.

It gives Monad a useful initialization experience without becoming a full application/framework generator.

## Current Presets

| Preset | Status | Meaning |
| --- | --- | --- |
| `basic` | Accepted | Friendly alias for the smallest useful Monad-aware repository scaffold. |
| `minimal` | Accepted | Backward-compatible name for the same scaffold as `basic`. |
| `polyglot-minimal` | Accepted | Minimal polyglot monorepo directory scaffold layered on top of the basic scaffold. |

## Recommended Default

The recommended user-facing default name is:

```text
basic
```

The `minimal` name remains supported because it was introduced first and is still accurate.

## `basic`

Preview:

```bash
monad init --preset=basic --dry-run
```

Apply after review:

```bash
monad init --preset=basic --yes
```

The `basic` preset creates:

```text
monad.toml
README.md
docs/README.md
work/README.md
.monad/.gitignore
```

## `minimal`

Preview:

```bash
monad init --preset=minimal --dry-run
```

Apply after review:

```bash
monad init --preset=minimal --yes
```

`minimal` is equivalent to `basic`.

It creates the same file set:

```text
monad.toml
README.md
docs/README.md
work/README.md
.monad/.gitignore
```

## `polyglot-minimal`

Preview:

```bash
monad init --preset=polyglot-minimal --dry-run
```

Apply after review:

```bash
monad init --preset=polyglot-minimal --yes
```

The `polyglot-minimal` preset includes the basic scaffold plus:

```text
apps/.gitkeep
packages/.gitkeep
services/.gitkeep
tools/.gitkeep
```

It does not add Bazel, Pants, Buck2, or Nx.

It does not install package managers.

It does not create language-specific manifests.

Those behaviors require separate future command contracts.

## Safety Boundary

All presets follow the same safety model:

- preview with `--dry-run`;
- apply only with `--yes`;
- refuse to overwrite existing files;
- abort when conflicts are detected;
- run no Git commands;
- call no remote services;
- install no dependencies;
- publish nothing.

## Relationship to Future Presets

Future presets may include:

```text
rust-library
rust-cli
typescript-app
python-service
go-service
java-service
monorepo-polyglot
```

Those are intentionally out of scope for E11 until the preset/schema/template architecture is more mature.

## Verification

```bash
cargo run -p monad-cli -- init --preset=basic --dry-run
cargo run -p monad-cli -- init --preset=minimal --dry-run
cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run
```

For write-path verification, run inside an empty temporary directory only:

```bash
cargo run --manifest-path /path/to/monad-workspace/Cargo.toml -p monad-cli -- init --preset=basic --yes
cargo run --manifest-path /path/to/monad-workspace/Cargo.toml -p monad-cli -- init --preset=polyglot-minimal --yes
```

## Outcome

Accepted.

`basic`, `minimal`, and `polyglot-minimal` are the initial `monad init` preset surface.
EOF

python3 - <<'PY'
from pathlib import Path

cmd_path = Path("docs/commands/INIT.md")
if cmd_path.exists():
    text = cmd_path.read_text(encoding="utf-8")
    if "## WP-E11-005 Implementation Note" not in text:
        text += '''

## WP-E11-005 Implementation Note

WP-E11-005 clarifies the initial preset surface.

Supported presets:

```text
basic
minimal
polyglot-minimal
```

`basic` and `minimal` currently produce the same scaffold. `basic` is the recommended user-facing name.

`polyglot-minimal` adds the initial monorepo directories:

```text
apps/
packages/
services/
tools/
```

No package managers, language manifests, Bazel, Pants, Buck2, or Nx files are generated by these presets.
'''
    cmd_path.write_text(text, encoding="utf-8")

ref_path = Path("docs/project/MVP-COMMAND-REFERENCE.md")
if ref_path.exists():
    text = ref_path.read_text(encoding="utf-8")
    if "cargo run -p monad-cli -- init --preset=basic --dry-run" not in text:
        text = text.replace(
            "cargo run -p monad-cli -- init --dry-run\n",
            "cargo run -p monad-cli -- init --dry-run\n"
            "cargo run -p monad-cli -- init --preset=basic --dry-run\n",
            1,
        )
    if "* supports `basic`, `minimal`, and `polyglot-minimal` presets" not in text:
        text = text.replace(
            "* applies the scaffold only when `--yes` is provided and no conflicts exist\n",
            "* supports `basic`, `minimal`, and `polyglot-minimal` presets\n"
            "* applies the scaffold only when `--yes` is provided and no conflicts exist\n",
            1,
        )
    ref_path.write_text(text, encoding="utf-8")

readme_path = Path("README.md")
if readme_path.exists():
    text = readme_path.read_text(encoding="utf-8")
    if "monad init --preset=basic --dry-run" not in text:
        text = text.replace(
            "monad init --dry-run\n",
            "monad init --dry-run\nmonad init --preset=basic --dry-run\n",
            1,
        )
    readme_path.write_text(text, encoding="utf-8")
PY

cat > work/deliverables/E11/WP-E11-005-basic-polyglot-minimal-preset.md <<'EOF'
---
title: "WP-E11-005 Basic and Polyglot-Minimal Preset Deliverable"
document_type: "deliverable-record"
status: accepted
owner: "Thomas Carter"
created: 2026-06-04
updated: 2026-06-04
version: 1.0.0
project: Monad
epic: E11
work_packet: WP-E11-005
tags:
  - monad
  - e11
  - init
  - preset
  - scaffold
related:
  - crates/monad-core/src/init.rs
  - crates/monad-cli/src/main.rs
  - docs/commands/INIT.md
  - docs/commands/INIT-PRESETS.md
---

# WP-E11-005 Basic and Polyglot-Minimal Preset Deliverable

## Work Packet

WP-E11-005 — Add basic/polyglot-minimal preset.

## Outcome

Implemented.

## Summary

This work packet hardens the initial `monad init` preset UX.

The implemented preset surface is:

```text
basic
minimal
polyglot-minimal
```

`basic` is the recommended user-facing name for the minimal scaffold.

`minimal` remains supported as an equivalent alias for the same scaffold.

`polyglot-minimal` remains the first monorepo-shaped scaffold preset.

## Deliverables

- `crates/monad-core/src/init.rs`
- `crates/monad-cli/src/main.rs`
- `docs/commands/INIT.md`
- `docs/commands/INIT-PRESETS.md`
- `docs/project/MVP-COMMAND-REFERENCE.md`
- `README.md`
- `work/deliverables/E11/WP-E11-005-basic-polyglot-minimal-preset.md`

## Safety Boundary

This work packet adds no new destructive behavior.

The existing guarded write model remains:

- `--dry-run` previews;
- `--yes` applies only after conflict checks;
- existing target files block writes;
- no Git commands are run;
- no packages are installed;
- no remote services are called.

## Verification

Run:

```bash
git status --short
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p monad-cli -- init --preset=basic --dry-run
cargo run -p monad-cli -- init --preset=minimal --dry-run
cargo run -p monad-cli -- init --preset=polyglot-minimal --dry-run
cargo run -p monad-cli -- --help | grep "init --preset=basic --dry-run"
tools/scripts/verify.sh
git status --short
```

For write-path verification, use an empty temporary directory.

## Recommended Commit

```bash
git commit -m "feat(init): add basic preset alias"
```

## Closeout Note

WP-E11-005 is complete once the preset alias/docs are committed and the corresponding GitHub work-packet issue or tracking item is closed.

## Next Work Packet

```text
WP-E11-006 — Add init smoke tests and verification evidence
```
EOF

cargo fmt

echo
echo "WP-E11-005 files updated:"
echo "  crates/monad-core/src/init.rs"
echo "  crates/monad-cli/src/main.rs"
echo "  docs/commands/INIT.md"
echo "  docs/commands/INIT-PRESETS.md"
echo "  docs/project/MVP-COMMAND-REFERENCE.md"
echo "  README.md"
echo "  work/deliverables/E11/WP-E11-005-basic-polyglot-minimal-preset.md"
echo
echo "Next verification:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  cargo run -p monad-cli -- init --preset=basic --dry-run"
echo "  tools/scripts/verify.sh"
