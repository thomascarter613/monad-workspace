//! Read-only Git working tree status inspection.

use crate::{CommandSpec, MonadResult, WorkspaceContext};

/// Basic working tree status derived from `git status --porcelain=v1 --branch`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitWorkingTreeStatus {
    branch: Option<String>,
    detached_head: bool,
    tracked_change_count: usize,
    untracked_path_count: usize,
    ahead_count: usize,
    behind_count: usize,
    raw_lines: Vec<String>,
}

impl GitWorkingTreeStatus {
    /// Creates a Git working tree status value.
    #[must_use]
    pub fn new(
        branch: Option<String>,
        detached_head: bool,
        tracked_change_count: usize,
        untracked_path_count: usize,
        ahead_count: usize,
        behind_count: usize,
        raw_lines: Vec<String>,
    ) -> Self {
        Self {
            branch,
            detached_head,
            tracked_change_count,
            untracked_path_count,
            ahead_count,
            behind_count,
            raw_lines,
        }
    }

    /// Returns the current branch name, if known.
    #[must_use]
    pub fn branch(&self) -> Option<&str> {
        self.branch.as_deref()
    }

    /// Returns true when the repository appears to be in detached HEAD state.
    #[must_use]
    pub const fn detached_head(&self) -> bool {
        self.detached_head
    }

    /// Returns the number of tracked changes reported by porcelain status.
    #[must_use]
    pub const fn tracked_change_count(&self) -> usize {
        self.tracked_change_count
    }

    /// Returns the number of untracked paths reported by porcelain status.
    #[must_use]
    pub const fn untracked_path_count(&self) -> usize {
        self.untracked_path_count
    }

    /// Returns ahead count when reported by Git.
    #[must_use]
    pub const fn ahead_count(&self) -> usize {
        self.ahead_count
    }

    /// Returns behind count when reported by Git.
    #[must_use]
    pub const fn behind_count(&self) -> usize {
        self.behind_count
    }

    /// Returns raw status lines.
    #[must_use]
    pub fn raw_lines(&self) -> &[String] {
        &self.raw_lines
    }

    /// Returns true when there are no tracked or untracked changes.
    #[must_use]
    pub const fn is_clean(&self) -> bool {
        self.tracked_change_count == 0 && self.untracked_path_count == 0
    }

    /// Returns true when Git state suggests direct apply should be refused.
    #[must_use]
    pub const fn has_unsafe_state_for_direct_apply(&self) -> bool {
        !self.is_clean() || self.detached_head
    }

    /// Returns a conservative safety recommendation.
    #[must_use]
    pub fn safety(&self) -> GitEvolutionSafety {
        if self.detached_head {
            GitEvolutionSafety::RequireIsolation {
                reason: "repository is in detached HEAD state".to_string(),
            }
        } else if !self.is_clean() {
            GitEvolutionSafety::RequireCleanWorkingTree {
                reason: format!(
                    "working tree is not clean: {} tracked change(s), {} untracked path(s)",
                    self.tracked_change_count, self.untracked_path_count
                ),
            }
        } else {
            GitEvolutionSafety::DirectDryRunAllowed {
                reason: "working tree is clean; dry-run planning is safe".to_string(),
            }
        }
    }
}

/// Conservative safety recommendation derived from Git status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitEvolutionSafety {
    /// Dry-run planning may proceed directly.
    DirectDryRunAllowed {
        /// Human-readable reason.
        reason: String,
    },

    /// Apply behavior should require a clean working tree first.
    RequireCleanWorkingTree {
        /// Human-readable reason.
        reason: String,
    },

    /// Future write behavior should require branch/worktree isolation.
    RequireIsolation {
        /// Human-readable reason.
        reason: String,
    },
}

impl GitEvolutionSafety {
    /// Returns a stable safety label.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::DirectDryRunAllowed { .. } => "direct_dry_run_allowed",
            Self::RequireCleanWorkingTree { .. } => "require_clean_working_tree",
            Self::RequireIsolation { .. } => "require_isolation",
        }
    }

    /// Returns the human-readable reason.
    #[must_use]
    pub fn reason(&self) -> &str {
        match self {
            Self::DirectDryRunAllowed { reason }
            | Self::RequireCleanWorkingTree { reason }
            | Self::RequireIsolation { reason } => reason,
        }
    }
}

/// Runs a read-only Git status command for the workspace.
pub fn inspect_git_working_tree(context: &WorkspaceContext) -> MonadResult<GitWorkingTreeStatus> {
    let result = CommandSpec::new("git")
        .arg("status")
        .arg("--porcelain=v1")
        .arg("--branch")
        .working_directory(context.root())
        .run()?;

    if result.success() {
        Ok(parse_git_status_porcelain(result.stdout()))
    } else {
        Err(crate::MonadError::internal(format!(
            "git status failed with exit code {:?}: {}",
            result.exit_code(),
            first_non_empty_line(result.stderr()).unwrap_or_else(|| "no stderr output".to_string())
        )))
    }
}

/// Parses `git status --porcelain=v1 --branch` output.
#[must_use]
pub fn parse_git_status_porcelain(output: &str) -> GitWorkingTreeStatus {
    let mut branch = None;
    let mut detached_head = false;
    let mut tracked_change_count = 0;
    let mut untracked_path_count = 0;
    let mut ahead_count = 0;
    let mut behind_count = 0;
    let mut raw_lines = Vec::new();

    for line in output.lines() {
        let line = line.trim_end().to_string();

        if line.is_empty() {
            continue;
        }

        if let Some(branch_line) = line.strip_prefix("## ") {
            let parsed = parse_branch_line(branch_line);
            branch = parsed.branch;
            detached_head = parsed.detached_head;
            ahead_count = parsed.ahead_count;
            behind_count = parsed.behind_count;
            raw_lines.push(line);
            continue;
        }

        if line.starts_with("??") {
            untracked_path_count += 1;
        } else {
            tracked_change_count += 1;
        }

        raw_lines.push(line);
    }

    GitWorkingTreeStatus::new(
        branch,
        detached_head,
        tracked_change_count,
        untracked_path_count,
        ahead_count,
        behind_count,
        raw_lines,
    )
}

struct ParsedBranchLine {
    branch: Option<String>,
    detached_head: bool,
    ahead_count: usize,
    behind_count: usize,
}

fn parse_branch_line(line: &str) -> ParsedBranchLine {
    let detached_head = line.contains("HEAD (no branch)") || line.contains("detached");

    let branch = if detached_head {
        None
    } else {
        line.split("...")
            .next()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    };

    ParsedBranchLine {
        branch,
        detached_head,
        ahead_count: parse_counter(line, "ahead"),
        behind_count: parse_counter(line, "behind"),
    }
}

fn parse_counter(line: &str, label: &str) -> usize {
    let Some(start) = line.find(label) else {
        return 0;
    };

    let after_label = &line[start + label.len()..];

    after_label
        .trim_start()
        .trim_start_matches([':', ','])
        .trim_start()
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse::<usize>()
        .unwrap_or(0)
}

fn first_non_empty_line(text: &str) -> Option<String> {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToOwned::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_clean_branch_status() {
        let status = parse_git_status_porcelain("## main...origin/main\n");

        assert_eq!(status.branch(), Some("main"));
        assert!(!status.detached_head());
        assert!(status.is_clean());
        assert_eq!(status.tracked_change_count(), 0);
        assert_eq!(status.untracked_path_count(), 0);
        assert_eq!(status.safety().as_str(), "direct_dry_run_allowed");
    }

    #[test]
    fn parses_dirty_status_counts() {
        let status = parse_git_status_porcelain(
            "## main...origin/main [ahead 1, behind 2]\n M README.md\nA  src/lib.rs\n?? scratch.txt\n",
        );

        assert_eq!(status.branch(), Some("main"));
        assert_eq!(status.ahead_count(), 1);
        assert_eq!(status.behind_count(), 2);
        assert_eq!(status.tracked_change_count(), 2);
        assert_eq!(status.untracked_path_count(), 1);
        assert!(!status.is_clean());
        assert_eq!(status.safety().as_str(), "require_clean_working_tree");
        assert!(status.safety().reason().contains("not clean"));
    }

    #[test]
    fn parses_detached_head_as_isolation_required() {
        let status = parse_git_status_porcelain("## HEAD (no branch)\n");

        assert_eq!(status.branch(), None);
        assert!(status.detached_head());
        assert_eq!(status.safety().as_str(), "require_isolation");
        assert!(status.safety().reason().contains("detached HEAD"));
    }

    #[test]
    fn parses_branch_without_remote_tracking() {
        let status = parse_git_status_porcelain("## feature/evolution\n");

        assert_eq!(status.branch(), Some("feature/evolution"));
        assert!(status.is_clean());
    }
}
