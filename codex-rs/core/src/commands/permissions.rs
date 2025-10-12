//! Permission model for command execution.

pub use super::parser::CommandPermissions;

/// Validates command permissions against system policy.
pub async fn validate_command_permissions(
    _permissions: &CommandPermissions,
) -> anyhow::Result<ValidationResult> {
    // TODO: Integrate with execpolicy
    Ok(ValidationResult::Allowed)
}

/// Result of permission validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    /// Command is allowed to execute.
    Allowed,
    /// Command requires user approval.
    RequiresApproval,
    /// Command is forbidden.
    Forbidden(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_permission_validation() {
        let perms = CommandPermissions {
            read_files: true,
            write_files: false,
            execute_shell: false,
        };

        let result = validate_command_permissions(&perms).await.unwrap();
        assert_eq!(result, ValidationResult::Allowed);
    }
}
