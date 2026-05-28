//! File operation plan and summary model.

use crate::{FileOperationKind, PlannedFileOperation};

/// Reviewable plan containing proposed file operations.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileOperationPlan {
    operations: Vec<PlannedFileOperation>,
}

impl FileOperationPlan {
    /// Creates an empty file operation plan.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a plan from operations.
    #[must_use]
    pub fn from_operations(operations: Vec<PlannedFileOperation>) -> Self {
        Self { operations }
    }

    /// Adds one operation to the plan.
    pub fn push(&mut self, operation: PlannedFileOperation) {
        self.operations.push(operation);
    }

    /// Returns planned operations in insertion order.
    #[must_use]
    pub fn operations(&self) -> &[PlannedFileOperation] {
        &self.operations
    }

    /// Returns the number of planned operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns true when the plan is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns true when the plan has blocking conflicts.
    #[must_use]
    pub fn has_blocking_operations(&self) -> bool {
        self.operations
            .iter()
            .any(PlannedFileOperation::is_blocking)
    }

    /// Returns true when the plan contains destructive operations.
    #[must_use]
    pub fn has_destructive_operations(&self) -> bool {
        self.operations
            .iter()
            .any(PlannedFileOperation::is_destructive)
    }

    /// Builds a summary for this plan.
    #[must_use]
    pub fn summary(&self) -> FileOperationSummary {
        FileOperationSummary::from_operations(&self.operations)
    }
}

/// Deterministic count summary for a file operation plan.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FileOperationSummary {
    total: usize,
    create_count: usize,
    update_count: usize,
    delete_count: usize,
    skip_count: usize,
    conflict_count: usize,
    no_op_count: usize,
}

impl FileOperationSummary {
    /// Builds a summary from planned operations.
    #[must_use]
    pub fn from_operations(operations: &[PlannedFileOperation]) -> Self {
        let mut summary = Self::default();

        for operation in operations {
            summary.total += 1;

            match operation.kind() {
                FileOperationKind::Create => summary.create_count += 1,
                FileOperationKind::Update => summary.update_count += 1,
                FileOperationKind::Delete => summary.delete_count += 1,
                FileOperationKind::Skip => summary.skip_count += 1,
                FileOperationKind::Conflict => summary.conflict_count += 1,
                FileOperationKind::NoOp => summary.no_op_count += 1,
            }
        }

        summary
    }

    /// Returns total operations.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.total
    }

    /// Returns planned create count.
    #[must_use]
    pub const fn create_count(&self) -> usize {
        self.create_count
    }

    /// Returns planned update count.
    #[must_use]
    pub const fn update_count(&self) -> usize {
        self.update_count
    }

    /// Returns planned delete count.
    #[must_use]
    pub const fn delete_count(&self) -> usize {
        self.delete_count
    }

    /// Returns skipped operation count.
    #[must_use]
    pub const fn skip_count(&self) -> usize {
        self.skip_count
    }

    /// Returns conflict count.
    #[must_use]
    pub const fn conflict_count(&self) -> usize {
        self.conflict_count
    }

    /// Returns no-op count.
    #[must_use]
    pub const fn no_op_count(&self) -> usize {
        self.no_op_count
    }

    /// Returns true when conflicts are present.
    #[must_use]
    pub const fn has_conflicts(&self) -> bool {
        self.conflict_count > 0
    }

    /// Returns true when destructive operations are present.
    #[must_use]
    pub const fn has_deletes(&self) -> bool {
        self.delete_count > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_starts_empty() {
        let plan = FileOperationPlan::new();

        assert!(plan.is_empty());
        assert_eq!(plan.len(), 0);
        assert_eq!(plan.summary().total(), 0);
    }

    #[test]
    fn plan_preserves_operation_order() {
        let plan = FileOperationPlan::from_operations(vec![
            PlannedFileOperation::create("a.txt", "create a"),
            PlannedFileOperation::update("b.txt", "update b"),
            PlannedFileOperation::skip("c.txt", "skip c"),
        ]);

        let paths = plan
            .operations()
            .iter()
            .map(|operation| operation.target().display_path())
            .collect::<Vec<_>>();

        assert_eq!(
            paths,
            vec![
                "a.txt".to_string(),
                "b.txt".to_string(),
                "c.txt".to_string()
            ]
        );
    }

    #[test]
    fn plan_summary_counts_operation_kinds() {
        let plan = FileOperationPlan::from_operations(vec![
            PlannedFileOperation::create("create.txt", "create file"),
            PlannedFileOperation::update("update.txt", "update file"),
            PlannedFileOperation::delete("delete.txt", "delete file", true),
            PlannedFileOperation::skip("skip.txt", "skip file"),
            PlannedFileOperation::conflict("conflict.txt", "conflict file"),
            PlannedFileOperation::no_op("noop.txt", "nothing to do"),
        ]);

        let summary = plan.summary();

        assert_eq!(summary.total(), 6);
        assert_eq!(summary.create_count(), 1);
        assert_eq!(summary.update_count(), 1);
        assert_eq!(summary.delete_count(), 1);
        assert_eq!(summary.skip_count(), 1);
        assert_eq!(summary.conflict_count(), 1);
        assert_eq!(summary.no_op_count(), 1);
        assert!(summary.has_conflicts());
        assert!(summary.has_deletes());
        assert!(plan.has_blocking_operations());
        assert!(plan.has_destructive_operations());
    }

    #[test]
    fn plan_can_be_built_incrementally() {
        let mut plan = FileOperationPlan::new();

        plan.push(PlannedFileOperation::create(
            "docs/README.md",
            "create docs README",
        ));

        assert_eq!(plan.len(), 1);
        assert_eq!(plan.summary().create_count(), 1);
    }
}
