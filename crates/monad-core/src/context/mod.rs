//! Context bridge module for Monad.
//!
//! The context bridge provides repo-native context generation. It reads
//! repository state and produces structured context artifacts that help humans
//! and AI assistants understand the current project state.
//!
//! This module is the root of the context subsystem. Individual artifact
//! generators live in submodules.

pub mod bootstrap;
pub mod current_state;
pub mod handoff;
pub mod pack;

pub use bootstrap::{
    BootstrapPromptArtifact, generate_bootstrap_prompt, render_bootstrap_prompt_markdown,
    write_bootstrap_prompt_artifact,
};

pub use current_state::{
    CurrentStateArtifact, CurrentStateEpicEntry, generate_current_state,
    write_current_state_artifact,
};

pub use handoff::{
    HandoffArtifact, HandoffWorkPacketEntry, generate_handoff, write_handoff_artifact,
};

pub use pack::{
    ContextPackArtifact, generate_context_pack, render_context_pack_markdown,
    write_context_pack_artifact,
};
