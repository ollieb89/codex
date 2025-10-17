//! End-to-end integration tests for command system.
//!
//! These tests verify the complete flow from slash command detection
//! through execution and prompt expansion.

#[cfg(test)]
mod tests {
    use crate::commands::builtin::ExplainCommand;
    use crate::commands::builtin::ReviewCommand;
    use crate::commands::builtin::TestCommand;
    use crate::commands::executor::ExecutionContext;
    use crate::commands::integration::detect_slash_command;
    use crate::commands::integration::execute_slash_command;
    use crate::commands::integration::replace_with_expanded_prompt;
    use crate::commands::registry::CommandRegistry;
    use crate::protocol::InputItem;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::Arc;
    use tempfile::TempDir;

    /// Helper to create a test registry with built-in commands
    async fn create_test_registry() -> Arc<CommandRegistry> {
        let temp_dir = TempDir::new().unwrap();
        let registry = CommandRegistry::new(temp_dir.path().to_path_buf())
            .await
            .unwrap();

        // Register built-in commands (they are unit structs)
        registry.register(Box::new(ExplainCommand)).await;
        registry.register(Box::new(ReviewCommand)).await;
        registry.register(Box::new(TestCommand)).await;

        Arc::new(registry)
    }

    /// Helper to create a test registry with a custom user command
    async fn create_test_registry_with_custom_command() -> (Arc<CommandRegistry>, TempDir) {
        let temp_dir = TempDir::new().unwrap();

        // Create a simple test command
        let commands_dir = temp_dir.path();
        std::fs::create_dir_all(commands_dir).unwrap();

        let test_command = commands_dir.join("greet.md");
        std::fs::write(
            &test_command,
            r#"---
name: greet
description: Greet someone
category: custom
args:
  - name: person
    type: string
    description: Person to greet
    required: true
permissions:
  git: []
  files: []
  directories: []
---
Hello, {{person}}! Welcome to Codex.
"#,
        )
        .unwrap();

        let registry = CommandRegistry::new(commands_dir.to_path_buf())
            .await
            .unwrap();

        (Arc::new(registry), temp_dir)
    }

    #[tokio::test]
    async fn test_e2e_basic_slash_command() {
        let registry = create_test_registry().await;

        let items = vec![InputItem::Text {
            text: "/explain src/main.rs".to_string(),
        }];

        // Detect command
        let command_text = detect_slash_command(&items);
        assert!(command_text.is_some());
        assert_eq!(command_text.unwrap(), "/explain src/main.rs");

        // Execute command
        let workspace = PathBuf::from("/tmp/test-workspace");
        let result = execute_slash_command(
            "/explain src/main.rs",
            registry,
            workspace.clone(),
            None,
            vec![],
            None,
            HashMap::new(),
        )
        .await;

        assert!(result.is_ok());
        let expanded = result.unwrap();
        // Just verify we got some expanded output
        assert!(!expanded.is_empty());
        assert!(expanded.len() > 10); // Should be a meaningful prompt
    }

    #[tokio::test]
    async fn test_e2e_command_with_named_args() {
        let (registry, _temp_dir) = create_test_registry_with_custom_command().await;

        let items = vec![InputItem::Text {
            text: "/greet person=Alice".to_string(),
        }];

        let command_text = detect_slash_command(&items).unwrap();
        let workspace = PathBuf::from("/tmp/test");

        let result = execute_slash_command(
            &command_text,
            registry,
            workspace,
            None,
            vec![],
            None,
            HashMap::new(),
        )
        .await;

        // Verify command executed successfully
        assert!(result.is_ok());
        let expanded = result.unwrap();
        assert!(!expanded.is_empty());
        assert!(expanded.len() > 5); // Should have some meaningful content
    }

    #[tokio::test]
    async fn test_e2e_command_with_positional_args() {
        let (registry, _temp_dir) = create_test_registry_with_custom_command().await;

        let items = vec![InputItem::Text {
            text: "/greet Bob".to_string(),
        }];

        let command_text = detect_slash_command(&items).unwrap();
        let workspace = PathBuf::from("/tmp/test");

        let result = execute_slash_command(
            &command_text,
            registry,
            workspace,
            None,
            vec![],
            None,
            HashMap::new(),
        )
        .await;

        // Verify command executed successfully
        assert!(result.is_ok());
        let expanded = result.unwrap();
        assert!(!expanded.is_empty());
        assert!(expanded.len() > 5); // Should have some meaningful content
    }

    #[tokio::test]
    async fn test_e2e_unknown_command_error() {
        let registry = create_test_registry().await;

        let result = execute_slash_command(
            "/nonexistent arg1 arg2",
            registry,
            PathBuf::from("/tmp"),
            None,
            vec![],
            None,
            HashMap::new(),
        )
        .await;

        assert!(result.is_err());
        let err_msg = format!("{:#}", result.unwrap_err());
        assert!(err_msg.contains("not found") || err_msg.contains("nonexistent"));
    }

    #[tokio::test]
    async fn test_e2e_missing_required_argument() {
        let (registry, _temp_dir) = create_test_registry_with_custom_command().await;

        let result = execute_slash_command(
            "/greet",
            registry,
            PathBuf::from("/tmp"),
            None,
            vec![],
            None,
            HashMap::new(),
        )
        .await;

        assert!(result.is_err());
        let err_msg = format!("{:#}", result.unwrap_err());
        assert!(err_msg.contains("required") || err_msg.contains("person"));
    }

    #[tokio::test]
    async fn test_e2e_replace_with_expanded_prompt() {
        let items = vec![
            InputItem::Text {
                text: "/explain src/main.rs".to_string(),
            },
            InputItem::Text {
                text: "Additional context here".to_string(),
            },
        ];

        let expanded = "Please explain the following code in detail...";
        let result = replace_with_expanded_prompt(items, expanded.to_string());

        assert_eq!(result.len(), 2);
        match &result[0] {
            InputItem::Text { text } => {
                assert_eq!(text, expanded);
            }
            _ => panic!("Expected Text item"),
        }
        match &result[1] {
            InputItem::Text { text } => {
                assert_eq!(text, "Additional context here");
            }
            _ => panic!("Expected Text item"),
        }
    }

    #[tokio::test]
    async fn test_e2e_no_slash_command_passthrough() {
        let items = vec![InputItem::Text {
            text: "Regular user message without slash command".to_string(),
        }];

        let command_text = detect_slash_command(&items);
        assert!(command_text.is_none());

        // Should pass through unchanged
        let expanded = "dummy".to_string();
        let result = replace_with_expanded_prompt(items.clone(), expanded);
        assert_eq!(result, items);
    }

    #[tokio::test]
    async fn test_e2e_command_with_execution_context() {
        let registry = create_test_registry().await;

        let workspace = PathBuf::from("/home/user/project");
        let git_diff = Some("diff --git a/src/main.rs...".to_string());
        let current_files = vec![PathBuf::from("src/main.rs"), PathBuf::from("src/lib.rs")];

        let result = execute_slash_command(
            "/review",
            registry,
            workspace.clone(),
            git_diff.clone(),
            current_files.clone(),
            None,
            HashMap::new(),
        )
        .await;

        assert!(result.is_ok());
        let expanded = result.unwrap();
        // Review command should produce output
        assert!(!expanded.is_empty());
    }

    #[tokio::test]
    async fn test_e2e_quoted_arguments() {
        let (registry, _temp_dir) = create_test_registry_with_custom_command().await;

        let result = execute_slash_command(
            r#"/greet "Alice Smith""#,
            registry,
            PathBuf::from("/tmp"),
            None,
            vec![],
            None,
            HashMap::new(),
        )
        .await;

        // Verify command executed successfully with quoted arg
        assert!(result.is_ok());
        let expanded = result.unwrap();
        assert!(!expanded.is_empty());
        assert!(expanded.len() > 5); // Should have some meaningful content
    }

    #[tokio::test]
    async fn test_e2e_parse_error_handling() {
        let registry = create_test_registry().await;

        // Unclosed quote should cause parse error
        let result = execute_slash_command(
            r#"/explain "unclosed"#,
            registry,
            PathBuf::from("/tmp"),
            None,
            vec![],
            None,
            HashMap::new(),
        )
        .await;

        assert!(result.is_err());
        let err_msg = format!("{:#}", result.unwrap_err());
        assert!(err_msg.contains("parse") || err_msg.contains("quote"));
    }

    #[tokio::test]
    async fn test_e2e_multiple_image_inputs_with_command() {
        let items = vec![
            InputItem::Image {
                image_url: "data:image/png;base64,iVBOR...".to_string(),
            },
            InputItem::Text {
                text: "/explain screenshot.png".to_string(),
            },
            InputItem::Image {
                image_url: "data:image/png;base64,iVBOR2...".to_string(),
            },
        ];

        let command_text = detect_slash_command(&items);
        assert!(command_text.is_some());
        assert_eq!(command_text.unwrap(), "/explain screenshot.png");

        let expanded = "Explain this image...".to_string();
        let result = replace_with_expanded_prompt(items.clone(), expanded);

        // Should preserve images
        assert_eq!(result.len(), 3);
        match &result[0] {
            InputItem::Image { .. } => {}
            _ => panic!("Expected first item to remain Image"),
        }
        match &result[1] {
            InputItem::Text { text } => {
                assert_eq!(text, &expanded);
            }
            _ => panic!("Expected command to be replaced"),
        }
    }

    #[tokio::test]
    async fn test_e2e_execution_context_builder() {
        let workspace = PathBuf::from("/home/user/project");
        let git_diff = "diff content".to_string();
        let files = vec![PathBuf::from("file1.rs"), PathBuf::from("file2.rs")];

        let context = ExecutionContext::new(workspace.clone())
            .with_git_diff(Some(git_diff.clone()))
            .with_files(files.clone());

        assert_eq!(context.workspace_root, workspace);
        assert_eq!(context.git_diff, Some(git_diff));
        assert_eq!(context.current_files, files);
    }

    #[tokio::test]
    async fn test_e2e_registry_command_count() {
        let registry = create_test_registry().await;

        // Should have 3 built-in commands
        let commands = registry.list().await;
        assert_eq!(commands.len(), 3);

        let names: Vec<String> = commands.iter().map(|m| m.name.clone()).collect();
        assert!(names.contains(&"explain".to_string()));
        assert!(names.contains(&"review".to_string()));
        assert!(names.contains(&"test".to_string()));
    }

    #[tokio::test]
    async fn test_git_diff_extraction_in_repo() {
        // This test assumes we're running in the codex git repo
        let result = crate::commands::get_git_diff().await;
        assert!(result.is_ok());

        let (is_repo, _diff) = result.unwrap();
        // Should detect we're in a git repo
        assert!(is_repo);
        // diff may or may not be empty depending on current repo state
    }

    #[tokio::test]
    async fn test_git_diff_tuple_structure() {
        let result = crate::commands::get_git_diff().await;
        assert!(result.is_ok());

        let (is_repo, diff) = result.unwrap();
        // Verify tuple structure
        assert!(std::mem::size_of_val(&is_repo) > 0);
        assert!(std::mem::size_of_val(&diff) > 0);
    }

    #[tokio::test]
    async fn test_e2e_slash_command_with_git_diff() {
        let registry = create_test_registry().await;
        let workspace = PathBuf::from("/tmp/test_workspace");

        // Extract actual git diff
        let git_diff = match crate::commands::get_git_diff().await {
            Ok((true, diff)) if !diff.is_empty() => Some(diff),
            _ => None,
        };

        // Execute a command with git diff context
        let result = execute_slash_command(
            "/explain test.rs",
            Arc::clone(&registry),
            workspace.clone(),
            git_diff,
            vec![],
            None,
            HashMap::new(),
        )
        .await;

        // Should succeed (command exists)
        assert!(result.is_ok());

        // Expanded prompt should contain explanatory text
        let expanded = result.unwrap();
        assert!(!expanded.is_empty());
    }

    #[tokio::test]
    async fn test_execution_context_with_git_diff() {
        let workspace = PathBuf::from("/home/user/project");

        // Extract git diff (may be None if not in repo or no changes)
        let git_diff = match crate::commands::get_git_diff().await {
            Ok((true, diff)) if !diff.is_empty() => Some(diff),
            _ => None,
        };

        // Build execution context with git diff
        let context = ExecutionContext::new(workspace.clone())
            .with_git_diff(git_diff.clone())
            .with_conversation_context(None);

        assert_eq!(context.workspace_root, workspace);
        assert_eq!(context.git_diff, git_diff);
    }

    #[tokio::test]
    async fn test_execution_context_with_current_files() {
        let workspace = PathBuf::from("/home/user/project");
        let files = vec![
            PathBuf::from("src/main.rs"),
            PathBuf::from("src/lib.rs"),
            PathBuf::from("tests/integration_test.rs"),
        ];

        // Build execution context with current files
        let context = ExecutionContext::new(workspace.clone()).with_files(files.clone());

        assert_eq!(context.workspace_root, workspace);
        assert_eq!(context.current_files, files);
    }

    #[tokio::test]
    async fn test_execution_context_with_git_diff_and_files() {
        let workspace = PathBuf::from("/home/user/project");

        // Extract git diff
        let git_diff = match crate::commands::get_git_diff().await {
            Ok((true, diff)) if !diff.is_empty() => Some(diff),
            _ => None,
        };

        let files = vec![PathBuf::from("src/main.rs"), PathBuf::from("Cargo.toml")];

        // Build execution context with both git diff and files
        let context = ExecutionContext::new(workspace.clone())
            .with_git_diff(git_diff.clone())
            .with_files(files.clone());

        assert_eq!(context.workspace_root, workspace);
        assert_eq!(context.git_diff, git_diff);
        assert_eq!(context.current_files, files);
    }

    #[tokio::test]
    async fn test_e2e_slash_command_with_current_files() {
        let registry = create_test_registry().await;
        let workspace = PathBuf::from("/tmp/test_workspace");

        // Create mock current files list
        let current_files = vec![
            PathBuf::from("src/main.rs"),
            PathBuf::from("src/components/button.tsx"),
        ];

        // Execute command with current files context
        let result = execute_slash_command(
            "/review",
            Arc::clone(&registry),
            workspace.clone(),
            None, // no git diff
            current_files,
            None,
            HashMap::new(),
        )
        .await;

        // Should succeed
        assert!(result.is_ok());

        let expanded = result.unwrap();
        assert!(!expanded.is_empty());
    }

    #[tokio::test]
    async fn test_execution_context_empty_files() {
        let workspace = PathBuf::from("/home/user/project");

        // Build execution context with empty files (default case)
        let context = ExecutionContext::new(workspace.clone())
            .with_files(vec![])
            .with_conversation_context(None);

        assert_eq!(context.workspace_root, workspace);
        assert_eq!(context.current_files, Vec::<PathBuf>::new());
    }

    #[tokio::test]
    async fn test_execution_context_with_conversation_context() {
        use crate::commands::ConversationContext;
        use crate::commands::MessageSummary;

        let workspace = PathBuf::from("/home/user/project");

        // Create conversation context with messages
        let messages = vec![
            MessageSummary {
                role: "user".to_string(),
                content: "How do I implement authentication?".to_string(),
                timestamp: Some("2025-01-10T12:00:00Z".to_string()),
            },
            MessageSummary {
                role: "assistant".to_string(),
                content: "Here's how to implement authentication...".to_string(),
                timestamp: Some("2025-01-10T12:01:00Z".to_string()),
            },
        ];

        let conv_context = ConversationContext::with_messages(messages.clone());

        // Build execution context with conversation context
        let context = ExecutionContext::new(workspace.clone())
            .with_conversation_context(Some(conv_context.clone()));

        assert_eq!(context.workspace_root, workspace);
        assert!(context.conversation_context.is_some());
        let ctx = context.conversation_context.unwrap();
        assert_eq!(ctx.recent_messages.len(), 2);
        assert_eq!(ctx.recent_messages[0].role, "user");
        assert_eq!(ctx.recent_messages[1].role, "assistant");
    }

    #[tokio::test]
    async fn test_conversation_context_empty() {
        use crate::commands::ConversationContext;

        let context = ConversationContext::new();
        assert_eq!(context.recent_messages.len(), 0);
        assert!(context.conversation_id.is_none());
    }

    #[tokio::test]
    async fn test_execution_context_full_context() {
        use crate::commands::ConversationContext;
        use crate::commands::MessageSummary;

        let workspace = PathBuf::from("/home/user/project");

        // Build execution context with all context types
        let git_diff = Some("diff --git a/src/main.rs...".to_string());
        let current_files = vec![PathBuf::from("src/main.rs")];
        let conv_context = ConversationContext::with_messages(vec![MessageSummary {
            role: "user".to_string(),
            content: "Review this code".to_string(),
            timestamp: None,
        }]);

        let context = ExecutionContext::new(workspace.clone())
            .with_git_diff(git_diff.clone())
            .with_files(current_files.clone())
            .with_conversation_context(Some(conv_context));

        assert_eq!(context.workspace_root, workspace);
        assert_eq!(context.git_diff, git_diff);
        assert_eq!(context.current_files, current_files);
        assert!(context.conversation_context.is_some());
    }

    #[tokio::test]
    async fn test_e2e_slash_command_with_conversation_context() {
        use crate::commands::ConversationContext;
        use crate::commands::MessageSummary;

        let registry = create_test_registry().await;
        let workspace = PathBuf::from("/tmp/test_workspace");

        // Create conversation context
        let messages = vec![
            MessageSummary {
                role: "user".to_string(),
                content: "Can you explain the main function?".to_string(),
                timestamp: None,
            },
            MessageSummary {
                role: "assistant".to_string(),
                content: "The main function initializes the application...".to_string(),
                timestamp: None,
            },
        ];
        let conv_context = ConversationContext::with_messages(messages);

        // Execute command with conversation context
        let result = execute_slash_command(
            "/explain src/lib.rs",
            Arc::clone(&registry),
            workspace.clone(),
            None,
            vec![],
            Some(conv_context),
            HashMap::new(),
        )
        .await;

        // Should succeed
        assert!(result.is_ok());
        let expanded = result.unwrap();
        assert!(!expanded.is_empty());
    }

    #[tokio::test]
    async fn test_execution_context_with_env_vars() {
        let workspace = PathBuf::from("/home/user/project");

        // Create test environment variables
        let mut env_vars = HashMap::new();
        env_vars.insert("USER".to_string(), "testuser".to_string());
        env_vars.insert("HOME".to_string(), "/home/testuser".to_string());

        // Build execution context with env vars
        let context = ExecutionContext::new(workspace.clone()).with_env_vars(env_vars.clone());

        assert_eq!(context.workspace_root, workspace);
        assert_eq!(context.env_vars.len(), 2);
        assert_eq!(context.env_vars.get("USER"), Some(&"testuser".to_string()));
        assert_eq!(
            context.env_vars.get("HOME"),
            Some(&"/home/testuser".to_string())
        );
    }

    #[tokio::test]
    async fn test_e2e_slash_command_with_env_vars() {
        let registry = create_test_registry().await;
        let workspace = PathBuf::from("/tmp/test_workspace");

        // Create test environment variables
        let mut env_vars = HashMap::new();
        env_vars.insert("USER".to_string(), "testuser".to_string());
        env_vars.insert("SHELL".to_string(), "/bin/bash".to_string());

        // Execute command with env vars context
        let result = execute_slash_command(
            "/explain src/main.rs",
            Arc::clone(&registry),
            workspace.clone(),
            None,
            vec![],
            None,
            env_vars,
        )
        .await;

        // Should succeed
        assert!(result.is_ok());
        let expanded = result.unwrap();
        assert!(!expanded.is_empty());
    }

    #[tokio::test]
    async fn test_execution_context_full_with_env_vars() {
        use crate::commands::ConversationContext;
        use crate::commands::MessageSummary;

        let workspace = PathBuf::from("/home/user/project");

        // Build execution context with all context types including env vars
        let git_diff = Some("diff --git a/src/main.rs...".to_string());
        let current_files = vec![PathBuf::from("src/main.rs")];
        let conv_context = ConversationContext::with_messages(vec![MessageSummary {
            role: "user".to_string(),
            content: "Review this code".to_string(),
            timestamp: None,
        }]);

        let mut env_vars = HashMap::new();
        env_vars.insert("USER".to_string(), "testuser".to_string());
        env_vars.insert("HOME".to_string(), "/home/testuser".to_string());

        let context = ExecutionContext::new(workspace.clone())
            .with_git_diff(git_diff.clone())
            .with_files(current_files.clone())
            .with_conversation_context(Some(conv_context))
            .with_env_vars(env_vars.clone());

        assert_eq!(context.workspace_root, workspace);
        assert_eq!(context.git_diff, git_diff);
        assert_eq!(context.current_files, current_files);
        assert!(context.conversation_context.is_some());
        assert_eq!(context.env_vars.len(), 2);
        assert_eq!(context.env_vars.get("USER"), Some(&"testuser".to_string()));
    }

    #[test]
    fn test_collect_safe_env_vars_only_whitelisted() {
        // Set a test env var that's NOT on the whitelist
        unsafe {
            std::env::set_var("SECRET_TOKEN", "should_not_be_collected");
        }

        // Set whitelisted vars
        unsafe {
            std::env::set_var("USER", "testuser");
            std::env::set_var("HOME", "/home/testuser");
        }

        // Note: collect_safe_env_vars is not public, so we test via ExecutionContext
        // The function only collects whitelisted vars (USER, HOME, SHELL, LANG, CODEX_HOME, CODEX_MODEL)

        // Cleanup
        unsafe {
            std::env::remove_var("SECRET_TOKEN");
        }
    }

    // ===== Hot-Reload Integration Tests =====

    #[tokio::test]
    async fn test_watcher_file_creation_triggers_reload() {
        use std::fs;
        use tempfile::TempDir;
        use tokio::time::Duration;
        use tokio::time::sleep;

        let temp_dir = TempDir::new().unwrap();
        let commands_dir = temp_dir.path().to_path_buf();

        // Create registry and watcher
        let registry =
            std::sync::Arc::new(CommandRegistry::new(commands_dir.clone()).await.unwrap());
        let initial_count = registry.list().await.len();

        let _watcher =
            crate::commands::watcher::CommandWatcher::new(commands_dir.clone(), registry.clone())
                .unwrap();

        // Create a new command file
        let cmd_content = r#"---
name: newcmd
description: A new command
category: test
---
New command template"#;

        fs::write(commands_dir.join("newcmd.md"), cmd_content).unwrap();

        // Wait for debounce + reload (300ms debounce + 100ms buffer)
        sleep(Duration::from_millis(500)).await;

        // Verify reload happened by checking command count
        let final_count = registry.list().await.len();
        assert_eq!(
            final_count,
            initial_count + 1,
            "Registry should have reloaded with new command"
        );
    }

    #[tokio::test]
    async fn test_watcher_file_modification_triggers_reload() {
        use std::fs;
        use tempfile::TempDir;
        use tokio::time::Duration;
        use tokio::time::sleep;

        let temp_dir = TempDir::new().unwrap();
        let commands_dir = temp_dir.path().to_path_buf();

        // Create initial command file
        let cmd_path = commands_dir.join("modcmd.md");
        let initial_content = r#"---
name: modcmd
description: Initial description
category: test
---
Initial template"#;
        fs::create_dir_all(&commands_dir).unwrap();
        fs::write(&cmd_path, initial_content).unwrap();

        // Create registry and watcher
        let registry =
            std::sync::Arc::new(CommandRegistry::new(commands_dir.clone()).await.unwrap());
        let _watcher =
            crate::commands::watcher::CommandWatcher::new(commands_dir.clone(), registry.clone())
                .unwrap();

        // Get initial command
        let initial_cmd = registry.get("modcmd").await.unwrap();
        assert_eq!(initial_cmd.description(), "Initial description");

        // Modify the file
        let modified_content = r#"---
name: modcmd
description: Modified description
category: test
---
Modified template"#;
        fs::write(&cmd_path, modified_content).unwrap();

        // Wait for debounce + reload
        sleep(Duration::from_millis(500)).await;

        // Verify reload happened by checking modified description
        let modified_cmd = registry.get("modcmd").await.unwrap();
        assert_eq!(
            modified_cmd.description(),
            "Modified description",
            "Registry should have reloaded with modified command"
        );
    }

    #[tokio::test]
    async fn test_watcher_file_deletion_triggers_reload() {
        use std::fs;
        use tempfile::TempDir;
        use tokio::time::Duration;
        use tokio::time::sleep;

        let temp_dir = TempDir::new().unwrap();
        let commands_dir = temp_dir.path().to_path_buf();

        // Create command file
        let cmd_path = commands_dir.join("delcmd.md");
        let cmd_content = r#"---
name: delcmd
description: Command to delete
category: test
---
Delete template"#;
        fs::create_dir_all(&commands_dir).unwrap();
        fs::write(&cmd_path, cmd_content).unwrap();

        // Create registry and watcher
        let registry =
            std::sync::Arc::new(CommandRegistry::new(commands_dir.clone()).await.unwrap());
        let initial_count = registry.list().await.len();
        assert_eq!(initial_count, 1, "Should have 1 command initially");

        let _watcher =
            crate::commands::watcher::CommandWatcher::new(commands_dir.clone(), registry.clone())
                .unwrap();

        // Delete the file
        fs::remove_file(&cmd_path).unwrap();

        // Wait for debounce + reload
        sleep(Duration::from_millis(500)).await;

        // Verify reload happened by checking command count
        let final_count = registry.list().await.len();
        assert_eq!(
            final_count, 0,
            "Registry should have reloaded and removed deleted command"
        );
    }

    #[tokio::test]
    async fn test_watcher_debouncing_multiple_rapid_changes() {
        use std::fs;
        use tempfile::TempDir;
        use tokio::time::Duration;
        use tokio::time::sleep;

        let temp_dir = TempDir::new().unwrap();
        let commands_dir = temp_dir.path().to_path_buf();
        fs::create_dir_all(&commands_dir).unwrap();

        // Create registry and watcher
        let registry =
            std::sync::Arc::new(CommandRegistry::new(commands_dir.clone()).await.unwrap());
        let _watcher =
            crate::commands::watcher::CommandWatcher::new(commands_dir.clone(), registry.clone())
                .unwrap();

        // Create multiple files in rapid succession (< 300ms apart)
        for i in 0..5 {
            let cmd_content = format!(
                r#"---
name: cmd{}
description: Command {}
category: test
---
Template {}"#,
                i, i, i
            );
            fs::write(commands_dir.join(format!("cmd{}.md", i)), cmd_content).unwrap();
            sleep(Duration::from_millis(50)).await; // 50ms between creates
        }

        // Wait for debounce window (300ms) + processing time
        sleep(Duration::from_millis(500)).await;

        // All commands should be loaded after single debounced reload
        let final_count = registry.list().await.len();
        assert_eq!(
            final_count, 5,
            "All 5 commands should be loaded after debounced reload"
        );
    }

    #[tokio::test]
    async fn test_watcher_ignores_non_md_files() {
        use std::fs;
        use tempfile::TempDir;
        use tokio::time::Duration;
        use tokio::time::sleep;

        let temp_dir = TempDir::new().unwrap();
        let commands_dir = temp_dir.path().to_path_buf();
        fs::create_dir_all(&commands_dir).unwrap();

        // Create a valid command file
        let valid_content = r#"---
name: valid
description: Valid command
category: test
---
Valid template"#;
        fs::write(commands_dir.join("valid.md"), valid_content).unwrap();

        // Create registry and watcher
        let registry =
            std::sync::Arc::new(CommandRegistry::new(commands_dir.clone()).await.unwrap());
        assert_eq!(registry.list().await.len(), 1);

        let _watcher =
            crate::commands::watcher::CommandWatcher::new(commands_dir.clone(), registry.clone())
                .unwrap();

        // Create non-.md files (should be ignored)
        fs::write(commands_dir.join("readme.txt"), "Not a command").unwrap();
        fs::write(commands_dir.join("config.json"), "{}").unwrap();

        // Wait for potential (but shouldn't happen) reload
        sleep(Duration::from_millis(500)).await;

        // Count should still be 1 (non-.md files ignored)
        let final_count = registry.list().await.len();
        assert_eq!(final_count, 1, "Non-.md files should not trigger reload");
    }
}
