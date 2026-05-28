//! Dry-run evaluation for planned file operations.
//!
//! Dry-run evaluation answers: "what would happen if this plan were applied?"
//! It does not write files, delete files, create directories, generate patches,
//! or apply templates.

use std::path::Path;

use crate::{FileOperationKind, FileOperationPlan, FileOperationTarget, PlannedFileOperation};

/// Dry-run outcome for one planned file operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DryRunOperationKind {
    /// The file would be created.
    WouldCreate,

    /// The file would be updated.
    WouldUpdate,

    /// The file would be deleted.
    WouldDelete,

    /// The operation would be skipped.
    WouldSkip,

    /// The operation would do nothing.
    WouldNoOp,

    /// The operation has a conflict and should not be applied.
    Conflict,
}

impl DryRunOperationKind {
    /// Returns a stable dry-run outcome label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WouldCreate => "would_create",
            Self::WouldUpdate => "would_update",
            Self::WouldDelete => "would_delete",
            Self::WouldSkip => "would_skip",
            Self::WouldNoOp => "would_no_op",
            Self::Conflict => "conflict",
        }
    }

    /// Returns true when this outcome blocks safe apply behavior.
    #[must_use]
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Conflict)
    }

    /// Returns true when this outcome would mutate the filesystem.
    #[must_use]
    pub const fn would_mutate(self) -> bool {
        matches!(
            self,
            Self::WouldCreate | Self::WouldUpdate | Self::WouldDelete
        )
    }
}

/// Evaluated dry-run operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DryRunFileOperation {
    requested_kind: FileOperationKind,
    outcome_kind: DryRunOperationKind,
    target: FileOperationTarget,
    message: String,
}

impl DryRunFileOperation {
    /// Creates a dry-run operation result.
    #[must_use]
    pub fn new(
        requested_kind: FileOperationKind,
        outcome_kind: DryRunOperationKind,
        target: FileOperationTarget,
        message: impl Into<String>,
    ) -> Self {
        Self {
            requested_kind,
            outcome_kind,
            target,
            message: message.into(),
        }
    }

    /// Returns the originally requested operation kind.
    #[must_use]
    pub const fn requested_kind(&self) -> FileOperationKind {
        self.requested_kind
    }

    /// Returns the dry-run outcome kind.
    #[must_use]
    pub const fn outcome_kind(&self) -> DryRunOperationKind {
        self.outcome_kind
    }

    /// Returns the target path.
    #[must_use]
    pub const fn target(&self) -> &FileOperationTarget {
        &self.target
    }

    /// Returns the human-readable dry-run message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns true when this operation blocks safe apply behavior.
    #[must_use]
    pub const fn is_blocking(&self) -> bool {
        self.outcome_kind.is_blocking()
    }

    /// Returns true when this operation would mutate the filesystem.
    #[must_use]
    pub const fn would_mutate(&self) -> bool {
        self.outcome_kind.would_mutate()
    }
}

/// Dry-run result for a full file operation plan.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DryRunPlan {
    operations: Vec<DryRunFileOperation>,
}

impl DryRunPlan {
    /// Creates a dry-run plan from evaluated operations.
    #[must_use]
    pub fn new(operations: Vec<DryRunFileOperation>) -> Self {
        Self { operations }
    }

    /// Returns dry-run operations in deterministic plan order.
    #[must_use]
    pub fn operations(&self) -> &[DryRunFileOperation] {
        &self.operations
    }

    /// Returns operation count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns true when there are no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns true when any operation blocks safe apply behavior.
    #[must_use]
    pub fn has_conflicts(&self) -> bool {
        self.operations.iter().any(DryRunFileOperation::is_blocking)
    }

    /// Returns true when the plan appears safe to apply.
    ///
    /// "Safe" here means no conflicts were detected. This does not mean the
    /// operation has user approval, branch safety, worktree safety, or write
    /// behavior yet; those arrive in later E5 slices.
    #[must_use]
    pub fn appears_safe_to_apply(&self) -> bool {
        !self.has_conflicts()
    }

    /// Returns true when the plan would mutate the filesystem if applied.
    #[must_use]
    pub fn would_mutate(&self) -> bool {
        self.operations
            .iter()
            .any(DryRunFileOperation::would_mutate)
    }

    /// Builds a deterministic summary.
    #[must_use]
    pub fn summary(&self) -> DryRunSummary {
        DryRunSummary::from_operations(&self.operations)
    }
}

/// Summary counts for a dry-run plan.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DryRunSummary {
    total: usize,
    would_create_count: usize,
    would_update_count: usize,
    would_delete_count: usize,
    would_skip_count: usize,
    would_no_op_count: usize,
    conflict_count: usize,
}

impl DryRunSummary {
    /// Builds a summary from dry-run operations.
    #[must_use]
    pub fn from_operations(operations: &[DryRunFileOperation]) -> Self {
        let mut summary = Self::default();

        for operation in operations {
            summary.total += 1;

            match operation.outcome_kind() {
                DryRunOperationKind::WouldCreate => summary.would_create_count += 1,
                DryRunOperationKind::WouldUpdate => summary.would_update_count += 1,
                DryRunOperationKind::WouldDelete => summary.would_delete_count += 1,
                DryRunOperationKind::WouldSkip => summary.would_skip_count += 1,
                DryRunOperationKind::WouldNoOp => summary.would_no_op_count += 1,
                DryRunOperationKind::Conflict => summary.conflict_count += 1,
            }
        }

        summary
    }

    /// Returns total operations.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.total
    }

    /// Returns would-create count.
    #[must_use]
    pub const fn would_create_count(&self) -> usize {
        self.would_create_count
    }

    /// Returns would-update count.
    #[must_use]
    pub const fn would_update_count(&self) -> usize {
        self.would_update_count
    }

    /// Returns would-delete count.
    #[must_use]
    pub const fn would_delete_count(&self) -> usize {
        self.would_delete_count
    }

    /// Returns would-skip count.
    #[must_use]
    pub const fn would_skip_count(&self) -> usize {
        self.would_skip_count
    }

    /// Returns would-no-op count.
    #[must_use]
    pub const fn would_no_op_count(&self) -> usize {
        self.would_no_op_count
    }

    /// Returns conflict count.
    #[must_use]
    pub const fn conflict_count(&self) -> usize {
        self.conflict_count
    }

    /// Returns true when conflicts are present.
    #[must_use]
    pub const fn has_conflicts(&self) -> bool {
        self.conflict_count > 0
    }
}

/// Evaluates a file operation plan against a filesystem root without writing.
#[must_use]
pub fn evaluate_file_operation_plan(
    root: impl AsRef<Path>,
    plan: &FileOperationPlan,
) -> DryRunPlan {
    let root = root.as_ref();

    let operations = plan
        .operations()
        .iter()
        .map(|operation| evaluate_operation(root, operation))
        .collect();

    DryRunPlan::new(operations)
}

fn evaluate_operation(root: &Path, operation: &PlannedFileOperation) -> DryRunFileOperation {
    let target = operation.target().clone();
    let full_path = root.join(target.path());

    match operation {
        PlannedFileOperation::Create { .. } if full_path.exists() => DryRunFileOperation::new(
            operation.kind(),
            DryRunOperationKind::Conflict,
            target,
            "create target already exists; applying would overwrite an existing path",
        ),
        PlannedFileOperation::Create { .. } => DryRunFileOperation::new(
            operation.kind(),
            DryRunOperationKind::WouldCreate,
            target,
            operation.explanation(),
        ),
        PlannedFileOperation::Update { .. } if full_path.is_file() => DryRunFileOperation::new(
            operation.kind(),
            DryRunOperationKind::WouldUpdate,
            target,
            operation.explanation(),
        ),
        PlannedFileOperation::Update { .. } if full_path.exists() => DryRunFileOperation::new(
            operation.kind(),
            DryRunOperationKind::Conflict,
            target,
            "update target exists but is not a regular file",
        ),
        PlannedFileOperation::Update { .. } => DryRunFileOperation::new(
            operation.kind(),
            DryRunOperationKind::Conflict,
            target,
            "update target does not exist",
        ),
        PlannedFileOperation::Delete { .. } if full_path.exists() => DryRunFileOperation::new(
            operation.kind(),
            DryRunOperationKind::WouldDelete,
            target,
            operation.explanation(),
        ),
        PlannedFileOperation::Delete { .. } => DryRunFileOperation::new(
            operation.kind(),
            DryRunOperationKind::WouldSkip,
            target,
            "delete target does not exist; nothing would be deleted",
        ),
        PlannedFileOperation::Skip { .. } => DryRunFileOperation::new(
            operation.kind(),
            DryRunOperationKind::WouldSkip,
            target,
            operation.explanation(),
        ),
        PlannedFileOperation::Conflict { .. } => DryRunFileOperation::new(
            operation.kind(),
            DryRunOperationKind::Conflict,
            target,
            operation.explanation(),
        ),
        PlannedFileOperation::NoOp { .. } => DryRunFileOperation::new(
            operation.kind(),
            DryRunOperationKind::WouldNoOp,
            target,
            operation.explanation(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{MonadError, MonadResult};

    use super::*;

    fn unique_temp_root(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);

        std::env::temp_dir().join(format!("monad-dry-run-{name}-{unique}"))
    }

    #[test]
    fn dry_run_previews_create_when_file_is_missing() -> MonadResult<()> {
        let root = unique_temp_root("create-missing");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let plan = FileOperationPlan::from_operations(vec![PlannedFileOperation::create(
            "new-file.txt",
            "create new file",
        )]);

        let dry_run = evaluate_file_operation_plan(&root, &plan);

        assert_eq!(dry_run.len(), 1);
        assert_eq!(
            dry_run.operations()[0].outcome_kind(),
            DryRunOperationKind::WouldCreate
        );
        assert!(dry_run.appears_safe_to_apply());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn dry_run_detects_create_conflict_when_file_exists() -> MonadResult<()> {
        let root = unique_temp_root("create-conflict");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;
        fs::write(root.join("existing.txt"), "already here\n").map_err(|error| {
            MonadError::internal(format!("test file should be written: {error}"))
        })?;

        let plan = FileOperationPlan::from_operations(vec![PlannedFileOperation::create(
            "existing.txt",
            "create existing file",
        )]);

        let dry_run = evaluate_file_operation_plan(&root, &plan);

        assert_eq!(
            dry_run.operations()[0].outcome_kind(),
            DryRunOperationKind::Conflict
        );
        assert!(dry_run.has_conflicts());
        assert!(!dry_run.appears_safe_to_apply());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn dry_run_previews_update_when_file_exists() -> MonadResult<()> {
        let root = unique_temp_root("update-existing");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;
        fs::write(root.join("README.md"), "old\n").map_err(|error| {
            MonadError::internal(format!("test file should be written: {error}"))
        })?;

        let plan = FileOperationPlan::from_operations(vec![PlannedFileOperation::update(
            "README.md",
            "refresh README",
        )]);

        let dry_run = evaluate_file_operation_plan(&root, &plan);

        assert_eq!(
            dry_run.operations()[0].outcome_kind(),
            DryRunOperationKind::WouldUpdate
        );
        assert!(dry_run.would_mutate());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn dry_run_detects_update_conflict_when_file_is_missing() -> MonadResult<()> {
        let root = unique_temp_root("update-missing");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;

        let plan = FileOperationPlan::from_operations(vec![PlannedFileOperation::update(
            "missing.md",
            "refresh missing file",
        )]);

        let dry_run = evaluate_file_operation_plan(&root, &plan);

        assert_eq!(
            dry_run.operations()[0].outcome_kind(),
            DryRunOperationKind::Conflict
        );
        assert!(dry_run.summary().has_conflicts());

        fs::remove_dir_all(&root).ok();

        Ok(())
    }

    #[test]
    fn dry_run_summary_counts_outcomes() -> MonadResult<()> {
        let root = unique_temp_root("summary");
        fs::create_dir_all(&root).map_err(|error| {
            MonadError::internal(format!("test root should be created: {error}"))
        })?;
        fs::write(root.join("update.txt"), "old\n").map_err(|error| {
            MonadError::internal(format!("test file should be written: {error}"))
        })?;

        let plan = FileOperationPlan::from_operations(vec![
            PlannedFileOperation::create("create.txt", "create"),
            PlannedFileOperation::update("update.txt", "update"),
            PlannedFileOperation::delete("missing-delete.txt", "delete missing", true),
            PlannedFileOperation::skip("skip.txt", "skip"),
            PlannedFileOperation::conflict("conflict.txt", "known conflict"),
            PlannedFileOperation::no_op("noop.txt", "nothing"),
        ]);

        let summary = evaluate_file_operation_plan(&root, &plan).summary();

        assert_eq!(summary.total(), 6);
        assert_eq!(summary.would_create_count(), 1);
        assert_eq!(summary.would_update_count(), 1);
        assert_eq!(summary.would_delete_count(), 0);
        assert_eq!(summary.would_skip_count(), 2);
        assert_eq!(summary.would_no_op_count(), 1);
        assert_eq!(summary.conflict_count(), 1);

        fs::remove_dir_all(&root).ok();

        Ok(())
    }
}
