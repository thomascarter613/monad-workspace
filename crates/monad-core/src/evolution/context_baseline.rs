//! Context baseline evolution workflow.
//!
//! This module plans a minimal repo-native context bridge baseline. The first
//! version is dry-run only and does not write files.
//!
//! The goal is to demonstrate Monad's context philosophy in other repositories:
//! AI-readable state, handoff readiness, and durable repo-resident context.

use crate::{
    FileOperationPlan, MonadError, MonadResult, PlannedFileOperation, WorkspaceContext,
    evaluate_file_operation_plan, initial_template_registry, render_dry_run_plan,
};

const CONTEXT_BASELINE_TEMPLATE_ID: &str = "context-baseline.readme";

/// Builds the initial context baseline file operation plan.
///
/// This function creates planned operations only. It does not inspect the
/// filesystem, write files, call AI services, or generate summaries.
pub fn build_context_baseline_plan() -> MonadResult<FileOperationPlan> {
    let registry = initial_template_registry()?;
    let readme_template = registry
        .get_by_str(CONTEXT_BASELINE_TEMPLATE_ID)
        .ok_or_else(|| {
            MonadError::not_found(format!(
                "context baseline template `{CONTEXT_BASELINE_TEMPLATE_ID}` was not found"
            ))
        })?;

    Ok(FileOperationPlan::from_operations(vec![
        PlannedFileOperation::create(
            readme_template.metadata().target_path().to_path_buf(),
            format!(
                "create `{}` from embedded template `{}`",
                readme_template.metadata().target_path().display(),
                readme_template.id().as_str()
            ),
        ),
        PlannedFileOperation::create(
            ".monad/context/current-state.md",
            "create current-state context placeholder for future handoff workflows",
        ),
        PlannedFileOperation::create(
            ".monad/context/latest-handoff.md",
            "create latest-handoff context placeholder for future session transfer workflows",
        ),
    ]))
}

/// Renders the dry-run output for the context baseline workflow.
pub fn render_context_baseline_dry_run(context: &WorkspaceContext) -> MonadResult<String> {
    let plan = build_context_baseline_plan()?;
    let dry_run = evaluate_file_operation_plan(context.root(), &plan);
    let mut output = render_dry_run_plan(&dry_run);

    output.push_str("\n\nMode: dry-run");
    output.push_str("\nNo files were written.");
    output.push_str("\nNo AI summarization was performed.");

    Ok(output)
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

        std::env::temp_dir().join(format!("monad-context-baseline-{name}-{unique}"))
    }

    #[test]
    fn context_baseline_plan_contains_core_context_targets() -> MonadResult<()> {
        let plan = build_context_baseline_plan()?;
        let targets = plan
            .operations()
            .iter()
            .map(|operation| operation.target().display_path())
            .collect::<Vec<_>>();

        assert_eq!(
            targets,
            vec![
                "docs/ai/README.md".to_string(),
                ".monad/context/current-state.md".to_string(),
                ".monad/context/latest-handoff.md".to_string(),
            ]
        );

        Ok(())
    }

    #[test]
    fn context_baseline_dry_run_previews_creates_when_targets_are_missing() -> MonadResult<()> {
        let root = unique_temp_root("missing-targets");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let plan = build_context_baseline_plan()?;
        let dry_run = evaluate_file_operation_plan(context.root(), &plan);

        assert_eq!(dry_run.len(), 3);
        assert!(
            dry_run
                .operations()
                .iter()
                .all(|operation| { operation.outcome_kind() == DryRunOperationKind::WouldCreate })
        );
        assert!(dry_run.appears_safe_to_apply());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn context_baseline_dry_run_detects_existing_file_conflicts() -> MonadResult<()> {
        let root = unique_temp_root("existing-targets");
        fs::create_dir_all(root.join(".monad/context")).map_err(|error| {
            MonadError::internal(format!("test context dir should be created: {error}"))
        })?;
        fs::write(
            root.join(".monad/context/current-state.md"),
            "# Existing State\n",
        )
        .map_err(|error| {
            MonadError::internal(format!("test current-state should be written: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let plan = build_context_baseline_plan()?;
        let dry_run = evaluate_file_operation_plan(context.root(), &plan);

        assert!(dry_run.operations().iter().any(|operation| {
            operation.target().display_path() == ".monad/context/current-state.md"
                && operation.outcome_kind() == DryRunOperationKind::Conflict
        }));
        assert!(dry_run.has_conflicts());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn context_baseline_dry_run_output_states_no_files_or_ai_work() -> MonadResult<()> {
        let root = unique_temp_root("render");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let output = render_context_baseline_dry_run(&context)?;

        assert!(output.contains("Monad dry-run file operation plan"));
        assert!(output.contains("docs/ai/README.md"));
        assert!(output.contains(".monad/context/current-state.md"));
        assert!(output.contains(".monad/context/latest-handoff.md"));
        assert!(output.contains("Mode: dry-run"));
        assert!(output.contains("No files were written."));
        assert!(output.contains("No AI summarization was performed."));

        fs::remove_dir_all(&root).ok();

        Ok(())
    }
}
