//! Agent execution toolkit with permission enforcement.

use std::path::{Path, PathBuf};

use super::{AgentId, AgentPermissions};

/// Toolkit for agent execution.
///
/// Provides file access, command execution, and workspace utilities
/// with permission validation.
pub struct AgentToolkit {
    agent_id: AgentId,
    permissions: AgentPermissions,
    workspace_root: PathBuf,
}

impl AgentToolkit {
    /// Creates a new agent toolkit.
    pub fn new(agent_id: AgentId, permissions: AgentPermissions, workspace_root: PathBuf) -> Self {
        Self {
            agent_id,
            permissions,
            workspace_root,
        }
    }

    /// Reads a file with permission validation.
    pub async fn read_file(&self, path: &Path) -> anyhow::Result<String> {
        // Validate permissions
        if !self.permissions.can_read_file(path) {
            anyhow::bail!("Permission denied: cannot read {:?}", path);
        }

        // Read file
        tokio::fs::read_to_string(path)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read file: {}", e))
    }

    /// Writes a file with permission validation.
    pub async fn write_file(&self, path: &Path, content: &str) -> anyhow::Result<()> {
        // Validate permissions
        if !self.permissions.can_write_file(path) {
            anyhow::bail!("Permission denied: cannot write to {:?}", path);
        }

        // Write file
        tokio::fs::write(path, content)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to write file: {}", e))
    }

    /// Executes a shell command with permission validation.
    pub async fn execute_command(
        &self,
        cmd: &str,
        args: &[String],
    ) -> anyhow::Result<CommandOutput> {
        // Validate shell execution permission
        if !self.permissions.shell_execution {
            anyhow::bail!("Permission denied: shell execution not allowed");
        }

        // TODO: Validate through execpolicy
        // For now, execute directly
        self.execute_shell(cmd, args).await
    }

    async fn execute_shell(&self, cmd: &str, args: &[String]) -> anyhow::Result<CommandOutput> {
        use tokio::process::Command;

        let output = Command::new(cmd)
            .args(args)
            .current_dir(&self.workspace_root)
            .output()
            .await?;

        Ok(CommandOutput {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
        })
    }

    /// Returns the workspace root directory.
    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    /// Returns the agent ID.
    pub fn agent_id(&self) -> &AgentId {
        &self.agent_id
    }
}

/// Output from command execution.
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::permissions::FileAccessPolicy;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_read_with_permissions() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.txt");
        tokio::fs::write(&file_path, "test content").await.unwrap();

        let permissions = AgentPermissions {
            file_access: FileAccessPolicy::ReadOnly,
            shell_execution: false,
            network_access: false,
            allowed_tools: vec![],
            max_iterations: 5,
            can_delegate: false,
        };

        let toolkit = AgentToolkit::new(
            AgentId::from("test-agent"),
            permissions,
            temp.path().to_path_buf(),
        );

        let content = toolkit.read_file(&file_path).await.unwrap();
        assert_eq!(content, "test content");
    }

    #[tokio::test]
    async fn test_file_write_permission_denied() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.txt");

        let permissions = AgentPermissions {
            file_access: FileAccessPolicy::ReadOnly,
            shell_execution: false,
            network_access: false,
            allowed_tools: vec![],
            max_iterations: 5,
            can_delegate: false,
        };

        let toolkit = AgentToolkit::new(
            AgentId::from("test-agent"),
            permissions,
            temp.path().to_path_buf(),
        );

        let result = toolkit.write_file(&file_path, "test").await;
        assert!(result.is_err());
    }
}
