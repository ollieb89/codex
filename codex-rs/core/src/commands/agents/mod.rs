//! Agent integration for command execution.
//!
//! This module provides the bridge between the command system and the agent framework,
//! enabling commands to be executed by AI agents rather than simple template expansion.

mod context_builder;
mod executor;
mod formatter;

pub use context_builder::AgentContextBuilder;
pub use executor::AgentCommandExecutor;
pub use formatter::{AgentResultFormatter, OutputFormat};
