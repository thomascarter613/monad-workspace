//! Command execution result model for Monad.

/// Result returned after running one local command.
///
/// The result is intentionally transparent: it records the command display text,
/// working directory, exit status, stdout, and stderr. Monad should not hide
/// native tool behavior from the user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandResult {
    command: String,
    working_directory: String,
    exit_code: Option<i32>,
    success: bool,
    stdout: String,
    stderr: String,
}

impl CommandResult {
    /// Creates a command result.
    #[must_use]
    pub fn new(
        command: impl Into<String>,
        working_directory: impl Into<String>,
        exit_code: Option<i32>,
        success: bool,
        stdout: impl Into<String>,
        stderr: impl Into<String>,
    ) -> Self {
        Self {
            command: command.into(),
            working_directory: working_directory.into(),
            exit_code,
            success,
            stdout: stdout.into(),
            stderr: stderr.into(),
        }
    }

    /// Returns the display form of the command that ran.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Returns the working directory used for execution.
    #[must_use]
    pub fn working_directory(&self) -> &str {
        &self.working_directory
    }

    /// Returns the process exit code when the platform provides one.
    #[must_use]
    pub const fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    /// Returns true when the process exited successfully.
    #[must_use]
    pub const fn success(&self) -> bool {
        self.success
    }

    /// Returns true when the process failed.
    #[must_use]
    pub const fn failed(&self) -> bool {
        !self.success
    }

    /// Returns captured stdout as UTF-8 text.
    #[must_use]
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    /// Returns captured stderr as UTF-8 text.
    #[must_use]
    pub fn stderr(&self) -> &str {
        &self.stderr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_result_exposes_execution_fields() {
        let result = CommandResult::new("cargo test", ".", Some(0), true, "tests passed\n", "");

        assert_eq!(result.command(), "cargo test");
        assert_eq!(result.working_directory(), ".");
        assert_eq!(result.exit_code(), Some(0));
        assert!(result.success());
        assert!(!result.failed());
        assert_eq!(result.stdout(), "tests passed\n");
        assert_eq!(result.stderr(), "");
    }

    #[test]
    fn command_result_can_represent_failure() {
        let result = CommandResult::new("cargo test", ".", Some(101), false, "", "test failure\n");

        assert!(!result.success());
        assert!(result.failed());
        assert_eq!(result.exit_code(), Some(101));
        assert_eq!(result.stderr(), "test failure\n");
    }
}
