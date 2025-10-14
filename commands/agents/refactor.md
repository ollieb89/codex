---
name: refactor
description: Intelligent code refactoring with AI agent
category: agents
agent: true
agent_id: refactor-agent
activation_hints:
  - refactor
  - improve
  - cleanup
  - simplify
  - restructure
permissions:
  read_files: true
  write_files: true
  execute_shell: false
args:
  - name: target
    type: string
    required: true
    description: Code to refactor (file, directory, or function)
  - name: goal
    type: string
    required: false
    description: Refactoring goal (readability, performance, testability, modularity)
    default: "readability"
---

# Code Refactoring Agent

Intelligently refactor code to improve quality, maintainability, and adherence to best practices.

## What You'll Get

The refactoring agent will:

- **Analyze Current Code**: Understand structure, patterns, and issues
- **Propose Improvements**: Suggest specific refactoring strategies
- **Apply Changes**: Safely refactor while preserving functionality
- **Validate**: Ensure tests pass and behavior is unchanged
- **Document**: Explain what was changed and why

## Refactoring Goals

Specify your primary refactoring goal:
- `readability` - Improve code clarity and comprehension (default)
- `performance` - Optimize for speed and efficiency
- `testability` - Make code easier to test
- `modularity` - Improve separation of concerns and reusability

## Safety Guarantees

The agent will:
- Preserve existing functionality
- Maintain backward compatibility where possible
- Run tests to validate changes
- Provide rollback instructions if needed

## Usage Examples

```
/refactor src/parser.rs
/refactor src/handlers/ goal=modularity
/refactor src/utils.rs goal=performance
```

## Template

Refactor the code at: {{target}}

{{#if goal}}
Primary goal: {{goal}}
{{/if}}

Please:
1. Analyze the current code structure and identify improvement opportunities
2. Propose specific refactoring strategies with rationale
3. Apply the refactoring changes safely
4. Validate that tests pass and functionality is preserved
5. Summarize the changes made and their benefits

Focus on incremental, safe improvements that maintain correctness while achieving the stated goal.
