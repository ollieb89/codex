# Codex CLI User Guide

Welcome to Codex, OpenAI's powerful AI-powered coding assistant that runs locally on your machine. This guide will help you get started with Codex and make the most of its features.

## Table of Contents

1. [Installation](#installation)
2. [Getting Started](#getting-started)
3. [Core Features](#core-features)
4. [Interactive Mode (TUI)](#interactive-mode-tui)
5. [Command Palette](#command-palette)
6. [Non-Interactive Mode](#non-interactive-mode)
7. [Configuration](#configuration)
8. [Advanced Features](#advanced-features)
9. [Best Practices](#best-practices)
10. [Troubleshooting](#troubleshooting)

## Installation

### Using npm (Recommended)
```bash
npm install -g @openai/codex
```

### Using Homebrew (macOS)
```bash
brew install codex
```

### Direct Download
Download platform-specific releases from [GitHub Releases](https://github.com/openai/codex/releases).

### Verify Installation
```bash
codex --version
```

## Getting Started

### First Launch
Simply run:
```bash
codex
```

This launches Codex in interactive mode (TUI - Terminal User Interface).

### Initial Setup

1. **Authentication**: On first run, you'll need to authenticate with OpenAI:
   ```bash
   codex login
   ```

   Choose one of the authentication methods:
   - API Key (if you have one)
   - Device code flow (recommended for most users)
   - ChatGPT integration

2. **Configuration**: Codex creates a configuration directory at `~/.codex/` with:
   - `config.toml` - Your configuration settings
   - `logs/` - Application logs
   - Session data and cached information

## Core Features

### Interactive Mode (TUI)

The default mode when you run `codex`. This provides a full-screen terminal interface with:

#### Main Interface Components
- **Chat View**: Your conversation with the AI assistant
- **Input Area**: Where you type your requests
- **Status Bar**: Shows current model, session info, and other details
- **Command Palette**: Access with `Ctrl+K` for quick commands

#### Key Shortcuts
- `Ctrl+K` - Open command palette
- `Ctrl+C` - Cancel current operation
- `Ctrl+D` - Exit Codex
- `Ctrl+L` - Clear screen
- `Tab` - Auto-complete file paths
- `Up/Down` - Navigate history
- `Ctrl+R` - Search command history

### Command Palette

The command palette (activated with `Ctrl+K`) provides quick access to powerful commands:

#### Available Commands

##### Code Analysis Commands
- **`explain`** - Explain code in simple terms
- **`review`** 🤖 - AI-powered code review assistant
- **`security`** 🤖 - Security vulnerability scanner
- **`refactor`** - Suggest code improvements
- **`optimize`** - Performance optimization suggestions

##### Code Generation Commands
- **`test`** - Generate comprehensive tests
- **`implement`** - Implement new features
- **`fix`** - Fix bugs and errors
- **`document`** - Generate documentation

##### Utility Commands
- **`diff`** - Show changes made
- **`apply`** - Apply suggested changes
- **`undo`** - Undo last change
- **`settings`** - Open settings

*Note: Commands marked with 🤖 are powered by specialized AI agents*

#### Using the Command Palette
1. Press `Ctrl+K` to open
2. Start typing to filter commands
3. Use `Up/Down` arrows to navigate
4. Press `Enter` to execute
5. Press `Esc` to close

### File Operations

#### Referencing Files
Use `@` to reference files:
```
@main.py - explain this code
@src/ - review all files in src directory
@**/*.ts - find TypeScript files
```

#### File Search Patterns
- `@filename` - Exact file name
- `@*.extension` - All files with extension
- `@directory/` - All files in directory
- `@**/pattern` - Recursive search

## Non-Interactive Mode

### Exec Command
Run Codex non-interactively for automation:

```bash
# Execute a single task
codex exec "refactor the authentication module for better security"

# Pipe input
echo "explain this regex: ^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$" | codex exec

# With specific model
codex exec --model gpt-4o "generate unit tests for user.py"
```

### Apply Command
Apply the latest changes from Codex:
```bash
codex apply
# Or shorthand
codex a
```

## Configuration

### Configuration File
Located at `~/.codex/config.toml`:

```toml
# Model selection
model = "gpt-5-codex"  # or "o3", "gpt-4o", etc.

# Theme and appearance
theme = "dark"  # or "light"

# Editor integration
editor = "vim"  # or "emacs", "nano"

# Shell environment
[shell_environment_policy]
include = ["PATH", "HOME", "USER", "LANG"]

# API configuration
[model_providers.openai]
name = "OpenAI"
base_url = "https://api.openai.com/v1"
env_key = "OPENAI_API_KEY"
```

### Command-Line Overrides
Override configuration per-command:

```bash
# Use specific model
codex --model o3

# Override any config value
codex --config model="gpt-4o"
codex --config theme="light"

# Multiple overrides
codex -c model="o3" -c max_tokens=4000
```

## Advanced Features

### MCP (Model Context Protocol) Integration

#### As MCP Client
Connect to MCP servers on startup by configuring in `config.toml`:

```toml
[[mcp_servers]]
name = "my-server"
command = ["node", "/path/to/server.js"]
```

#### As MCP Server (Experimental)
Run Codex as an MCP server for other tools:

```bash
# Start MCP server
codex mcp-server

# Test with inspector
npx @modelcontextprotocol/inspector codex mcp-server
```

### Session Management

#### Resume Previous Sessions
```bash
# Show session picker
codex resume

# Resume last session
codex resume --last
```

#### Session Files
Sessions are stored in `~/.codex/sessions/` and can be:
- Resumed at any time
- Shared with team members
- Used for debugging

### Notifications

Configure desktop notifications in `config.toml`:

```toml
[notify]
command = ["/usr/local/bin/terminal-notifier", "-title", "Codex", "-message"]
```

### Sandbox Mode

Run commands in a secure sandbox:

```bash
# Debug with sandbox
codex sandbox --debug

# Run specific command in sandbox
codex sandbox -- ls -la
```

## Best Practices

### 1. Clear and Specific Prompts
**Good**: "Add input validation to the login form in auth.py, checking for email format and password strength"

**Better**: "Add input validation to the login() function in @auth.py that:
- Validates email format using regex
- Ensures password is at least 8 characters with mixed case and numbers
- Returns specific error messages for each validation failure"

### 2. Use File References
Always use `@` to reference specific files:
```
Fix the bug in @src/api/users.py where duplicate emails cause a crash
```

### 3. Incremental Changes
Break large tasks into smaller steps:
1. "First, show me the current structure of @database.py"
2. "Add connection pooling to the Database class"
3. "Now add retry logic for failed connections"

### 4. Review Generated Code
Always review AI-generated code before applying:
```bash
# Generate changes
codex exec "add error handling to API endpoints"

# Review diff
git diff

# Apply if satisfied
codex apply
```

### 5. Use Command Palette for Common Tasks
Learn the keyboard shortcuts:
- `Ctrl+K` then type "review" for quick code review
- `Ctrl+K` then type "test" to generate tests
- `Ctrl+K` then type "security" for vulnerability scan

### 6. Configure for Your Workflow
Customize `~/.codex/config.toml` for your needs:
- Set your preferred model
- Configure MCP servers
- Set up notifications
- Customize key bindings

## Troubleshooting

### Common Issues

#### Authentication Errors
```bash
# Re-authenticate
codex logout
codex login
```

#### Model Not Available
```bash
# Check available models
codex --config model_provider="openai" --model gpt-4o
```

#### Slow Performance
- Check network connection
- Try a different model
- Clear cache: `rm -rf ~/.codex/cache`

#### Session Issues
```bash
# Clear corrupted sessions
rm -rf ~/.codex/sessions/*

# Start fresh
codex --new-session
```

### Debug Mode
Enable detailed logging:
```bash
RUST_LOG=debug codex
```

### Getting Help

1. **Built-in Help**:
   ```bash
   codex --help
   codex exec --help
   ```

2. **Documentation**:
   - GitHub: https://github.com/openai/codex
   - Config Guide: See `docs/config.md`

3. **Support**:
   - File issues on GitHub
   - Check existing issues for solutions

## Tips and Tricks

### Power User Features

1. **Batch Operations**:
   ```bash
   # Process multiple files
   codex exec "add type hints to all Python files in @src/"
   ```

2. **Custom Commands**:
   Create aliases in your shell:
   ```bash
   alias cr="codex exec 'review this code for issues'"
   alias ct="codex exec 'generate comprehensive tests'"
   ```

3. **Integration with Git**:
   ```bash
   # Review changes before commit
   git diff | codex exec "review these changes"
   ```

4. **Quick Fixes**:
   ```bash
   # Pipe errors directly to Codex
   python script.py 2>&1 | codex exec "fix this error"
   ```

5. **Learning Mode**:
   Use Codex to learn new concepts:
   ```
   Explain how async/await works in Python with examples
   ```

### Productivity Shortcuts

- Use Tab completion for file paths
- Use history (Up arrow) to repeat commands
- Create templates for common requests
- Use `--model` flag for task-specific models
- Combine with other tools via pipes

## Conclusion

Codex is a powerful AI coding assistant that adapts to your workflow. Start with simple commands, explore the command palette, and gradually incorporate advanced features as you become comfortable. The key is to be specific in your requests and always review generated code before applying changes.

Happy coding with Codex! 🚀