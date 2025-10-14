//! Agent context building from command invocations.

use crate::agents::{ExecutionMode, GitContext, Task, TaskContext};
use crate::commands::args::ArgumentMapper;
use crate::commands::invocation::CommandInvocation;
use crate::commands::parser::CommandMetadata;
use anyhow::Result;
use handlebars::Handlebars;
use std::collections::HashMap;
use std::path::Path;

/// Builds TaskContext from command invocations for agent execution.
pub struct AgentContextBuilder;

impl AgentContextBuilder {
    /// Builds a Task from a command invocation.
    ///
    /// # Arguments
    ///
    /// * `invocation` - The parsed command invocation
    /// * `metadata` - Command metadata including template and arguments
    /// * `template` - The command template to render
    /// * `git_context` - Optional git context for the workspace
    /// * `workspace_root` - Path to the workspace root directory (used to extract file paths)
    ///
    /// # Returns
    ///
    /// A Task ready for agent execution, or an error if building fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use codex_core::commands::agents::AgentContextBuilder;
    /// # use codex_core::commands::invocation::InvocationParser;
    /// # use codex_core::commands::parser::{CommandMetadata, CommandPermissions};
    /// # use std::path::PathBuf;
    /// # fn example() -> anyhow::Result<()> {
    /// let invocation = InvocationParser::parse("/review src/main.rs")?;
    /// let metadata = CommandMetadata {
    ///     name: "review".to_string(),
    ///     description: "Code review".to_string(),
    ///     category: "agents".to_string(),
    ///     permissions: CommandPermissions::default(),
    ///     args: vec![],
    ///     agent: true,
    ///     agent_id: Some("code-review".to_string()),
    ///     activation_hints: vec!["review".to_string()],
    /// };
    ///
    /// let task = AgentContextBuilder::build_task(
    ///     &invocation,
    ///     &metadata,
    ///     "Review: {{files}}",
    ///     None,
    ///     &PathBuf::from("/workspace"),
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build_task(
        invocation: &CommandInvocation,
        metadata: &CommandMetadata,
        template: &str,
        git_context: Option<GitContext>,
        workspace_root: &Path,
    ) -> Result<Task> {
        // Step 1: Map arguments from invocation to command's expected parameters
        let mapped_args = ArgumentMapper::map_arguments(invocation, metadata)?;

        // Step 2: Render template with arguments to get user intent
        let user_intent = Self::render_template(template, &mapped_args)?;

        // Step 3: Extract file paths from arguments (heuristic: any arg that looks like a path)
        let file_paths = Self::extract_file_paths(&mapped_args, workspace_root);

        // Step 4: Build TaskContext
        let context = TaskContext {
            file_paths,
            file_contents: None, // Will be loaded by agent if needed
            git_context,
            execution_mode: ExecutionMode::Interactive,
            user_intent,
        };

        // Step 5: Build Task
        Ok(Task {
            context,
            additional_instructions: None,
        })
    }

    /// Extracts file paths from argument map.
    ///
    /// This is a heuristic approach - looks for arguments that appear to be file paths.
    fn extract_file_paths(
        args: &HashMap<String, String>,
        _workspace_root: &Path,
    ) -> Vec<std::path::PathBuf> {
        args.values()
            .filter_map(|value| {
                // Simple heuristic: if it contains path separators or file extensions, treat as path
                if value.contains('/') || value.contains('.') {
                    Some(std::path::PathBuf::from(value))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Renders a Handlebars template with the given arguments.
    ///
    /// # Arguments
    ///
    /// * `template` - The Handlebars template string
    /// * `args` - HashMap of argument names to values
    ///
    /// # Returns
    ///
    /// The rendered template string, or an error if rendering fails.
    fn render_template(template: &str, args: &HashMap<String, String>) -> Result<String> {
        let handlebars = Handlebars::new();
        let rendered = handlebars.render_template(template, args)?;
        Ok(rendered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::invocation::InvocationParser;
    use crate::commands::parser::{ArgDefinition, ArgType, CommandPermissions};
    use std::path::PathBuf;

    fn create_test_metadata() -> CommandMetadata {
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
            agent_id: Some("code-review".to_string()),
            activation_hints: vec!["review".to_string()],
        }
    }

    #[test]
    fn test_build_task_with_all_parameters() {
        let invocation = InvocationParser::parse("/review src/main.rs").unwrap();
        let metadata = create_test_metadata();
        let template = "Review the following files: {{files}}";

        let git_context = Some(GitContext {
            diff: "mock diff".to_string(),
            branch: "main".to_string(),
            changed_files: vec![],
        });

        let workspace_root = PathBuf::from("/workspace");

        let task = AgentContextBuilder::build_task(
            &invocation,
            &metadata,
            template,
            git_context.clone(),
            &workspace_root,
        )
        .unwrap();

        assert!(task.context.git_context.is_some());
        assert_eq!(task.context.execution_mode, ExecutionMode::Interactive);
        assert!(task.context.user_intent.contains("src/main.rs"));
        assert!(
            task.context
                .file_paths
                .iter()
                .any(|p| p.to_str().unwrap().contains("src/main.rs"))
        );
    }

    #[test]
    fn test_build_task_minimal() {
        let invocation = InvocationParser::parse("/review src/main.rs").unwrap();
        let metadata = create_test_metadata();
        let template = "Review: {{files}}";
        let workspace_root = PathBuf::from("/workspace");

        let task = AgentContextBuilder::build_task(
            &invocation,
            &metadata,
            template,
            None,
            &workspace_root,
        )
        .unwrap();

        assert!(task.context.git_context.is_none());
        assert!(task.context.user_intent.contains("src/main.rs"));
    }

    #[test]
    fn test_build_task_with_template_rendering() {
        let invocation = InvocationParser::parse("/review src/main.rs src/lib.rs").unwrap();
        let metadata = create_test_metadata();
        let template = "Please review the files: {{files}}";
        let workspace_root = PathBuf::from("/workspace");

        let task = AgentContextBuilder::build_task(
            &invocation,
            &metadata,
            template,
            None,
            &workspace_root,
        )
        .unwrap();

        // Verify template was rendered with arguments
        assert!(
            task.context
                .user_intent
                .contains("Please review the files:")
        );
        // ArgumentMapper should map first positional arg to "files"
        assert!(task.context.user_intent.contains("src/main.rs"));
    }

    #[test]
    fn test_build_task_missing_required_args() {
        let invocation = InvocationParser::parse("/review").unwrap(); // Missing required "files" arg
        let metadata = create_test_metadata();
        let template = "Review: {{files}}";
        let workspace_root = PathBuf::from("/workspace");

        let result = AgentContextBuilder::build_task(
            &invocation,
            &metadata,
            template,
            None,
            &workspace_root,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("missing"));
    }

    #[test]
    fn test_build_task_git_context_optional() {
        let invocation = InvocationParser::parse("/review src/main.rs").unwrap();
        let metadata = create_test_metadata();
        let template = "Review: {{files}}";
        let workspace_root = PathBuf::from("/workspace");

        // Should succeed without git context
        let task = AgentContextBuilder::build_task(
            &invocation,
            &metadata,
            template,
            None,
            &workspace_root,
        )
        .unwrap();

        assert!(task.context.git_context.is_none());
        assert!(task.context.user_intent.contains("src/main.rs"));
    }

    #[test]
    fn test_build_task_performance() {
        use std::time::Instant;

        let invocation = InvocationParser::parse("/review src/main.rs").unwrap();
        let metadata = create_test_metadata();
        let template = "Review: {{files}}";
        let workspace_root = PathBuf::from("/workspace");

        let start = Instant::now();
        let _task = AgentContextBuilder::build_task(
            &invocation,
            &metadata,
            template,
            None,
            &workspace_root,
        )
        .unwrap();
        let elapsed = start.elapsed();

        // Assert task building completes in <100ms
        assert!(
            elapsed.as_millis() < 100,
            "Task building took {}ms, expected <100ms",
            elapsed.as_millis()
        );
    }

    #[test]
    fn test_render_template_basic() {
        let template = "Hello {{name}}!";
        let mut args = HashMap::new();
        args.insert("name".to_string(), "World".to_string());

        let rendered = AgentContextBuilder::render_template(template, &args).unwrap();
        assert_eq!(rendered, "Hello World!");
    }

    #[test]
    fn test_render_template_multiple_vars() {
        let template = "Review {{files}} with focus on {{focus}}";
        let mut args = HashMap::new();
        args.insert("files".to_string(), "src/main.rs".to_string());
        args.insert("focus".to_string(), "security".to_string());

        let rendered = AgentContextBuilder::render_template(template, &args).unwrap();
        assert_eq!(rendered, "Review src/main.rs with focus on security");
    }
}
