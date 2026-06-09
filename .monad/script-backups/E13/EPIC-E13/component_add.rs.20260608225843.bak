//! Component add planning.
//!
//! This module backs `monad add <kind> <name> --dry-run`.
//!
//! WP-E12-003 introduces an embedded component scaffold template layer.
//!
//! The important design idea is now:
//!
//! - user input chooses a component kind and name;
//! - the component kind chooses the component root;
//! - embedded templates describe the scaffold files;
//! - the planner resolves template-relative paths into component paths;
//! - the dry-run evaluator checks the filesystem without writing files.
//!
//! WP-E12-003 remains dry-run only. It does not write component files.

use std::fs;
use std::path::{Path, PathBuf};

use crate::{
    FileOperationPlan, MonadError, MonadResult, PlannedFileOperation, WorkspaceContext,
    evaluate_file_operation_plan, render_dry_run_plan,
};

/// Supported component kinds for the first `monad add` implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComponentKind {
    /// User-facing application component, placed under `apps/`.
    App,

    /// Shared package/library component, placed under `packages/`.
    Package,

    /// Backend/service component, placed under `services/`.
    Service,

    /// Repo-local tool component, placed under `tools/`.
    Tool,
}

impl ComponentKind {
    /// Parses a user-facing component kind.
    pub fn parse(value: &str) -> MonadResult<Self> {
        match value {
            "app" => Ok(Self::App),
            "package" => Ok(Self::Package),
            "service" => Ok(Self::Service),
            "tool" => Ok(Self::Tool),
            other => Err(MonadError::invalid_input(format!(
                "unsupported component kind `{other}`; supported kinds: app, package, service, tool"
            ))),
        }
    }

    /// Returns the stable user-facing label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::App => "app",
            Self::Package => "package",
            Self::Service => "service",
            Self::Tool => "tool",
        }
    }

    /// Returns the root directory family for the component kind.
    #[must_use]
    pub const fn base_dir(self) -> &'static str {
        match self {
            Self::App => "apps",
            Self::Package => "packages",
            Self::Service => "services",
            Self::Tool => "tools",
        }
    }
}

/// Supported language-aware scaffold IDs.
///
/// WP-E13-002 adds the language model and dry-run parsing path only. Later E13
/// packets add concrete language-specific template files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComponentLanguage {
    /// Rust component scaffold.
    Rust,

    /// TypeScript component scaffold.
    Typescript,

    /// Python component scaffold.
    Python,

    /// Go component scaffold.
    Go,
}

impl ComponentLanguage {
    /// Parses a user-facing language ID.
    pub fn parse(value: &str) -> MonadResult<Self> {
        match value {
            "rust" => Ok(Self::Rust),
            "typescript" => Ok(Self::Typescript),
            "python" => Ok(Self::Python),
            "go" => Ok(Self::Go),
            other => Err(MonadError::invalid_input(format!(
                "unsupported component language `{other}`; supported languages: {}",
                Self::supported_values().join(", ")
            ))),
        }
    }

    /// Returns the stable user-facing language ID.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Typescript => "typescript",
            Self::Python => "python",
            Self::Go => "go",
        }
    }

    /// Returns the stable list of supported language IDs.
    #[must_use]
    pub fn supported_values() -> &'static [&'static str] {
        static SUPPORTED: [&str; 4] = ["rust", "typescript", "python", "go"];
        &SUPPORTED
    }
}

/// Filesystem-safe component name.
///
/// This type is deliberately stricter than a general path. Users give Monad a
/// component name, not a path. Monad decides the path from the component kind.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentName(String);

impl ComponentName {
    /// Parses and validates a component name.
    pub fn parse(value: &str) -> MonadResult<Self> {
        if value.is_empty() {
            return Err(MonadError::invalid_input(
                "component name must not be empty",
            ));
        }

        let first = value
            .chars()
            .next()
            .ok_or_else(|| MonadError::invalid_input("component name must not be empty"))?;

        if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
            return Err(MonadError::invalid_input(format!(
                "invalid component name `{value}`; names must start with a lowercase letter or number"
            )));
        }

        if !value.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        }) {
            return Err(MonadError::invalid_input(format!(
                "invalid component name `{value}`; use lowercase letters, numbers, and hyphens only"
            )));
        }

        if value.contains("--") || value.ends_with('-') {
            return Err(MonadError::invalid_input(format!(
                "invalid component name `{value}`; avoid repeated or trailing hyphens"
            )));
        }

        Ok(Self(value.to_string()))
    }

    /// Returns the safe component name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Embedded scaffold template for a component.
///
/// This intentionally avoids a full templating engine. A small explicit
/// placeholder replacement is enough for the first E12 scaffold layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentScaffoldTemplate {
    id: &'static str,
    relative_path: &'static str,
    description: &'static str,
    content: &'static str,
}

impl ComponentScaffoldTemplate {
    /// Returns the stable template ID.
    #[must_use]
    pub const fn id(self) -> &'static str {
        self.id
    }

    /// Returns the path relative to the component root.
    #[must_use]
    pub const fn relative_path(self) -> &'static str {
        self.relative_path
    }

    /// Returns a human-readable description.
    #[must_use]
    pub const fn description(self) -> &'static str {
        self.description
    }

    /// Renders template content for a component.
    #[must_use]
    pub fn render_content(self, options: &AddPlanOptions) -> String {
        self.content
            .replace("{{component_kind}}", options.kind().as_str())
            .replace("{{component_name}}", options.name().as_str())
            .replace("{{component_language}}", options.language_label())
            .replace(
                "{{component_root}}",
                &options.component_root().display().to_string(),
            )
    }
}

const COMPONENT_SCAFFOLD_TEMPLATES: [ComponentScaffoldTemplate; 2] = [
    ComponentScaffoldTemplate {
        id: "component.readme",
        relative_path: "README.md",
        description: "component documentation entry point",
        content: "# {{component_name}}\n\nKind: `{{component_kind}}`\n\nRoot: `{{component_root}}`\n\nThis component was planned by Monad.\n",
    },
    ComponentScaffoldTemplate {
        id: "component.gitkeep",
        relative_path: ".gitkeep",
        description: "component directory placeholder",
        content: "",
    },
];

/// Returns the embedded component scaffold templates.
#[must_use]
pub const fn component_scaffold_templates() -> &'static [ComponentScaffoldTemplate] {
    &COMPONENT_SCAFFOLD_TEMPLATES
}

/// Options used to build a component add plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddPlanOptions {
    kind: ComponentKind,
    name: ComponentName,
    language: Option<ComponentLanguage>,
}

impl AddPlanOptions {
    /// Creates add plan options.
    #[must_use]
    pub fn new(kind: ComponentKind, name: ComponentName) -> Self {
        Self {
            kind,
            name,
            language: None,
        }
    }

    /// Returns a copy of these options with an optional language-aware scaffold selected.
    #[must_use]
    pub fn with_language(mut self, language: Option<ComponentLanguage>) -> Self {
        self.language = language;
        self
    }

    /// Returns the optional language-aware scaffold ID.
    #[must_use]
    pub const fn language(&self) -> Option<ComponentLanguage> {
        self.language
    }

    /// Returns the printable language label used in dry-run output.
    #[must_use]
    pub fn language_label(&self) -> &'static str {
        match self.language {
            Some(language) => language.as_str(),
            None => "generic",
        }
    }

    /// Returns the printable template source label used in dry-run output.
    #[must_use]
    pub fn template_source_label(&self) -> &'static str {
        if self.language.is_some() {
            "embedded language-aware scaffold model"
        } else {
            "embedded component scaffold templates"
        }
    }

    /// Returns the component kind.
    #[must_use]
    pub const fn kind(&self) -> ComponentKind {
        self.kind
    }

    /// Returns the component name.
    #[must_use]
    pub const fn name(&self) -> &ComponentName {
        &self.name
    }

    /// Returns the planned component root, relative to the workspace root.
    #[must_use]
    pub fn component_root(&self) -> PathBuf {
        Path::new(self.kind.base_dir()).join(self.name.as_str())
    }
}

/// Builds the component add file-operation plan.
///
/// This is intentionally minimal. Language-specific files are left to later
/// E12 work packets and future language adapter epics.
#[must_use]
pub fn build_add_plan(options: &AddPlanOptions) -> FileOperationPlan {
    let operations = component_scaffold_templates()
        .iter()
        .map(|template| {
            let target_path = options.component_root().join(template.relative_path());
            let rendered_content = template.render_content(options);

            PlannedFileOperation::create(
                target_path.clone(),
                format!(
                    "create `{}` from embedded component template `{}` ({}, {} bytes)",
                    target_path.display(),
                    template.id(),
                    template.description(),
                    rendered_content.len()
                ),
            )
        })
        .collect::<Vec<_>>();

    FileOperationPlan::from_operations(operations)
}

/// Renders the dry-run output for `monad add`.
pub fn render_add_dry_run(
    context: &WorkspaceContext,
    options: &AddPlanOptions,
) -> MonadResult<String> {
    let plan = build_add_plan(options);
    let dry_run = evaluate_file_operation_plan(context.root(), &plan);
    let root = options.component_root();

    let mut output = vec![
        "Monad add dry-run plan".to_string(),
        String::new(),
        "Component:".to_string(),
        format!("  kind: {}", options.kind().as_str()),
        format!("  name: {}", options.name().as_str()),
        format!("  language: {}", options.language_label()),
        format!("  root: {}", root.display()),
        String::new(),
        "Template source:".to_string(),
        format!("  registry: {}", options.template_source_label()),
        format!("  templates: {}", component_scaffold_templates().len()),
        String::new(),
        "Safety:".to_string(),
        "  mode: dry-run".to_string(),
        "  writes: disabled".to_string(),
        "  apply: guarded by --yes".to_string(),
        "  approval_flag: --yes".to_string(),
        String::new(),
        render_dry_run_plan(&dry_run),
        String::new(),
        "No files were written.".to_string(),
        "Next: review this plan before a future guarded write path is added.".to_string(),
    ];

    if dry_run.has_conflicts() {
        output.push(
            "Conflicts were detected. Resolve existing target paths before any future write path."
                .to_string(),
        );
    }

    Ok(output.join("\n"))
}

/// Result of guarded component add application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddApplyResult {
    kind: ComponentKind,
    name: ComponentName,
    component_root: PathBuf,
    created_files: Vec<PathBuf>,
    total_bytes_written: usize,
}

impl AddApplyResult {
    /// Creates an add apply result.
    #[must_use]
    pub fn new(
        kind: ComponentKind,
        name: ComponentName,
        component_root: PathBuf,
        created_files: Vec<PathBuf>,
        total_bytes_written: usize,
    ) -> Self {
        Self {
            kind,
            name,
            component_root,
            created_files,
            total_bytes_written,
        }
    }

    /// Returns component kind.
    #[must_use]
    pub const fn kind(&self) -> ComponentKind {
        self.kind
    }

    /// Returns component name.
    #[must_use]
    pub const fn name(&self) -> &ComponentName {
        &self.name
    }

    /// Returns component root.
    #[must_use]
    pub fn component_root(&self) -> &Path {
        &self.component_root
    }

    /// Returns created files.
    #[must_use]
    pub fn created_files(&self) -> &[PathBuf] {
        &self.created_files
    }

    /// Returns created file count.
    #[must_use]
    pub fn file_count(&self) -> usize {
        self.created_files.len()
    }

    /// Returns total bytes written.
    #[must_use]
    pub const fn total_bytes_written(&self) -> usize {
        self.total_bytes_written
    }
}

/// Applies the component scaffold after a conflict-free dry-run evaluation.
///
/// This is the guarded write path for `monad add <kind> <name> --yes`.
/// It refuses to overwrite existing files and performs no Git operations.
pub fn apply_add_plan(
    context: &WorkspaceContext,
    options: &AddPlanOptions,
) -> MonadResult<AddApplyResult> {
    if let Some(language) = options.language() {
        return Err(MonadError::invalid_input(format!(
            "language-aware add writes for `{}` are deferred until E13 language templates are implemented; use --dry-run to preview the plan",
            language.as_str()
        )));
    }

    let plan = build_add_plan(options);
    let dry_run = evaluate_file_operation_plan(context.root(), &plan);

    if dry_run.has_conflicts() {
        return Err(MonadError::invalid_input(
            "add plan has conflicts; run `monad add <kind> <name> --dry-run` and resolve existing target paths before using --yes",
        ));
    }

    let mut created_files = Vec::new();
    let mut total_bytes_written = 0usize;

    for template in component_scaffold_templates() {
        let relative_path = options.component_root().join(template.relative_path());
        let target_path = context.root().join(&relative_path);

        if target_path.exists() {
            return Err(MonadError::invalid_input(format!(
                "add target `{}` already exists; refusing to overwrite",
                relative_path.display()
            )));
        }

        if let Some(parent) = target_path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).map_err(|error| {
                MonadError::internal(format!(
                    "failed to create parent directory `{}`: {error}",
                    parent.display()
                ))
            })?;
        }

        let content = template.render_content(options);
        fs::write(&target_path, content.as_bytes()).map_err(|error| {
            MonadError::internal(format!(
                "failed to write add target `{}`: {error}",
                relative_path.display()
            ))
        })?;

        total_bytes_written += content.len();
        created_files.push(relative_path);
    }

    Ok(AddApplyResult::new(
        options.kind(),
        options.name().clone(),
        options.component_root(),
        created_files,
        total_bytes_written,
    ))
}

/// Renders the guarded component add apply result.
#[must_use]
pub fn render_add_apply_result(result: &AddApplyResult) -> String {
    let mut lines = vec![
        "Monad add applied".to_string(),
        format!("  kind: {}", result.kind().as_str()),
        format!("  name: {}", result.name().as_str()),
        format!("  root: {}", result.component_root().display()),
        format!("  files_created: {}", result.file_count()),
        format!("  bytes_written: {}", result.total_bytes_written()),
        "  created_files:".to_string(),
    ];

    for path in result.created_files() {
        lines.push(format!("    - {}", path.display()));
    }

    lines.push("No Git commands were run.".to_string());

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{DryRunOperationKind, MonadError};

    use super::*;

    fn unique_temp_root(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);

        std::env::temp_dir().join(format!("monad-add-{name}-{unique}"))
    }

    #[test]
    fn component_kind_parses_supported_values() -> MonadResult<()> {
        assert_eq!(ComponentKind::parse("app")?, ComponentKind::App);
        assert_eq!(ComponentKind::parse("package")?, ComponentKind::Package);
        assert_eq!(ComponentKind::parse("service")?, ComponentKind::Service);
        assert_eq!(ComponentKind::parse("tool")?, ComponentKind::Tool);

        Ok(())
    }

    #[test]
    fn component_kind_rejects_unknown_values() {
        let error = ComponentKind::parse("website").expect_err("unknown kind should fail");

        assert!(error.to_string().contains("unsupported component kind"));
    }

    #[test]
    fn component_language_parses_supported_values() -> MonadResult<()> {
        assert_eq!(ComponentLanguage::parse("rust")?, ComponentLanguage::Rust);
        assert_eq!(
            ComponentLanguage::parse("typescript")?,
            ComponentLanguage::Typescript
        );
        assert_eq!(
            ComponentLanguage::parse("python")?,
            ComponentLanguage::Python
        );
        assert_eq!(ComponentLanguage::parse("go")?, ComponentLanguage::Go);

        Ok(())
    }

    #[test]
    fn component_language_rejects_unknown_values() {
        let error = ComponentLanguage::parse("ruby").expect_err("unknown language should fail");

        assert!(error.to_string().contains("unsupported component language"));
        assert!(error.to_string().contains("rust"));
        assert!(error.to_string().contains("typescript"));
        assert!(error.to_string().contains("python"));
        assert!(error.to_string().contains("go"));
    }

    #[test]
    fn add_plan_options_can_carry_language() -> MonadResult<()> {
        let options = AddPlanOptions::new(ComponentKind::Service, ComponentName::parse("api")?)
            .with_language(Some(ComponentLanguage::Rust));

        assert_eq!(options.language(), Some(ComponentLanguage::Rust));
        assert_eq!(options.language_label(), "rust");
        assert_eq!(
            options.template_source_label(),
            "embedded language-aware scaffold model"
        );

        Ok(())
    }

    #[test]
    fn language_aware_dry_run_output_lists_language() -> MonadResult<()> {
        let root = unique_temp_root("language-render");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let options = AddPlanOptions::new(ComponentKind::Service, ComponentName::parse("api")?)
            .with_language(Some(ComponentLanguage::Rust));
        let output = render_add_dry_run(&context, &options)?;

        assert!(output.contains("kind: service"));
        assert!(output.contains("name: api"));
        assert!(output.contains("language: rust"));
        assert!(output.contains("embedded language-aware scaffold model"));
        assert!(output.contains("services/api/README.md"));
        assert!(output.contains("No files were written."));

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn apply_add_plan_rejects_language_aware_write_until_templates_exist() -> MonadResult<()> {
        let root = unique_temp_root("language-apply");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let options = AddPlanOptions::new(ComponentKind::Service, ComponentName::parse("api")?)
            .with_language(Some(ComponentLanguage::Rust));

        let error = apply_add_plan(&context, &options)
            .expect_err("language-aware writes should be deferred in WP-E13-002");

        assert!(error.to_string().contains("language-aware add writes"));
        assert!(!root.join("services/api/README.md").exists());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn component_name_accepts_safe_names() -> MonadResult<()> {
        assert_eq!(ComponentName::parse("web")?.as_str(), "web");
        assert_eq!(ComponentName::parse("shared-ui")?.as_str(), "shared-ui");
        assert_eq!(ComponentName::parse("worker-1")?.as_str(), "worker-1");

        Ok(())
    }

    #[test]
    fn component_name_rejects_unsafe_names() {
        for value in [
            "", "../web", "apps/web", "Web", "web app", "web;", "-web", "web-",
        ] {
            assert!(
                ComponentName::parse(value).is_err(),
                "expected `{value}` to be rejected"
            );
        }
    }

    #[test]
    fn component_templates_are_embedded_and_stable() {
        let templates = component_scaffold_templates();

        assert_eq!(templates.len(), 2);
        assert_eq!(templates[0].id(), "component.readme");
        assert_eq!(templates[0].relative_path(), "README.md");
        assert_eq!(templates[1].id(), "component.gitkeep");
        assert_eq!(templates[1].relative_path(), ".gitkeep");
    }

    #[test]
    fn component_readme_template_renders_component_values() -> MonadResult<()> {
        let options = AddPlanOptions::new(ComponentKind::Service, ComponentName::parse("api")?);
        let readme = component_scaffold_templates()[0].render_content(&options);

        assert!(readme.contains("# api"));
        assert!(readme.contains("Kind: `service`"));
        assert!(readme.contains("Root: `services/api`"));

        Ok(())
    }

    #[test]
    fn add_plan_targets_kind_specific_root_from_templates() -> MonadResult<()> {
        let options = AddPlanOptions::new(ComponentKind::Service, ComponentName::parse("api")?);
        let plan = build_add_plan(&options);
        let targets = plan
            .operations()
            .iter()
            .map(|operation| operation.target().display_path())
            .collect::<Vec<_>>();

        assert_eq!(options.component_root(), PathBuf::from("services/api"));
        assert!(targets.contains(&"services/api/README.md".to_string()));
        assert!(targets.contains(&"services/api/.gitkeep".to_string()));

        assert!(
            plan.operations()
                .iter()
                .any(|operation| operation.explanation().contains("component.readme"))
        );

        Ok(())
    }

    #[test]
    fn add_dry_run_output_states_no_files_written() -> MonadResult<()> {
        let root = unique_temp_root("render");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let options = AddPlanOptions::new(ComponentKind::App, ComponentName::parse("web")?);
        let output = render_add_dry_run(&context, &options)?;

        assert!(output.contains("Monad add dry-run plan"));
        assert!(output.contains("kind: app"));
        assert!(output.contains("name: web"));
        assert!(output.contains("Template source:"));
        assert!(output.contains("component.readme"));
        assert!(output.contains("apps/web/README.md"));
        assert!(output.contains("No files were written."));

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn add_dry_run_detects_existing_readme_conflict() -> MonadResult<()> {
        let root = unique_temp_root("conflict");
        let component_root = root.join("apps/web");
        fs::create_dir_all(&component_root).map_err(|error| {
            MonadError::internal(format!("test component root should be created: {error}"))
        })?;
        fs::write(component_root.join("README.md"), "# Existing\n").map_err(|error| {
            MonadError::internal(format!("test README should be written: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let options = AddPlanOptions::new(ComponentKind::App, ComponentName::parse("web")?);
        let plan = build_add_plan(&options);
        let dry_run = evaluate_file_operation_plan(context.root(), &plan);

        assert!(
            dry_run
                .operations()
                .iter()
                .any(
                    |operation| operation.target().display_path() == "apps/web/README.md"
                        && operation.outcome_kind() == DryRunOperationKind::Conflict
                )
        );

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn apply_add_plan_writes_component_scaffold() -> MonadResult<()> {
        let root = unique_temp_root("apply");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let options = AddPlanOptions::new(ComponentKind::App, ComponentName::parse("web")?);
        let result = apply_add_plan(&context, &options)?;

        assert_eq!(result.kind(), ComponentKind::App);
        assert_eq!(result.name().as_str(), "web");
        assert_eq!(result.component_root(), Path::new("apps/web"));
        assert_eq!(result.file_count(), 2);
        assert!(root.join("apps/web/README.md").is_file());
        assert!(root.join("apps/web/.gitkeep").is_file());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn apply_add_plan_refuses_existing_component_file() -> MonadResult<()> {
        let root = unique_temp_root("apply-conflict");
        fs::create_dir_all(root.join("apps/web")).map_err(|error| {
            MonadError::internal(format!("test component root should be created: {error}"))
        })?;
        fs::write(root.join("apps/web/README.md"), "# Existing\n").map_err(|error| {
            MonadError::internal(format!("test README should be written: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let options = AddPlanOptions::new(ComponentKind::App, ComponentName::parse("web")?);
        let error = apply_add_plan(&context, &options)
            .expect_err("existing component file should block add apply");

        assert!(error.to_string().contains("add plan has conflicts"));
        assert!(!root.join("apps/web/.gitkeep").exists());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn render_add_apply_result_lists_created_files() -> MonadResult<()> {
        let result = AddApplyResult::new(
            ComponentKind::Tool,
            ComponentName::parse("repo-lint")?,
            PathBuf::from("tools/repo-lint"),
            vec![
                PathBuf::from("tools/repo-lint/README.md"),
                PathBuf::from("tools/repo-lint/.gitkeep"),
            ],
            99,
        );

        let rendered = render_add_apply_result(&result);

        assert!(rendered.contains("Monad add applied"));
        assert!(rendered.contains("kind: tool"));
        assert!(rendered.contains("name: repo-lint"));
        assert!(rendered.contains("files_created: 2"));
        assert!(rendered.contains("No Git commands were run."));

        Ok(())
    }
}
