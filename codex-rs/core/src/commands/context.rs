//! Context builder for template variable expansion.

use std::collections::HashMap;
use std::path::PathBuf;

use super::executor::ConversationContext;

/// Context for template expansion.
///
/// Contains variables and state used during template expansion,
/// including user arguments, git context, and workspace information.
#[derive(Debug, Clone, Default)]
pub struct CommandContext {
    /// User-provided arguments.
    pub args: HashMap<String, String>,
    /// Current git diff (if available).
    pub git_diff: Option<String>,
    /// Files in context.
    pub files: Vec<PathBuf>,
    /// Workspace root directory.
    pub workspace_root: PathBuf,
    /// Environment variables (whitelisted for security).
    pub env_vars: HashMap<String, String>,
    /// Optional conversation context for context-aware commands.
    pub conversation_context: Option<ConversationContext>,
}

impl CommandContext {
    /// Creates a new context builder.
    pub fn builder() -> CommandContextBuilder {
        CommandContextBuilder::default()
    }

    /// Builds context from user arguments.
    pub async fn build(args: HashMap<String, String>) -> anyhow::Result<Self> {
        let workspace_root = std::env::current_dir()?;

        Ok(Self {
            args,
            git_diff: None,
            files: Vec::new(),
            workspace_root,
            env_vars: HashMap::new(),
            conversation_context: None,
        })
    }
}

/// Builder for CommandContext.
#[derive(Debug, Default)]
pub struct CommandContextBuilder {
    args: HashMap<String, String>,
    git_diff: Option<String>,
    files: Vec<PathBuf>,
    workspace_root: Option<PathBuf>,
    env_vars: HashMap<String, String>,
    conversation_context: Option<ConversationContext>,
}

impl CommandContextBuilder {
    /// Adds an argument.
    pub fn arg(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.args.insert(key.into(), value.into());
        self
    }

    /// Sets arguments.
    pub fn args(mut self, args: HashMap<String, String>) -> Self {
        self.args = args;
        self
    }

    /// Sets git diff.
    pub fn git_diff(mut self, diff: impl Into<String>) -> Self {
        self.git_diff = Some(diff.into());
        self
    }

    /// Adds a file.
    pub fn file(mut self, path: impl Into<PathBuf>) -> Self {
        self.files.push(path.into());
        self
    }

    /// Sets workspace root.
    pub fn workspace_root(mut self, path: impl Into<PathBuf>) -> Self {
        self.workspace_root = Some(path.into());
        self
    }

    /// Sets environment variables.
    pub fn env_vars(mut self, env_vars: HashMap<String, String>) -> Self {
        self.env_vars = env_vars;
        self
    }

    /// Sets conversation context.
    pub fn conversation_context(mut self, context: Option<ConversationContext>) -> Self {
        self.conversation_context = context;
        self
    }

    /// Builds the context.
    pub fn build(self) -> CommandContext {
        CommandContext {
            args: self.args,
            git_diff: self.git_diff,
            files: self.files,
            workspace_root: self.workspace_root.unwrap_or_else(|| PathBuf::from(".")),
            env_vars: self.env_vars,
            conversation_context: self.conversation_context,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_builder() {
        let context = CommandContext::builder()
            .arg("path", "src/main.rs")
            .arg("depth", "normal")
            .workspace_root("/workspace")
            .build();

        assert_eq!(context.args.get("path").unwrap(), "src/main.rs");
        assert_eq!(context.args.get("depth").unwrap(), "normal");
    }
}
