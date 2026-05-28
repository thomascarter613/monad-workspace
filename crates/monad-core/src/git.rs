//! Conservative Git state inspection.
//!
//! This module is intentionally read-only. It exists so future evolution
//! workflows can decide whether direct apply, branch isolation, or worktree
//! isolation should be recommended.
//!
//! WP-E5-006 does not create branches, create worktrees, commit, push, clean,
//! reset, merge, or mutate Git state.

pub mod status;

pub use status::{
    GitEvolutionSafety, GitWorkingTreeStatus, inspect_git_working_tree, parse_git_status_porcelain,
};
