//! Command-line entrypoint for Monad.
//!
//! This crate should stay thin. Its job is to parse command-line concerns,
//! call `monad-core`, and present results to the user.
//!
//! Durable product behavior belongs in `monad-core`.

use std::process::ExitCode;

use monad_core::{RuntimeIdentity, runtime_identity};

/// Formats the startup message shown by the CLI.
///
/// Keeping this as a small function makes it easy to test without spawning a
/// separate process.
fn startup_message(identity: RuntimeIdentity) -> String {
    identity.banner()
}

/// Program entrypoint.
///
/// Rust binaries start in `main`. Returning `ExitCode` lets us clearly signal
/// success or failure to the operating system.
fn main() -> ExitCode {
    let identity = runtime_identity();

    println!("{}", startup_message(identity));

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_message_comes_from_core_runtime_identity() {
        let message = startup_message(runtime_identity());

        assert!(message.contains("Monad"));
        assert!(message.contains("monad-core"));
        assert!(message.contains("local-first"));
    }
}
