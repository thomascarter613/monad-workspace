//! Context bridge module for Monad.
//!
//! The context bridge provides repo-native context generation. It reads
//! repository state and produces structured context artifacts that help humans
//! and AI assistants understand the current project state.
//!
//! This module is the root of the context subsystem. Individual artifact
//! generators live in submodules.

pub mod current_state;
pub mod handoff;

pub use current_state::{
    CurrentStateArtifact, CurrentStateEpicEntry, generate_current_state,
    write_current_state_artifact,
};

pub use handoff::{
    HandoffArtifact, HandoffWorkPacketEntry, generate_handoff, write_handoff_artifact,
};
