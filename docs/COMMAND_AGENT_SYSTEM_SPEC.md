# Command & Agent System Implementation Specification

## Executive Summary

This specification outlines a comprehensive implementation plan for adding Claude Code-style commands, prompts, and agents to Codex. The design follows a **hybrid three-tier architecture** that balances user extensibility, built-in power, and intelligent automation while maintaining backward compatibility with Codex's existing infrastructure.

**Key Design Decisions:**
- ✅ **Hybrid Model**: User-extensible commands + Built-in commands + Agent system
- ✅ **File-Based Config**: Markdown command definitions for simplicity, TOML for agents
- ✅ **Agile Phased Approach**: 3 phases over 9-12 weeks
- ✅ **Safety-First**: Full integration with execpolicy sandbox system
- ✅ **Backward Compatible**: Extends existing exec_command and executor modules

---

## Architecture Overview

### Three-Tier System Design

```
┌─────────────────────────────────────────────────────────────┐
│                    USER INTERFACE LAYER                      │
│  TUI: Command palette, Agent cards, Multi-agent dashboard   │
│  Exec: Streaming agent output, Command invocation           │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   COMMAND SYSTEM LAYER                       │
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ User-Defined │  │   Built-In   │  │    Agent     │      │
│  │  Commands    │  │  Commands    │  │  Commands    │      │
│  │  (Markdown)  │  │    (Rust)    │  │   (Hybrid)   │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│         ↓                  ↓                  ↓              │
│  ┌─────────────────────────────────────────────────┐        │
│  │         Command Registry & Router                │        │
│  │   - Parse & validate                             │        │
│  │   - Template expansion                           │        │
│  │   - Permission checking                          │        │
│  └─────────────────────────────────────────────────┘        │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                     AGENT SYSTEM LAYER                       │
│                                                               │
│  ┌─────────────────────────────────────────────────┐        │
│  │              Agent Router                        │        │
│  │   - Context analysis                             │        │
│  │   - Agent selection                              │        │
│  │   - Activation scoring                           │        │
│  └─────────────────────────────────────────────────┘        │
│         ↓                  ↓                  ↓              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │Single Agent  │  │  Multi-Agent │  │   Agent      │      │
│  │  Executor    │  │ Orchestrator │  │  Toolkit     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   EXECUTION LAYER                            │
│  (Existing Codex Infrastructure)                             │
│                                                               │
│  exec_command → executor → execpolicy → backend-client      │
└─────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

**Tier 1: User-Extensible Commands**
- Location: `~/.codex/commands/*.md`
- Format: Markdown with YAML frontmatter
- Purpose: Simple prompt templates with variable interpolation
- Capabilities: Prompt expansion, context injection, basic logic
- Security: Sandboxed execution, no elevated privileges

**Tier 2: Built-In Commands**
- Location: `codex-rs/core/src/commands/builtin/`
- Format: Rust structs implementing `CommandHandler` trait
- Purpose: Complex operations requiring full Codex API access
- Capabilities: Direct system access, MCP integration, advanced logic
- Security: Trusted code path with internal API access

**Tier 3: Agent Commands**
- Location: `~/.codex/agents/*.toml` + built-in implementations
- Format: TOML config + Rust agent implementations
- Purpose: Intelligent task execution with specialized expertise
- Capabilities: Multi-turn dialogues, tool orchestration, parallel execution
- Security: Agent-specific permission model, tool access control

---

## Phase 1: Command System Foundation (2-3 weeks)

### Objectives
- Implement user-extensible slash command system
- Enable prompt template expansion with variables
- Integrate with existing exec_command flow
- Add basic TUI visualization

### Implementation Tasks

#### 1.1 Command File Format (Week 1)

**Markdown Command Definition:**
```markdown
---
name: review-security
description: Security-focused code review
category: analysis
permissions:
  - read_files
  - execute_tools
args:
  - name: path
    type: string
    required: false
    description: Path to review (defaults to current directory)
  - name: depth
    type: enum
    values: [shallow, normal, deep]
    default: normal
---

Please perform a {{depth}} security review of {{path | default: "the current codebase"}}.

Focus on:
- Authentication and authorization vulnerabilities
- Input validation and sanitization
- Sensitive data exposure
- Known security patterns and anti-patterns

{{#if context.git_diff}}
Pay special attention to changes in:
{{context.git_diff}}
{{/if}}

{{#if args.depth == "deep"}}
Additionally analyze:
- Dependency vulnerabilities
- Cryptographic implementations
- Access control mechanisms
{{/if}}
```

**Features:**
- YAML frontmatter for metadata
- Handlebars-style variable interpolation
- Conditional logic (`{{#if}}`, `{{#each}}`)
- Context variables (`context.git_diff`, `context.files`, etc.)
- Argument validation and type checking

#### 1.2 Command Registry Implementation (Week 1-2)

**Core Module Structure:**
```
codex-rs/core/src/commands/
├── mod.rs                    # Module exports
├── registry.rs               # CommandRegistry for discovery
├── parser.rs                 # Markdown parser & validator
├── expander.rs               # Template expansion engine
├── permissions.rs            # Permission model
├── context.rs                # Context builder
├── builtin/                  # Built-in commands
│   ├── mod.rs
│   └── review.rs             # Example: /review command
└── user/                     # User command loading
    └── loader.rs
```

**Key Traits:**
```rust
// Command definition trait
pub trait Command: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn category(&self) -> CommandCategory;
    fn permissions(&self) -> &CommandPermissions;
    fn args_schema(&self) -> &ArgSchema;
    fn expand(&self, context: &CommandContext, args: &CommandArgs) -> Result<String>;
}

// Command permissions
pub struct CommandPermissions {
    pub read_files: bool,
    pub write_files: bool,
    pub execute_shell: bool,
    pub network_access: bool,
    pub allowed_tools: Vec<String>,
}

// Command context for template expansion
pub struct CommandContext {
    pub git_diff: Option<String>,
    pub current_files: Vec<PathBuf>,
    pub workspace_root: PathBuf,
    pub user_vars: HashMap<String, String>,
}
```

#### 1.3 Integration with exec_command (Week 2)

**Modify exec_command flow:**

```rust
// codex-rs/core/src/exec_command/exec_command_params.rs
pub enum ExecCommandInput {
    UserMessage(String),
    SlashCommand {
        command: String,
        args: Vec<String>,
    },
    AgentInvocation {
        agent: String,
        task: String,
    },
}

impl ExecCommandParams {
    pub async fn from_input(input: &str) -> Result<Self> {
        if input.starts_with('/') {
            let (cmd, args) = parse_slash_command(input)?;

            // Load command from registry
            let registry = CommandRegistry::global();
            let command = registry.get(&cmd)?;

            // Build context
            let context = CommandContext::build().await?;

            // Expand template
            let expanded = command.expand(&context, &args)?;

            // Create params with expanded prompt
            Ok(Self {
                message: expanded,
                command_metadata: Some(CommandMetadata { name: cmd, ... }),
                ...
            })
        } else {
            // Standard user message
            Ok(Self::from_message(input))
        }
    }
}
```

#### 1.4 TUI Command Palette (Week 2-3)

**New TUI Component:**
```rust
// codex-rs/tui/src/commands_view.rs
pub struct CommandsView {
    commands: Vec<CommandInfo>,
    filter: String,
    selected: usize,
}

impl CommandsView {
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // Render filterable list of available commands
        // Show: name, description, category, keybinding
        // Support fuzzy search
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> Option<String> {
        // Return selected command for execution
    }
}
```

**Integration:**
- Add `Ctrl+K` keybinding to open command palette
- Show all registered commands (user + built-in)
- Display command args and description on hover
- Execute on Enter with argument prompt

### Deliverables
- ✅ Command file format specification
- ✅ Command registry with hot-reloading
- ✅ Template expansion engine
- ✅ Integration with exec_command
- ✅ TUI command palette
- ✅ 5 example built-in commands
- ✅ Documentation for creating custom commands

---

## Phase 2: Agent System Core (3-4 weeks)

### Objectives
- Implement agent registry and persona system
- Build agent router with context-based activation
- Create 5 core specialized agents
- Integrate agent permissions with execpolicy
- Add agent visualization to TUI

### Implementation Tasks

#### 2.1 Agent Architecture (Week 1)

**Agent Trait Definition:**
```rust
// codex-rs/core/src/agents/mod.rs
#[async_trait]
pub trait Agent: Send + Sync {
    /// Unique agent identifier
    fn id(&self) -> AgentId;

    /// Human-readable name
    fn name(&self) -> &str;

    /// Agent description and expertise
    fn description(&self) -> &str;

    /// Analyze context and score activation relevance (0.0-1.0)
    fn can_handle(&self, context: &TaskContext) -> ActivationScore;

    /// Execute task with provided toolkit
    async fn execute(&self, task: Task, toolkit: &AgentToolkit) -> AgentResult;

    /// Agent permissions for safety checks
    fn permissions(&self) -> &AgentPermissions;

    /// System prompt for this agent's persona
    fn system_prompt(&self) -> &str;
}

pub struct AgentPermissions {
    pub allowed_tools: Vec<ToolId>,
    pub file_access: FileAccessPolicy,
    pub shell_execution: bool,
    pub network_access: bool,
    pub max_iterations: u32,
}

pub struct TaskContext {
    pub file_paths: Vec<PathBuf>,
    pub file_contents: Option<HashMap<PathBuf, String>>,
    pub git_context: Option<GitContext>,
    pub execution_mode: ExecutionMode,
    pub user_intent: String,
}
```

**Agent Toolkit:**
```rust
// codex-rs/core/src/agents/toolkit.rs
pub struct AgentToolkit {
    execpolicy: Arc<ExecPolicy>,
    file_ops: FileOperations,
    git_tools: GitTooling,
    mcp_client: Option<McpClient>,
}

impl AgentToolkit {
    pub async fn read_file(&self, path: &Path) -> Result<String> {
        // Validate against agent permissions
        // Execute with sandbox restrictions
    }

    pub async fn execute_command(&self, cmd: &str) -> Result<CommandOutput> {
        // Pass through execpolicy validation
        // Respect agent permission constraints
    }

    pub async fn invoke_mcp_tool(&self, tool: &str, args: Value) -> Result<Value> {
        // MCP tool invocation with permission checks
    }
}
```

#### 2.2 Core Agent Implementations (Week 2-3)

**Agent 1: Code Reviewer**
```rust
// codex-rs/core/src/agents/builtin/code_reviewer.rs
pub struct CodeReviewerAgent;

impl Agent for CodeReviewerAgent {
    fn id(&self) -> AgentId { AgentId::from("code-reviewer") }

    fn can_handle(&self, ctx: &TaskContext) -> ActivationScore {
        let mut score = 0.0;

        // Check for review keywords
        if ctx.user_intent.contains("review") || ctx.user_intent.contains("analyze") {
            score += 0.5;
        }

        // Check for code files
        if ctx.file_paths.iter().any(|p| is_code_file(p)) {
            score += 0.3;
        }

        // Check for git diff context
        if ctx.git_context.is_some() {
            score += 0.2;
        }

        ActivationScore(score)
    }

    fn system_prompt(&self) -> &str {
        "You are an expert code reviewer focused on code quality, \
         maintainability, and best practices. Analyze code for:\n\
         - Code structure and organization\n\
         - Naming conventions and clarity\n\
         - Potential bugs and edge cases\n\
         - Performance considerations\n\
         - Test coverage gaps\n\
         Provide constructive, actionable feedback."
    }

    async fn execute(&self, task: Task, toolkit: &AgentToolkit) -> AgentResult {
        // Implementation: read files, analyze patterns, generate review
        let files = task.context.file_paths;
        let mut findings = Vec::new();

        for path in files {
            let content = toolkit.read_file(&path).await?;
            let analysis = self.analyze_code(&content)?;
            findings.push(CodeReviewFinding { path, analysis });
        }

        Ok(AgentResult::CodeReview { findings })
    }
}
```

**Agent 2: Security Analyst**
```rust
pub struct SecurityAnalystAgent;

impl Agent for SecurityAnalystAgent {
    fn can_handle(&self, ctx: &TaskContext) -> ActivationScore {
        let mut score = 0.0;

        // Security keywords
        if ctx.user_intent.contains("security") || ctx.user_intent.contains("vulnerability") {
            score += 0.7;
        }

        // Check for security-sensitive files (auth, crypto, etc.)
        if ctx.file_paths.iter().any(|p| is_security_sensitive(p)) {
            score += 0.3;
        }

        ActivationScore(score)
    }

    fn system_prompt(&self) -> &str {
        "You are a security expert specialized in vulnerability analysis. \
         Focus on:\n\
         - Authentication and authorization flaws\n\
         - Input validation and sanitization\n\
         - Cryptographic implementations\n\
         - Sensitive data exposure\n\
         - Known vulnerability patterns (OWASP Top 10)\n\
         Provide severity ratings and remediation guidance."
    }

    async fn execute(&self, task: Task, toolkit: &AgentToolkit) -> AgentResult {
        // Security-specific analysis implementation
        // ...
    }
}
```

**Additional Core Agents:**
- **refactoring-expert**: Code structure improvements
- **test-engineer**: Test coverage and quality
- **performance-optimizer**: Performance analysis

#### 2.3 Agent Router (Week 3)

```rust
// codex-rs/core/src/agents/router.rs
pub struct AgentRouter {
    agents: HashMap<AgentId, Arc<dyn Agent>>,
    activation_threshold: f64,
}

impl AgentRouter {
    pub async fn select_agent(&self, context: &TaskContext) -> Option<Arc<dyn Agent>> {
        let mut scores: Vec<_> = self.agents
            .values()
            .map(|agent| (agent.clone(), agent.can_handle(context).0))
            .collect();

        scores.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

        if let Some((agent, score)) = scores.first() {
            if *score >= self.activation_threshold {
                return Some(agent.clone());
            }
        }

        None
    }

    pub async fn suggest_agents(&self, context: &TaskContext, top_k: usize) -> Vec<AgentSuggestion> {
        // Return top-k agents for user selection
    }
}
```

#### 2.4 Agent Execution Integration (Week 4)

**Modify executor to support agents:**
```rust
// codex-rs/core/src/executor/agent_runner.rs
pub struct AgentRunner {
    router: AgentRouter,
    toolkit: AgentToolkit,
}

impl AgentRunner {
    pub async fn run(&self, task: Task) -> Result<AgentResult> {
        // 1. Build context from task
        let context = TaskContext::from_task(&task)?;

        // 2. Select appropriate agent
        let agent = self.router.select_agent(&context).await
            .ok_or(AgentError::NoSuitableAgent)?;

        // 3. Validate permissions
        self.validate_permissions(agent.permissions(), &context)?;

        // 4. Execute agent
        let result = agent.execute(task, &self.toolkit).await?;

        // 5. Post-process and return
        Ok(result)
    }
}
```

**Add to tasks module:**
```rust
// codex-rs/core/src/tasks/agent_task.rs
pub struct AgentTask {
    pub agent_id: Option<AgentId>,  // None = auto-select
    pub task: Task,
    pub permissions: AgentPermissions,
}

impl AgentTask {
    pub async fn execute(&self, runner: &AgentRunner) -> Result<AgentResult> {
        runner.run(self.task.clone()).await
    }
}
```

#### 2.5 TUI Agent Visualization (Week 4)

```rust
// codex-rs/tui/src/agent_view.rs
pub struct AgentView {
    active_agent: Option<AgentInfo>,
    execution_status: ExecutionStatus,
    findings: Vec<AgentFinding>,
}

impl AgentView {
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // Render agent card with:
        // - Agent name and icon
        // - Current status (thinking, executing, completed)
        // - Progress indicator
        // - Findings/results as they stream in
    }
}
```

### Deliverables
- ✅ Agent trait and architecture
- ✅ Agent toolkit with permission enforcement
- ✅ 5 core agent implementations
- ✅ Agent router with context-based selection
- ✅ Integration with executor and tasks
- ✅ TUI agent status visualization
- ✅ Agent configuration system

---

## Phase 3: Multi-Agent Orchestration (4-5 weeks)

### Objectives
- Enable parallel agent execution
- Implement task delegation and sub-agent spawning
- Build agent coordination framework
- Advanced MCP integration for agent tools
- Multi-agent dashboard in TUI

### Implementation Tasks

#### 3.1 Agent Orchestrator (Week 1-2)

```rust
// codex-rs/core/src/agents/orchestrator.rs
pub struct AgentOrchestrator {
    router: AgentRouter,
    max_parallel: usize,
    coordination_strategy: CoordinationStrategy,
}

pub enum CoordinationStrategy {
    /// Execute agents in parallel, combine results
    Parallel,
    /// Execute sequentially, pass results forward
    Sequential,
    /// Execute in parallel, agents can communicate
    Collaborative,
}

impl AgentOrchestrator {
    /// Decompose complex task into sub-tasks
    pub async fn decompose_task(&self, task: &Task) -> Result<Vec<SubTask>> {
        // Analyze task complexity
        // Identify independent sub-tasks
        // Map sub-tasks to agent capabilities
    }

    /// Execute multiple agents in coordination
    pub async fn coordinate(&self, task: Task) -> Result<OrchestratedResult> {
        let sub_tasks = self.decompose_task(&task).await?;

        match self.coordination_strategy {
            CoordinationStrategy::Parallel => {
                self.execute_parallel(sub_tasks).await
            }
            CoordinationStrategy::Sequential => {
                self.execute_sequential(sub_tasks).await
            }
            CoordinationStrategy::Collaborative => {
                self.execute_collaborative(sub_tasks).await
            }
        }
    }

    async fn execute_parallel(&self, tasks: Vec<SubTask>) -> Result<OrchestratedResult> {
        use tokio::task::JoinSet;

        let mut join_set = JoinSet::new();

        for sub_task in tasks {
            let agent = self.router.select_agent(&sub_task.context).await?;
            let toolkit = self.create_toolkit(&agent).await?;

            join_set.spawn(async move {
                agent.execute(sub_task.into(), &toolkit).await
            });
        }

        let mut results = Vec::new();
        while let Some(result) = join_set.join_next().await {
            results.push(result??);
        }

        self.synthesize_results(results).await
    }
}
```

#### 3.2 Agent Communication Protocol (Week 2)

```rust
// Enable agents to share context and findings
pub struct AgentContext {
    pub findings: Vec<AgentFinding>,
    pub shared_data: HashMap<String, Value>,
    pub agent_messages: Vec<AgentMessage>,
}

pub struct AgentMessage {
    pub from: AgentId,
    pub to: Option<AgentId>,  // None = broadcast
    pub message_type: MessageType,
    pub content: Value,
}

pub enum MessageType {
    Finding,
    Question,
    Suggestion,
    Result,
}

// Agents can publish findings for other agents
impl AgentToolkit {
    pub async fn publish_finding(&self, finding: AgentFinding) -> Result<()> {
        self.context.findings.push(finding);
        self.broadcast(AgentMessage {
            from: self.agent_id,
            to: None,
            message_type: MessageType::Finding,
            content: serde_json::to_value(finding)?,
        }).await
    }

    pub async fn query_agent(&self, target: AgentId, query: &str) -> Result<Value> {
        // Inter-agent communication
    }
}
```

#### 3.3 Advanced MCP Integration (Week 3)

```rust
// Expose MCP tools to agents through toolkit
impl AgentToolkit {
    pub async fn mcp_tools(&self) -> Vec<McpToolInfo> {
        if let Some(client) = &self.mcp_client {
            client.list_tools().await.unwrap_or_default()
        } else {
            vec![]
        }
    }

    pub async fn invoke_mcp(&self, tool: &str, args: Value) -> Result<Value> {
        // Permission check
        if !self.agent_permissions.can_use_tool(tool) {
            return Err(AgentError::PermissionDenied(tool.to_string()));
        }

        // Invoke MCP tool
        self.mcp_client
            .as_ref()
            .ok_or(AgentError::McpNotAvailable)?
            .call_tool(tool, args)
            .await
    }
}

// Agent can use MCP capabilities
impl SecurityAnalystAgent {
    async fn execute(&self, task: Task, toolkit: &AgentToolkit) -> AgentResult {
        // Check if security scanner MCP tool is available
        let tools = toolkit.mcp_tools().await;

        if tools.iter().any(|t| t.name == "security-scanner") {
            // Use MCP security scanner
            let result = toolkit.invoke_mcp(
                "security-scanner",
                json!({ "path": task.context.file_paths[0] })
            ).await?;

            return Ok(AgentResult::from_mcp(result));
        }

        // Fallback to built-in analysis
        self.builtin_analysis(task, toolkit).await
    }
}
```

#### 3.4 Task Delegation System (Week 4)

```rust
// Agent can spawn sub-agents
impl AgentToolkit {
    pub async fn delegate_to_agent(&self, agent_id: AgentId, task: Task) -> Result<AgentResult> {
        // Permission check
        if !self.agent_permissions.can_delegate {
            return Err(AgentError::DelegationNotAllowed);
        }

        // Create sub-agent context
        let sub_context = TaskContext {
            parent_agent: Some(self.agent_id),
            ..task.context
        };

        // Execute sub-agent
        self.orchestrator.execute_agent(agent_id, task).await
    }
}

// Example: Code reviewer delegates to security analyst
impl CodeReviewerAgent {
    async fn execute(&self, task: Task, toolkit: &AgentToolkit) -> AgentResult {
        // Perform general review
        let mut findings = self.general_review(&task, toolkit).await?;

        // Detect security-sensitive code
        if self.has_security_concerns(&task) {
            // Delegate to security specialist
            let security_result = toolkit
                .delegate_to_agent(
                    AgentId::from("security-analyst"),
                    task.clone()
                )
                .await?;

            findings.merge(security_result.findings);
        }

        Ok(AgentResult::CodeReview { findings })
    }
}
```

#### 3.5 Multi-Agent TUI Dashboard (Week 5)

```rust
// codex-rs/tui/src/multi_agent_view.rs
pub struct MultiAgentDashboard {
    active_agents: Vec<AgentStatus>,
    communication_log: Vec<AgentMessage>,
    synthesis_view: Option<SynthesisResult>,
}

pub struct AgentStatus {
    pub agent: AgentInfo,
    pub status: ExecutionStatus,
    pub progress: f32,
    pub findings_count: usize,
    pub current_action: String,
}

impl MultiAgentDashboard {
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // Layout:
        // ┌─────────────────────────────────────┐
        // │  Active Agents (3)                  │
        // ├─────────────────────────────────────┤
        // │ 🔍 code-reviewer    ████░░ 80%      │
        // │    Analyzing auth.rs                │
        // │                                      │
        // │ 🛡️  security-analyst ██████ 100%    │
        // │    Found 2 vulnerabilities          │
        // │                                      │
        // │ ⚡ performance-opt   ██░░░░ 40%     │
        // │    Profiling hot paths              │
        // ├─────────────────────────────────────┤
        // │  Communication Log                  │
        // ├─────────────────────────────────────┤
        // │ security → code-reviewer:           │
        // │   "Auth bypass in login handler"    │
        // │                                      │
        // │ code-reviewer → all:                │
        // │   "Completed initial scan"          │
        // └─────────────────────────────────────┘
    }
}
```

### Deliverables
- ✅ Agent orchestrator with coordination strategies
- ✅ Inter-agent communication protocol
- ✅ Task delegation system
- ✅ MCP tool integration for agents
- ✅ Multi-agent TUI dashboard
- ✅ Advanced workflow examples
- ✅ Performance optimization for parallel execution

---

## Security & Permission Model

### Integration with execpolicy

**Permission Validation Flow:**
```rust
// Before executing any shell command from agent
pub async fn validate_agent_command(
    agent: &dyn Agent,
    command: &str,
    args: &[String],
) -> Result<ValidationResult> {
    // 1. Check agent permissions
    if !agent.permissions().shell_execution {
        return Ok(ValidationResult::Forbidden(
            "Agent not allowed to execute shell commands"
        ));
    }

    // 2. Pass through execpolicy
    let policy = ExecPolicy::load_default()?;
    let result = policy.check(command, args)?;

    match result.result {
        PolicyResult::Safe => Ok(ValidationResult::Allowed),
        PolicyResult::Match => {
            // Extract writeable files from policy match
            let write_files: Vec<_> = result.match_data
                .args
                .iter()
                .filter(|a| a.arg_type == ArgType::WriteableFile)
                .map(|a| &a.value)
                .collect();

            // Check if agent can write to these files
            if write_files.iter().all(|f| agent.permissions().can_write(f)) {
                Ok(ValidationResult::Allowed)
            } else {
                Ok(ValidationResult::RequiresApproval { write_files })
            }
        }
        PolicyResult::Forbidden => {
            Ok(ValidationResult::Forbidden(result.reason))
        }
        PolicyResult::Unverified => {
            Ok(ValidationResult::RequiresApproval {
                write_files: vec![]
            })
        }
    }
}
```

**Agent Permission Model:**
```toml
# ~/.codex/agents/security-analyst.toml
[agent]
id = "security-analyst"
name = "Security Analyst"
description = "Security vulnerability detection and analysis"

[permissions]
file_access = "read-only"
shell_execution = true
network_access = false
max_iterations = 10

[permissions.allowed_tools]
grep = true
rg = true
git = { commands = ["diff", "log", "show"] }
security-scanner = true  # MCP tool

[permissions.file_patterns]
allow_read = ["**/*.rs", "**/*.toml", "**/Cargo.lock"]
deny_read = ["**/.env", "**/secrets/**"]
```

### Safety Guarantees

1. **Sandbox Inheritance**: All agent commands run in same sandbox as user commands
2. **Permission Cascading**: Sub-agents inherit parent agent's permissions (cannot escalate)
3. **Tool Whitelisting**: Only explicitly allowed tools can be invoked
4. **File Access Control**: Pattern-based file access restrictions
5. **Audit Logging**: All agent actions logged for review

---

## Configuration & User Experience

### Global Configuration

**~/.codex/config.toml additions:**
```toml
[commands]
# Directory for user commands
user_commands_dir = "~/.codex/commands"
# Auto-reload on file changes
auto_reload = true
# Default command execution mode
default_mode = "sandbox"

[agents]
# Directory for user-defined agent configs
user_agents_dir = "~/.codex/agents"
# Agent activation threshold (0.0-1.0)
activation_threshold = 0.6
# Enable multi-agent orchestration
enable_orchestration = true
# Max parallel agents
max_parallel_agents = 3

[agents.suggestions]
# Auto-suggest agents based on context
auto_suggest = true
# Number of suggestions to show
suggestion_count = 3
```

### User Interaction Patterns

**Command Invocation:**
```bash
# Explicit command
codex /review-security src/auth.rs

# Command with arguments
codex /refactor --strategy=extract-method --target=auth.rs:45-67

# List available commands
codex /commands

# Command help
codex /help review-security
```

**Agent Invocation:**
```bash
# Explicit agent
codex /agent security-analyst "analyze authentication"

# Auto-select agent
codex "perform security review of auth module"
# → Auto-activates security-analyst agent

# Multi-agent workflow
codex /orchestrate "comprehensive code review"
# → Activates code-reviewer, security-analyst, test-engineer in parallel
```

**TUI Shortcuts:**
- `Ctrl+K`: Open command palette
- `Ctrl+Shift+K`: Open agent selector
- `Ctrl+A`: Show active agents dashboard
- `/`: Start typing slash command
- `@`: Mention agent in conversation

---

## Implementation Timeline

### Phase 1: Command System (2-3 weeks)
**Week 1:**
- [ ] Design and implement command file format
- [ ] Build command parser and validator
- [ ] Create template expansion engine

**Week 2:**
- [ ] Implement command registry
- [ ] Integrate with exec_command flow
- [ ] Build 3 example built-in commands

**Week 3:**
- [ ] Develop TUI command palette
- [ ] Add user command hot-reloading
- [ ] Write documentation and examples

### Phase 2: Agent System (3-4 weeks)
**Week 1:**
- [ ] Design agent trait and architecture
- [ ] Implement agent toolkit
- [ ] Build permission system

**Week 2:**
- [ ] Implement code-reviewer agent
- [ ] Implement security-analyst agent
- [ ] Build agent router

**Week 3:**
- [ ] Implement remaining core agents
- [ ] Integrate with executor
- [ ] Add agent task type

**Week 4:**
- [ ] Build TUI agent visualization
- [ ] Implement agent configuration
- [ ] Testing and refinement

### Phase 3: Multi-Agent Orchestration (4-5 weeks)
**Week 1-2:**
- [ ] Build agent orchestrator
- [ ] Implement coordination strategies
- [ ] Design inter-agent communication

**Week 3:**
- [ ] MCP integration for agents
- [ ] Tool discovery and invocation
- [ ] Permission validation

**Week 4:**
- [ ] Task delegation system
- [ ] Sub-agent spawning
- [ ] Result synthesis

**Week 5:**
- [ ] Multi-agent TUI dashboard
- [ ] Advanced workflows
- [ ] Performance optimization

**Total: 9-12 weeks**

---

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parsing() {
        let cmd = parse_slash_command("/review-security src/auth.rs --depth=deep").unwrap();
        assert_eq!(cmd.name, "review-security");
        assert_eq!(cmd.args.get("path"), Some(&"src/auth.rs".into()));
        assert_eq!(cmd.args.get("depth"), Some(&"deep".into()));
    }

    #[tokio::test]
    async fn test_agent_selection() {
        let router = AgentRouter::new();
        let context = TaskContext {
            user_intent: "review security vulnerabilities".into(),
            ..Default::default()
        };

        let agent = router.select_agent(&context).await.unwrap();
        assert_eq!(agent.id(), AgentId::from("security-analyst"));
    }

    #[tokio::test]
    async fn test_permission_validation() {
        let agent = SecurityAnalystAgent;
        let result = validate_agent_command(
            &agent,
            "rm",
            &["-rf", "/"]
        ).await.unwrap();

        assert!(matches!(result, ValidationResult::Forbidden(_)));
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_multi_agent_workflow() {
    let orchestrator = AgentOrchestrator::new();
    let task = Task {
        context: TaskContext {
            user_intent: "comprehensive code review".into(),
            file_paths: vec![PathBuf::from("src/")],
            ..Default::default()
        },
        ..Default::default()
    };

    let result = orchestrator.coordinate(task).await.unwrap();

    // Verify multiple agents executed
    assert!(result.agent_results.len() >= 2);
    assert!(result.agent_results.iter().any(|r| r.agent_id == "code-reviewer"));
    assert!(result.agent_results.iter().any(|r| r.agent_id == "security-analyst"));
}
```

### End-to-End Tests
- Command palette interaction
- Agent auto-activation
- Multi-agent coordination
- Permission enforcement
- MCP tool integration

---

## Performance Considerations

### Optimization Strategies

1. **Command Registry Caching**
   - Parse command files once
   - Cache compiled templates
   - Hot-reload only changed files

2. **Lazy Agent Loading**
   - Load agents on-demand
   - Cache agent instances
   - Parallel agent initialization

3. **Parallel Execution**
   - Use tokio for async operations
   - Parallel sub-agent spawning
   - Concurrent MCP tool calls

4. **Memory Management**
   - Stream agent results
   - Limit context size
   - Clean up completed tasks

### Performance Targets
- Command invocation: <50ms
- Agent activation: <100ms
- Multi-agent coordination: <2s for 3 agents
- Memory per agent: <50MB

---

## Migration & Backward Compatibility

### Compatibility Guarantees
- ✅ Existing Codex workflows unchanged
- ✅ No breaking changes to exec/TUI modes
- ✅ Opt-in agent system (disabled by default initially)
- ✅ Command system extends, doesn't replace

### Migration Path
1. Phase 1: Commands available but optional
2. Phase 2: Agents available behind feature flag
3. Phase 3: Enable by default with stable release

### Rollback Strategy
- Environment variable to disable: `CODEX_DISABLE_AGENTS=1`
- Config flag: `agents.enabled = false`
- Remove `~/.codex/commands` to disable custom commands

---

## Documentation Requirements

### User Documentation
- [ ] Command creation tutorial
- [ ] Agent configuration guide
- [ ] Permission model explanation
- [ ] Built-in commands reference
- [ ] Built-in agents reference
- [ ] Troubleshooting guide

### Developer Documentation
- [ ] Architecture overview
- [ ] Agent trait implementation guide
- [ ] Command trait implementation guide
- [ ] Testing guidelines
- [ ] Contributing new agents

### Examples
- [ ] 10 example custom commands
- [ ] 5 agent configuration examples
- [ ] Multi-agent workflow examples
- [ ] MCP integration examples

---

## Success Metrics

### Phase 1 Success Criteria
- ✅ 5 built-in commands implemented
- ✅ User can create custom command in <5 minutes
- ✅ Command palette accessible via keyboard
- ✅ Template expansion supports variables and conditionals

### Phase 2 Success Criteria
- ✅ 5 core agents implemented
- ✅ Agent auto-activation >80% accuracy
- ✅ Permission system prevents unauthorized actions
- ✅ Agent results visible in TUI

### Phase 3 Success Criteria
- ✅ Multi-agent coordination functional
- ✅ Inter-agent communication working
- ✅ MCP tools accessible to agents
- ✅ <2s latency for 3-agent workflow

---

## Appendix

### A. Example Command Definitions

**Simple Command:**
```markdown
---
name: explain
description: Explain code in simple terms
category: documentation
---

Please explain the following code in simple, beginner-friendly terms:

{{#if args.file}}
{{file_content args.file}}
{{else}}
{{selection}}
{{/if}}

Focus on:
- What the code does
- Why it's structured this way
- Any important patterns or concepts
```

**Advanced Command:**
```markdown
---
name: migrate-framework
description: Migrate code to new framework version
category: refactoring
agent: refactoring-expert
permissions:
  - read_files
  - write_files
args:
  - name: framework
    type: string
    required: true
  - name: from_version
    type: string
    required: true
  - name: to_version
    type: string
    required: true
---

Migrate the codebase from {{framework}} v{{from_version}} to v{{to_version}}.

Steps:
1. Analyze current usage patterns
2. Identify breaking changes
3. Generate migration plan
4. Apply transformations
5. Update tests
6. Verify functionality

{{#if context.git_diff}}
Focus on recently changed files:
{{context.git_diff}}
{{/if}}
```

### B. Example Agent Configurations

**Rust Expert Agent:**
```toml
[agent]
id = "rust-expert"
name = "Rust Expert"
description = "Rust language patterns, idioms, and best practices"
category = "language-specific"

[activation]
# File patterns that trigger this agent
file_patterns = ["**/*.rs", "**/Cargo.toml"]
# Keywords in user intent
keywords = ["rust", "borrow", "lifetime", "trait", "macro"]
# Minimum activation score
threshold = 0.6

[permissions]
file_access = "read-write"
shell_execution = true
allowed_commands = ["cargo", "rustc", "clippy"]

[prompt]
system = """
You are a Rust programming expert with deep knowledge of:
- Ownership, borrowing, and lifetimes
- Trait system and generics
- Async/await and concurrency
- Unsafe code and FFI
- Performance optimization
- Cargo ecosystem

Provide idiomatic Rust solutions following best practices.
"""
```

### C. Built-in Agents Specification

| Agent | Purpose | Activation Context | Key Capabilities |
|-------|---------|-------------------|------------------|
| **code-reviewer** | General code quality | Code files, "review" keywords | Pattern analysis, best practices, bug detection |
| **security-analyst** | Security vulnerabilities | Auth/crypto files, "security" keywords | OWASP patterns, vulnerability scanning |
| **refactoring-expert** | Code structure improvements | Complex files, "refactor" keywords | Extract method, simplify, DRY principles |
| **test-engineer** | Test coverage & quality | Test files, low coverage | Test generation, coverage analysis |
| **performance-optimizer** | Performance analysis | Slow code, "optimize" keywords | Profiling, algorithm complexity |

---

## Conclusion

This specification provides a comprehensive roadmap for implementing Claude Code-style commands, prompts, and agents into Codex. The **hybrid three-tier architecture** balances:

- **Accessibility**: Users can easily create custom commands
- **Power**: Built-in commands leverage full Codex capabilities
- **Intelligence**: Agents provide specialized, context-aware assistance

The **phased approach** enables incremental delivery:
- Phase 1 (2-3 weeks): Core command infrastructure
- Phase 2 (3-4 weeks): Agent system foundation
- Phase 3 (4-5 weeks): Multi-agent orchestration

**Key Technical Decisions:**
✅ Markdown-based command format for simplicity
✅ Rust trait-based agent architecture for performance
✅ Full integration with execpolicy for security
✅ Backward compatible with existing Codex workflows
✅ Extensible design for future enhancements

The implementation maintains Codex's core strengths (security, modularity, performance) while adding powerful new capabilities for command extensibility and intelligent task automation.
