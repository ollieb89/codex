//! Command invocation parsing and argument extraction.
//!
//! This module handles parsing slash command syntax (`/command args`) and
//! extracting the command name and arguments.

use anyhow::{Result, bail};
use std::collections::HashMap;

/// Represents a parsed command invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandInvocation {
    /// The command name (without the leading slash)
    pub command_name: String,
    /// Named arguments extracted from key=value pairs
    pub args: HashMap<String, String>,
    /// Raw positional arguments (not in key=value format)
    pub raw_args: Vec<String>,
}

/// Parser for slash command invocations.
pub struct InvocationParser;

impl InvocationParser {
    /// Parses a slash command string into a CommandInvocation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use codex_core::commands::invocation::InvocationParser;
    /// let inv = InvocationParser::parse("/explain src/main.rs").unwrap();
    /// assert_eq!(inv.command_name, "explain");
    /// assert_eq!(inv.raw_args, vec!["src/main.rs"]);
    ///
    /// let inv = InvocationParser::parse("/review depth=deep src/").unwrap();
    /// assert_eq!(inv.command_name, "review");
    /// assert_eq!(inv.args.get("depth"), Some(&"deep".to_string()));
    /// assert_eq!(inv.raw_args, vec!["src/"]);
    /// ```
    pub fn parse(input: &str) -> Result<CommandInvocation> {
        let trimmed = input.trim();

        // Validate slash command format
        if !trimmed.starts_with('/') {
            bail!("Command must start with '/'");
        }

        if trimmed.len() == 1 {
            bail!("Command name cannot be empty");
        }

        // Split into tokens, respecting quoted strings
        let tokens = Self::tokenize(&trimmed[1..])?;

        if tokens.is_empty() {
            bail!("Command name cannot be empty");
        }

        let command_name = tokens[0].clone();

        // Validate command name
        if !Self::is_valid_command_name(&command_name) {
            bail!(
                "Invalid command name '{}': must contain only alphanumeric characters, '-', or '_'",
                command_name
            );
        }

        // Parse arguments
        let mut args = HashMap::new();
        let mut raw_args = Vec::new();

        for token in &tokens[1..] {
            if let Some((key, value)) = Self::parse_key_value(token) {
                args.insert(key, value);
            } else {
                raw_args.push(token.clone());
            }
        }

        Ok(CommandInvocation {
            command_name,
            args,
            raw_args,
        })
    }

    /// Tokenizes input string, respecting quoted strings and escapes.
    fn tokenize(input: &str) -> Result<Vec<String>> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut in_quotes = false;
        let mut escape_next = false;
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];

            if escape_next {
                current_token.push(ch);
                escape_next = false;
            } else if ch == '\\' {
                escape_next = true;
            } else if ch == '"' {
                in_quotes = !in_quotes;
            } else if ch.is_whitespace() && !in_quotes {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            } else {
                current_token.push(ch);
            }

            i += 1;
        }

        // Handle unclosed quotes
        if in_quotes {
            bail!("Unclosed quotes in command");
        }

        // Handle trailing escape
        if escape_next {
            bail!("Trailing escape character");
        }

        // Add final token if present
        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        Ok(tokens)
    }

    /// Parses a key=value pair, returning None if not in that format.
    fn parse_key_value(token: &str) -> Option<(String, String)> {
        // Split on first '=' only
        if let Some(eq_pos) = token.find('=') {
            let key = token[..eq_pos].trim().to_string();
            let value = token[eq_pos + 1..].trim().to_string();

            if !key.is_empty() && Self::is_valid_arg_name(&key) {
                return Some((key, value));
            }
        }

        None
    }

    /// Validates command name format.
    fn is_valid_command_name(name: &str) -> bool {
        !name.is_empty()
            && name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }

    /// Validates argument name format.
    fn is_valid_arg_name(name: &str) -> bool {
        !name.is_empty()
            && name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        let inv = InvocationParser::parse("/explain").unwrap();
        assert_eq!(inv.command_name, "explain");
        assert!(inv.args.is_empty());
        assert!(inv.raw_args.is_empty());
    }

    #[test]
    fn test_command_with_positional_arg() {
        let inv = InvocationParser::parse("/explain src/main.rs").unwrap();
        assert_eq!(inv.command_name, "explain");
        assert_eq!(inv.raw_args, vec!["src/main.rs"]);
        assert!(inv.args.is_empty());
    }

    #[test]
    fn test_command_with_multiple_positional_args() {
        let inv = InvocationParser::parse("/review src/main.rs src/lib.rs").unwrap();
        assert_eq!(inv.command_name, "review");
        assert_eq!(inv.raw_args, vec!["src/main.rs", "src/lib.rs"]);
    }

    #[test]
    fn test_command_with_key_value() {
        let inv = InvocationParser::parse("/review depth=deep").unwrap();
        assert_eq!(inv.command_name, "review");
        assert_eq!(inv.args.get("depth"), Some(&"deep".to_string()));
        assert!(inv.raw_args.is_empty());
    }

    #[test]
    fn test_command_with_mixed_args() {
        let inv = InvocationParser::parse("/review depth=deep src/").unwrap();
        assert_eq!(inv.command_name, "review");
        assert_eq!(inv.args.get("depth"), Some(&"deep".to_string()));
        assert_eq!(inv.raw_args, vec!["src/"]);
    }

    #[test]
    fn test_quoted_argument_with_spaces() {
        let inv = InvocationParser::parse(r#"/test "my file.rs""#).unwrap();
        assert_eq!(inv.command_name, "test");
        assert_eq!(inv.raw_args, vec!["my file.rs"]);
    }

    #[test]
    fn test_multiple_quoted_arguments() {
        let inv = InvocationParser::parse(r#"/test "file one.rs" "file two.rs""#).unwrap();
        assert_eq!(inv.raw_args, vec!["file one.rs", "file two.rs"]);
    }

    #[test]
    fn test_escaped_characters() {
        let inv = InvocationParser::parse(r#"/test file\"name.rs"#).unwrap();
        assert_eq!(inv.raw_args, vec![r#"file"name.rs"#]);
    }

    #[test]
    fn test_key_value_with_equals_in_value() {
        let inv = InvocationParser::parse("/config url=http://example.com?key=value").unwrap();
        assert_eq!(
            inv.args.get("url"),
            Some(&"http://example.com?key=value".to_string())
        );
    }

    #[test]
    fn test_empty_command_error() {
        let result = InvocationParser::parse("/");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_missing_slash_error() {
        let result = InvocationParser::parse("explain");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("must start with '/'")
        );
    }

    #[test]
    fn test_invalid_command_name() {
        let result = InvocationParser::parse("/explain!");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid command"));
    }

    #[test]
    fn test_unclosed_quotes_error() {
        let result = InvocationParser::parse(r#"/test "unclosed"#);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unclosed quotes"));
    }

    #[test]
    fn test_trailing_escape_error() {
        let result = InvocationParser::parse(r#"/test file\"#);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Trailing escape"));
    }

    #[test]
    fn test_hyphenated_command_name() {
        let inv = InvocationParser::parse("/my-command").unwrap();
        assert_eq!(inv.command_name, "my-command");
    }

    #[test]
    fn test_underscored_command_name() {
        let inv = InvocationParser::parse("/my_command").unwrap();
        assert_eq!(inv.command_name, "my_command");
    }

    #[test]
    fn test_extra_whitespace_handling() {
        let inv = InvocationParser::parse("  /explain   src/main.rs  ").unwrap();
        assert_eq!(inv.command_name, "explain");
        assert_eq!(inv.raw_args, vec!["src/main.rs"]);
    }
}
