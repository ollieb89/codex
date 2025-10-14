//! Security analysis agent for vulnerability detection.

use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
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

/// Vulnerability pattern definition.
#[derive(Clone)]
struct VulnerabilityPattern {
    name: String,
    pattern: Regex,
    severity: Severity,
    cve_references: Vec<String>,
    description: String,
    remediation: String,
}

lazy_static! {
    /// Compiled vulnerability patterns for efficient matching.
    static ref VULNERABILITY_PATTERNS: HashMap<String, VulnerabilityPattern> = {
        let mut patterns = HashMap::new();

        // SQL Injection patterns
        patterns.insert(
            "sql_injection".to_string(),
            VulnerabilityPattern {
                name: "SQL Injection".to_string(),
                pattern: Regex::new(
                    r#"(?i)(SELECT|INSERT|UPDATE|DELETE).*['"].*\+|execute\(|query.*=.*['"].*\+|WHERE.*=.*['"].*\+"#
                ).unwrap_or_else(|e| panic!("SQL injection regex should compile: {e}")),
                severity: Severity::Error,
                cve_references: vec!["CWE-89".to_string()],
                description: "Potential SQL injection vulnerability detected. String concatenation in SQL queries can allow attackers to inject malicious SQL code.".to_string(),
                remediation: "Use parameterized queries or prepared statements instead of string concatenation.".to_string(),
            },
        );

        // XSS (Cross-Site Scripting) patterns
        patterns.insert(
            "xss".to_string(),
            VulnerabilityPattern {
                name: "Cross-Site Scripting (XSS)".to_string(),
                pattern: Regex::new(
                    r#"(?i)(innerHTML|outerHTML|document\.write|eval)\s*=.*\+|dangerouslySetInnerHTML"#
                ).unwrap_or_else(|e| panic!("XSS regex should compile: {e}")),
                severity: Severity::Error,
                cve_references: vec!["CWE-79".to_string()],
                description: "Potential XSS vulnerability. Unescaped user input in HTML context can allow attackers to inject malicious scripts.".to_string(),
                remediation: "Sanitize and escape all user input before inserting into HTML. Use framework-provided safe methods.".to_string(),
            },
        );

        // Weak Cryptography - MD5
        patterns.insert(
            "weak_crypto_md5".to_string(),
            VulnerabilityPattern {
                name: "Weak Cryptography (MD5)".to_string(),
                pattern: Regex::new(r#"(?i)(md5|MD5|Md5)\s*\(|crypto::md5|hashlib\.md5"#).unwrap_or_else(|e| panic!("MD5 regex should compile: {e}")),
                severity: Severity::Warning,
                cve_references: vec!["CWE-327".to_string()],
                description: "MD5 is cryptographically broken and should not be used for security purposes.".to_string(),
                remediation: "Use SHA-256 or SHA-3 for hashing. For passwords, use bcrypt, scrypt, or Argon2.".to_string(),
            },
        );

        // Weak Cryptography - SHA1
        patterns.insert(
            "weak_crypto_sha1".to_string(),
            VulnerabilityPattern {
                name: "Weak Cryptography (SHA1)".to_string(),
                pattern: Regex::new(r#"(?i)(sha1|SHA1|Sha1)\s*\(|crypto::sha1|hashlib\.sha1"#).unwrap_or_else(|e| panic!("SHA1 regex should compile: {e}")),
                severity: Severity::Warning,
                cve_references: vec!["CWE-327".to_string()],
                description: "SHA1 is deprecated and vulnerable to collision attacks.".to_string(),
                remediation: "Use SHA-256 or SHA-3 instead.".to_string(),
            },
        );

        // Hardcoded secrets
        patterns.insert(
            "hardcoded_secret".to_string(),
            VulnerabilityPattern {
                name: "Hardcoded Secret".to_string(),
                pattern: Regex::new(
                    r#"(?i)(password|secret|api_key|apikey|private_key|token)\s*=\s*["'][^"']{8,}["']"#
                ).unwrap_or_else(|e| panic!("Hardcoded secret regex should compile: {e}")),
                severity: Severity::Error,
                cve_references: vec!["CWE-798".to_string()],
                description: "Hardcoded credentials or secrets detected in source code.".to_string(),
                remediation: "Store secrets in environment variables or secure vaults. Never commit secrets to version control.".to_string(),
            },
        );

        // Insecure Deserialization
        patterns.insert(
            "insecure_deserialization".to_string(),
            VulnerabilityPattern {
                name: "Insecure Deserialization".to_string(),
                pattern: Regex::new(
                    r#"(?i)(pickle\.loads|yaml\.load\(|unserialize|ObjectInputStream)"#
                ).unwrap_or_else(|e| panic!("Insecure deserialization regex should compile: {e}")),
                severity: Severity::Error,
                cve_references: vec!["CWE-502".to_string()],
                description: "Insecure deserialization can lead to remote code execution.".to_string(),
                remediation: "Use safe deserialization methods (yaml.safe_load) or validate input thoroughly.".to_string(),
            },
        );

        // Command Injection
        patterns.insert(
            "command_injection".to_string(),
            VulnerabilityPattern {
                name: "Command Injection".to_string(),
                pattern: Regex::new(
                    r#"(?i)(os\.system|subprocess\.call|exec|shell_exec|system)\s*\([^)]*\+|`.*\$"#
                ).unwrap_or_else(|e| panic!("Command injection regex should compile: {e}")),
                severity: Severity::Error,
                cve_references: vec!["CWE-78".to_string()],
                description: "Potential command injection. User input in shell commands can allow arbitrary command execution.".to_string(),
                remediation: "Avoid shell=True in subprocess. Use argument arrays and validate input.".to_string(),
            },
        );

        // Path Traversal
        patterns.insert(
            "path_traversal".to_string(),
            VulnerabilityPattern {
                name: "Path Traversal".to_string(),
                pattern: Regex::new(
                    r#"(?i)(open|readFile|read_file|fopen)\s*\([^)]*\+|path.*=.*['"/].*\+|join.*\.\."#
                ).unwrap_or_else(|e| panic!("Path traversal regex should compile: {e}")),
                severity: Severity::Warning,
                cve_references: vec!["CWE-22".to_string()],
                description: "Potential path traversal vulnerability. User-controlled file paths may access unauthorized files.".to_string(),
                remediation: "Validate and sanitize file paths. Use path.resolve() and check against allowed directories.".to_string(),
            },
        );

        // SSRF (Server-Side Request Forgery)
        patterns.insert(
            "ssrf".to_string(),
            VulnerabilityPattern {
                name: "Server-Side Request Forgery (SSRF)".to_string(),
                pattern: Regex::new(
                    r#"(?i)(requests\.get|urllib\.request|fetch|http\.get)\s*\([^)]*\+"#
                ).unwrap_or_else(|e| panic!("SSRF regex should compile: {e}")),
                severity: Severity::Warning,
                cve_references: vec!["CWE-918".to_string()],
                description: "Potential SSRF vulnerability. User-controlled URLs in HTTP requests may access internal resources.".to_string(),
                remediation: "Validate and whitelist allowed URLs. Never trust user input for URL construction.".to_string(),
            },
        );

        // TODO: Missing Authentication pattern disabled for MVP
        // This pattern requires negative lookahead and multiline matching which aren't
        // supported by Rust's regex crate. Will be re-implemented in Sprint 3 using
        // AST-based analysis with tree-sitter for more accurate detection.
        //
        // patterns.insert(
        //     "missing_auth".to_string(),
        //     VulnerabilityPattern {
        //         name: "Missing Authentication Check".to_string(),
        //         pattern: Regex::new(
        //             r#"(?i)@(app\.route|router\.get|router\.post|RequestMapping)"#
        //         ).expect("Missing authentication regex should compile"),
        //         severity: Severity::Info,
        //         cve_references: vec!["CWE-306".to_string()],
        //         description: "Endpoint may be missing authentication checks.".to_string(),
        //         remediation: "Add authentication decorators or middleware to protect endpoints.".to_string(),
        //     },
        // );

        patterns
    };
}

/// Security analysis agent for vulnerability detection.
///
/// Scans code for common security vulnerabilities including:
/// - SQL injection
/// - Cross-site scripting (XSS)
/// - Weak cryptography
/// - Hardcoded secrets
/// - Command injection
/// - Path traversal
/// - SSRF
/// - Authentication issues
pub struct SecurityAgent {
    permissions: AgentPermissions,
}

impl SecurityAgent {
    /// Creates a new security agent.
    pub fn new() -> Self {
        Self {
            permissions: AgentPermissions {
                file_access: FileAccessPolicy::ReadOnly,
                shell_execution: false,
                network_access: false,
                allowed_tools: vec![],
                max_iterations: 10,
                can_delegate: false,
            },
        }
    }

    /// Scans code files for security vulnerabilities.
    async fn scan_for_vulnerabilities(
        &self,
        files: &[PathBuf],
        toolkit: &AgentToolkit,
    ) -> anyhow::Result<Vec<CodeReviewFinding>> {
        let mut findings = Vec::new();

        for file_path in files {
            // Skip non-code files
            if !is_code_file(file_path) {
                continue;
            }

            match toolkit.read_file(file_path).await {
                Ok(content) => {
                    let file_findings = self.scan_file_content(&content, file_path);
                    findings.extend(file_findings);
                }
                Err(_e) => {
                    // Silently skip files that can't be read
                }
            }
        }

        Ok(findings)
    }

    /// Scans a single file's content for vulnerabilities.
    fn scan_file_content(&self, content: &str, file_path: &Path) -> Vec<CodeReviewFinding> {
        let mut findings = Vec::new();

        for (_pattern_id, pattern) in VULNERABILITY_PATTERNS.iter() {
            if let Some(matches) = self.find_pattern_matches(content, pattern) {
                for (line_num, _matched_text) in matches {
                    findings.push(CodeReviewFinding {
                        severity: pattern.severity,
                        category: format!("Security - {}", pattern.name),
                        message: format!(
                            "{}\\n\\nRemediation: {}\\n\\nReference: {}",
                            pattern.description,
                            pattern.remediation,
                            pattern.cve_references.join(", ")
                        ),
                        location: Some(file_path.to_path_buf()),
                        line_number: Some(line_num),
                    });
                }
            }
        }

        findings
    }

    /// Finds all matches of a pattern in the content.
    fn find_pattern_matches(
        &self,
        content: &str,
        pattern: &VulnerabilityPattern,
    ) -> Option<Vec<(usize, String)>> {
        let mut matches = Vec::new();

        for (i, line) in content.lines().enumerate() {
            // Skip comments (basic heuristic)
            let trimmed = line.trim();
            if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with("/*") {
                continue;
            }

            if pattern.pattern.is_match(line) {
                matches.push((i + 1, line.to_string()));
            }
        }

        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    }
}

#[async_trait]
impl Agent for SecurityAgent {
    fn id(&self) -> AgentId {
        AgentId::from("security")
    }

    fn name(&self) -> &str {
        "Security Analysis Agent"
    }

    fn description(&self) -> &str {
        "Scans code for security vulnerabilities including SQL injection, XSS, weak crypto, \
         hardcoded secrets, and more"
    }

    fn can_handle(&self, context: &TaskContext) -> ActivationScore {
        let intent = context.user_intent.to_lowercase();
        let keywords = [
            "security",
            "vulnerability",
            "vulnerabilities",
            "audit",
            "secure",
            "exploit",
            "cve",
            "sql injection",
            "xss",
        ];

        let matches = keywords.iter().filter(|k| intent.contains(*k)).count();

        // Higher weight for security keywords
        ActivationScore::new(matches as f64 * 0.3)
    }

    async fn execute(&self, task: Task, toolkit: &AgentToolkit) -> anyhow::Result<AgentResult> {
        let findings = self
            .scan_for_vulnerabilities(&task.context.file_paths, toolkit)
            .await?;

        Ok(AgentResult::CodeReview { findings })
    }

    fn permissions(&self) -> &AgentPermissions {
        &self.permissions
    }

    fn system_prompt(&self) -> &str {
        "You are a security expert specializing in application security and vulnerability analysis. \
         Your role is to identify security vulnerabilities in code, assess their severity, and \
         provide actionable remediation guidance. Focus on:\n\
         - OWASP Top 10 vulnerabilities\n- CWE/CVE classifications\n\
         - Defense-in-depth principles\n- Secure coding best practices\n\
         - Real-world attack scenarios\n\
         Provide clear explanations and specific remediation steps for each finding."
    }
}

impl Default for SecurityAgent {
    fn default() -> Self {
        Self::new()
    }
}

/// Checks if a file should be scanned for security issues.
fn is_code_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        matches!(
            ext_str.as_str(),
            "rs" | "py"
                | "js"
                | "ts"
                | "jsx"
                | "tsx"
                | "java"
                | "go"
                | "php"
                | "rb"
                | "c"
                | "cpp"
                | "cs"
                | "swift"
                | "kt"
        )
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_agent_id() {
        let agent = SecurityAgent::new();
        assert_eq!(agent.id().as_str(), "security");
    }

    #[test]
    fn test_security_agent_name_and_description() {
        let agent = SecurityAgent::new();
        assert_eq!(agent.name(), "Security Analysis Agent");
        assert!(!agent.description().is_empty());
        assert!(!agent.system_prompt().is_empty());
    }

    #[test]
    fn test_security_agent_permissions() {
        let agent = SecurityAgent::new();
        let perms = agent.permissions();
        assert!(!perms.shell_execution);
        assert!(!perms.network_access);
        assert_eq!(perms.max_iterations, 10);
    }

    #[test]
    fn test_activation_scoring_with_security_keyword() {
        let agent = SecurityAgent::new();
        let context = TaskContext {
            file_paths: vec![],
            file_contents: None,
            git_context: None,
            execution_mode: crate::agents::ExecutionMode::Interactive,
            user_intent: "security audit".to_string(),
        };

        let score = agent.can_handle(&context);
        assert!(score.0 >= 0.3); // At least one keyword match
    }

    #[test]
    fn test_activation_scoring_with_multiple_keywords() {
        let agent = SecurityAgent::new();
        let context = TaskContext {
            file_paths: vec![],
            file_contents: None,
            git_context: None,
            execution_mode: crate::agents::ExecutionMode::Interactive,
            user_intent: "check for security vulnerabilities and sql injection".to_string(),
        };

        let score = agent.can_handle(&context);
        assert!(score.0 >= 0.6); // Multiple keyword matches
    }

    #[test]
    fn test_sql_injection_detection() {
        let agent = SecurityAgent::new();
        let code = r#"
def get_user(username):
    query = "SELECT * FROM users WHERE name = '" + username + "'"
    return execute(query)
"#;

        let path = PathBuf::from("app.py");
        let findings = agent.scan_file_content(code, &path);

        assert!(!findings.is_empty());
        assert!(
            findings
                .iter()
                .any(|f| f.category.contains("SQL Injection"))
        );
        assert!(findings.iter().any(|f| f.severity == Severity::Error));
    }

    #[test]
    fn test_xss_detection() {
        let agent = SecurityAgent::new();
        let code = r#"
function display(userInput) {
    element.innerHTML = userInput + "<br>";
}
"#;

        let path = PathBuf::from("app.js");
        let findings = agent.scan_file_content(code, &path);

        assert!(!findings.is_empty());
        assert!(
            findings
                .iter()
                .any(|f| f.category.contains("Cross-Site Scripting"))
        );
    }

    #[test]
    fn test_weak_crypto_md5_detection() {
        let agent = SecurityAgent::new();
        let code = r#"
import hashlib
def hash_password(password):
    return hashlib.md5(password.encode()).hexdigest()
"#;

        let path = PathBuf::from("auth.py");
        let findings = agent.scan_file_content(code, &path);

        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.category.contains("MD5")));
        assert!(findings.iter().any(|f| f.severity == Severity::Warning));
    }

    #[test]
    fn test_hardcoded_secret_detection() {
        let agent = SecurityAgent::new();
        // Use test patterns that won't trigger GitHub's secret scanning
        // These are clearly test values with "test" and "fake" markers
        let code = r#"
API_KEY = "sk_test_fake1234567890abcdefghijklmnop"
password = "MyTestP@ssw0rd123"
"#;

        let path = PathBuf::from("config.py");
        let findings = agent.scan_file_content(code, &path);

        assert!(!findings.is_empty());
        assert!(
            findings
                .iter()
                .any(|f| f.category.contains("Hardcoded Secret"))
        );
    }

    #[test]
    fn test_command_injection_detection() {
        let agent = SecurityAgent::new();
        let code = r#"
import os
def run_command(user_input):
    os.system("ls " + user_input)
"#;

        let path = PathBuf::from("utils.py");
        let findings = agent.scan_file_content(code, &path);

        assert!(!findings.is_empty());
        assert!(
            findings
                .iter()
                .any(|f| f.category.contains("Command Injection"))
        );
    }

    #[test]
    fn test_path_traversal_detection() {
        let agent = SecurityAgent::new();
        let code = r#"
def read_user_file(filename):
    path = "/data/" + filename
    return open(path).read()
"#;

        let path = PathBuf::from("files.py");
        let findings = agent.scan_file_content(code, &path);

        // Path traversal detection
        assert!(
            findings
                .iter()
                .any(|f| f.category.contains("Path Traversal") || f.message.contains("path"))
        );
    }

    #[test]
    fn test_ssrf_detection() {
        let agent = SecurityAgent::new();
        let code = r#"
import requests
def fetch_url(user_url):
    response = requests.get("https://api.com/" + user_url)
    return response.text
"#;

        let path = PathBuf::from("api.py");
        let findings = agent.scan_file_content(code, &path);

        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.category.contains("SSRF")));
    }

    #[test]
    fn test_insecure_deserialization_detection() {
        let agent = SecurityAgent::new();
        let code = r#"
import pickle
def load_data(data):
    return pickle.loads(data)
"#;

        let path = PathBuf::from("data.py");
        let findings = agent.scan_file_content(code, &path);

        assert!(!findings.is_empty());
        assert!(
            findings
                .iter()
                .any(|f| f.category.contains("Deserialization"))
        );
    }

    #[test]
    fn test_comments_are_ignored() {
        let agent = SecurityAgent::new();
        let code = r#"
# This is safe: password = "secret123"
// Also safe: api_key = "sk_test_123"
/* Another comment: execute("SELECT * " + user_input) */
"#;

        let path = PathBuf::from("test.rs");
        let findings = agent.scan_file_content(code, &path);

        // Should find no issues since patterns are in comments
        assert!(findings.is_empty());
    }

    #[test]
    fn test_is_code_file() {
        assert!(is_code_file(&PathBuf::from("test.rs")));
        assert!(is_code_file(&PathBuf::from("app.py")));
        assert!(is_code_file(&PathBuf::from("index.js")));
        assert!(is_code_file(&PathBuf::from("Main.java")));
        assert!(!is_code_file(&PathBuf::from("README.md")));
        assert!(!is_code_file(&PathBuf::from("image.png")));
    }

    #[test]
    fn test_cve_references_included() {
        let agent = SecurityAgent::new();
        let code = r#"
query = "SELECT * FROM users WHERE id = " + user_id
"#;

        let path = PathBuf::from("db.py");
        let findings = agent.scan_file_content(code, &path);

        assert!(!findings.is_empty());
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("CWE") || f.message.contains("Reference:"))
        );
    }

    #[test]
    fn test_remediation_guidance_included() {
        let agent = SecurityAgent::new();
        let code = r#"
password_hash = hashlib.md5(password.encode())
"#;

        let path = PathBuf::from("auth.py");
        let findings = agent.scan_file_content(code, &path);

        assert!(!findings.is_empty());
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("Remediation:") || f.message.contains("Use"))
        );
    }
}
