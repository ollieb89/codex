//! Integration layer for command system with exec_command flow.
//!
//! This module provides the bridge between the command system and the
//! Codex execution flow, handling slash command detection and execution.

use super::executor::{CommandExecutor, ExecutionContext};
use super::invocation::InvocationParser;
use super::registry::CommandRegistry;
use crate::protocol::InputItem;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;

/// Checks if the given input items contain a slash command.
///
/// Returns `Some(command_text)` if a slash command is detected,
/// `None` otherwise.
pub fn detect_slash_command(items: &[InputItem]) -> Option<String> {
    // Only check the first text item for slash command
    items.iter().find_map(|item| match item {
        InputItem::Text { text } => {
            let trimmed = text.trim();
            if trimmed.starts_with('/') && trimmed.len() > 1 {
                Some(trimmed.to_string())
            } else {
                None
            }
        }
        _ => None,
    })
}

/// Executes a slash command and returns the expanded prompt.
///
/// # Arguments
///
/// * `command_text` - The slash command string (e.g., "/explain src/main.rs")
/// * `registry` - Command registry to look up commands
/// * `workspace_root` - Current workspace directory
/// * `git_diff` - Optional git diff context
/// * `current_files` - Currently open/selected files
/// * `conversation_context` - Optional conversation history context
/// * `env_vars` - Environment variables (whitelisted for security)
///
/// # Returns
///
/// The expanded prompt string ready for LLM input.
pub async fn execute_slash_command(
    command_text: &str,
    registry: Arc<CommandRegistry>,
    workspace_root: PathBuf,
    git_diff: Option<String>,
    current_files: Vec<PathBuf>,
    conversation_context: Option<super::ConversationContext>,
    env_vars: std::collections::HashMap<String, String>,
) -> Result<String> {
    // Parse the slash command
    let invocation =
        InvocationParser::parse(command_text).context("Failed to parse slash command")?;

    // Build execution context
    let exec_context = ExecutionContext::new(workspace_root)
        .with_git_diff(git_diff)
        .with_files(current_files)
        .with_conversation_context(conversation_context)
        .with_env_vars(env_vars);

    // Execute via CommandExecutor
    let executor = CommandExecutor::new(registry);
    let expanded_prompt = executor
        .execute(invocation, &exec_context)
        .await
        .context("Failed to execute slash command")?;

    Ok(expanded_prompt)
}

/// Replaces slash command input with expanded prompt.
///
/// Takes the original input items and replaces the slash command
/// with the expanded prompt text.
pub fn replace_with_expanded_prompt(
    items: Vec<InputItem>,
    expanded_prompt: String,
) -> Vec<InputItem> {
    let mut result = Vec::new();
    let mut replaced = false;

    for item in items {
        match item {
            InputItem::Text { text } if !replaced && text.trim().starts_with('/') => {
                // Replace slash command with expanded prompt
                result.push(InputItem::Text {
                    text: expanded_prompt.clone(),
                });
                replaced = true;
            }
            other => result.push(other),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_slash_command_present() {
        let items = vec![InputItem::Text {
            text: "/explain src/main.rs".to_string(),
        }];

        let result = detect_slash_command(&items);
        assert_eq!(result, Some("/explain src/main.rs".to_string()));
    }

    #[test]
    fn test_detect_slash_command_with_whitespace() {
        let items = vec![InputItem::Text {
            text: "  /review   ".to_string(),
        }];

        let result = detect_slash_command(&items);
        assert_eq!(result, Some("/review".to_string()));
    }

    #[test]
    fn test_detect_slash_command_not_at_start() {
        let items = vec![InputItem::Text {
            text: "Please /explain this".to_string(),
        }];

        let result = detect_slash_command(&items);
        assert_eq!(result, None);
    }

    #[test]
    fn test_detect_slash_command_just_slash() {
        let items = vec![InputItem::Text {
            text: "/".to_string(),
        }];

        let result = detect_slash_command(&items);
        assert_eq!(result, None);
    }

    #[test]
    fn test_detect_slash_command_empty() {
        let items = vec![];
        let result = detect_slash_command(&items);
        assert_eq!(result, None);
    }

    #[test]
    fn test_detect_slash_command_with_images() {
        let items = vec![
            InputItem::Image {
                image_url: "data:image/png;base64,...".to_string(),
            },
            InputItem::Text {
                text: "/test".to_string(),
            },
        ];

        let result = detect_slash_command(&items);
        assert_eq!(result, Some("/test".to_string()));
    }

    #[test]
    fn test_replace_with_expanded_prompt() {
        let items = vec![
            InputItem::Text {
                text: "/explain src/main.rs".to_string(),
            },
            InputItem::Text {
                text: "additional context".to_string(),
            },
        ];

        let result =
            replace_with_expanded_prompt(items, "Please explain the following code...".to_string());

        assert_eq!(result.len(), 2);
        match &result[0] {
            InputItem::Text { text } => {
                assert_eq!(text, "Please explain the following code...");
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_replace_preserves_non_command_items() {
        let items = vec![
            InputItem::Image {
                image_url: "data:...".to_string(),
            },
            InputItem::Text {
                text: "regular text".to_string(),
            },
        ];

        let result = replace_with_expanded_prompt(items.clone(), "expanded".to_string());

        // Should not modify non-command items
        assert_eq!(result, items);
    }
}
