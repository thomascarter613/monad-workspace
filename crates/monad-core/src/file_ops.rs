//! Safe file operation planning foundation.
//!
//! Monad's evolution engine must be trustworthy before it is powerful.
//! This module models proposed repository file changes before anything writes
//! to disk.
//!
//! WP-E5-001 intentionally does not implement writes, diffs, templates, branch
//! management, worktrees, or CLI behavior. It only defines the safe vocabulary
//! future evolution commands will use.

pub mod model;
pub mod plan;

pub use model::{FileOperationKind, FileOperationTarget, PlannedFileOperation};
pub use plan::{FileOperationPlan, FileOperationSummary};
