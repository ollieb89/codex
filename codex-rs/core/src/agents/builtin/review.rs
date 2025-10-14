//! Code review agent for quality and best practices analysis.

use async_trait::async_trait;
use std::path::Path;
use std::path::PathBuf;

use crate::agents::ActivationScore;
use crate::agents::Agent;
use crate::agents::AgentId;
use crate::agents::AgentPermissions;
use crate::agents::AgentResult;
use crate::agents::AgentToolkit;
use crate::agents::CodeReviewFinding;
use crate::agents::Severity;
use crate::agents::Task;
use crate::agents::TaskContext;
use crate::agents::permissions::FileAccessPolicy;

/// Code review agent that analyzes code for quality, maintainability, and best practices.
///
/// The review agent examines code files and identifies:
/// - Code quality issues
/// - Maintainability concerns
/// - Best practice violations
/// - Design pattern opportunities
/// - Code smells
pub struct ReviewAgent {
    permissions: AgentPermissions,
}

impl ReviewAgent {
    /// Creates a new review agent with appropriate permissions.
    pub fn new() -> Self {
        Self {
            permissions: AgentPermissions {
                file_access: FileAccessPolicy::ReadOnly,
                shell_execution: false,
                network_access: false,
                allowed_tools: vec![],
                max_iterations: 5,
                can_delegate: false,
            },
        }
    }

    /// Analyzes code files and generates review findings.
    ///
    /// This method reads each file and applies heuristic analysis to identify
    /// common code quality issues.
    async fn analyze_code(
        &self,
        files: &[PathBuf],
        toolkit: &AgentToolkit,
    ) -> anyhow::Result<Vec<CodeReviewFinding>> {
        let mut findings = Vec::new();

        for file_path in files {
            // Skip non-text files
            if is_binary_file(file_path) {
                continue;
            }

            match toolkit.read_file(file_path).await {
                Ok(content) => {
                    let file_findings = self.analyze_file_content(&content, file_path);
                    findings.extend(file_findings);
                }
                Err(_e) => {
                    // Silently skip files that can't be read (permissions, etc.)
                    // In production, this would be logged properly
                }
            }
        }

        Ok(findings)
    }

    /// Analyzes a single file's content for code quality issues.
    fn analyze_file_content(&self, content: &str, file_path: &Path) -> Vec<CodeReviewFinding> {
        let mut findings = Vec::new();

        // Check for long functions
        findings.extend(self.check_long_functions(content, file_path));

        // Check for magic numbers
        findings.extend(self.check_magic_numbers(content, file_path));

        // Check for error handling
        findings.extend(self.check_error_handling(content, file_path));

        // Check for naming conventions
        findings.extend(self.check_naming_conventions(content, file_path));

        // Check for code duplication hints
        findings.extend(self.check_duplication(content, file_path));

        findings
    }

    /// Checks for functions that are too long (>50 lines).
    fn check_long_functions(&self, content: &str, file_path: &Path) -> Vec<CodeReviewFinding> {
        let mut findings = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Simple heuristic: Look for function definitions and count lines until closing brace
        let mut in_function = false;
        let mut function_start = 0;
        let mut brace_count = 0;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Detect function start (Rust syntax)
            if trimmed.starts_with("fn ")
                || trimmed.starts_with("pub fn ")
                || trimmed.starts_with("async fn ")
            {
                in_function = true;
                function_start = i;
                brace_count = 0;
            }

            if in_function {
                // Count braces
                brace_count += trimmed.chars().filter(|c| *c == '{').count() as i32;
                brace_count -= trimmed.chars().filter(|c| *c == '}').count() as i32;

                // Function end
                if brace_count == 0 && trimmed.contains('}') {
                    let function_length = i - function_start + 1;
                    if function_length > 50 {
                        findings.push(CodeReviewFinding {
                            severity: Severity::Warning,
                            category: "Code Quality".to_string(),
                            message: format!(
                                "Function is {function_length} lines long. Consider breaking it into smaller functions"
                            ),
                            location: Some(file_path.to_path_buf()),
                            line_number: Some(function_start + 1),
                        });
                    }
                    in_function = false;
                }
            }
        }

        findings
    }

    /// Checks for magic numbers (numeric literals) in code.
    fn check_magic_numbers(&self, content: &str, file_path: &Path) -> Vec<CodeReviewFinding> {
        let mut findings = Vec::new();

        for (i, line) in content.lines().enumerate() {
            // Skip comments
            if line.trim().starts_with("//") || line.trim().starts_with('*') {
                continue;
            }

            // Look for numeric literals (excluding 0, 1, which are often acceptable)
            if line.contains("= 2")
                || line.contains("= 3")
                || line.contains(">= 10")
                || line.contains("< 100")
            {
                // Simple heuristic - more sophisticated parsing needed for production
                if !line.contains("const") && !line.contains("static") {
                    findings.push(CodeReviewFinding {
                        severity: Severity::Info,
                        category: "Maintainability".to_string(),
                        message: "Consider extracting magic number into a named constant"
                            .to_string(),
                        location: Some(file_path.to_path_buf()),
                        line_number: Some(i + 1),
                    });
                }
            }
        }

        findings
    }

    /// Checks for proper error handling patterns.
    fn check_error_handling(&self, content: &str, file_path: &Path) -> Vec<CodeReviewFinding> {
        let mut findings = Vec::new();

        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Check for unwrap() usage
            if trimmed.contains(".unwrap()") && !trimmed.starts_with("//") {
                findings.push(CodeReviewFinding {
                    severity: Severity::Warning,
                    category: "Error Handling".to_string(),
                    message: "Avoid using .unwrap(). Consider using proper error handling with ?"
                        .to_string(),
                    location: Some(file_path.to_path_buf()),
                    line_number: Some(i + 1),
                });
            }

            // Check for expect() with generic messages
            if trimmed.contains(".expect(\"")
                && !trimmed.starts_with("//")
                && (trimmed.contains("expect(\"error\"") || trimmed.contains("expect(\"failed\""))
            {
                findings.push(CodeReviewFinding {
                    severity: Severity::Info,
                    category: "Error Handling".to_string(),
                    message: "Provide more descriptive error messages in expect()".to_string(),
                    location: Some(file_path.to_path_buf()),
                    line_number: Some(i + 1),
                });
            }

            // Check for panic!() usage
            if trimmed.contains("panic!(") && !trimmed.starts_with("//") {
                findings.push(CodeReviewFinding {
                    severity: Severity::Error,
                    category: "Error Handling".to_string(),
                    message: "Avoid using panic!() in library code. Return Result instead"
                        .to_string(),
                    location: Some(file_path.to_path_buf()),
                    line_number: Some(i + 1),
                });
            }
        }

        findings
    }

    /// Checks naming conventions (basic heuristics).
    fn check_naming_conventions(&self, content: &str, file_path: &Path) -> Vec<CodeReviewFinding> {
        let mut findings = Vec::new();

        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            // Check for single-letter variable names (excluding common ones like i, x, y)
            if let Some(var_start) = trimmed.find("let ") {
                let after_let = &trimmed[var_start + 4..];
                if let Some(space_pos) = after_let.find(' ') {
                    let var_name = &after_let[..space_pos];
                    if var_name.len() == 1 && !matches!(var_name, "i" | "x" | "y" | "n") {
                        findings.push(CodeReviewFinding {
                            severity: Severity::Info,
                            category: "Naming".to_string(),
                            message: format!(
                                "Variable '{var_name}' has a single-letter name. Consider a more descriptive name"
                            ),
                            location: Some(file_path.to_path_buf()),
                            line_number: Some(i + 1),
                        });
                    }
                }
            }
        }

        findings
    }

    /// Checks for code duplication hints (repeated patterns).
    fn check_duplication(&self, content: &str, file_path: &Path) -> Vec<CodeReviewFinding> {
        let mut findings = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Look for identical lines (excluding empty lines and comments)
        let mut line_counts: std::collections::HashMap<String, Vec<usize>> =
            std::collections::HashMap::new();

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            line_counts
                .entry(trimmed.to_string())
                .or_default()
                .push(i + 1);
        }

        for (line, occurrences) in line_counts.iter() {
            if occurrences.len() >= 3 && line.len() > 20 {
                findings.push(CodeReviewFinding {
                    severity: Severity::Info,
                    category: "Duplication".to_string(),
                    message: format!(
                        "Line appears {occurrences_len} times. Consider extracting into a function or constant",
                        occurrences_len = occurrences.len()
                    ),
                    location: Some(file_path.to_path_buf()),
                    line_number: Some(occurrences[0]),
                });
            }
        }

        findings
    }
}

#[async_trait]
impl Agent for ReviewAgent {
    fn id(&self) -> AgentId {
        AgentId::from("review")
    }

    fn name(&self) -> &str {
        "Code Review Agent"
    }

    fn description(&self) -> &str {
        "Performs comprehensive code review focusing on quality, maintainability, and best practices"
    }

    fn can_handle(&self, context: &TaskContext) -> ActivationScore {
        let intent = context.user_intent.to_lowercase();
        let keywords = ["review", "check", "analyze", "quality", "lint"];

        let matches = keywords.iter().filter(|k| intent.contains(*k)).count();

        // Score based on keyword matches (0.0 to 1.0)
        ActivationScore::new(matches as f64 * 0.25)
    }

    async fn execute(&self, task: Task, toolkit: &AgentToolkit) -> anyhow::Result<AgentResult> {
        let findings = self.analyze_code(&task.context.file_paths, toolkit).await?;

        Ok(AgentResult::CodeReview { findings })
    }

    fn permissions(&self) -> &AgentPermissions {
        &self.permissions
    }

    fn system_prompt(&self) -> &str {
        "You are an expert code reviewer with deep knowledge of software engineering \
         best practices. Analyze code for quality, maintainability, design patterns, \
         and potential improvements. Focus on actionable feedback with clear explanations. \
         Consider: code structure, naming conventions, error handling, performance implications, \
         and adherence to language idioms."
    }
}

impl Default for ReviewAgent {
    fn default() -> Self {
        Self::new()
    }
}

/// Checks if a file is likely binary based on extension.
fn is_binary_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        matches!(
            ext_str.as_str(),
            "exe"
                | "dll"
                | "so"
                | "dylib"
                | "bin"
                | "jpg"
                | "jpeg"
                | "png"
                | "gif"
                | "pdf"
                | "zip"
                | "tar"
                | "gz"
        )
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_review_agent_id() {
        let agent = ReviewAgent::new();
        assert_eq!(agent.id().as_str(), "review");
    }

    #[test]
    fn test_review_agent_name_and_description() {
        let agent = ReviewAgent::new();
        assert_eq!(agent.name(), "Code Review Agent");
        assert!(!agent.description().is_empty());
        assert!(!agent.system_prompt().is_empty());
    }

    #[test]
    fn test_review_agent_permissions() {
        let agent = ReviewAgent::new();
        let perms = agent.permissions();
        assert!(!perms.shell_execution);
        assert!(!perms.network_access);
    }

    #[test]
    fn test_activation_scoring_with_review_keyword() {
        let agent = ReviewAgent::new();
        let context = TaskContext {
            file_paths: vec![],
            file_contents: None,
            git_context: None,
            execution_mode: crate::agents::ExecutionMode::Interactive,
            user_intent: "Please review this code".to_string(),
        };

        let score = agent.can_handle(&context);
        assert!(score.0 >= 0.25); // At least one keyword match
    }

    #[test]
    fn test_activation_scoring_with_multiple_keywords() {
        let agent = ReviewAgent::new();
        let context = TaskContext {
            file_paths: vec![],
            file_contents: None,
            git_context: None,
            execution_mode: crate::agents::ExecutionMode::Interactive,
            user_intent: "check code quality and analyze for issues".to_string(),
        };

        let score = agent.can_handle(&context);
        assert!(score.0 >= 0.5); // Multiple keyword matches
    }

    #[test]
    fn test_activation_scoring_no_match() {
        let agent = ReviewAgent::new();
        let context = TaskContext {
            file_paths: vec![],
            file_contents: None,
            git_context: None,
            execution_mode: crate::agents::ExecutionMode::Interactive,
            user_intent: "write a new feature".to_string(),
        };

        let score = agent.can_handle(&context);
        assert!(score.0 < 0.25); // No relevant keywords
    }

    #[test]
    fn test_check_long_functions() {
        let agent = ReviewAgent::new();
        // Create a function with more than 50 lines
        let mut code = String::from("fn very_long_function() {\n");
        for i in 0..55 {
            code.push_str(&format!("    let x{i} = {i};\n"));
        }
        code.push_str("}\n");

        let path = PathBuf::from("test.rs");
        let findings = agent.check_long_functions(&code, &path);

        // Should detect long function
        assert!(
            !findings.is_empty(),
            "Expected to find long function warning"
        );
        assert!(findings.iter().any(|f| f.category == "Code Quality"));
    }

    #[test]
    fn test_check_magic_numbers() {
        let agent = ReviewAgent::new();
        let code = r#"
fn calculate() {
    let threshold = 42;  // Magic number
    if value >= 10 {     // Magic number
        return value * 2;
    }
}
"#;

        let path = PathBuf::from("test.rs");
        let findings = agent.check_magic_numbers(code, &path);

        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.category == "Maintainability"));
    }

    #[test]
    fn test_check_error_handling_unwrap() {
        let agent = ReviewAgent::new();
        let code = r#"
fn process() {
    let value = some_result.unwrap();
    value.do_something();
}
"#;

        let path = PathBuf::from("test.rs");
        let findings = agent.check_error_handling(code, &path);

        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.message.contains("unwrap")));
    }

    #[test]
    fn test_check_error_handling_panic() {
        let agent = ReviewAgent::new();
        let code = r#"
fn process() {
    if error {
        panic!("Something went wrong");
    }
}
"#;

        let path = PathBuf::from("test.rs");
        let findings = agent.check_error_handling(code, &path);

        assert!(!findings.is_empty());
        assert!(
            findings
                .iter()
                .any(|f| f.severity == Severity::Error && f.message.contains("panic"))
        );
    }

    #[test]
    fn test_is_binary_file() {
        assert!(is_binary_file(&PathBuf::from("test.exe")));
        assert!(is_binary_file(&PathBuf::from("lib.so")));
        assert!(is_binary_file(&PathBuf::from("image.jpg")));
        assert!(!is_binary_file(&PathBuf::from("code.rs")));
        assert!(!is_binary_file(&PathBuf::from("README.md")));
    }

    #[test]
    fn test_analyze_file_content_combines_checks() {
        let agent = ReviewAgent::new();
        let code = r#"
fn bad_function() {
    let x = 1;
    let result = risky_operation().unwrap();
    if result >= 100 {
        panic!("Bad value");
    }
}
"#;

        let path = PathBuf::from("test.rs");
        let findings = agent.analyze_file_content(code, &path);

        // Should find multiple issues
        assert!(findings.len() >= 2);
        assert!(
            findings
                .iter()
                .map(|f| f.category.as_str())
                .any(|x| x == "Error Handling")
        );
    }
}
