//! Local command execution foundation for Monad.
//!
//! Monad coordinates native tools instead of replacing them. This module gives
//! the rest of `monad-core` a small, transparent command-runner abstraction.
//!
//! WP-E4-002 intentionally keeps this simple:
//!
//! - no sandboxing;
//! - no PTY;
//! - no streaming UI;
//! - no retries;
//! - no agent-controlled execution;
//! - no secret redaction.
//!
//! Later verification work can build on this foundation.

pub mod command;
pub mod result;

pub use command::{CommandSpec, run_command};
pub use result::CommandResult;
