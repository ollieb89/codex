//! Argument mapping and validation for command invocations.
//!
//! This module handles mapping positional arguments to named arguments
//! based on command metadata, applying defaults, and validating requirements.

use super::invocation::CommandInvocation;
use super::parser::CommandMetadata;
use anyhow::Result;
use anyhow::bail;
use std::collections::HashMap;

/// Maps and validates command arguments.
pub struct ArgumentMapper;

impl ArgumentMapper {
    /// Maps raw invocation arguments to command's expected arguments.
    ///
    /// This function:
    /// 1. Maps positional arguments to the first N named arguments
    /// 2. Applies named arguments from key=value pairs (override positional)
    /// 3. Applies default values for missing optional arguments
    /// 4. Validates that all required arguments are present
    ///
    /// # Examples
    ///
    /// ```
    /// # use codex_core::commands::args::ArgumentMapper;
    /// # use codex_core::commands::invocation::{CommandInvocation, InvocationParser};
    /// # use codex_core::commands::parser::{CommandMetadata, ArgDefinition, ArgType, CommandPermissions};
    /// # use std::collections::HashMap;
    /// let metadata = CommandMetadata {
    ///     name: "test".to_string(),
    ///     description: "Test command".to_string(),
    ///     category: "testing".to_string(),
    ///     args: vec![
    ///         ArgDefinition {
    ///             name: "file".to_string(),
    ///             arg_type: ArgType::String,
    ///             description: "File to test".to_string(),
    ///             required: true,
    ///             default: None,
    ///         },
    ///     ],
    ///     permissions: CommandPermissions::default(),
    /// };
    ///
    /// let invocation = InvocationParser::parse("/test src/main.rs").unwrap();
    /// let args = ArgumentMapper::map_arguments(&invocation, &metadata).unwrap();
    /// assert_eq!(args.get("file"), Some(&"src/main.rs".to_string()));
    /// ```
    pub fn map_arguments(
        invocation: &CommandInvocation,
        metadata: &CommandMetadata,
    ) -> Result<HashMap<String, String>> {
        let mut result = HashMap::new();

        // Step 1: Map positional arguments to first N named arguments
        let positional_arg_defs: Vec<_> = metadata.args.iter().collect();

        for (i, raw_arg) in invocation.raw_args.iter().enumerate() {
            if i < positional_arg_defs.len() {
                let arg_def = positional_arg_defs[i];
                result.insert(arg_def.name.clone(), raw_arg.clone());
            } else {
                // Extra positional arguments - ignore or error?
                // For now, we'll be lenient and ignore extra args
                tracing::warn!(
                    "Extra positional argument ignored: '{}' (position {})",
                    raw_arg,
                    i
                );
            }
        }

        // Step 2: Apply named arguments (these override positional)
        for (key, value) in &invocation.args {
            // Validate that this is a known argument
            if !metadata.args.iter().any(|arg| &arg.name == key) {
                bail!("Unknown argument '{}' for command '{}'", key, metadata.name);
            }

            result.insert(key.clone(), value.clone());
        }

        // Step 3: Apply defaults for missing optional arguments
        for arg_def in &metadata.args {
            if !result.contains_key(&arg_def.name)
                && let Some(default) = &arg_def.default
            {
                result.insert(arg_def.name.clone(), default.clone());
            }
        }

        // Step 4: Validate required arguments are present
        for arg_def in &metadata.args {
            if arg_def.required && !result.contains_key(&arg_def.name) {
                bail!(
                    "Required argument '{}' missing for command '{}'",
                    arg_def.name,
                    metadata.name
                );
            }
        }

        Ok(result)
    }

    /// Validates argument types and coerces values if possible.
    ///
    /// Currently supports basic type validation. Future enhancements could include:
    /// - Type coercion (string → bool, number)
    /// - Format validation (URLs, paths, etc.)
    /// - Range validation for numeric values
    pub fn validate_and_coerce(
        _args: &HashMap<String, String>,
        _metadata: &CommandMetadata,
    ) -> Result<HashMap<String, String>> {
        // TODO: Implement type validation and coercion
        // For now, return args as-is (all strings)
        Ok(_args.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::invocation::InvocationParser;
    use crate::commands::parser::ArgDefinition;
    use crate::commands::parser::ArgType;
    use crate::commands::parser::CommandPermissions;

    fn create_test_metadata() -> CommandMetadata {
        CommandMetadata {
            name: "test".to_string(),
            description: "Test command".to_string(),
            category: "testing".to_string(),
            args: vec![
                ArgDefinition {
                    name: "file".to_string(),
                    arg_type: ArgType::String,
                    description: "File to test".to_string(),
                    required: true,
                    default: None,
                },
                ArgDefinition {
                    name: "format".to_string(),
                    arg_type: ArgType::String,
                    description: "Output format".to_string(),
                    required: false,
                    default: Some("standard".to_string()),
                },
            ],
            permissions: CommandPermissions::default(),
            agent: false,
            agent_id: None,
            activation_hints: vec![],
        }
    }

    #[test]
    fn test_positional_to_named_mapping() {
        let metadata = create_test_metadata();
        let invocation = InvocationParser::parse("/test src/main.rs").unwrap();

        let args = ArgumentMapper::map_arguments(&invocation, &metadata).unwrap();

        assert_eq!(args.get("file"), Some(&"src/main.rs".to_string()));
        assert_eq!(args.get("format"), Some(&"standard".to_string())); // default applied
    }

    #[test]
    fn test_named_arguments_override_positional() {
        let metadata = create_test_metadata();
        let invocation = InvocationParser::parse("/test src/main.rs format=json").unwrap();

        let args = ArgumentMapper::map_arguments(&invocation, &metadata).unwrap();

        assert_eq!(args.get("file"), Some(&"src/main.rs".to_string()));
        assert_eq!(args.get("format"), Some(&"json".to_string())); // overridden
    }

    #[test]
    fn test_named_only_arguments() {
        let metadata = create_test_metadata();
        let invocation = InvocationParser::parse("/test file=src/lib.rs format=yaml").unwrap();

        let args = ArgumentMapper::map_arguments(&invocation, &metadata).unwrap();

        assert_eq!(args.get("file"), Some(&"src/lib.rs".to_string()));
        assert_eq!(args.get("format"), Some(&"yaml".to_string()));
    }

    #[test]
    fn test_default_values_applied() {
        let metadata = create_test_metadata();
        let invocation = InvocationParser::parse("/test file=src/main.rs").unwrap();

        let args = ArgumentMapper::map_arguments(&invocation, &metadata).unwrap();

        assert_eq!(args.get("format"), Some(&"standard".to_string()));
    }

    #[test]
    fn test_required_argument_missing_error() {
        let metadata = create_test_metadata();
        let invocation = InvocationParser::parse("/test").unwrap();

        let result = ArgumentMapper::map_arguments(&invocation, &metadata);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Required argument 'file'")
        );
    }

    #[test]
    fn test_unknown_argument_error() {
        let metadata = create_test_metadata();
        let invocation = InvocationParser::parse("/test file=src/main.rs unknown=value").unwrap();

        let result = ArgumentMapper::map_arguments(&invocation, &metadata);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown argument"));
    }

    #[test]
    fn test_multiple_positional_arguments() {
        let metadata = CommandMetadata {
            name: "review".to_string(),
            description: "Review command".to_string(),
            category: "analysis".to_string(),
            args: vec![
                ArgDefinition {
                    name: "file".to_string(),
                    arg_type: ArgType::String,
                    description: "File to review".to_string(),
                    required: true,
                    default: None,
                },
                ArgDefinition {
                    name: "depth".to_string(),
                    arg_type: ArgType::String,
                    description: "Review depth".to_string(),
                    required: false,
                    default: Some("standard".to_string()),
                },
            ],
            permissions: CommandPermissions::default(),
            agent: false,
            agent_id: None,
            activation_hints: vec![],
        };

        let invocation = InvocationParser::parse("/review src/main.rs deep").unwrap();
        let args = ArgumentMapper::map_arguments(&invocation, &metadata).unwrap();

        assert_eq!(args.get("file"), Some(&"src/main.rs".to_string()));
        assert_eq!(args.get("depth"), Some(&"deep".to_string()));
    }

    #[test]
    fn test_all_optional_arguments_with_defaults() {
        let metadata = CommandMetadata {
            name: "format".to_string(),
            description: "Format code".to_string(),
            category: "utility".to_string(),
            args: vec![
                ArgDefinition {
                    name: "indent".to_string(),
                    arg_type: ArgType::String,
                    description: "Indentation".to_string(),
                    required: false,
                    default: Some("4".to_string()),
                },
                ArgDefinition {
                    name: "style".to_string(),
                    arg_type: ArgType::String,
                    description: "Code style".to_string(),
                    required: false,
                    default: Some("rust".to_string()),
                },
            ],
            permissions: CommandPermissions::default(),
            agent: false,
            agent_id: None,
            activation_hints: vec![],
        };

        let invocation = InvocationParser::parse("/format").unwrap();
        let args = ArgumentMapper::map_arguments(&invocation, &metadata).unwrap();

        assert_eq!(args.get("indent"), Some(&"4".to_string()));
        assert_eq!(args.get("style"), Some(&"rust".to_string()));
    }
}
