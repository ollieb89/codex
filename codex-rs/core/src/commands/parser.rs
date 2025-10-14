//! Markdown command file parser with YAML frontmatter support.

use serde::Deserialize;
use serde::Serialize;

/// Parsed command from a Markdown file.
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub metadata: CommandMetadata,
    pub template: String,
}

/// Command metadata from YAML frontmatter.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandMetadata {
    pub name: String,
    pub description: String,
    pub category: String,
    #[serde(default)]
    pub permissions: CommandPermissions,
    #[serde(default)]
    pub args: Vec<ArgDefinition>,
    /// Whether this command is backed by an agent.
    #[serde(default)]
    pub agent: bool,
    /// The agent ID to use for this command.
    #[serde(default)]
    pub agent_id: Option<String>,
    /// Activation hints for agent selection (keywords for context matching).
    #[serde(default)]
    pub activation_hints: Vec<String>,
}

/// Command permissions model.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandPermissions {
    #[serde(default)]
    pub read_files: bool,
    #[serde(default)]
    pub write_files: bool,
    #[serde(default)]
    pub execute_shell: bool,
}

/// Argument definition for command.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArgDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: ArgType,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub default: Option<String>,
}

/// Argument type.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ArgType {
    String,
    Number,
    Boolean,
    File,
}

/// Command parser for Markdown files with YAML frontmatter.
pub struct CommandParser;

impl CommandParser {
    /// Parses a command from Markdown content.
    ///
    /// # Format
    /// ```markdown
    /// ---
    /// name: command-name
    /// description: Command description
    /// category: analysis
    /// ---
    ///
    /// Command template with {{variables}}
    /// ```
    pub fn parse(content: &str) -> anyhow::Result<ParsedCommand> {
        let (frontmatter, template) = Self::split_frontmatter(content)?;

        let metadata: CommandMetadata = serde_yaml::from_str(&frontmatter)
            .map_err(|e| anyhow::anyhow!("Failed to parse YAML frontmatter: {}", e))?;

        Self::validate_metadata(&metadata)?;

        Ok(ParsedCommand {
            metadata,
            template: template.trim().to_string(),
        })
    }

    /// Validates command metadata.
    fn validate_metadata(metadata: &CommandMetadata) -> anyhow::Result<()> {
        if metadata.name.is_empty() {
            anyhow::bail!("Command name cannot be empty");
        }

        if !metadata
            .name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            anyhow::bail!(
                "Command name '{}' contains invalid characters (only alphanumeric, '-', '_' allowed)",
                metadata.name
            );
        }

        if metadata.description.is_empty() {
            anyhow::bail!("Command description cannot be empty");
        }

        if metadata.category.is_empty() {
            anyhow::bail!("Command category cannot be empty");
        }

        // Validate argument definitions
        for arg in &metadata.args {
            if arg.name.is_empty() {
                anyhow::bail!("Argument name cannot be empty");
            }
            if arg.required && arg.default.is_some() {
                anyhow::bail!(
                    "Argument '{}' cannot be both required and have a default value",
                    arg.name
                );
            }
        }

        Ok(())
    }

    fn split_frontmatter(content: &str) -> anyhow::Result<(String, String)> {
        let lines: Vec<&str> = content.lines().collect();

        if lines.first().is_none_or(|l| l.trim() != "---") {
            anyhow::bail!("Missing frontmatter delimiter");
        }

        let end_idx = lines[1..]
            .iter()
            .position(|l| l.trim() == "---")
            .ok_or_else(|| anyhow::anyhow!("No closing frontmatter delimiter"))?
            + 1;

        let frontmatter = lines[1..end_idx].join("\n");
        let body = lines[end_idx + 1..].join("\n");

        Ok((frontmatter, body))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_frontmatter() {
        let input = r#"---
name: test
---

Body content"#;

        let (frontmatter, body) = CommandParser::split_frontmatter(input).unwrap();
        assert_eq!(frontmatter, "name: test");
        assert!(body.contains("Body content"));
    }

    #[test]
    fn test_parse_valid_command() {
        let input = r#"---
name: explain
description: Explain code functionality
category: analysis
---

Please explain the following code:

{{code}}"#;

        let parsed = CommandParser::parse(input).unwrap();
        assert_eq!(parsed.metadata.name, "explain");
        assert_eq!(parsed.metadata.description, "Explain code functionality");
        assert_eq!(parsed.metadata.category, "analysis");
        assert!(parsed.template.contains("{{code}}"));
    }

    #[test]
    fn test_parse_with_permissions() {
        let input = r#"---
name: file-reader
description: Read files
category: utility
permissions:
  read_files: true
  write_files: false
  execute_shell: false
---

Read: {{file}}"#;

        let parsed = CommandParser::parse(input).unwrap();
        assert!(parsed.metadata.permissions.read_files);
        assert!(!parsed.metadata.permissions.write_files);
        assert!(!parsed.metadata.permissions.execute_shell);
    }

    #[test]
    fn test_parse_with_args() {
        let input = r#"---
name: search
description: Search code
category: utility
args:
  - name: pattern
    type: string
    required: true
    description: Search pattern
  - name: case_sensitive
    type: boolean
    required: false
    default: "false"
---

Search for: {{pattern}}"#;

        let parsed = CommandParser::parse(input).unwrap();
        assert_eq!(parsed.metadata.args.len(), 2);
        assert_eq!(parsed.metadata.args[0].name, "pattern");
        assert!(parsed.metadata.args[0].required);
        assert_eq!(parsed.metadata.args[1].name, "case_sensitive");
        assert!(!parsed.metadata.args[1].required);
    }

    #[test]
    fn test_missing_frontmatter_delimiter() {
        let input = "name: test\n\nBody";
        let result = CommandParser::parse(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Missing frontmatter")
        );
    }

    #[test]
    fn test_missing_closing_delimiter() {
        let input = "---\nname: test\n\nBody";
        let result = CommandParser::parse(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("closing frontmatter")
        );
    }

    #[test]
    fn test_empty_name_validation() {
        let input = r#"---
name: ""
description: Test
category: test
---

Body"#;

        let result = CommandParser::parse(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("name cannot be empty")
        );
    }

    #[test]
    fn test_invalid_name_characters() {
        let input = r#"---
name: "invalid/name"
description: Test
category: test
---

Body"#;

        let result = CommandParser::parse(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid characters")
        );
    }

    #[test]
    fn test_empty_description_validation() {
        let input = r#"---
name: test
description: ""
category: test
---

Body"#;

        let result = CommandParser::parse(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("description cannot be empty")
        );
    }

    #[test]
    fn test_required_arg_with_default_validation() {
        let input = r#"---
name: test
description: Test
category: test
args:
  - name: arg1
    type: string
    required: true
    default: "value"
---

Body"#;

        let result = CommandParser::parse(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("cannot be both required and have a default")
        );
    }

    #[test]
    fn test_invalid_yaml() {
        let input = r#"---
name: test
description: [invalid yaml structure
category: test
---

Body"#;

        let result = CommandParser::parse(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to parse YAML")
        );
    }

    #[test]
    fn test_parse_agent_command() {
        let input = r#"---
name: review
description: Code review with AI agent
category: agents
agent: true
agent_id: code-review
activation_hints:
  - review
  - quality
  - bugs
---

Perform code review on {{files}}"#;

        let parsed = CommandParser::parse(input).unwrap();
        assert_eq!(parsed.metadata.name, "review");
        assert!(parsed.metadata.agent);
        assert_eq!(parsed.metadata.agent_id, Some("code-review".to_string()));
        assert_eq!(parsed.metadata.activation_hints.len(), 3);
        assert!(
            parsed
                .metadata
                .activation_hints
                .contains(&"review".to_string())
        );
        assert!(
            parsed
                .metadata
                .activation_hints
                .contains(&"quality".to_string())
        );
        assert!(
            parsed
                .metadata
                .activation_hints
                .contains(&"bugs".to_string())
        );
    }

    #[test]
    fn test_parse_non_agent_command_defaults() {
        let input = r#"---
name: explain
description: Explain code functionality
category: analysis
---

Explain: {{code}}"#;

        let parsed = CommandParser::parse(input).unwrap();
        assert_eq!(parsed.metadata.name, "explain");
        assert!(!parsed.metadata.agent); // Defaults to false
        assert_eq!(parsed.metadata.agent_id, None); // Defaults to None
        assert!(parsed.metadata.activation_hints.is_empty()); // Defaults to empty vec
    }

    #[test]
    fn test_parse_agent_command_without_hints() {
        let input = r#"---
name: refactor
description: Refactoring assistant
category: agents
agent: true
agent_id: refactor-agent
---

Refactor {{code}}"#;

        let parsed = CommandParser::parse(input).unwrap();
        assert!(parsed.metadata.agent);
        assert_eq!(parsed.metadata.agent_id, Some("refactor-agent".to_string()));
        assert!(parsed.metadata.activation_hints.is_empty());
    }

    #[test]
    fn test_agent_command_with_partial_metadata() {
        let input = r#"---
name: analyze
description: Code analysis
category: agents
agent: true
---

Analyze {{code}}"#;

        let parsed = CommandParser::parse(input).unwrap();
        assert!(parsed.metadata.agent);
        assert_eq!(parsed.metadata.agent_id, None); // Optional field
        assert!(parsed.metadata.activation_hints.is_empty());
    }

    #[test]
    fn test_agent_id_without_agent_flag() {
        let input = r#"---
name: test
description: Test command
category: testing
agent_id: some-agent
---

Test {{code}}"#;

        let parsed = CommandParser::parse(input).unwrap();
        assert!(!parsed.metadata.agent); // Defaults to false
        assert_eq!(parsed.metadata.agent_id, Some("some-agent".to_string()));
    }

    #[test]
    fn test_activation_hints_single_item() {
        let input = r#"---
name: fix
description: Fix issues
category: agents
agent: true
agent_id: fix-agent
activation_hints:
  - fix
---

Fix {{issue}}"#;

        let parsed = CommandParser::parse(input).unwrap();
        assert_eq!(parsed.metadata.activation_hints.len(), 1);
        assert_eq!(parsed.metadata.activation_hints[0], "fix");
    }

    #[test]
    fn test_activation_hints_multiple_items() {
        let input = r#"---
name: optimize
description: Performance optimization
category: agents
agent: true
agent_id: perf-agent
activation_hints:
  - optimize
  - performance
  - speed
  - efficiency
---

Optimize {{code}}"#;

        let parsed = CommandParser::parse(input).unwrap();
        assert_eq!(parsed.metadata.activation_hints.len(), 4);
        assert!(
            parsed
                .metadata
                .activation_hints
                .contains(&"optimize".to_string())
        );
        assert!(
            parsed
                .metadata
                .activation_hints
                .contains(&"performance".to_string())
        );
        assert!(
            parsed
                .metadata
                .activation_hints
                .contains(&"speed".to_string())
        );
        assert!(
            parsed
                .metadata
                .activation_hints
                .contains(&"efficiency".to_string())
        );
    }

    #[test]
    fn test_parse_actual_review_command() {
        // Test parsing actual review.md template file structure
        let input = r#"---
name: review
description: Comprehensive code review with AI agent
category: agents
agent: true
agent_id: code-review
activation_hints:
  - review
  - quality
  - bugs
  - issues
  - analysis
permissions:
  read_files: true
  write_files: false
  execute_shell: false
args:
  - name: files
    type: string
    required: true
    description: Files or directories to review
  - name: focus
    type: string
    required: false
    description: Review focus area
    default: "general"
---

# Code Review Agent

Analyze the code at: {{files}}"#;

        let parsed = CommandParser::parse(input).unwrap();

        // Verify command metadata
        assert_eq!(parsed.metadata.name, "review");
        assert_eq!(
            parsed.metadata.description,
            "Comprehensive code review with AI agent"
        );
        assert_eq!(parsed.metadata.category, "agents");

        // Verify agent fields
        assert!(parsed.metadata.agent);
        assert_eq!(parsed.metadata.agent_id, Some("code-review".to_string()));
        assert_eq!(parsed.metadata.activation_hints.len(), 5);

        // Verify permissions
        assert!(parsed.metadata.permissions.read_files);
        assert!(!parsed.metadata.permissions.write_files);
        assert!(!parsed.metadata.permissions.execute_shell);

        // Verify arguments
        assert_eq!(parsed.metadata.args.len(), 2);
        assert_eq!(parsed.metadata.args[0].name, "files");
        assert!(parsed.metadata.args[0].required);
        assert_eq!(parsed.metadata.args[1].name, "focus");
        assert!(!parsed.metadata.args[1].required);
        assert_eq!(parsed.metadata.args[1].default, Some("general".to_string()));

        // Verify template content
        assert!(parsed.template.contains("Code Review Agent"));
        assert!(parsed.template.contains("{{files}}"));
    }
}
