//! Agent command executor for routing commands to AI agents.

use crate::agents::{Agent, AgentResult, AgentRouter, AgentToolkit, GitContext};
use crate::commands::invocation::CommandInvocation;
use crate::commands::parser::CommandMetadata;
use anyhow::{Context, Result};
use std::path::Path;
use std::sync::Arc;

use super::context_builder::AgentContextBuilder;

/// Executes agent-backed commands by routing to appropriate agents.
pub struct AgentCommandExecutor {
    router: Arc<AgentRouter>,
    toolkit: Arc<AgentToolkit>,
}

impl AgentCommandExecutor {
    /// Creates a new AgentCommandExecutor with the given router and toolkit.
    pub fn new(router: Arc<AgentRouter>, toolkit: Arc<AgentToolkit>) -> Self {
        Self { router, toolkit }
    }

    /// Executes an agent-backed command.
    ///
    /// # Execution Flow
    ///
    /// 1. Build TaskContext from command invocation
    /// 2. Select appropriate agent (explicit agent_id or router selection)
    /// 3. Execute agent with context
    /// 4. Return agent result
    ///
    /// # Arguments
    ///
    /// * `invocation` - The parsed command invocation
    /// * `metadata` - Command metadata including agent configuration
    /// * `template` - The command template to render
    /// * `git_context` - Optional git context
    /// * `workspace_root` - Workspace root directory
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use codex_core::commands::agents::AgentCommandExecutor;
    /// # use codex_core::agents::AgentRouter;
    /// # use codex_core::commands::invocation::InvocationParser;
    /// # use codex_core::commands::parser::{CommandMetadata, CommandPermissions};
    /// # use std::sync::Arc;
    /// # use std::path::PathBuf;
    /// # async fn example() -> anyhow::Result<()> {
    /// let router = Arc::new(AgentRouter::new());
    /// let toolkit = Arc::new(AgentToolkit::new(
    ///     AgentId::from("test-agent"),
    ///     AgentPermissions::default(),
    ///     PathBuf::from("/workspace"),
    /// ));
    /// let executor = AgentCommandExecutor::new(router, toolkit);
    ///
    /// let invocation = InvocationParser::parse("/review src/main.rs")?;
    /// let metadata = CommandMetadata {
    ///     name: "review".to_string(),
    ///     description: "Code review".to_string(),
    ///     category: "agents".to_string(),
    ///     permissions: CommandPermissions::default(),
    ///     args: vec![],
    ///     agent: true,
    ///     agent_id: Some("code-review".to_string()),
    ///     activation_hints: vec![],
    /// };
    ///
    /// let result = executor.execute_agent_command(
    ///     &invocation,
    ///     &metadata,
    ///     "Review: {{files}}",
    ///     None,
    ///     &PathBuf::from("/workspace"),
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute_agent_command(
        &self,
        invocation: &CommandInvocation,
        metadata: &CommandMetadata,
        template: &str,
        git_context: Option<GitContext>,
        workspace_root: &Path,
    ) -> Result<AgentResult> {
        // Step 1: Build task
        let task = AgentContextBuilder::build_task(
            invocation,
            metadata,
            template,
            git_context,
            workspace_root,
        )
        .context("Failed to build agent task")?;

        // Step 2: Select agent based on task context
        let agent: Arc<dyn Agent> =
            self.router
                .select_agent(&task.context)
                .await
                .with_context(|| {
                    if let Some(agent_id) = &metadata.agent_id {
                        format!("Agent '{}' not found or not available", agent_id)
                    } else {
                        "No suitable agent found for command".to_string()
                    }
                })?;

        // Step 3: Execute agent with task and toolkit
        let result = agent
            .execute(task, &self.toolkit)
            .await
            .context("Agent execution failed")?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::{ActivationScore, AgentId, TaskContext};
    use crate::commands::invocation::InvocationParser;
    use crate::commands::parser::{ArgDefinition, ArgType, CommandPermissions};
    use async_trait::async_trait;
    use std::path::PathBuf;

    // Mock agent for testing
    struct MockAgent {
        id: String,
        can_handle_score: f64,
        permissions: crate::agents::AgentPermissions,
    }

    #[async_trait]
    impl Agent for MockAgent {
        fn id(&self) -> AgentId {
            AgentId::from(&self.id)
        }

        fn can_handle(&self, _context: &TaskContext) -> ActivationScore {
            ActivationScore(self.can_handle_score)
        }

        async fn execute(
            &self,
            _task: crate::agents::Task,
            _toolkit: &crate::agents::AgentToolkit,
        ) -> Result<AgentResult> {
            Ok(AgentResult::Analysis {
                summary: "Mock agent executed".to_string(),
                details: std::collections::HashMap::new(),
            })
        }

        fn name(&self) -> &str {
            "Mock Agent"
        }

        fn description(&self) -> &str {
            "A mock agent for testing"
        }

        fn permissions(&self) -> &crate::agents::AgentPermissions {
            &self.permissions
        }

        fn system_prompt(&self) -> &str {
            "Mock agent"
        }
    }

    fn create_test_metadata_with_agent_id(agent_id: &str) -> CommandMetadata {
        CommandMetadata {
            name: "review".to_string(),
            description: "Code review".to_string(),
            category: "agents".to_string(),
            permissions: CommandPermissions::default(),
            args: vec![ArgDefinition {
                name: "files".to_string(),
                arg_type: ArgType::String,
                description: "Files to review".to_string(),
                required: true,
                default: None,
            }],
            agent: true,
            agent_id: Some(agent_id.to_string()),
            activation_hints: vec![],
        }
    }

    fn create_test_metadata_without_agent_id() -> CommandMetadata {
        CommandMetadata {
            name: "analyze".to_string(),
            description: "Code analysis".to_string(),
            category: "agents".to_string(),
            permissions: CommandPermissions::default(),
            args: vec![ArgDefinition {
                name: "target".to_string(),
                arg_type: ArgType::String,
                description: "Target to analyze".to_string(),
                required: true,
                default: None,
            }],
            agent: true,
            agent_id: None,
            activation_hints: vec!["analyze".to_string(), "inspect".to_string()],
        }
    }

    #[tokio::test]
    async fn test_execute_with_explicit_agent_id() {
        let mut router = AgentRouter::new();
        let mock_agent = Arc::new(MockAgent {
            id: "code-review".to_string(),
            can_handle_score: 0.8,
            permissions: crate::agents::AgentPermissions::default(),
        });
        router.register_agent(mock_agent);
        let router = Arc::new(router);
        let toolkit = Arc::new(AgentToolkit::new(
            AgentId::from("test-agent"),
            crate::agents::AgentPermissions::default(),
            PathBuf::from("/workspace"),
        ));

        let executor = AgentCommandExecutor::new(router, toolkit);

        let invocation = InvocationParser::parse("/review src/main.rs").unwrap();
        let metadata = create_test_metadata_with_agent_id("code-review");
        let template = "Review: {{files}}";
        let workspace_root = PathBuf::from("/workspace");

        let result = executor
            .execute_agent_command(&invocation, &metadata, template, None, &workspace_root)
            .await;

        assert!(result.is_ok());
        let agent_result = result.unwrap();
        match agent_result {
            AgentResult::Analysis { summary, .. } => {
                assert_eq!(summary, "Mock agent executed");
            }
            _ => panic!("Expected Analysis result"),
        }
    }

    #[tokio::test]
    async fn test_execute_with_router_selection() {
        let mut router = AgentRouter::new();
        let mock_agent = Arc::new(MockAgent {
            id: "analyzer".to_string(),
            can_handle_score: 0.7,
            permissions: crate::agents::AgentPermissions::default(),
        });
        router.register_agent(mock_agent);
        let router = Arc::new(router);
        let toolkit = Arc::new(AgentToolkit::new(
            AgentId::from("test-agent"),
            crate::agents::AgentPermissions::default(),
            PathBuf::from("/workspace"),
        ));

        let executor = AgentCommandExecutor::new(router, toolkit);

        let invocation = InvocationParser::parse("/analyze src/main.rs").unwrap();
        let metadata = create_test_metadata_without_agent_id();
        let template = "Analyze: {{target}}";
        let workspace_root = PathBuf::from("/workspace");

        let result = executor
            .execute_agent_command(&invocation, &metadata, template, None, &workspace_root)
            .await;

        assert!(result.is_ok());
        let agent_result = result.unwrap();
        match agent_result {
            AgentResult::Analysis { summary, .. } => {
                assert_eq!(summary, "Mock agent executed");
            }
            _ => panic!("Expected Analysis result"),
        }
    }

    #[tokio::test]
    async fn test_execute_agent_not_found() {
        let router = Arc::new(AgentRouter::new());
        let toolkit = Arc::new(AgentToolkit::new(
            AgentId::from("test-agent"),
            crate::agents::AgentPermissions::default(),
            PathBuf::from("/workspace"),
        ));
        let executor = AgentCommandExecutor::new(router, toolkit);

        let invocation = InvocationParser::parse("/review src/main.rs").unwrap();
        let metadata = create_test_metadata_with_agent_id("nonexistent-agent");
        let template = "Review: {{files}}";
        let workspace_root = PathBuf::from("/workspace");

        let result = executor
            .execute_agent_command(&invocation, &metadata, template, None, &workspace_root)
            .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_execute_router_no_match() {
        let mut router = AgentRouter::new();
        // Register agent with low score (below threshold 0.6)
        let mock_agent = Arc::new(MockAgent {
            id: "low-score-agent".to_string(),
            can_handle_score: 0.3,
            permissions: crate::agents::AgentPermissions::default(),
        });
        router.register_agent(mock_agent);
        let router = Arc::new(router);
        let toolkit = Arc::new(AgentToolkit::new(
            AgentId::from("test-agent"),
            crate::agents::AgentPermissions::default(),
            PathBuf::from("/workspace"),
        ));

        let executor = AgentCommandExecutor::new(router, toolkit);

        let invocation = InvocationParser::parse("/analyze src/main.rs").unwrap();
        let metadata = create_test_metadata_without_agent_id();
        let template = "Analyze: {{target}}";
        let workspace_root = PathBuf::from("/workspace");

        let result = executor
            .execute_agent_command(&invocation, &metadata, template, None, &workspace_root)
            .await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("No suitable agent")
        );
    }

    #[tokio::test]
    async fn test_execute_performance() {
        use std::time::Instant;

        let mut router = AgentRouter::new();
        let mock_agent = Arc::new(MockAgent {
            id: "code-review".to_string(),
            can_handle_score: 0.8,
            permissions: crate::agents::AgentPermissions::default(),
        });
        router.register_agent(mock_agent);
        let router = Arc::new(router);
        let toolkit = Arc::new(AgentToolkit::new(
            AgentId::from("test-agent"),
            crate::agents::AgentPermissions::default(),
            PathBuf::from("/workspace"),
        ));

        let executor = AgentCommandExecutor::new(router, toolkit);

        let invocation = InvocationParser::parse("/review src/main.rs").unwrap();
        let metadata = create_test_metadata_with_agent_id("code-review");
        let template = "Review: {{files}}";
        let workspace_root = PathBuf::from("/workspace");

        let start = Instant::now();
        let _result = executor
            .execute_agent_command(&invocation, &metadata, template, None, &workspace_root)
            .await
            .unwrap();
        let elapsed = start.elapsed();

        // Assert routing overhead is <50ms (excluding actual agent execution)
        // Since we're using a mock agent with instant execution, this tests the overhead
        assert!(
            elapsed.as_millis() < 50,
            "Agent routing took {}ms, expected <50ms",
            elapsed.as_millis()
        );
    }
}
