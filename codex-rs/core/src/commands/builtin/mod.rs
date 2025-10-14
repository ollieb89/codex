//! Built-in commands shipped with Codex.
//!
//! This module contains pre-defined commands that are available
//! by default without requiring user configuration.

use super::registry::{Command, CommandCategory};

/// Built-in "explain" command.
pub struct ExplainCommand;

impl Command for ExplainCommand {
    fn name(&self) -> &str {
        "explain"
    }

    fn description(&self) -> &str {
        "Explain code functionality with detailed analysis"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Analysis
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(ExplainCommand)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ExplainCommand {
    /// Gets the template for this command.
    pub fn template() -> &'static str {
        r#"Please provide a detailed explanation of the following code:

{{#if args.file}}
File: {{args.file}}
{{/if}}

{{#if git_diff}}
Recent changes:
```
{{git_diff}}
```
{{/if}}

{{#if args.code}}
```
{{args.code}}
```
{{else if files}}
Files to analyze:
{{#each files}}- {{this}}
{{/each}}
{{/if}}

Please explain:
1. What the code does
2. How it works (key logic and algorithms)
3. Any patterns or best practices used
4. Potential issues or improvements"#
    }
}

/// Built-in "review" command.
pub struct ReviewCommand;

impl Command for ReviewCommand {
    fn name(&self) -> &str {
        "review"
    }

    fn description(&self) -> &str {
        "Perform comprehensive code review"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Analysis
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(ReviewCommand)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ReviewCommand {
    /// Gets the template for this command.
    pub fn template() -> &'static str {
        r#"Please perform a comprehensive code review:

{{#if git_diff}}
Changes to review:
```
{{git_diff}}
```
{{else if files}}
Files to review:
{{#each files}}- {{this}}
{{/each}}
{{/if}}

Review checklist:
1. **Code Quality**
   - Readability and maintainability
   - Naming conventions
   - Code organization

2. **Best Practices**
   - Design patterns
   - Error handling
   - Resource management

3. **Potential Issues**
   - Bugs or logical errors
   - Performance concerns
   - Security vulnerabilities

4. **Testing**
   - Test coverage
   - Edge cases
   - Test quality

5. **Suggestions**
   - Improvements
   - Refactoring opportunities
   - Documentation needs"#
    }
}

/// Built-in "test" command.
pub struct TestCommand;

impl Command for TestCommand {
    fn name(&self) -> &str {
        "test"
    }

    fn description(&self) -> &str {
        "Generate comprehensive test cases"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Testing
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(TestCommand)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TestCommand {
    /// Gets the template for this command.
    pub fn template() -> &'static str {
        r#"Please generate comprehensive test cases for:

{{#if args.function}}
Function: {{args.function}}
{{/if}}

{{#if args.code}}
Code:
```
{{args.code}}
```
{{else if files}}
Files:
{{#each files}}- {{this}}
{{/each}}
{{/if}}

Generate tests covering:
1. **Happy Path**
   - Normal expected inputs
   - Successful execution flows

2. **Edge Cases**
   - Boundary values
   - Empty/null inputs
   - Maximum/minimum values

3. **Error Cases**
   - Invalid inputs
   - Error conditions
   - Exception handling

4. **Integration**
   - Dependencies
   - Side effects
   - State management

Format: {{#if args.format}}{{args.format}}{{else}}Framework-appropriate{{/if}}"#
    }
}

/// Returns all built-in commands.
pub fn all_commands() -> Vec<Box<dyn Command>> {
    vec![
        Box::new(ExplainCommand),
        Box::new(ReviewCommand),
        Box::new(TestCommand),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explain_command() {
        let cmd = ExplainCommand;
        assert_eq!(cmd.name(), "explain");
        assert_eq!(
            cmd.description(),
            "Explain code functionality with detailed analysis"
        );
    }

    #[test]
    fn test_review_command() {
        let cmd = ReviewCommand;
        assert_eq!(cmd.name(), "review");
        assert!(cmd.description().contains("code review"));
    }

    #[test]
    fn test_test_command() {
        let cmd = TestCommand;
        assert_eq!(cmd.name(), "test");
        assert!(cmd.description().contains("test cases"));
    }

    #[test]
    fn test_all_commands_count() {
        let commands = all_commands();
        assert_eq!(commands.len(), 3);
    }

    #[test]
    fn test_explain_template() {
        let template = ExplainCommand::template();
        assert!(template.contains("{{#if args.file}}"));
        assert!(template.contains("{{git_diff}}"));
        assert!(template.contains("{{args.code}}"));
    }

    #[test]
    fn test_review_template() {
        let template = ReviewCommand::template();
        assert!(template.contains("Code Quality"));
        assert!(template.contains("Best Practices"));
        assert!(template.contains("Potential Issues"));
    }

    #[test]
    fn test_test_template() {
        let template = TestCommand::template();
        assert!(template.contains("Happy Path"));
        assert!(template.contains("Edge Cases"));
        assert!(template.contains("Error Cases"));
    }
}
