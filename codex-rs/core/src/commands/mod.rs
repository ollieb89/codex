//! Command system module for Codex.
//!
//! This module provides a flexible command system that allows users to define
//! custom slash commands using Markdown files with YAML frontmatter.
//!
//! # Architecture
//!
//! - `parser`: Parses Markdown command files with YAML frontmatter
//! - `registry`: Discovers and manages available commands
//! - `expander`: Template expansion engine for command prompts
//! - `permissions`: Permission model for command execution
//! - `context`: Context builder for template variables
//! - `invocation`: Slash command parsing and argument extraction
//! - `args`: Argument mapping and validation
//! - `executor`: Command execution pipeline
//! - `git_utils`: Git diff extraction for context enhancement
//! - `integration`: Integration with exec_command flow
//! - `builtin`: Built-in commands shipped with Codex
//! - `user`: User-defined command loading
//! - `agents`: Agent integration for AI-powered command execution

pub mod agents;
pub mod args;
pub mod builtin;
pub mod context;
pub mod executor;
pub mod expander;
pub mod git_utils;
pub mod integration;
pub mod invocation;
pub mod parser;
pub mod permissions;
pub mod registry;
pub mod user;
pub mod watcher;

#[cfg(test)]
mod integration_tests;

// Re-export key types for convenience
pub use args::ArgumentMapper;
pub use context::CommandContext;
pub use executor::{CommandExecutor, ConversationContext, ExecutionContext, MessageSummary};
pub use expander::TemplateExpander;
pub use git_utils::get_git_diff;
pub use integration::{detect_slash_command, execute_slash_command, replace_with_expanded_prompt};
pub use invocation::CommandInvocation;
pub use parser::{CommandMetadata, ParsedCommand};
pub use permissions::CommandPermissions;
pub use registry::CommandRegistry;
