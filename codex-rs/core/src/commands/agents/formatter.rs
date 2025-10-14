//! Agent result formatting for different output formats.

use crate::agents::AgentResult;
use crate::agents::CodeReviewFinding;
use crate::agents::Severity;

/// Output format for agent results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Markdown format with rich formatting.
    Markdown,
    /// JSON format for programmatic consumption.
    Json,
    /// Plain text format for simple display.
    PlainText,
}

/// Formats agent results into different output formats.
pub struct AgentResultFormatter;

impl AgentResultFormatter {
    /// Formats an agent result into the specified output format.
    ///
    /// # Arguments
    ///
    /// * `result` - The agent result to format
    /// * `format` - The desired output format
    ///
    /// # Returns
    ///
    /// A formatted string representation of the agent result.
    ///
    /// # Examples
    ///
    /// ```
    /// # use codex_core::commands::agents::formatter::{AgentResultFormatter, OutputFormat};
    /// # use codex_core::agents::AgentResult;
    /// # use std::collections::HashMap;
    /// let result = AgentResult::Analysis {
    ///     summary: "Code looks good".to_string(),
    ///     details: HashMap::new(),
    /// };
    ///
    /// let markdown = AgentResultFormatter::format(&result, OutputFormat::Markdown);
    /// assert!(markdown.contains("# Agent Analysis"));
    /// ```
    pub fn format(result: &AgentResult, format: OutputFormat) -> String {
        match format {
            OutputFormat::Markdown => Self::format_markdown(result),
            OutputFormat::Json => Self::format_json(result),
            OutputFormat::PlainText => Self::format_plain(result),
        }
    }

    /// Formats result as Markdown with rich formatting.
    fn format_markdown(result: &AgentResult) -> String {
        match result {
            AgentResult::Analysis { summary, details } => {
                let mut output = format!("# Agent Analysis\n\n{summary}\n");
                if !details.is_empty() {
                    output.push_str("\n## Details\n\n");
                    for (key, value) in details {
                        output.push_str(&format!("- **{key}**: {value}\n"));
                    }
                }
                output
            }
            AgentResult::CodeReview { findings } => {
                let mut output = String::from("# Code Review\n\n");
                if findings.is_empty() {
                    output.push_str("✅ No issues found.\n");
                } else {
                    output.push_str(&format!("Found {len} issue(s):\n\n", len = findings.len()));

                    // Group by severity
                    let errors: Vec<_> = findings
                        .iter()
                        .filter(|f| f.severity == Severity::Error)
                        .collect();
                    let warnings: Vec<_> = findings
                        .iter()
                        .filter(|f| f.severity == Severity::Warning)
                        .collect();
                    let info: Vec<_> = findings
                        .iter()
                        .filter(|f| f.severity == Severity::Info)
                        .collect();

                    if !errors.is_empty() {
                        output.push_str(&format!("## ❌ Errors ({})\n\n", errors.len()));
                        for finding in errors {
                            output.push_str(&Self::format_finding_markdown(finding));
                        }
                    }

                    if !warnings.is_empty() {
                        output.push_str(&format!("## ⚠️  Warnings ({})\n\n", warnings.len()));
                        for finding in warnings {
                            output.push_str(&Self::format_finding_markdown(finding));
                        }
                    }

                    if !info.is_empty() {
                        output.push_str(&format!("## ℹ️  Info ({})\n\n", info.len()));
                        for finding in info {
                            output.push_str(&Self::format_finding_markdown(finding));
                        }
                    }
                }
                output
            }
            AgentResult::Suggestions { items } => {
                let mut output = String::from("# Suggestions\n\n");
                if items.is_empty() {
                    output.push_str("💡 No suggestions available.\n");
                } else {
                    for (i, suggestion) in items.iter().enumerate() {
                        output.push_str(&format!(
                            "## {num}. {title}\n\n",
                            num = i + 1,
                            title = suggestion.title
                        ));
                        output.push_str(&format!("{desc}\n", desc = suggestion.description));
                        if let Some(code) = &suggestion.code_change {
                            output.push_str(&format!("\n```\n{code}\n```\n"));
                        }
                        output.push('\n');
                    }
                }
                output
            }
        }
    }

    /// Formats a single code review finding as Markdown.
    fn format_finding_markdown(finding: &CodeReviewFinding) -> String {
        let mut output = format!(
            "**{cat}**: {msg}\n",
            cat = finding.category,
            msg = finding.message
        );
        if let Some(location) = &finding.location {
            output.push_str(&format!(
                "  📍 Location: `{path}:{line}`\n",
                path = location.display(),
                line = finding
                    .line_number
                    .map_or(String::from("?"), |n| n.to_string())
            ));
        }
        output.push('\n');
        output
    }

    /// Formats result as JSON for programmatic consumption.
    fn format_json(result: &AgentResult) -> String {
        match result {
            AgentResult::Analysis { summary, details } => {
                let obj = serde_json::json!({
                    "type": "analysis",
                    "summary": summary,
                    "details": details,
                });
                serde_json::to_string_pretty(&obj).unwrap_or_else(|_| "{}".to_string())
            }
            AgentResult::CodeReview { findings } => {
                let findings_json: Vec<_> = findings
                    .iter()
                    .map(|f| {
                        let mut obj = serde_json::json!({
                            "severity": match f.severity {
                                Severity::Error => "error",
                                Severity::Warning => "warning",
                                Severity::Info => "info",
                            },
                            "category": f.category,
                            "message": f.message,
                        });
                        if let Some(location) = &f.location {
                            obj["location"] = serde_json::json!({
                                "path": location.to_string_lossy(),
                                "line": f.line_number,
                            });
                        }
                        obj
                    })
                    .collect();

                let obj = serde_json::json!({
                    "type": "code_review",
                    "findings": findings_json,
                    "count": findings.len(),
                });
                serde_json::to_string_pretty(&obj).unwrap_or_else(|_| "{}".to_string())
            }
            AgentResult::Suggestions { items } => {
                let items_json: Vec<_> = items
                    .iter()
                    .map(|s| {
                        let mut obj = serde_json::json!({
                            "title": s.title,
                            "description": s.description,
                        });
                        if let Some(code) = &s.code_change {
                            obj["code_change"] = serde_json::Value::String(code.clone());
                        }
                        obj
                    })
                    .collect();

                let obj = serde_json::json!({
                    "type": "suggestions",
                    "items": items_json,
                    "count": items.len(),
                });
                serde_json::to_string_pretty(&obj).unwrap_or_else(|_| "{}".to_string())
            }
        }
    }

    /// Formats result as plain text without special formatting.
    fn format_plain(result: &AgentResult) -> String {
        match result {
            AgentResult::Analysis { summary, details } => {
                let mut output = format!(
                    "Agent Analysis\n{sep}\n\n{summary}\n",
                    sep = "=".repeat(15),
                    summary = summary
                );
                if !details.is_empty() {
                    output.push_str("\nDetails:\n\n");
                    for (key, value) in details {
                        output.push_str(&format!("- {key}: {value}\n"));
                    }
                }
                output
            }
            AgentResult::CodeReview { findings } => {
                let mut output = String::from("Code Review\n===========\n\n");
                if findings.is_empty() {
                    output.push_str("No issues found.\n");
                } else {
                    output.push_str(&format!("Found {len} issue(s):\n\n", len = findings.len()));
                    for (i, finding) in findings.iter().enumerate() {
                        let severity_str = match finding.severity {
                            Severity::Error => "ERROR",
                            Severity::Warning => "WARNING",
                            Severity::Info => "INFO",
                        };
                        output.push_str(&format!(
                            "{index}. [{sev}] {cat}: {msg}\n",
                            index = i + 1,
                            sev = severity_str,
                            cat = finding.category,
                            msg = finding.message
                        ));
                        if let Some(location) = &finding.location {
                            output.push_str(&format!(
                                "   Location: {}:{}\n",
                                location.display(),
                                finding
                                    .line_number
                                    .map_or(String::from("?"), |n| n.to_string())
                            ));
                        }
                        output.push('\n');
                    }
                }
                output
            }
            AgentResult::Suggestions { items } => {
                let mut output = String::from("Suggestions\n===========\n\n");
                if items.is_empty() {
                    output.push_str("No suggestions available.\n");
                } else {
                    for (i, suggestion) in items.iter().enumerate() {
                        output.push_str(&format!(
                            "{num}. {title}\n\n",
                            num = i + 1,
                            title = suggestion.title
                        ));
                        output.push_str(&format!("   {desc}\n", desc = suggestion.description));
                        if let Some(code) = &suggestion.code_change {
                            output.push_str(&format!(
                                "\n   Code change:\n   {}\n",
                                code.replace('\n', "\n   ")
                            ));
                        }
                        output.push('\n');
                    }
                }
                output
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::Suggestion;
    use std::collections::HashMap;
    use std::path::PathBuf;

    // ===== Markdown Format Tests =====

    #[test]
    fn test_markdown_analysis_basic() {
        let mut details = HashMap::new();
        details.insert("complexity".to_string(), "Low".to_string());
        details.insert("test_coverage".to_string(), "85%".to_string());

        let result = AgentResult::Analysis {
            summary: "Code quality is excellent".to_string(),
            details,
        };

        let output = AgentResultFormatter::format(&result, OutputFormat::Markdown);

        assert!(output.contains("# Agent Analysis"));
        assert!(output.contains("Code quality is excellent"));
        assert!(output.contains("## Details"));
        assert!(output.contains("**complexity**: Low"));
        assert!(output.contains("**test_coverage**: 85%"));
    }

    #[test]
    fn test_markdown_code_review_with_findings() {
        let findings = vec![
            CodeReviewFinding {
                severity: Severity::Error,
                category: "Security".to_string(),
                message: "SQL injection vulnerability".to_string(),
                location: Some(PathBuf::from("src/db.rs")),
                line_number: Some(42),
            },
            CodeReviewFinding {
                severity: Severity::Warning,
                category: "Performance".to_string(),
                message: "Inefficient loop".to_string(),
                location: Some(PathBuf::from("src/utils.rs")),
                line_number: Some(100),
            },
            CodeReviewFinding {
                severity: Severity::Info,
                category: "Style".to_string(),
                message: "Consider using const".to_string(),
                location: None,
                line_number: None,
            },
        ];

        let result = AgentResult::CodeReview { findings };

        let output = AgentResultFormatter::format(&result, OutputFormat::Markdown);

        assert!(output.contains("# Code Review"));
        assert!(output.contains("Found 3 issue(s)"));
        assert!(output.contains("## ❌ Errors (1)"));
        assert!(output.contains("## ⚠️  Warnings (1)"));
        assert!(output.contains("## ℹ️  Info (1)"));
        assert!(output.contains("SQL injection vulnerability"));
        assert!(output.contains("📍 Location: `src/db.rs:42`"));
    }

    #[test]
    fn test_markdown_suggestions_with_code() {
        let items = vec![
            Suggestion {
                title: "Use async/await".to_string(),
                description: "Replace callbacks with async/await for better readability"
                    .to_string(),
                code_change: Some(
                    "async fn fetch_data() -> Result<Data> {\n    Ok(data)\n}".to_string(),
                ),
            },
            Suggestion {
                title: "Add error handling".to_string(),
                description: "Wrap in Result type".to_string(),
                code_change: None,
            },
        ];

        let result = AgentResult::Suggestions { items };

        let output = AgentResultFormatter::format(&result, OutputFormat::Markdown);

        assert!(output.contains("# Suggestions"));
        assert!(output.contains("## 1. Use async/await"));
        assert!(output.contains("Replace callbacks with async/await"));
        assert!(output.contains("```\nasync fn fetch_data()"));
        assert!(output.contains("## 2. Add error handling"));
    }

    #[test]
    fn test_markdown_empty_results() {
        let result = AgentResult::Analysis {
            summary: "Analysis complete".to_string(),
            details: HashMap::new(),
        };
        let output = AgentResultFormatter::format(&result, OutputFormat::Markdown);
        assert!(output.contains("# Agent Analysis"));
        assert!(!output.contains("## Details")); // No details section when empty

        let result = AgentResult::CodeReview { findings: vec![] };
        let output = AgentResultFormatter::format(&result, OutputFormat::Markdown);
        assert!(output.contains("✅ No issues found"));

        let result = AgentResult::Suggestions { items: vec![] };
        let output = AgentResultFormatter::format(&result, OutputFormat::Markdown);
        assert!(output.contains("💡 No suggestions available"));
    }

    #[test]
    fn test_markdown_special_characters() {
        let mut details = HashMap::new();
        details.insert(
            "note".to_string(),
            "Use `Option<T>` instead of nullable".to_string(),
        );

        let result = AgentResult::Analysis {
            summary: "Code uses **bold** and *italic* syntax".to_string(),
            details,
        };

        let output = AgentResultFormatter::format(&result, OutputFormat::Markdown);

        // Special characters should be preserved in Markdown
        assert!(output.contains("**bold**"));
        assert!(output.contains("*italic*"));
        assert!(output.contains("`Option<T>`"));
    }

    // ===== JSON Format Tests =====

    #[test]
    fn test_json_analysis_structure() {
        let mut details = HashMap::new();
        details.insert("metric1".to_string(), "value1".to_string());

        let result = AgentResult::Analysis {
            summary: "Test summary".to_string(),
            details,
        };

        let output = AgentResultFormatter::format(&result, OutputFormat::Json);

        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&output).expect("Valid JSON");
        assert_eq!(parsed["type"], "analysis");
        assert_eq!(parsed["summary"], "Test summary");
        assert_eq!(parsed["details"]["metric1"], "value1");
    }

    #[test]
    fn test_json_code_review_valid_json() {
        let findings = vec![CodeReviewFinding {
            severity: Severity::Error,
            category: "Test".to_string(),
            message: "Test message".to_string(),
            location: Some(PathBuf::from("test.rs")),
            line_number: Some(10),
        }];

        let result = AgentResult::CodeReview { findings };

        let output = AgentResultFormatter::format(&result, OutputFormat::Json);

        let parsed: serde_json::Value = serde_json::from_str(&output).expect("Valid JSON");
        assert_eq!(parsed["type"], "code_review");
        assert_eq!(parsed["count"], 1);
        assert_eq!(parsed["findings"][0]["severity"], "error");
        assert_eq!(parsed["findings"][0]["category"], "Test");
        assert_eq!(parsed["findings"][0]["location"]["line"], 10);
    }

    #[test]
    fn test_json_suggestions_parseable() {
        let items = vec![Suggestion {
            title: "Suggestion title".to_string(),
            description: "Description here".to_string(),
            code_change: Some("fn test() {}".to_string()),
        }];

        let result = AgentResult::Suggestions { items };

        let output = AgentResultFormatter::format(&result, OutputFormat::Json);

        let parsed: serde_json::Value = serde_json::from_str(&output).expect("Valid JSON");
        assert_eq!(parsed["type"], "suggestions");
        assert_eq!(parsed["count"], 1);
        assert_eq!(parsed["items"][0]["title"], "Suggestion title");
        assert_eq!(parsed["items"][0]["code_change"], "fn test() {}");
    }

    #[test]
    fn test_json_empty_arrays() {
        let result = AgentResult::CodeReview { findings: vec![] };
        let output = AgentResultFormatter::format(&result, OutputFormat::Json);

        let parsed: serde_json::Value = serde_json::from_str(&output).expect("Valid JSON");
        assert_eq!(parsed["count"], 0);
        assert!(parsed["findings"].is_array());
        assert_eq!(parsed["findings"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_json_escaping() {
        let result = AgentResult::Analysis {
            summary: "Contains \"quotes\" and\nnewlines".to_string(),
            details: HashMap::new(),
        };

        let output = AgentResultFormatter::format(&result, OutputFormat::Json);

        // Should be valid JSON despite special characters
        let parsed: serde_json::Value =
            serde_json::from_str(&output).expect("Valid JSON with escaping");
        assert!(parsed["summary"].as_str().unwrap().contains("quotes"));
        assert!(parsed["summary"].as_str().unwrap().contains("newlines"));
    }

    // ===== Plain Text Format Tests =====

    #[test]
    fn test_plain_analysis_readable() {
        let mut details = HashMap::new();
        details.insert("key".to_string(), "value".to_string());

        let result = AgentResult::Analysis {
            summary: "Plain text summary".to_string(),
            details,
        };

        let output = AgentResultFormatter::format(&result, OutputFormat::PlainText);

        assert!(output.contains("Agent Analysis"));
        assert!(output.contains("==========="));
        assert!(output.contains("Plain text summary"));
        assert!(output.contains("Details:"));
        assert!(output.contains("- key: value"));
        // Should not contain Markdown syntax
        assert!(!output.contains("#"));
        assert!(!output.contains("**"));
    }

    #[test]
    fn test_plain_code_review_severity_display() {
        let findings = vec![
            CodeReviewFinding {
                severity: Severity::Error,
                category: "Bug".to_string(),
                message: "Error message".to_string(),
                location: None,
                line_number: None,
            },
            CodeReviewFinding {
                severity: Severity::Warning,
                category: "Style".to_string(),
                message: "Warning message".to_string(),
                location: None,
                line_number: None,
            },
        ];

        let result = AgentResult::CodeReview { findings };

        let output = AgentResultFormatter::format(&result, OutputFormat::PlainText);

        assert!(output.contains("[ERROR]"));
        assert!(output.contains("[WARNING]"));
        assert!(output.contains("Bug: Error message"));
        // No emoji in plain text
        assert!(!output.contains("❌"));
        assert!(!output.contains("⚠️"));
    }

    #[test]
    fn test_plain_suggestions_numbered() {
        let items = vec![
            Suggestion {
                title: "First suggestion".to_string(),
                description: "Description 1".to_string(),
                code_change: None,
            },
            Suggestion {
                title: "Second suggestion".to_string(),
                description: "Description 2".to_string(),
                code_change: Some("code here".to_string()),
            },
        ];

        let result = AgentResult::Suggestions { items };

        let output = AgentResultFormatter::format(&result, OutputFormat::PlainText);

        assert!(output.contains("1. First suggestion"));
        assert!(output.contains("2. Second suggestion"));
        assert!(output.contains("Code change:"));
        assert!(output.contains("code here"));
    }

    #[test]
    fn test_plain_no_markdown_syntax() {
        let result = AgentResult::Analysis {
            summary: "No **bold** or *italic* here".to_string(),
            details: HashMap::new(),
        };

        let output = AgentResultFormatter::format(&result, OutputFormat::PlainText);

        // Markdown syntax should be preserved as-is (not interpreted)
        assert!(output.contains("**bold**"));
        assert!(output.contains("*italic*"));
        // But no Markdown headers
        assert!(!output.starts_with("#"));
    }

    #[test]
    fn test_plain_unicode_safe() {
        let findings = vec![CodeReviewFinding {
            severity: Severity::Info,
            category: "Unicode Test 中文".to_string(),
            message: "Message with émojis 🎉".to_string(),
            location: None,
            line_number: None,
        }];

        let result = AgentResult::CodeReview { findings };

        let output = AgentResultFormatter::format(&result, OutputFormat::PlainText);

        // Unicode should be preserved
        assert!(output.contains("中文"));
        assert!(output.contains("émojis"));
        // But formatter-added emoji should not appear
        // (Plain text uses text labels instead)
        assert!(!output.contains("ℹ️"));
    }
}
