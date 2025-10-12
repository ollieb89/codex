//! Agent permission model.

use std::path::Path;

/// Agent permissions defining allowed operations.
#[derive(Debug, Clone)]
pub struct AgentPermissions {
    pub file_access: FileAccessPolicy,
    pub shell_execution: bool,
    pub network_access: bool,
    pub allowed_tools: Vec<String>,
    pub max_iterations: u32,
    pub can_delegate: bool,
}

impl Default for AgentPermissions {
    fn default() -> Self {
        Self {
            file_access: FileAccessPolicy::NoAccess,
            shell_execution: false,
            network_access: false,
            allowed_tools: Vec::new(),
            max_iterations: 5,
            can_delegate: false,
        }
    }
}

impl AgentPermissions {
    /// Checks if the agent can read a file.
    pub fn can_read_file(&self, path: &Path) -> bool {
        match &self.file_access {
            FileAccessPolicy::NoAccess => false,
            FileAccessPolicy::ReadOnly | FileAccessPolicy::ReadWrite { .. } => {
                self.matches_patterns(path)
            }
        }
    }

    /// Checks if the agent can write to a file.
    pub fn can_write_file(&self, path: &Path) -> bool {
        match &self.file_access {
            FileAccessPolicy::ReadWrite {
                allow_patterns,
                deny_patterns,
            } => {
                // Check deny patterns first
                if deny_patterns.iter().any(|p| Self::matches(path, p)) {
                    return false;
                }
                // Then check allow patterns
                allow_patterns.iter().any(|p| Self::matches(path, p))
            }
            _ => false,
        }
    }

    fn matches_patterns(&self, _path: &Path) -> bool {
        // For now, allow all reads if policy allows reading
        // TODO: Implement pattern matching
        true
    }

    fn matches(path: &Path, pattern: &str) -> bool {
        // Simple glob pattern matching implementation
        // Support for basic patterns like "**/*.rs" and "**/secrets/**"
        let path_str = path.to_string_lossy();

        if pattern == "**" || pattern == "*" {
            return true;
        }

        if pattern.contains("**") {
            // Handle recursive patterns like "**/*.rs" or "**/secrets/**"
            if let Some((prefix, suffix)) = pattern.split_once("**") {
                // Check prefix if not empty
                if !prefix.is_empty() && !path_str.starts_with(prefix) {
                    return false;
                }

                // Handle suffix patterns
                if suffix.starts_with("/*") {
                    // Pattern like "**/*.rs"
                    if let Some(extension) = suffix.strip_prefix("/*.") {
                        return path_str.ends_with(&format!(".{}", extension));
                    }
                } else if suffix.starts_with('/') {
                    // Pattern like "**/secrets/**" or "**/secrets/"
                    let dir_pattern = suffix.trim_start_matches('/').trim_end_matches("/**");
                    return path_str.contains(&format!("/{}/", dir_pattern))
                        || path_str.contains(&format!("\\{}\\", dir_pattern))
                        || path_str.contains(&format!("/{}", dir_pattern))
                        || path_str.contains(&format!("\\{}", dir_pattern));
                }
            }
        } else if pattern.starts_with("*.") {
            // Simple extension pattern like "*.rs"
            let extension = pattern.strip_prefix("*.").unwrap();
            return path_str.ends_with(&format!(".{}", extension));
        }

        // Default: exact match or contains
        path_str.contains(pattern)
    }
}

/// File access policy for agents.
#[derive(Debug, Clone)]
pub enum FileAccessPolicy {
    /// No file access allowed.
    NoAccess,
    /// Read-only access to all files.
    ReadOnly,
    /// Read-write access with pattern-based filtering.
    ReadWrite {
        allow_patterns: Vec<String>,
        deny_patterns: Vec<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_read_only_permissions() {
        let perms = AgentPermissions {
            file_access: FileAccessPolicy::ReadOnly,
            ..Default::default()
        };

        assert!(perms.can_read_file(&PathBuf::from("test.rs")));
        assert!(!perms.can_write_file(&PathBuf::from("test.rs")));
    }

    #[test]
    fn test_no_access_permissions() {
        let perms = AgentPermissions::default();

        assert!(!perms.can_read_file(&PathBuf::from("test.rs")));
        assert!(!perms.can_write_file(&PathBuf::from("test.rs")));
    }

    #[test]
    fn test_read_write_permissions() {
        let perms = AgentPermissions {
            file_access: FileAccessPolicy::ReadWrite {
                allow_patterns: vec!["**/*.rs".into()],
                deny_patterns: vec!["**/secrets/**".into()],
            },
            ..Default::default()
        };

        assert!(perms.can_read_file(&PathBuf::from("src/main.rs")));
        assert!(perms.can_write_file(&PathBuf::from("src/main.rs")));
    }
}
