//! Component add planning.
//!
//! This module backs `monad add <kind> <name> --dry-run`.
//!
//! The important design idea is separation of responsibility:
//!
//! - `ComponentKind` knows the allowed component families.
//! - `ComponentName` validates a safe filesystem component name.
//! - `AddPlanOptions` carries the user's requested component.
//! - `build_add_plan` creates a reviewable file-operation plan.
//! - `render_add_dry_run` evaluates and prints that plan.
//!
//! WP-E12-002 is dry-run only. It does not write component files.

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

/// Options used to build a component add plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddPlanOptions {
    kind: ComponentKind,
    name: ComponentName,
}

impl AddPlanOptions {
    /// Creates add plan options.
    #[must_use]
    pub fn new(kind: ComponentKind, name: ComponentName) -> Self {
        Self { kind, name }
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
    let root = options.component_root();
    let readme_path = root.join("README.md");
    let gitkeep_path = root.join(".gitkeep");

    FileOperationPlan::from_operations(vec![
        PlannedFileOperation::create(
            readme_path.clone(),
            format!(
                "create `{}` as the component documentation entry point",
                readme_path.display()
            ),
        ),
        PlannedFileOperation::create(
            gitkeep_path.clone(),
            format!(
                "create `{}` so the component directory is represented before language-specific files exist",
                gitkeep_path.display()
            ),
        ),
    ])
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
        format!("  root: {}", root.display()),
        String::new(),
        "Safety:".to_string(),
        "  mode: dry-run".to_string(),
        "  writes: disabled".to_string(),
        "  apply: not implemented in WP-E12-002".to_string(),
        "  approval_flag: --yes reserved for a later E12 work packet".to_string(),
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
    fn add_plan_targets_kind_specific_root() -> MonadResult<()> {
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
}
