# Command Implementation Guide for Codex

This document explains how to implement new commands and prompts for the Codex CLI, using the modular structure of the codebase. It covers the main components, best practices, and step-by-step instructions for extending command functionality.

---

## 1. Overview of Key Modules

- **exec_command**: Manages command parameters, sessions, and API responses. Use this to define new command types and handle session lifecycles.
- **executor**: Contains logic for running commands, including backend selection, caching, sandboxing, and orchestration. Implement command execution here.
- **tasks**: Defines task flows (compact, regular, review) and wraps command execution in a structured process.
- **prompts**: Stores prompt templates for deduplication, labeling, and other interactive CLI features.

---

## 2. Steps to Implement a New Command

### Step 1: Define Command Parameters
- Add parameter structs and logic in `exec_command/exec_command_params.rs`.
- Ensure session management is handled in `exec_command_session.rs` if needed.

### Step 2: Implement Command Logic
- Add execution logic in `executor/runner.rs`.
- Use backends from `executor/backends.rs` for different environments.
- Leverage caching and sandboxing as appropriate.

### Step 3: Integrate with Task Flows
- Wrap your command in a task type (`compact`, `regular`, or `review`) in the `tasks` module.
- Register the new task in `tasks/mod.rs`.

### Step 4: Add or Update Prompts
- Create or modify prompt templates in `.github/prompts/`.
- Reference these prompts in your CLI or task logic for interactive features.

### Step 5: Connect to CLI
- Register your command in the CLI entry point (e.g., `codex-cli/bin/codex.js`).
- Ensure parameters, execution, and prompts are wired together for user interaction.

---

## 3. Best Practices

- **Separation of Concerns**: Keep parameter/session management, execution logic, and task orchestration in their respective modules.
- **Reusability**: Use modular backends and runners to support multiple environments and command types.
- **Prompt Consistency**: Store prompt templates centrally and reference them programmatically.
- **Extensibility**: Design new commands and prompts to be easily discoverable and maintainable.

---

## 4. Example: Adding a New Command

1. Define a struct for your command parameters in `exec_command_params.rs`.
2. Implement the command logic in `executor/runner.rs`, using the appropriate backend.
3. Add a new task type or extend an existing one in `tasks/`.
4. Create a prompt template in `.github/prompts/` if needed.
5. Register the command in the CLI entry point.

---

## 5. Reference Documentation

For more details, see:
- `docs/exec.md` for execution details
- `docs/prompts.md` for prompt usage
- Source files in `codex-rs/core/src/exec_command/`, `executor/`, and `tasks/`

---

**Summary:**
Implementing new commands and prompts in Codex is straightforward due to its modular design. Follow the steps above to ensure your additions are robust, maintainable, and consistent with project standards.
