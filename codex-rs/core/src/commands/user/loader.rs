//! User command loader from filesystem.

use std::path::{Path, PathBuf};

use super::super::parser::{CommandMetadata, CommandParser};
use super::super::registry::{Command, CommandCategory};

/// Loads user-defined commands from a directory.
pub struct UserCommandLoader {
    commands_dir: PathBuf,
}

impl UserCommandLoader {
    /// Creates a new user command loader.
    ///
    /// # Arguments
    /// * `commands_dir` - Directory containing user command files
    pub fn new(commands_dir: PathBuf) -> Self {
        Self { commands_dir }
    }

    /// Loads all commands from the commands directory.
    ///
    /// Scans for .md files and parses them as commands.
    pub async fn load_all(&self) -> anyhow::Result<Vec<Box<dyn Command>>> {
        let mut commands = Vec::new();

        // Create directory if it doesn't exist
        tokio::fs::create_dir_all(&self.commands_dir).await?;

        // Read directory entries
        let mut entries = tokio::fs::read_dir(&self.commands_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            // Only process .md files
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }

            // Skip if not a file
            if !path.is_file() {
                continue;
            }

            // Load and parse command
            match self.load_command(&path).await {
                Ok(cmd) => commands.push(cmd),
                Err(e) => {
                    tracing::warn!("Failed to load command from {:?}: {}", path, e);
                    continue;
                }
            }
        }

        Ok(commands)
    }

    async fn load_command(&self, path: &Path) -> anyhow::Result<Box<dyn Command>> {
        // Read file content
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read file {:?}: {}", path, e))?;

        // Parse command
        let parsed = CommandParser::parse(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse command from {:?}: {}", path, e))?;

        // Convert to UserCommand
        Ok(Box::new(UserCommand {
            metadata: parsed.metadata,
            template: parsed.template,
        }))
    }
}

/// User-defined command loaded from a file.
pub struct UserCommand {
    pub metadata: CommandMetadata,
    pub template: String,
}

impl Command for UserCommand {
    fn name(&self) -> &str {
        &self.metadata.name
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::from_str(&self.metadata.category)
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(Self {
            metadata: self.metadata.clone(),
            template: self.template.clone(),
        })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl CommandCategory {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "analysis" => CommandCategory::Analysis,
            "refactoring" => CommandCategory::Refactoring,
            "testing" => CommandCategory::Testing,
            "documentation" => CommandCategory::Documentation,
            _ => CommandCategory::Custom,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_loader_creates_directory() {
        let temp = TempDir::new().unwrap();
        let commands_dir = temp.path().join("commands");

        let loader = UserCommandLoader::new(commands_dir.clone());
        let _commands = loader.load_all().await.unwrap();

        assert!(commands_dir.exists());
    }

    #[tokio::test]
    async fn test_load_valid_command() {
        let temp = TempDir::new().unwrap();
        let commands_dir = temp.path().join("commands");
        tokio::fs::create_dir_all(&commands_dir).await.unwrap();

        // Create a valid command file
        let command_content = r#"---
name: explain
description: Explain code
category: analysis
---

Please explain: {{code}}"#;

        tokio::fs::write(commands_dir.join("explain.md"), command_content)
            .await
            .unwrap();

        let loader = UserCommandLoader::new(commands_dir);
        let commands = loader.load_all().await.unwrap();

        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name(), "explain");
        assert_eq!(commands[0].description(), "Explain code");
    }

    #[tokio::test]
    async fn test_load_multiple_commands() {
        let temp = TempDir::new().unwrap();
        let commands_dir = temp.path().join("commands");
        tokio::fs::create_dir_all(&commands_dir).await.unwrap();

        // Create multiple command files
        for (name, desc) in [
            ("explain", "Explain"),
            ("review", "Review"),
            ("test", "Test"),
        ] {
            let content = format!(
                r#"---
name: {}
description: {}
category: analysis
---

Template"#,
                name, desc
            );
            tokio::fs::write(commands_dir.join(format!("{}.md", name)), content)
                .await
                .unwrap();
        }

        let loader = UserCommandLoader::new(commands_dir);
        let commands = loader.load_all().await.unwrap();

        assert_eq!(commands.len(), 3);
    }

    #[tokio::test]
    async fn test_skip_non_md_files() {
        let temp = TempDir::new().unwrap();
        let commands_dir = temp.path().join("commands");
        tokio::fs::create_dir_all(&commands_dir).await.unwrap();

        // Create .md and non-.md files
        let valid_command = r#"---
name: valid
description: Valid
category: test
---

Template"#;

        tokio::fs::write(commands_dir.join("valid.md"), valid_command)
            .await
            .unwrap();

        tokio::fs::write(commands_dir.join("README.txt"), "Not a command")
            .await
            .unwrap();

        tokio::fs::write(commands_dir.join("config.yaml"), "config: value")
            .await
            .unwrap();

        let loader = UserCommandLoader::new(commands_dir);
        let commands = loader.load_all().await.unwrap();

        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name(), "valid");
    }

    #[tokio::test]
    async fn test_skip_invalid_commands() {
        let temp = TempDir::new().unwrap();
        let commands_dir = temp.path().join("commands");
        tokio::fs::create_dir_all(&commands_dir).await.unwrap();

        // Valid command
        let valid_command = r#"---
name: valid
description: Valid
category: test
---

Template"#;

        tokio::fs::write(commands_dir.join("valid.md"), valid_command)
            .await
            .unwrap();

        // Invalid command (missing frontmatter)
        tokio::fs::write(commands_dir.join("invalid.md"), "Just markdown content")
            .await
            .unwrap();

        let loader = UserCommandLoader::new(commands_dir);
        let commands = loader.load_all().await.unwrap();

        // Should only load the valid command, skipping invalid
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name(), "valid");
    }

    #[tokio::test]
    async fn test_category_parsing() {
        let temp = TempDir::new().unwrap();
        let commands_dir = temp.path().join("commands");
        tokio::fs::create_dir_all(&commands_dir).await.unwrap();

        for category in ["analysis", "utility", "testing", "documentation", "custom"] {
            let content = format!(
                r#"---
name: cmd-{}
description: Test
category: {}
---

Template"#,
                category, category
            );
            tokio::fs::write(commands_dir.join(format!("{}.md", category)), content)
                .await
                .unwrap();
        }

        let loader = UserCommandLoader::new(commands_dir);
        let commands = loader.load_all().await.unwrap();

        assert_eq!(commands.len(), 5);
    }
}
