//! Command registry for discovering and managing commands.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Command registry that discovers and manages available commands.
///
/// The registry loads commands from the user's commands directory and
/// provides methods to query and execute them.
pub struct CommandRegistry {
    commands: Arc<RwLock<HashMap<String, Box<dyn Command>>>>,
    commands_dir: PathBuf,
}

impl CommandRegistry {
    /// Creates a new command registry.
    ///
    /// # Arguments
    /// * `commands_dir` - Directory to load commands from
    pub async fn new(commands_dir: PathBuf) -> anyhow::Result<Self> {
        let registry = Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
            commands_dir,
        };

        // Initial load
        registry.reload().await?;

        Ok(registry)
    }

    /// Reloads all commands from the commands directory.
    pub async fn reload(&self) -> anyhow::Result<()> {
        use super::user::loader::UserCommandLoader;

        let loader = UserCommandLoader::new(self.commands_dir.clone());
        let loaded_commands = loader.load_all().await?;

        let mut map = self.commands.write().await;
        map.clear();

        for cmd in loaded_commands {
            map.insert(cmd.name().to_string(), cmd);
        }

        Ok(())
    }

    /// Gets a command by name.
    pub async fn get(&self, name: &str) -> Option<Box<dyn Command>> {
        let map = self.commands.read().await;
        map.get(name).map(|cmd| cmd.clone_box())
    }

    /// Lists all available commands.
    pub async fn list(&self) -> Vec<CommandInfo> {
        let map = self.commands.read().await;
        map.values()
            .map(|cmd| CommandInfo {
                name: cmd.name().to_string(),
                description: cmd.description().to_string(),
                category: cmd.category(),
            })
            .collect()
    }

    /// Filters commands by category.
    pub async fn filter_by_category(&self, category: CommandCategory) -> Vec<CommandInfo> {
        let map = self.commands.read().await;
        map.values()
            .filter(|cmd| {
                std::mem::discriminant(&cmd.category()) == std::mem::discriminant(&category)
            })
            .map(|cmd| CommandInfo {
                name: cmd.name().to_string(),
                description: cmd.description().to_string(),
                category: cmd.category(),
            })
            .collect()
    }

    /// Registers a command in the registry.
    ///
    /// This is useful for adding built-in commands or programmatically
    /// created commands to the registry.
    pub async fn register(&self, command: Box<dyn Command>) {
        let mut map = self.commands.write().await;
        map.insert(command.name().to_string(), command);
    }
}

/// Trait for commands that can be executed.
pub trait Command: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn category(&self) -> CommandCategory;
    fn clone_box(&self) -> Box<dyn Command>;

    /// Provides downcasting support for accessing concrete command types.
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Command category for organization.
#[derive(Debug, Clone, Copy)]
pub enum CommandCategory {
    Analysis,
    Refactoring,
    Documentation,
    Testing,
    Custom,
}

/// Information about an available command.
#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub category: CommandCategory,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_registry_initialization() {
        let temp = TempDir::new().unwrap();
        let commands_dir = temp.path().join("commands");

        let registry = CommandRegistry::new(commands_dir).await.unwrap();
        let commands = registry.list().await;

        assert!(commands.is_empty());
    }

    #[tokio::test]
    async fn test_registry_load_commands() {
        let temp = TempDir::new().unwrap();
        let commands_dir = temp.path().join("commands");
        tokio::fs::create_dir_all(&commands_dir).await.unwrap();

        // Create test commands
        let explain_cmd = r#"---
name: explain
description: Explain code
category: analysis
---

Template"#;

        let review_cmd = r#"---
name: review
description: Review code
category: testing
---

Template"#;

        tokio::fs::write(commands_dir.join("explain.md"), explain_cmd)
            .await
            .unwrap();

        tokio::fs::write(commands_dir.join("review.md"), review_cmd)
            .await
            .unwrap();

        let registry = CommandRegistry::new(commands_dir).await.unwrap();
        let commands = registry.list().await;

        assert_eq!(commands.len(), 2);
    }

    #[tokio::test]
    async fn test_registry_get_command() {
        let temp = TempDir::new().unwrap();
        let commands_dir = temp.path().join("commands");
        tokio::fs::create_dir_all(&commands_dir).await.unwrap();

        let cmd_content = r#"---
name: test-cmd
description: Test command
category: testing
---

Test template"#;

        tokio::fs::write(commands_dir.join("test.md"), cmd_content)
            .await
            .unwrap();

        let registry = CommandRegistry::new(commands_dir).await.unwrap();

        let cmd = registry.get("test-cmd").await;
        assert!(cmd.is_some());
        assert_eq!(cmd.unwrap().name(), "test-cmd");

        let missing = registry.get("nonexistent").await;
        assert!(missing.is_none());
    }

    #[tokio::test]
    async fn test_registry_reload() {
        let temp = TempDir::new().unwrap();
        let commands_dir = temp.path().join("commands");
        tokio::fs::create_dir_all(&commands_dir).await.unwrap();

        let registry = CommandRegistry::new(commands_dir.clone()).await.unwrap();
        assert_eq!(registry.list().await.len(), 0);

        // Add a command file
        let cmd_content = r#"---
name: new-cmd
description: New command
category: analysis
---

Template"#;

        tokio::fs::write(commands_dir.join("new.md"), cmd_content)
            .await
            .unwrap();

        // Reload registry
        registry.reload().await.unwrap();

        let commands = registry.list().await;
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name, "new-cmd");
    }

    #[tokio::test]
    async fn test_registry_filter_by_category() {
        let temp = TempDir::new().unwrap();
        let commands_dir = temp.path().join("commands");
        tokio::fs::create_dir_all(&commands_dir).await.unwrap();

        for (name, category) in [
            ("analysis1", "analysis"),
            ("test1", "testing"),
            ("analysis2", "analysis"),
        ] {
            let content = format!(
                r#"---
name: {}
description: Test
category: {}
---

Template"#,
                name, category
            );
            tokio::fs::write(commands_dir.join(format!("{}.md", name)), content)
                .await
                .unwrap();
        }

        let registry = CommandRegistry::new(commands_dir).await.unwrap();
        let analysis_cmds = registry.filter_by_category(CommandCategory::Analysis).await;

        assert_eq!(analysis_cmds.len(), 2);
    }
}
