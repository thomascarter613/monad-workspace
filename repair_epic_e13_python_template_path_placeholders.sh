#!/usr/bin/env bash
set -euo pipefail

# Repair — Epic E13 Python template path placeholders
#
# Symptom:
#   python_plan_normalizes_module_name fails because the planned target path still
#   contains the literal template placeholder:
#
#     src/{{python_module_name}}/__init__.py
#
#   instead of:
#
#     src/my_worker/__init__.py
#
# Cause:
#   Epic E13 added placeholder rendering for template CONTENT, but template
#   RELATIVE PATHS are also templated for Python module names.
#
# Fix:
#   Add ComponentScaffoldTemplate::render_relative_path(...)
#   and use it anywhere add planning/apply resolves template target paths.

echo "==> Repair: E13 Python template relative path placeholders"

if git rev-parse --show-toplevel >/dev/null 2>&1; then
  REPO_ROOT="$(git rev-parse --show-toplevel)"
else
  REPO_ROOT="$(pwd)"
fi

cd "$REPO_ROOT"

CORE_FILE="crates/monad-core/src/component_add.rs"

if [ ! -f "$CORE_FILE" ]; then
  echo "ERROR: expected file not found: $CORE_FILE" >&2
  echo "Run this from the Monad repository root." >&2
  exit 1
fi

if ! grep -q "{{python_module_name}}" "$CORE_FILE"; then
  echo "ERROR: expected Python template placeholder not found in $CORE_FILE" >&2
  echo "This repair is intended for the E13 language-aware scaffold implementation." >&2
  exit 1
fi

mkdir -p .monad/script-backups/E13/REPAIR-python-template-paths
BACKUP_STAMP="$(date +%Y%m%d%H%M%S)"
cp "$CORE_FILE" ".monad/script-backups/E13/REPAIR-python-template-paths/component_add.rs.$BACKUP_STAMP.bak"

python3 <<'PY'
from pathlib import Path

path = Path("crates/monad-core/src/component_add.rs")
text = path.read_text()


def replace_once(text: str, old: str, new: str, label: str) -> str:
    if old not in text:
        raise SystemExit(f"ERROR: could not find expected block for {label}")
    return text.replace(old, new, 1)


# Add a relative-path rendering method alongside the existing content renderer.
if "pub fn render_relative_path(self, options: &AddPlanOptions) -> PathBuf" not in text:
    marker = '''    /// Renders template content for a component.
    #[must_use]
    pub fn render_content(self, options: &AddPlanOptions) -> String {
'''
    insertion = '''    /// Renders the path relative to the component root.
    ///
    /// Most templates use static relative paths such as `README.md`.
    /// Python module templates need a dynamic path segment because component
    /// names use hyphens while Python import modules use underscores.
    #[must_use]
    pub fn render_relative_path(self, options: &AddPlanOptions) -> PathBuf {
        let python_module_name = options.python_module_name();
        let go_module_path = options.go_module_path();

        let rendered = self
            .relative_path
            .replace("{{component_kind}}", options.kind().as_str())
            .replace("{{component_name}}", options.name().as_str())
            .replace("{{component_language}}", options.language_label())
            .replace("{{python_module_name}}", &python_module_name)
            .replace("{{go_module_path}}", &go_module_path);

        PathBuf::from(rendered)
    }

'''
    if marker not in text:
        raise SystemExit("ERROR: could not find ComponentScaffoldTemplate render_content marker")
    text = text.replace(marker, insertion + marker, 1)


# Use rendered relative paths in planning.
text = text.replace(
    "let target_path = options.component_root().join(template.relative_path());",
    "let target_path = options.component_root().join(template.render_relative_path(options));",
)

# Use rendered relative paths in guarded writes.
text = text.replace(
    "let relative_path = options.component_root().join(template.relative_path());",
    "let relative_path = options.component_root().join(template.render_relative_path(options));",
)


# Add focused regression test if missing.
if "fn template_relative_paths_render_python_module_name" not in text:
    test = '''
    #[test]
    fn template_relative_paths_render_python_module_name() -> MonadResult<()> {
        let options = AddPlanOptions::new(ComponentKind::Service, ComponentName::parse("my-worker")?)
            .with_language(Some(ComponentLanguage::Python));
        let python_templates = component_scaffold_templates_for_options(&options);
        let rendered_paths = python_templates
            .iter()
            .map(|template| template.render_relative_path(&options))
            .collect::<Vec<_>>();

        assert!(rendered_paths.contains(&PathBuf::from("src/my_worker/__init__.py")));
        assert!(!rendered_paths.contains(&PathBuf::from(
            "src/{{python_module_name}}/__init__.py"
        )));

        Ok(())
    }

'''
    marker = "\n    #[test]\n    fn python_plan_normalizes_module_name()"
    if marker not in text:
        raise SystemExit("ERROR: could not find python_plan_normalizes_module_name test marker")
    text = text.replace(marker, "\n" + test + marker, 1)

path.write_text(text)
PY

echo "==> Formatting Rust code"
cargo fmt

echo
echo "==> Repair complete."
echo
echo "Recommended focused verification:"
echo "  cargo test -p monad-core --lib component_add::tests::template_relative_paths_render_python_module_name"
echo "  cargo test -p monad-core --lib component_add::tests::python_plan_normalizes_module_name"
echo "  cargo test -p monad-core --lib component_add::tests::apply_add_plan_writes_language_aware_python_scaffold"
echo
echo "Then full verification:"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-add-language.sh"
echo "  tools/scripts/verify-e13.sh"
