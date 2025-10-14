//! Template expansion engine for command prompts.

use handlebars::Handlebars;
use serde_json::json;

use super::context::CommandContext;

/// Template expander for command prompts.
///
/// Supports variable interpolation and conditional logic using
/// Handlebars-compatible syntax.
pub struct TemplateExpander {
    handlebars: Handlebars<'static>,
}

impl TemplateExpander {
    /// Creates a new template expander.
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false);

        Self { handlebars }
    }

    /// Expands a template with the given context.
    ///
    /// # Arguments
    /// * `template` - Template string with {{variable}} placeholders
    /// * `context` - Context containing variable values
    ///
    /// # Example
    /// ```ignore
    /// let expander = TemplateExpander::new();
    /// let context = CommandContext::default();
    /// let result = expander.expand("Hello {{name}}", &context)?;
    /// ```
    pub fn expand(&self, template: &str, context: &CommandContext) -> anyhow::Result<String> {
        // Build template data from context
        let mut data = json!({
            "args": context.args,
            "git_diff": context.git_diff,
            "files": context.files.iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<_>>(),
            "workspace_root": context.workspace_root.to_string_lossy().to_string(),
            "env": context.env_vars,
        });

        // Add conversation context if available
        if let Some(conv_ctx) = &context.conversation_context {
            let conversation = json!({
                "messages": conv_ctx.recent_messages.iter().map(|msg| {
                    json!({
                        "role": msg.role,
                        "content": msg.content,
                        "timestamp": msg.timestamp,
                    })
                }).collect::<Vec<_>>(),
                "conversation_id": conv_ctx.conversation_id,
            });
            if let Some(map) = data.as_object_mut() {
                map.insert("conversation".to_string(), conversation);
            }
        }

        // Render template
        self.handlebars
            .render_template(template, &data)
            .map_err(|e| anyhow::anyhow!("Template expansion failed: {}", e))
    }
}

impl Default for TemplateExpander {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn test_basic_expansion() {
        let expander = TemplateExpander::new();
        let context = CommandContext::default();
        let result = expander.expand("Hello world", &context).unwrap();
        assert_eq!(result, "Hello world");
    }

    #[test]
    fn test_variable_interpolation() {
        let expander = TemplateExpander::new();
        let mut args = HashMap::new();
        args.insert("name".to_string(), "Codex".to_string());
        args.insert("version".to_string(), "1.0".to_string());

        let context = CommandContext {
            args,
            env_vars: HashMap::new(),
            conversation_context: None,
            ..Default::default()
        };

        let result = expander
            .expand("Hello {{args.name}} v{{args.version}}", &context)
            .unwrap();

        assert_eq!(result, "Hello Codex v1.0");
    }

    #[test]
    fn test_git_diff_expansion() {
        let expander = TemplateExpander::new();
        let context = CommandContext {
            git_diff: Some("+ added line\n- removed line".to_string()),
            env_vars: HashMap::new(),
            conversation_context: None,
            ..Default::default()
        };

        let result = expander
            .expand("Git diff:\n{{git_diff}}", &context)
            .unwrap();

        assert!(result.contains("+ added line"));
        assert!(result.contains("- removed line"));
    }

    #[test]
    fn test_files_expansion() {
        let expander = TemplateExpander::new();
        let context = CommandContext {
            files: vec![PathBuf::from("src/main.rs"), PathBuf::from("README.md")],
            env_vars: HashMap::new(),
            conversation_context: None,
            ..Default::default()
        };

        let result = expander
            .expand("Files: {{#each files}}{{this}}, {{/each}}", &context)
            .unwrap();

        assert!(result.contains("src/main.rs"));
        assert!(result.contains("README.md"));
    }

    #[test]
    fn test_workspace_root_expansion() {
        let expander = TemplateExpander::new();
        let context = CommandContext {
            workspace_root: PathBuf::from("/home/user/project"),
            env_vars: HashMap::new(),
            conversation_context: None,
            ..Default::default()
        };

        let result = expander
            .expand("Working in: {{workspace_root}}", &context)
            .unwrap();

        assert!(result.contains("/home/user/project"));
    }

    #[test]
    fn test_conditional_expansion() {
        let expander = TemplateExpander::new();

        // With git_diff
        let with_diff = CommandContext {
            git_diff: Some("changes".to_string()),
            env_vars: HashMap::new(),
            conversation_context: None,
            ..Default::default()
        };

        let result = expander
            .expand(
                "{{#if git_diff}}Has diff: {{git_diff}}{{else}}No diff{{/if}}",
                &with_diff,
            )
            .unwrap();

        assert_eq!(result, "Has diff: changes");

        // Without git_diff
        let without_diff = CommandContext::default();

        let result = expander
            .expand(
                "{{#if git_diff}}Has diff{{else}}No diff{{/if}}",
                &without_diff,
            )
            .unwrap();

        assert_eq!(result, "No diff");
    }

    #[test]
    fn test_missing_variable_non_strict() {
        let expander = TemplateExpander::new();
        let context = CommandContext::default();

        // Non-strict mode should not fail on missing variables
        let result = expander
            .expand("Value: {{args.missing}}", &context)
            .unwrap();

        // Should render empty string for missing variable
        assert_eq!(result, "Value: ");
    }

    #[test]
    fn test_complex_template() {
        let expander = TemplateExpander::new();
        let mut args = HashMap::new();
        args.insert("file".to_string(), "main.rs".to_string());
        args.insert("line".to_string(), "42".to_string());

        let context = CommandContext {
            args,
            files: vec![PathBuf::from("src/main.rs")],
            git_diff: Some("+ new code".to_string()),
            workspace_root: PathBuf::from("/project"),
            env_vars: HashMap::new(),
            conversation_context: None,
        };

        let template = r#"
Analyze {{args.file}} at line {{args.line}}

Files:
{{#each files}}- {{this}}
{{/each}}
{{#if git_diff}}
Changes:
{{git_diff}}
{{/if}}

Root: {{workspace_root}}
"#;

        let result = expander.expand(template, &context).unwrap();

        assert!(result.contains("Analyze main.rs at line 42"));
        assert!(result.contains("- src/main.rs"));
        assert!(result.contains("Changes:"));
        assert!(result.contains("+ new code"));
        assert!(result.contains("Root: /project"));
    }

    #[test]
    fn test_env_vars_expansion() {
        let expander = TemplateExpander::new();
        let mut env_vars = HashMap::new();
        env_vars.insert("USER".to_string(), "testuser".to_string());
        env_vars.insert("HOME".to_string(), "/home/testuser".to_string());

        let context = CommandContext {
            env_vars,
            ..Default::default()
        };

        let result = expander
            .expand("User: {{env.USER}}, Home: {{env.HOME}}", &context)
            .unwrap();

        assert_eq!(result, "User: testuser, Home: /home/testuser");
    }

    #[test]
    fn test_conversation_context_expansion() {
        use super::super::executor::ConversationContext;
        use super::super::executor::MessageSummary;

        let expander = TemplateExpander::new();
        let messages = vec![
            MessageSummary {
                role: "user".to_string(),
                content: "Hello".to_string(),
                timestamp: Some("2024-01-01T00:00:00Z".to_string()),
            },
            MessageSummary {
                role: "assistant".to_string(),
                content: "Hi there!".to_string(),
                timestamp: None,
            },
        ];
        let conv_context = ConversationContext {
            recent_messages: messages,
            conversation_id: Some("conv-123".to_string()),
        };

        let context = CommandContext {
            conversation_context: Some(conv_context),
            ..Default::default()
        };

        let template = r#"
Conversation ID: {{conversation.conversation_id}}
Messages:
{{#each conversation.messages}}
- {{this.role}}: {{this.content}}
{{/each}}
"#;

        let result = expander.expand(template, &context).unwrap();

        assert!(result.contains("Conversation ID: conv-123"));
        assert!(result.contains("user: Hello"));
        assert!(result.contains("assistant: Hi there!"));
    }

    #[test]
    fn test_conversation_without_id() {
        use super::super::executor::ConversationContext;
        use super::super::executor::MessageSummary;

        let expander = TemplateExpander::new();
        let messages = vec![MessageSummary {
            role: "user".to_string(),
            content: "Test message".to_string(),
            timestamp: None,
        }];
        let conv_context = ConversationContext {
            recent_messages: messages,
            conversation_id: None,
        };

        let context = CommandContext {
            conversation_context: Some(conv_context),
            ..Default::default()
        };

        let template = "{{#if conversation.conversation_id}}ID: {{conversation.conversation_id}}{{else}}No ID{{/if}}";

        let result = expander.expand(template, &context).unwrap();

        assert_eq!(result, "No ID");
    }

    #[test]
    fn test_comprehensive_template_with_all_variables() {
        use super::super::executor::ConversationContext;
        use super::super::executor::MessageSummary;

        let expander = TemplateExpander::new();

        let mut args = HashMap::new();
        args.insert("task".to_string(), "review".to_string());

        let mut env_vars = HashMap::new();
        env_vars.insert("USER".to_string(), "developer".to_string());

        let messages = vec![MessageSummary {
            role: "user".to_string(),
            content: "Please review this code".to_string(),
            timestamp: None,
        }];
        let conv_context = ConversationContext {
            recent_messages: messages,
            conversation_id: Some("conv-456".to_string()),
        };

        let context = CommandContext {
            args,
            files: vec![PathBuf::from("src/app.rs")],
            git_diff: Some("+ new feature".to_string()),
            workspace_root: PathBuf::from("/workspace"),
            env_vars,
            conversation_context: Some(conv_context),
        };

        let template = r#"
Task: {{args.task}}
User: {{env.USER}}
Files: {{#each files}}{{this}} {{/each}}
{{#if git_diff}}Changes: {{git_diff}}{{/if}}
Root: {{workspace_root}}
{{#if conversation}}Conv: {{conversation.conversation_id}}{{/if}}
"#;

        let result = expander.expand(template, &context).unwrap();

        assert!(result.contains("Task: review"));
        assert!(result.contains("User: developer"));
        assert!(result.contains("src/app.rs"));
        assert!(result.contains("Changes: + new feature"));
        assert!(result.contains("Root: /workspace"));
        assert!(result.contains("Conv: conv-456"));
    }
}
