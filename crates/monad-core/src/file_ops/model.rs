//! Safe file operation domain model.
//!
//! This file defines the vocabulary for proposed file operations. The model is
//! intentionally descriptive and reviewable: it explains what would happen
//! before any future command is allowed to write files.

use std::path::{Path, PathBuf};

/// Stable operation kind for a planned file operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileOperationKind {
    /// Create a file that does not already exist.
    Create,

    /// Update a file that already exists.
    Update,

    /// Delete a file.
    ///
    /// Delete is modeled early but should remain conservative. Future apply
    /// behavior must gate destructive actions carefully.
    Delete,

    /// Skip an operation because no safe write should occur.
    Skip,

    /// Represent an unsafe or ambiguous requested operation.
    Conflict,

    /// Represent a deliberate no-op.
    NoOp,
}

impl FileOperationKind {
    /// Returns a stable label for this operation kind.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Update => "update",
            Self::Delete => "delete",
            Self::Skip => "skip",
            Self::Conflict => "conflict",
            Self::NoOp => "no_op",
        }
    }

    /// Returns true when this operation would write file content.
    #[must_use]
    pub const fn writes_content(self) -> bool {
        matches!(self, Self::Create | Self::Update)
    }

    /// Returns true when this operation is destructive.
    #[must_use]
    pub const fn is_destructive(self) -> bool {
        matches!(self, Self::Delete)
    }

    /// Returns true when this operation prevents safe application.
    #[must_use]
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Conflict)
    }
}

/// Target path for a planned file operation.
///
/// This wrapper keeps paths as filesystem paths rather than plain strings while
/// still giving future renderers a stable display helper.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileOperationTarget {
    path: PathBuf,
}

impl FileOperationTarget {
    /// Creates a target from a repository-relative or caller-provided path.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Returns the target path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the path in display form.
    #[must_use]
    pub fn display_path(&self) -> String {
        self.path.display().to_string()
    }
}

/// Planned file operation.
///
/// This enum is the heart of the safe evolution model. It describes what Monad
/// proposes to do, but it does not apply anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlannedFileOperation {
    /// Create a file.
    Create {
        /// Target file path.
        target: FileOperationTarget,

        /// Human-readable content summary, not necessarily full content.
        content_summary: String,
    },

    /// Update a file.
    Update {
        /// Target file path.
        target: FileOperationTarget,

        /// Human-readable update summary.
        content_summary: String,
    },

    /// Delete a file.
    Delete {
        /// Target file path.
        target: FileOperationTarget,

        /// Human-readable deletion reason.
        reason: String,

        /// Whether the operation requires explicit approval.
        requires_approval: bool,
    },

    /// Skip a file operation.
    Skip {
        /// Target file path.
        target: FileOperationTarget,

        /// Human-readable skip reason.
        reason: String,
    },

    /// Represent a conflict.
    Conflict {
        /// Target file path.
        target: FileOperationTarget,

        /// Human-readable conflict reason.
        reason: String,
    },

    /// Represent a deliberate no-op.
    NoOp {
        /// Target file path.
        target: FileOperationTarget,

        /// Human-readable no-op reason.
        reason: String,
    },
}

impl PlannedFileOperation {
    /// Creates a planned create operation.
    #[must_use]
    pub fn create(target: impl Into<PathBuf>, content_summary: impl Into<String>) -> Self {
        Self::Create {
            target: FileOperationTarget::new(target),
            content_summary: content_summary.into(),
        }
    }

    /// Creates a planned update operation.
    #[must_use]
    pub fn update(target: impl Into<PathBuf>, content_summary: impl Into<String>) -> Self {
        Self::Update {
            target: FileOperationTarget::new(target),
            content_summary: content_summary.into(),
        }
    }

    /// Creates a planned delete operation.
    #[must_use]
    pub fn delete(
        target: impl Into<PathBuf>,
        reason: impl Into<String>,
        requires_approval: bool,
    ) -> Self {
        Self::Delete {
            target: FileOperationTarget::new(target),
            reason: reason.into(),
            requires_approval,
        }
    }

    /// Creates a planned skip operation.
    #[must_use]
    pub fn skip(target: impl Into<PathBuf>, reason: impl Into<String>) -> Self {
        Self::Skip {
            target: FileOperationTarget::new(target),
            reason: reason.into(),
        }
    }

    /// Creates a planned conflict operation.
    #[must_use]
    pub fn conflict(target: impl Into<PathBuf>, reason: impl Into<String>) -> Self {
        Self::Conflict {
            target: FileOperationTarget::new(target),
            reason: reason.into(),
        }
    }

    /// Creates a planned no-op operation.
    #[must_use]
    pub fn no_op(target: impl Into<PathBuf>, reason: impl Into<String>) -> Self {
        Self::NoOp {
            target: FileOperationTarget::new(target),
            reason: reason.into(),
        }
    }

    /// Returns this operation's kind.
    #[must_use]
    pub const fn kind(&self) -> FileOperationKind {
        match self {
            Self::Create { .. } => FileOperationKind::Create,
            Self::Update { .. } => FileOperationKind::Update,
            Self::Delete { .. } => FileOperationKind::Delete,
            Self::Skip { .. } => FileOperationKind::Skip,
            Self::Conflict { .. } => FileOperationKind::Conflict,
            Self::NoOp { .. } => FileOperationKind::NoOp,
        }
    }

    /// Returns this operation's target.
    #[must_use]
    pub const fn target(&self) -> &FileOperationTarget {
        match self {
            Self::Create { target, .. }
            | Self::Update { target, .. }
            | Self::Delete { target, .. }
            | Self::Skip { target, .. }
            | Self::Conflict { target, .. }
            | Self::NoOp { target, .. } => target,
        }
    }

    /// Returns the human-readable operation explanation.
    #[must_use]
    pub fn explanation(&self) -> &str {
        match self {
            Self::Create {
                content_summary, ..
            }
            | Self::Update {
                content_summary, ..
            } => content_summary,
            Self::Delete { reason, .. }
            | Self::Skip { reason, .. }
            | Self::Conflict { reason, .. }
            | Self::NoOp { reason, .. } => reason,
        }
    }

    /// Returns true when this operation would write file content.
    #[must_use]
    pub const fn writes_content(&self) -> bool {
        self.kind().writes_content()
    }

    /// Returns true when this operation is destructive.
    #[must_use]
    pub const fn is_destructive(&self) -> bool {
        self.kind().is_destructive()
    }

    /// Returns true when this operation blocks safe application.
    #[must_use]
    pub const fn is_blocking(&self) -> bool {
        self.kind().is_blocking()
    }

    /// Returns true when this operation requires explicit approval.
    #[must_use]
    pub const fn requires_approval(&self) -> bool {
        match self {
            Self::Delete {
                requires_approval, ..
            } => *requires_approval,
            Self::Conflict { .. } => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_kind_labels_are_stable() {
        assert_eq!(FileOperationKind::Create.as_str(), "create");
        assert_eq!(FileOperationKind::Update.as_str(), "update");
        assert_eq!(FileOperationKind::Delete.as_str(), "delete");
        assert_eq!(FileOperationKind::Skip.as_str(), "skip");
        assert_eq!(FileOperationKind::Conflict.as_str(), "conflict");
        assert_eq!(FileOperationKind::NoOp.as_str(), "no_op");
    }

    #[test]
    fn target_preserves_path() {
        let target = FileOperationTarget::new("docs/README.md");

        assert_eq!(target.path(), Path::new("docs/README.md"));
        assert_eq!(target.display_path(), "docs/README.md");
    }

    #[test]
    fn create_operation_is_content_write() {
        let operation = PlannedFileOperation::create("docs/README.md", "create project README");

        assert_eq!(operation.kind(), FileOperationKind::Create);
        assert_eq!(operation.target().path(), Path::new("docs/README.md"));
        assert_eq!(operation.explanation(), "create project README");
        assert!(operation.writes_content());
        assert!(!operation.is_destructive());
        assert!(!operation.requires_approval());
    }

    #[test]
    fn update_operation_is_content_write() {
        let operation = PlannedFileOperation::update("docs/README.md", "refresh README contents");

        assert_eq!(operation.kind(), FileOperationKind::Update);
        assert!(operation.writes_content());
        assert!(!operation.is_blocking());
    }

    #[test]
    fn delete_operation_can_require_approval() {
        let operation =
            PlannedFileOperation::delete("legacy.txt", "remove deprecated generated file", true);

        assert_eq!(operation.kind(), FileOperationKind::Delete);
        assert!(operation.is_destructive());
        assert!(operation.requires_approval());
    }

    #[test]
    fn skip_and_conflict_are_reviewable_states() {
        let skip = PlannedFileOperation::skip(
            "README.md",
            "file already exists and overwrite was not requested",
        );
        let conflict = PlannedFileOperation::conflict(
            "Cargo.toml",
            "existing file differs from planned template",
        );

        assert_eq!(skip.kind(), FileOperationKind::Skip);
        assert!(!skip.is_blocking());

        assert_eq!(conflict.kind(), FileOperationKind::Conflict);
        assert!(conflict.is_blocking());
        assert!(conflict.requires_approval());
    }
}
