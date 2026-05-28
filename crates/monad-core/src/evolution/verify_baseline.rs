//! Verification baseline evolution workflow.
//!
//! This module creates a reviewable file operation plan for a minimal
//! verification baseline. The first implementation is intentionally small:
//! it uses the embedded template registry and dry-run planning, but it does
//! not write files.

use crate::{
    FileOperationPlan, MonadError, MonadResult, PlannedFileOperation, WorkspaceContext,
    evaluate_file_operation_plan, initial_template_registry, render_dry_run_plan,
};

const VERIFY_BASELINE_TEMPLATE_ID: &str = "verify-baseline.readme";

/// Builds the initial verification baseline file operation plan.
///
/// This function produces planned operations only. It does not evaluate the
/// filesystem and it does not write files.
pub fn build_verify_baseline_plan() -> MonadResult<FileOperationPlan> {
    let registry = initial_template_registry()?;
    let template = registry
        .get_by_str(VERIFY_BASELINE_TEMPLATE_ID)
        .ok_or_else(|| {
            MonadError::not_found(format!(
                "verification baseline template `{VERIFY_BASELINE_TEMPLATE_ID}` was not found"
            ))
        })?;

    Ok(FileOperationPlan::from_operations(vec![
        PlannedFileOperation::create(
            template.metadata().target_path().to_path_buf(),
            format!(
                "create `{}` from embedded template `{}`",
                template.metadata().target_path().display(),
                template.id().as_str()
            ),
        ),
    ]))
}

/// Renders the dry-run output for the verification baseline workflow.
///
/// This is the core behavior used by the CLI. Keeping it in `monad-core` keeps
/// the CLI thin and makes the workflow testable without spawning a process.
pub fn render_verify_baseline_dry_run(context: &WorkspaceContext) -> MonadResult<String> {
    let plan = build_verify_baseline_plan()?;
    let dry_run = evaluate_file_operation_plan(context.root(), &plan);
    let mut output = render_dry_run_plan(&dry_run);

    output.push_str("\n\nMode: dry-run");
    output.push_str("\nNo files were written.");

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

        std::env::temp_dir().join(format!("monad-verify-baseline-{name}-{unique}"))
    }

    #[test]
    fn verify_baseline_plan_uses_embedded_template_target() -> MonadResult<()> {
        let plan = build_verify_baseline_plan()?;

        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan.operations()[0].target().display_path(),
            "docs/verification/README.md"
        );
        assert!(
            plan.operations()[0]
                .explanation()
                .contains("verify-baseline.readme")
        );

        Ok(())
    }

    #[test]
    fn verify_baseline_dry_run_previews_create_when_target_is_missing() -> MonadResult<()> {
        let root = unique_temp_root("missing-target");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let plan = build_verify_baseline_plan()?;
        let dry_run = evaluate_file_operation_plan(context.root(), &plan);

        assert_eq!(
            dry_run.operations()[0].outcome_kind(),
            DryRunOperationKind::WouldCreate
        );
        assert!(dry_run.appears_safe_to_apply());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn verify_baseline_dry_run_detects_existing_file_conflict() -> MonadResult<()> {
        let root = unique_temp_root("existing-target");
        fs::create_dir_all(root.join("docs/verification")).map_err(|error| {
            MonadError::internal(format!("test verification dir should be created: {error}"))
        })?;
        fs::write(
            root.join("docs/verification/README.md"),
            "# Existing Verification\n",
        )
        .map_err(|error| {
            MonadError::internal(format!(
                "test verification README should be written: {error}"
            ))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let plan = build_verify_baseline_plan()?;
        let dry_run = evaluate_file_operation_plan(context.root(), &plan);

        assert_eq!(
            dry_run.operations()[0].outcome_kind(),
            DryRunOperationKind::Conflict
        );
        assert!(dry_run.has_conflicts());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn verify_baseline_dry_run_output_states_no_files_written() -> MonadResult<()> {
        let root = unique_temp_root("render");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let context = WorkspaceContext::new(&root)?;
        let output = render_verify_baseline_dry_run(&context)?;

        assert!(output.contains("Monad dry-run file operation plan"));
        assert!(output.contains("docs/verification/README.md"));
        assert!(output.contains("Mode: dry-run"));
        assert!(output.contains("No files were written."));

        fs::remove_dir_all(&root).ok();

        Ok(())
    }
}
