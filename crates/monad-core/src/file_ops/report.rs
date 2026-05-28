//! Human-readable dry-run report rendering.

use crate::{DryRunOperationKind, DryRunPlan};

/// Renders a dry-run plan as a human-readable summary.
///
/// This is intentionally simple and deterministic. A fuller diff renderer can
/// be added later after the apply model exists.
#[must_use]
pub fn render_dry_run_plan(plan: &DryRunPlan) -> String {
    let summary = plan.summary();
    let mut lines = Vec::new();

    lines.push("Monad dry-run file operation plan".to_string());
    lines.push(String::new());
    lines.push("Summary:".to_string());
    lines.push(format!("- Operations: {}", summary.total()));
    lines.push(format!("- Would create: {}", summary.would_create_count()));
    lines.push(format!("- Would update: {}", summary.would_update_count()));
    lines.push(format!("- Would delete: {}", summary.would_delete_count()));
    lines.push(format!("- Would skip: {}", summary.would_skip_count()));
    lines.push(format!("- Would no-op: {}", summary.would_no_op_count()));
    lines.push(format!("- Conflicts: {}", summary.conflict_count()));
    lines.push(format!(
        "- Appears safe to apply: {}",
        plan.appears_safe_to_apply()
    ));
    lines.push(String::new());
    lines.push("Operations:".to_string());

    if plan.is_empty() {
        lines.push("- No file operations planned.".to_string());
    } else {
        for operation in plan.operations() {
            lines.push(format!(
                "- [{}] {}: {}",
                render_outcome_label(operation.outcome_kind()),
                operation.target().display_path(),
                operation.message()
            ));
        }
    }

    lines.join("\n")
}

fn render_outcome_label(kind: DryRunOperationKind) -> &'static str {
    match kind {
        DryRunOperationKind::WouldCreate => "CREATE",
        DryRunOperationKind::WouldUpdate => "UPDATE",
        DryRunOperationKind::WouldDelete => "DELETE",
        DryRunOperationKind::WouldSkip => "SKIP",
        DryRunOperationKind::WouldNoOp => "NO-OP",
        DryRunOperationKind::Conflict => "CONFLICT",
    }
}

#[cfg(test)]
mod tests {
    use crate::{DryRunFileOperation, DryRunOperationKind, FileOperationKind, FileOperationTarget};

    use super::*;

    #[test]
    fn dry_run_report_renders_empty_plan() {
        let rendered = render_dry_run_plan(&DryRunPlan::new(Vec::new()));

        assert!(rendered.contains("Monad dry-run file operation plan"));
        assert!(rendered.contains("- Operations: 0"));
        assert!(rendered.contains("- No file operations planned."));
    }

    #[test]
    fn dry_run_report_renders_operations_and_conflicts() {
        let plan = DryRunPlan::new(vec![
            DryRunFileOperation::new(
                FileOperationKind::Create,
                DryRunOperationKind::WouldCreate,
                FileOperationTarget::new("docs/README.md"),
                "create docs README",
            ),
            DryRunFileOperation::new(
                FileOperationKind::Create,
                DryRunOperationKind::Conflict,
                FileOperationTarget::new("Cargo.toml"),
                "create target already exists",
            ),
        ]);

        let rendered = render_dry_run_plan(&plan);

        assert!(rendered.contains("- Would create: 1"));
        assert!(rendered.contains("- Conflicts: 1"));
        assert!(rendered.contains("- Appears safe to apply: false"));
        assert!(rendered.contains("[CREATE] docs/README.md"));
        assert!(rendered.contains("[CONFLICT] Cargo.toml"));
    }
}
