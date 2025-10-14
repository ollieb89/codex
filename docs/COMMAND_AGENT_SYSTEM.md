# Command Agent System - User Guide

**Version**: 1.0
**Date**: October 10, 2025
**Epic**: 3.1 - Agent-Backed Commands

---

## Table of Contents

1. [Overview](#overview)
2. [What Are Agent Commands?](#what-are-agent-commands)
3. [Using Agent Commands](#using-agent-commands)
4. [Visual Indicators](#visual-indicators)
5. [How It Works](#how-it-works)
6. [Available Agent Commands](#available-agent-commands)
7. [Benefits](#benefits)
8. [For Developers](#for-developers)
9. [Troubleshooting](#troubleshooting)

---

## Overview

The **Command Agent System** enhances Codex CLI with intelligent, context-aware command execution powered by specialized AI agents. Instead of simple template expansion, agent-backed commands analyze your code, understand context, and provide rich, actionable results.

### Key Features

- **🤖 Intelligent Execution**: Commands backed by specialized AI agents
- **🎯 Context-Aware**: Agents analyze your code and project structure
- **📊 Rich Output**: Formatted results with actionable insights
- **⚡ Seamless Integration**: Works transparently with the command palette

---

## What Are Agent Commands?

Agent commands are slash commands that leverage AI agents to perform specialized tasks like code review, refactoring analysis, and security audits. Unlike regular template-based commands, agent commands:

1. **Analyze Context**: Understand your code structure and intent
2. **Make Decisions**: Apply domain expertise to your specific situation
3. **Provide Insights**: Deliver structured, actionable recommendations

### Regular Command vs Agent Command

**Regular Command** (`/explain`):
```
Template: "Explain the following code: {{code}}"
Output: Simple text explanation
```

**Agent Command** (`/review`):
```
Agent: Code Review Agent
Process:
  1. Analyzes code structure
  2. Identifies patterns and anti-patterns
  3. Checks best practices
  4. Generates categorized findings
Output: Structured review with severity levels, categories, and line numbers
```

---

## Using Agent Commands

### Opening the Command Palette

Press **Ctrl+K** (or **Cmd+K** on macOS) to open the command palette:

```
> _

   explain           Explain code in simple terms
🤖 review            Code review assistant
   test              Generate comprehensive tests
🤖 refactor          Improve code structure

4 of 4 commands
```

### Identifying Agent Commands

Agent-backed commands are marked with the **🤖 robot icon**, making them easy to identify in the command palette.

### Executing an Agent Command

1. **Open Command Palette**: Press `Ctrl+K`
2. **Filter Commands** (optional): Type keywords to narrow results
3. **Select Agent Command**: Use arrow keys or mouse to select
4. **Press Enter**: Execute the command

**Example**:
```
> rev_

🤖 review            Code review assistant

1 of 4 commands
```

---

## Visual Indicators

### Command Palette Display

| Indicator | Meaning |
|-----------|---------|
| `   explain` | Regular template-based command |
| `🤖 review` | Agent-backed intelligent command |

### Filtering

When you type in the command palette, both regular and agent commands are filtered. The 🤖 icon persists to help you identify agent commands:

```
> test_

   test              Generate comprehensive tests
🤖 test-coverage     AI-powered coverage analysis

2 of 10 commands
```

---

## How It Works

### Architecture Flow

```
User Input (Ctrl+K)
    ↓
Command Palette
    ↓
Command Selection (with 🤖 indicator)
    ↓
Command Execution Router
    ↓
    ├─→ Regular Command → Template Expansion
    └─→ Agent Command → Agent Execution
            ↓
        Agent Analysis
            ↓
        Structured Results
            ↓
        Formatted Output (Markdown/JSON/Plain)
```

### Execution Pipeline

1. **Command Discovery**: Registry loads commands with metadata
2. **Routing Decision**: Executor checks `metadata.agent` flag
3. **Agent Selection**: Router picks appropriate agent based on context
4. **Agent Execution**: Agent analyzes code and generates results
5. **Result Formatting**: Formatter converts results to user-friendly output

### Metadata-Driven Routing

Each command has metadata that determines its execution path:

```yaml
---
name: review
description: Code review assistant
category: analysis
agent: true                    # ← Marks command as agent-backed
agent_id: code-review-agent    # ← Specifies which agent to use
---
```

---

## Available Agent Commands

### 🤖 review - Code Review Agent

**Purpose**: Comprehensive code analysis with best practice recommendations

**Usage**:
```
/review src/main.rs
```

**Output**:
```markdown
# Code Review Results

## Performance Issues (2)
⚠️ **Warning** - Line 42, src/main.rs
Consider using a more efficient algorithm
Category: Performance

## Style Issues (1)
ℹ️ **Info** - Line 15, src/main.rs
Variable naming could be more descriptive
Category: Style

## Summary
- 0 errors
- 2 warnings
- 1 info
```

### 🤖 refactor - Refactoring Assistant

**Purpose**: Identifies refactoring opportunities and suggests improvements

**Usage**:
```
/refactor src/utils.rs
```

**Output**:
```markdown
# Refactoring Suggestions

## Suggestions (3)
1. **Extract method complexity**
   Reduce cognitive complexity by extracting helper methods

2. **Apply dependency injection**
   Improve testability and maintainability

3. **Consolidate error handling**
   Use consistent error types across module
```

---

## Benefits

### For Users

1. **🎯 Better Results**: Context-aware analysis vs simple templates
2. **⚡ Faster Workflow**: No need to manually specify context
3. **📊 Actionable Insights**: Structured output with priorities
4. **🔍 Deep Analysis**: Agents understand patterns, not just syntax

### For Code Quality

1. **Consistent Reviews**: Same standards applied across codebase
2. **Best Practices**: Agents trained on industry standards
3. **Early Detection**: Find issues before they become bugs
4. **Learning Tool**: Understand why changes are recommended

---

## For Developers

### Creating Agent Commands

Create a new command file in `.claude/commands/`:

```markdown
---
name: security-audit
description: Security vulnerability analysis
category: analysis
agent: true
agent_id: security-agent
activation_hints:
  - security
  - vulnerability
  - audit
---

Analyze the code for security vulnerabilities including:
- SQL injection risks
- XSS vulnerabilities
- Authentication issues
- Data exposure
```

### Command Metadata Fields

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Command identifier (used in `/command`) |
| `description` | string | Short description for command palette |
| `category` | string | Category for organization |
| `agent` | boolean | `true` for agent-backed commands |
| `agent_id` | string | Specific agent to use (e.g., "code-review-agent") |
| `activation_hints` | array | Keywords for context matching |

### Agent Activation

Agents use activation hints to match user context:

```rust
fn can_handle(&self, context: &TaskContext) -> ActivationScore {
    if context.user_intent.contains("security") {
        ActivationScore::new(0.9)  // High confidence
    } else {
        ActivationScore::new(0.1)  // Low confidence
    }
}
```

### Testing Agent Commands

Use the E2E test suite to verify agent command behavior:

```rust
#[tokio::test]
async fn test_agent_command_execution() -> Result<()> {
    let command = create_agent_command("review");
    assert!(command.metadata().agent);

    let result = execute_agent_command(command).await?;
    assert!(matches!(result, AgentResult::CodeReview { .. }));

    Ok(())
}
```

---

## Troubleshooting

### Command Not Found

**Issue**: Agent command doesn't appear in palette

**Solutions**:
1. Check command file is in `.claude/commands/`
2. Verify YAML frontmatter is valid
3. Ensure `agent: true` is set in metadata
4. Restart Codex CLI to reload commands

### Agent Not Executing

**Issue**: Command runs but uses template instead of agent

**Solutions**:
1. Verify `agent_id` matches registered agent
2. Check agent activation hints match context
3. Review agent permissions configuration
4. Check Codex logs for routing errors

### Formatting Issues

**Issue**: Agent results not formatted correctly

**Solutions**:
1. Verify agent returns proper `AgentResult` enum
2. Check `OutputFormat` setting (Markdown/JSON/PlainText)
3. Ensure formatter handles all result variants
4. Review result structure in agent implementation

### Performance Issues

**Issue**: Agent commands feel slow

**Expected Latency**:
- Command dispatch: <10μs
- Agent selection: <50ms
- Agent execution: Variable (depends on task)
- Result formatting: <10ms

**Solutions**:
1. Check agent activation scoring efficiency
2. Optimize context gathering in agents
3. Use caching for repeated analyses
4. Profile agent execution with benchmarks

---

## Technical Details

### Performance Metrics

From E2E tests:

| Operation | Latency | Target |
|-----------|---------|--------|
| Metadata access | <10μs | <100μs |
| Agent detection | <1μs | <10μs |
| Command routing | <100μs | <1ms |
| Result formatting | <10ms | <50ms |

### Output Formats

Agent results support three output formats:

1. **Markdown**: Rich formatting with headers, lists, and code blocks
2. **JSON**: Structured data for programmatic consumption
3. **PlainText**: Simple text output for minimal terminals

### Result Types

```rust
pub enum AgentResult {
    CodeReview { findings: Vec<CodeReviewFinding> },
    Analysis { summary: String, details: HashMap<String, String> },
    Suggestions { items: Vec<Suggestion> },
}
```

Each result type is formatted appropriately for the chosen output format.

---

## FAQ

### Q: How do I know if a command is agent-backed?

**A**: Look for the 🤖 robot icon in the command palette. Agent commands always show this icon.

### Q: Can I use agent commands in scripts?

**A**: Yes! Agent commands work in both interactive TUI mode and exec mode:
```bash
codex exec "/review src/main.rs" --output json
```

### Q: Do agent commands require internet?

**A**: Agent commands use the same model provider as regular Codex commands. If you're configured for local models, agents work offline.

### Q: How do I create custom agents?

**A**: Custom agent development requires Rust development. See the [Agent Development Guide](./AGENT_DEVELOPMENT.md) for details.

### Q: Are agent commands slower than regular commands?

**A**: Agent commands involve more processing (context analysis, decision-making), so they take longer than simple template expansion. However, command dispatch and routing overhead is <100μs.

### Q: Can multiple agents handle the same command?

**A**: Commands are typically bound to a specific agent via `agent_id`. However, the router can select the best agent based on activation scores if no specific agent is configured.

---

## Additional Resources

- **Epic 3.1 Documentation**: See [EPIC_3.1_COMPLETION.md](./EPIC_3.1_COMPLETION.md)
- **Day 25 Completion Report**: See [DAY_25_COMPLETION.md](./DAY_25_COMPLETION.md)
- **Agent Development**: See [AGENT_DEVELOPMENT.md](./AGENT_DEVELOPMENT.md)
- **Command System**: See [COMMAND_SYSTEM.md](./COMMAND_SYSTEM.md)

---

## Changelog

### Version 1.0 (2025-10-10)

- Initial release
- Command palette integration with 🤖 visual indicators
- E2E testing suite with 10 integration tests
- Performance benchmarks (<100μs routing overhead)
- Support for Markdown, JSON, and PlainText output formats

---

## Feedback & Support

For issues, feature requests, or questions:

1. **GitHub Issues**: [openai/codex/issues](https://github.com/openai/codex/issues)
2. **Documentation**: [Codex Docs](https://docs.openai.com/codex)
3. **Community**: [Codex Discord](#) (link TBD)

---

**Note**: This document covers the MVP agent command system delivered in Epic 3.1. Additional features (agent chaining, multi-agent collaboration, custom agent frameworks) are planned for future epics.
