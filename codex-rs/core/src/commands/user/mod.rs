//! User-defined command loading.
//!
//! This module handles discovery and loading of user-defined commands
//! from the ~/.codex/commands directory.

pub mod loader;

pub use loader::UserCommandLoader;
