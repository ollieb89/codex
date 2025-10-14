---
name: security
description: Security vulnerability analysis with AI agent
category: agents
agent: true
agent_id: security-agent
activation_hints:
  - security
  - vulnerability
  - exploit
  - unsafe
  - audit
  - cve
permissions:
  read_files: true
  write_files: false
  execute_shell: false
args:
  - name: target
    type: string
    required: true
    description: Code to analyze (file, directory, or component)
  - name: depth
    type: string
    required: false
    description: Analysis depth (quick, standard, deep)
    default: "standard"
---

# Security Analysis Agent

Perform comprehensive security vulnerability analysis on your codebase.

## What You'll Get

The security agent will analyze:

- **Injection Vulnerabilities**: SQL, command, code injection risks
- **Authentication/Authorization**: Access control weaknesses
- **Data Validation**: Input validation gaps, sanitization issues
- **Cryptography**: Weak algorithms, insecure key management
- **Dependencies**: Known CVEs in dependencies
- **Memory Safety**: Buffer overflows, use-after-free (for unsafe code)
- **Information Disclosure**: Sensitive data leaks, logging issues
- **Configuration**: Insecure defaults, hardcoded secrets

## Analysis Depth

Choose your analysis depth:
- `quick` - Fast scan for common vulnerabilities
- `standard` - Comprehensive analysis of major security categories (default)
- `deep` - Exhaustive analysis including advanced attack vectors

## Severity Levels

Issues are categorized by severity:
- **Critical**: Immediate exploitation risk, requires urgent fix
- **High**: Significant security impact, fix soon
- **Medium**: Moderate risk, should be addressed
- **Low**: Minor issues, consider fixing
- **Info**: Security hardening opportunities

## Usage Examples

```
/security src/auth/
/security src/api/handlers.rs depth=deep
/security . depth=quick
```

## Template

Perform security analysis on: {{target}}

{{#if depth}}
Analysis depth: {{depth}}
{{/if}}

Please:
1. Scan for common vulnerability patterns (OWASP Top 10)
2. Analyze authentication and authorization flows
3. Review input validation and sanitization
4. Check for cryptographic weaknesses
5. Identify information disclosure risks
6. Review dependency security

For each finding:
- Severity level (Critical/High/Medium/Low/Info)
- Vulnerability description and location
- Potential impact and exploitation scenario
- Specific remediation recommendations
- Code examples for fixes

Prioritize findings by severity and provide actionable next steps.
