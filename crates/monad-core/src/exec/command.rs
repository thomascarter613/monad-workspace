//! Local command execution for Monad.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{CommandResult, MonadError, MonadResult};

/// Specification for one local command execution.
///
/// This type is deliberately small. It describes what Monad intends to run,
/// not a full shell language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSpec {
    program: String,
    args: Vec<String>,
    working_directory: Option<PathBuf>,
}

impl CommandSpec {
    /// Creates a command spec with a program name or path.
    #[must_use]
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            working_directory: None,
        }
    }

    /// Adds one argument.
    #[must_use]
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Adds multiple arguments.
    #[must_use]
    pub fn args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    /// Sets the working directory.
    #[must_use]
    pub fn working_directory(mut self, working_directory: impl Into<PathBuf>) -> Self {
        self.working_directory = Some(working_directory.into());
        self
    }

    /// Returns the program.
    #[must_use]
    pub fn program(&self) -> &str {
        &self.program
    }

    /// Returns the arguments.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.args
    }

    /// Returns the configured working directory, if any.
    #[must_use]
    pub fn working_directory_path(&self) -> Option<&Path> {
        self.working_directory.as_deref()
    }

    /// Returns a deterministic display string for the command.
    ///
    /// This is not shell-escaped. It is meant for transparent reporting.
    #[must_use]
    pub fn display_command(&self) -> String {
        std::iter::once(self.program.as_str())
            .chain(self.args.iter().map(String::as_str))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Runs this command locally.
    pub fn run(&self) -> MonadResult<CommandResult> {
        run_command(self)
    }
}

/// Runs a local command and captures stdout, stderr, and exit status.
pub fn run_command(spec: &CommandSpec) -> MonadResult<CommandResult> {
    if spec.program().trim().is_empty() {
        return Err(MonadError::invalid_input(
            "command program must not be empty",
        ));
    }

    let working_directory = spec
        .working_directory_path()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    let output = Command::new(spec.program())
        .args(spec.arguments())
        .current_dir(&working_directory)
        .output()
        .map_err(|error| {
            MonadError::internal(format!(
                "failed to run command `{}` in `{}`: {error}",
                spec.display_command(),
                working_directory.display()
            ))
        })?;

    Ok(CommandResult::new(
        spec.display_command(),
        working_directory.display().to_string(),
        output.status.code(),
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn successful_echo_command() -> CommandSpec {
        if cfg!(windows) {
            CommandSpec::new("cmd").args(["/C", "echo monad"])
        } else {
            CommandSpec::new("sh").args(["-c", "printf monad"])
        }
    }

    fn failing_command() -> CommandSpec {
        if cfg!(windows) {
            CommandSpec::new("cmd").args(["/C", "echo monad failure 1>&2 && exit /B 7"])
        } else {
            CommandSpec::new("sh").args(["-c", "printf 'monad failure' >&2; exit 7"])
        }
    }

    #[test]
    fn command_spec_builds_display_command() {
        let spec = CommandSpec::new("cargo")
            .args(["test", "--workspace"])
            .working_directory(".");

        assert_eq!(spec.program(), "cargo");
        assert_eq!(
            spec.arguments(),
            &["test".to_string(), "--workspace".to_string()]
        );
        assert_eq!(spec.display_command(), "cargo test --workspace");
        assert_eq!(
            spec.working_directory_path(),
            Some(std::path::Path::new("."))
        );
    }

    #[test]
    fn command_runner_captures_success_stdout_and_exit_code() -> MonadResult<()> {
        let result = successful_echo_command().run()?;

        assert!(result.success());
        assert_eq!(result.exit_code(), Some(0));
        assert!(result.stdout().contains("monad"));
        assert!(result.stderr().is_empty());

        Ok(())
    }

    #[test]
    fn command_runner_captures_failure_stderr_and_exit_code() -> MonadResult<()> {
        let result = failing_command().run()?;

        assert!(result.failed());
        assert_eq!(result.exit_code(), Some(7));
        assert!(result.stderr().contains("monad failure"));

        Ok(())
    }

    #[test]
    fn command_runner_rejects_empty_program() -> MonadResult<()> {
        let Err(error) = CommandSpec::new("   ").run() else {
            return Err(MonadError::internal(
                "empty program should fail but command runner returned success",
            ));
        };

        assert_eq!(error.code(), "MONAD2001");
        assert!(error.message().contains("must not be empty"));

        Ok(())
    }
}
