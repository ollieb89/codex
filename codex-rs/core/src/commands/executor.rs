//! Command execution pipeline.
//!
//! This module coordinates command execution by:
//! 1. Looking up commands in the registry
//! 2. Mapping and validating arguments
//! 3. Building execution context
//! 4. Expanding templates
//! 5. Returning expanded prompts for execution

use super::agents::AgentCommandExecutor;
use super::agents::AgentResultFormatter;
use super::agents::OutputFormat;
use super::args::ArgumentMapper;
use super::context::CommandContext;
use super::expander::TemplateExpander;
use super::invocation::CommandInvocation;
use super::registry::CommandRegistry;
use crate::agents::GitContext;
use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// Executes commands by coordinating registry lookup, argument mapping,
/// context building, and template expansion.
pub struct CommandExecutor {
    registry: Arc<CommandRegistry>,
    expander: TemplateExpander,
    agent_executor: Option<Arc<AgentCommandExecutor>>,
}

impl CommandExecutor {
    /// Creates a new CommandExecutor with the given registry.
    pub fn new(registry: Arc<CommandRegistry>) -> Self {
        Self {
            registry,
            expander: TemplateExpander::new(),
            agent_executor: None,
        }
    }

    /// Adds agent execution support to this executor.
    pub fn with_agent_executor(mut self, agent_executor: Arc<AgentCommandExecutor>) -> Self {
        self.agent_executor = Some(agent_executor);
        self
    }

    /// Executes a command invocation and returns the expanded prompt.
    ///
    /// # Execution Flow
    ///
    /// 1. Look up command in registry by name
    /// 2. Map invocation arguments to command's expected arguments
    /// 3. Build CommandContext with mapped arguments
    /// 4. Expand command template with context
    /// 5. Return expanded prompt string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use codex_core::commands::executor::CommandExecutor;
    /// # use codex_core::commands::registry::CommandRegistry;
    /// # use codex_core::commands::invocation::InvocationParser;
    /// # use std::sync::Arc;
    /// # use std::path::PathBuf;
    /// # async fn example() -> anyhow::Result<()> {
    /// let registry = Arc::new(CommandRegistry::new(PathBuf::from(".claude/commands")).await?);
    /// let executor = CommandExecutor::new(registry);
    ///
    /// let invocation = InvocationParser::parse("/explain src/main.rs")?;
    /// let context = ExecutionContext {
    ///     workspace_root: PathBuf::from("/workspace"),
    ///     git_diff: None,
    ///     current_files: vec![],
    /// };
    ///
    /// let prompt = executor.execute(invocation, &context).await?;
    /// // prompt now contains the expanded template ready for LLM
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute(
        &self,
        invocation: CommandInvocation,
        exec_context: &ExecutionContext,
    ) -> Result<String> {
        // Step 1: Look up command in registry
        let command = self
            .registry
            .get(&invocation.command_name)
            .await
            .context(format!(
                "Command '{}' not found. Run `codex commands list` to see available commands.",
                invocation.command_name
            ))?;

        // Step 2: Check if this is an agent-backed command
        if let Some(user_cmd) = command
            .as_any()
            .downcast_ref::<super::user::loader::UserCommand>()
        {
            // Check if this is an agent command
            if user_cmd.metadata.agent {
                return self
                    .execute_agent_command(&invocation, user_cmd, exec_context)
                    .await;
            }

            // Regular user command - map arguments using metadata
            let mapped_args = ArgumentMapper::map_arguments(&invocation, &user_cmd.metadata)?;

            // Continue with template expansion
            return self
                .execute_template_command(mapped_args, exec_context, &user_cmd.template)
                .await;
        }

        // Built-in command - use invocation args directly
        let mut mapped_args = HashMap::new();
        for (i, arg) in invocation.raw_args.iter().enumerate() {
            mapped_args.insert(format!("arg{i}"), arg.clone());
        }
        mapped_args.extend(invocation.args.clone());

        // Get template and execute
        let template = self.get_command_template(&*command)?;
        self.execute_template_command(mapped_args, exec_context, &template)
            .await
    }

    /// Executes an agent-backed command.
    async fn execute_agent_command(
        &self,
        invocation: &CommandInvocation,
        user_cmd: &super::user::loader::UserCommand,
        exec_context: &ExecutionContext,
    ) -> Result<String> {
        // Check if agent executor is available
        let agent_executor = self.agent_executor.as_ref().ok_or_else(|| {
            anyhow::anyhow!(
                "Agent command '{}' requires agent support, but no agent executor is configured",
                user_cmd.metadata.name
            )
        })?;

        // Build GitContext from ExecutionContext
        let git_context = exec_context.git_diff.as_ref().map(|diff| GitContext {
            diff: diff.clone(),
            branch: "main".to_string(), // TODO: Get actual branch from git
            changed_files: vec![],      // TODO: Parse from diff or get from git
        });

        // Execute agent command
        let agent_result = agent_executor
            .execute_agent_command(
                invocation,
                &user_cmd.metadata,
                &user_cmd.template,
                git_context,
                &exec_context.workspace_root,
            )
            .await
            .context("Agent execution failed")?;

        // Format result using AgentResultFormatter
        Ok(AgentResultFormatter::format(
            &agent_result,
            OutputFormat::Markdown,
        ))
    }

    /// Executes a template-based command.
    async fn execute_template_command(
        &self,
        mapped_args: HashMap<String, String>,
        exec_context: &ExecutionContext,
        template: &str,
    ) -> Result<String> {
        // Step 3: Build CommandContext
        let mut builder = CommandContext::builder()
            .args(mapped_args)
            .workspace_root(exec_context.workspace_root.clone())
            .env_vars(exec_context.env_vars.clone())
            .conversation_context(exec_context.conversation_context.clone());

        // Add git diff if available
        if let Some(diff) = &exec_context.git_diff {
            builder = builder.git_diff(diff.clone());
        }

        // Add files
        for file in &exec_context.current_files {
            builder = builder.file(file.clone());
        }

        let cmd_context = builder.build();

        // Expand template
        let expanded = self
            .expander
            .expand(template, &cmd_context)
            .context("Failed to expand command template")?;

        Ok(expanded)
    }

    /// Gets the template string from a command.
    fn get_command_template(&self, command: &dyn super::registry::Command) -> Result<String> {
        // Check if it's a UserCommand
        if let Some(user_cmd) = command
            .as_any()
            .downcast_ref::<super::user::loader::UserCommand>()
        {
            return Ok(user_cmd.template.clone());
        }

        // Check if it's a built-in command
        use super::builtin::*;
        let template = match command.name() {
            "explain" => ExplainCommand::template(),
            "review" => ReviewCommand::template(),
            "test" => TestCommand::template(),
            _ => anyhow::bail!("Unknown command type: {}", command.name()),
        };

        Ok(template.to_string())
    }
}

/// Summary of a conversation message for command context.
#[derive(Debug, Clone)]
pub struct MessageSummary {
    /// Role of the message sender ("user" or "assistant")
    pub role: String,
    /// Message content
    pub content: String,
    /// Optional timestamp
    pub timestamp: Option<String>,
}

/// Conversation context for commands to reference recent history.
///
/// Currently a placeholder - will be enhanced in future sprints to provide
/// actual conversation history for context-aware command execution.
#[derive(Debug, Clone)]
pub struct ConversationContext {
    /// Last N messages from conversation
    pub recent_messages: Vec<MessageSummary>,
    /// Current conversation ID
    pub conversation_id: Option<String>,
}

impl ConversationContext {
    /// Creates a new empty conversation context.
    pub fn new() -> Self {
        Self {
            recent_messages: Vec::new(),
            conversation_id: None,
        }
    }
    /// Creates conversation context with messages.
    pub fn with_messages(messages: Vec<MessageSummary>) -> Self {
        Self {
            recent_messages: messages,
            conversation_id: None,
        }
    }
}

impl Default for ConversationContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Execution context provided by the calling environment.
///
/// This contains information about the current execution state that
/// commands can use in their templates.
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// The workspace root directory
    pub workspace_root: PathBuf,
    /// Optional git diff output
    pub git_diff: Option<String>,
    /// Currently open/selected files
    pub current_files: Vec<PathBuf>,
    /// Optional conversation context for context-aware commands
    pub conversation_context: Option<ConversationContext>,
    /// Environment variables (whitelisted for security)
    pub env_vars: HashMap<String, String>,
}

impl ExecutionContext {
    /// Creates a new execution context.
    pub fn new(workspace_root: PathBuf) -> Self {
        Self {
            workspace_root,
            git_diff: None,
            current_files: Vec::new(),
            conversation_context: None,
            env_vars: HashMap::new(),
        }
    }

    /// Sets the git diff for this context.
    pub fn with_git_diff(mut self, diff: Option<String>) -> Self {
        self.git_diff = diff;
        self
    }

    /// Sets the current files for this context.
    pub fn with_files(mut self, files: Vec<PathBuf>) -> Self {
        self.current_files = files;
        self
    }

    /// Sets the conversation context for this context.
    pub fn with_conversation_context(mut self, context: Option<ConversationContext>) -> Self {
        self.conversation_context = context;
        self
    }

    /// Sets the environment variables for this context.
    pub fn with_env_vars(mut self, env_vars: HashMap<String, String>) -> Self {
        self.env_vars = env_vars;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::invocation::InvocationParser;
    
    
    
    
    
    use std::path::PathBuf;
    use tempfile::TempDir;

    async fn create_test_registry() -> (Arc<CommandRegistry>, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let commands_dir = temp_dir.path().join("commands");
        std::fs::create_dir_all(&commands_dir).unwrap();

        // Create a test command file
        let cmd_file = commands_dir.join("greet.md");
        std::fs::write(
            &cmd_file,
            r#"---
name: greet
description: Greet someone
category: utility
args:
  - name: name
    type: string
    required: true
    description: Name to greet
---

Hello {{args.name}}! Welcome to the workspace at {{workspace_root}}.
"#,
        )
        .unwrap();

        let registry = Arc::new(CommandRegistry::new(commands_dir).await.unwrap());
        (registry, temp_dir)
    }

    #[tokio::test]
    async fn test_execute_user_command() {
        let (registry, _temp) = create_test_registry().await;
        let executor = CommandExecutor::new(registry);

        let invocation = InvocationParser::parse("/greet Alice").unwrap();
        let context = ExecutionContext::new(PathBuf::from("/workspace"));

        let result = executor.execute(invocation, &context).await.unwrap();

        assert!(result.contains("Hello Alice!"));
        assert!(result.contains("/workspace"));
    }

    #[tokio::test]
    async fn test_execute_with_named_args() {
        let (registry, _temp) = create_test_registry().await;
        let executor = CommandExecutor::new(registry);

        let invocation = InvocationParser::parse("/greet name=Bob").unwrap();
        let context = ExecutionContext::new(PathBuf::from("/test"));

        let result = executor.execute(invocation, &context).await.unwrap();

        assert!(result.contains("Hello Bob!"));
        assert!(result.contains("/test"));
    }

    #[tokio::test]
    async fn test_execute_command_not_found() {
        let (registry, _temp) = create_test_registry().await;
        let executor = CommandExecutor::new(registry);

        let invocation = InvocationParser::parse("/nonexistent").unwrap();
        let context = ExecutionContext::new(PathBuf::from("/workspace"));

        let result = executor.execute(invocation, &context).await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Command 'nonexistent' not found")
        );
    }

    #[tokio::test]
    async fn test_execute_with_git_diff() {
        let (registry, _temp) = create_test_registry().await;
        let executor = CommandExecutor::new(registry);

        let invocation = InvocationParser::parse("/greet Charlie").unwrap();
        let context = ExecutionContext::new(PathBuf::from("/workspace"))
            .with_git_diff(Some("diff --git a/file.txt".to_string()));

        let result = executor.execute(invocation, &context).await.unwrap();

        assert!(result.contains("Hello Charlie!"));
    }

    #[tokio::test]
    async fn test_execute_builtin_command() {
        let (registry, _temp) = create_test_registry().await;

        // Add built-in commands to registry
        for cmd in super::super::builtin::all_commands() {
            registry.register(cmd).await;
        }

        let executor = CommandExecutor::new(registry);

        let invocation = InvocationParser::parse("/explain").unwrap();
        let context = ExecutionContext::new(PathBuf::from("/workspace"));

        let result = executor.execute(invocation, &context).await.unwrap();

        assert!(result.contains("Please provide a detailed explanation"));
    }

    // ===== Agent Integration Tests =====

    #[tokio::test]
    async fn test_agent_command_routing() {
        use crate::agents::ActivationScore;
        use crate::agents::Agent;
        use crate::agents::AgentId;
        use crate::agents::AgentPermissions;
        use crate::agents::AgentResult;
        use crate::agents::AgentRouter;
        use crate::agents::AgentToolkit;
        use crate::agents::Task;
        use crate::agents::TaskContext;
        use async_trait::async_trait;
        

        // Create mock agent
        struct MockAgent {
            permissions: AgentPermissions,
        }

        #[async_trait]
        impl Agent for MockAgent {
            fn id(&self) -> AgentId {
                AgentId::from("test-agent")
            }

            fn name(&self) -> &str {
                "Test Agent"
            }

            fn description(&self) -> &str {
                "Test agent for integration testing"
            }

            fn can_handle(&self, _context: &TaskContext) -> ActivationScore {
                ActivationScore(0.9)
            }

            async fn execute(&self, _task: Task, _toolkit: &AgentToolkit) -> Result<AgentResult> {
                Ok(AgentResult::Analysis {
                    summary: "Agent executed successfully".to_string(),
                    details: HashMap::new(),
                })
            }

            fn permissions(&self) -> &AgentPermissions {
                &self.permissions
            }

            fn system_prompt(&self) -> &str {
                "Test prompt"
            }
        }

        // Set up registry with agent command
        let temp_dir = TempDir::new().unwrap();
        let commands_dir = temp_dir.path().join("commands");
        std::fs::create_dir_all(&commands_dir).unwrap();

        std::fs::write(
            commands_dir.join("analyze.md"),
            r#"---
name: analyze
description: AI-powered code analysis
category: agents
agent: true
agent_id: test-agent
args:
  - name: target
    type: string
    required: true
    description: Target to analyze
---
Analyze the code at: {{target}}
"#,
        )
        .unwrap();

        let registry = Arc::new(CommandRegistry::new(commands_dir).await.unwrap());

        // Create agent infrastructure
        let mut router = AgentRouter::new();
        router.register_agent(Arc::new(MockAgent {
            permissions: AgentPermissions::default(),
        }));
        let agent_executor = Arc::new(AgentCommandExecutor::new(
            Arc::new(router),
            Arc::new(AgentToolkit::new(
                AgentId::from("test-toolkit"),
                AgentPermissions::default(),
                PathBuf::from("/workspace"),
            )),
        ));

        // Create executor with agent support
        let executor = CommandExecutor::new(registry).with_agent_executor(agent_executor);

        // Execute agent command
        let invocation = InvocationParser::parse("/analyze src/main.rs").unwrap();
        let context = ExecutionContext::new(PathBuf::from("/workspace"));

        let result = executor.execute(invocation, &context).await.unwrap();

        // Verify agent execution
        assert!(result.contains("Agent Analysis"));
        assert!(result.contains("Agent executed successfully"));
    }

    #[tokio::test]
    async fn test_normal_command_unchanged() {
        // Verify that non-agent commands still work with agent executor present
        use crate::agents::AgentId;
        use crate::agents::AgentPermissions;
        use crate::agents::AgentRouter;
        use crate::agents::AgentToolkit;

        let temp_dir = TempDir::new().unwrap();
        let commands_dir = temp_dir.path().join("commands");
        std::fs::create_dir_all(&commands_dir).unwrap();

        // Create normal (non-agent) command
        std::fs::write(
            commands_dir.join("hello.md"),
            r#"---
name: hello
description: Simple greeting
category: utility
agent: false
args:
  - name: name
    type: string
    required: true
    description: Name to greet
---
Hello {{args.name}}!
"#,
        )
        .unwrap();

        let registry = Arc::new(CommandRegistry::new(commands_dir).await.unwrap());

        // Create agent infrastructure (but command won't use it)
        let router = AgentRouter::new();
        let agent_executor = Arc::new(AgentCommandExecutor::new(
            Arc::new(router),
            Arc::new(AgentToolkit::new(
                AgentId::from("test-toolkit"),
                AgentPermissions::default(),
                PathBuf::from("/workspace"),
            )),
        ));

        let executor = CommandExecutor::new(registry).with_agent_executor(agent_executor);

        let invocation = InvocationParser::parse("/hello World").unwrap();
        let context = ExecutionContext::new(PathBuf::from("/workspace"));

        let result = executor.execute(invocation, &context).await.unwrap();

        // Should execute as template, not agent
        assert_eq!(result.trim(), "Hello World!");
        assert!(!result.contains("Agent Analysis"));
    }

    #[tokio::test]
    async fn test_agent_command_without_executor() {
        // Verify proper error when agent command executed without agent executor
        let temp_dir = TempDir::new().unwrap();
        let commands_dir = temp_dir.path().join("commands");
        std::fs::create_dir_all(&commands_dir).unwrap();

        std::fs::write(
            commands_dir.join("review.md"),
            r#"---
name: review
description: AI code review
category: agents
agent: true
agent_id: code-review
args:
  - name: files
    type: string
    required: true
    description: Files to review
---
Review: {{files}}
"#,
        )
        .unwrap();

        let registry = Arc::new(CommandRegistry::new(commands_dir).await.unwrap());

        // Create executor WITHOUT agent support
        let executor = CommandExecutor::new(registry);

        let invocation = InvocationParser::parse("/review src/main.rs").unwrap();
        let context = ExecutionContext::new(PathBuf::from("/workspace"));

        let result = executor.execute(invocation, &context).await;

        // Should return error
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("requires agent support"));
        assert!(error_msg.contains("review"));
    }
}
