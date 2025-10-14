---
name: review
description: Comprehensive code review with AI agent
category: agents
agent: true
agent_id: code-review
activation_hints:
  - review
  - quality
  - bugs
  - issues
  - analysis
permissions:
  read_files: true
  write_files: false
  execute_shell: false
args:
  - name: files
    type: string
    required: true
    description: Files or directories to review
  - name: focus
    type: string
    required: false
    description: Review focus area (security, performance, maintainability)
    default: "general"
---

# Code Review Agent

Perform comprehensive code review on the specified files or directories.

## What You'll Get

The code review agent will analyze:

- **Code Quality**: Readability, maintainability, adherence to best practices
- **Potential Bugs**: Logic errors, edge cases, error handling gaps
- **Security Issues**: Vulnerabilities, unsafe patterns, injection risks
- **Performance**: Inefficiencies, optimization opportunities
- **Architecture**: Design patterns, modularity, separation of concerns
- **Testing**: Test coverage gaps, testability issues

## Review Focus

You can optionally specify a focus area:
- `security` - Deep security analysis
- `performance` - Performance optimization opportunities
- `maintainability` - Code structure and maintainability
- `general` - Comprehensive review of all aspects (default)

## Usage Examples

```
/review src/main.rs
/review src/handlers/ focus=security
/review . focus=performance
```

## Template

Analyze the code at: {{files}}

{{#if focus}}
Focus on: {{focus}}
{{/if}}

Provide a comprehensive review covering:
1. Overall code quality assessment
2. Identified issues (critical, high, medium, low priority)
3. Specific recommendations for improvement
4. Positive aspects worth maintaining

Format the output as structured markdown with clear sections and actionable recommendations.
