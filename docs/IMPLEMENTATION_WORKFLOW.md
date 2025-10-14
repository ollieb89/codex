# Command & Agent System - Agile Implementation Workflow

## Executive Summary

This document provides a **detailed, actionable implementation workflow** for the Command & Agent System specification, organized using **Agile methodology** with 2-week sprints. The workflow spans **7 sprints (14 weeks)** and includes parallel workstreams, risk mitigation strategies, and comprehensive quality gates.

**Key Characteristics:**
- 🔄 **Iterative Development**: Incremental value delivery with continuous feedback
- 🚀 **Parallel Execution**: Multiple independent workstreams for faster delivery
- ✅ **Quality-First**: Quality gates and acceptance criteria for every sprint
- 📊 **Risk-Aware**: Proactive risk identification and mitigation
- 🎯 **Goal-Oriented**: Clear sprint objectives and measurable outcomes

---

## Table of Contents

1. [Sprint Structure Overview](#sprint-structure-overview)
2. [Parallel Workstreams](#parallel-workstreams)
3. [Sprint 0: Foundation & Architecture](#sprint-0-foundation--architecture-1-week)
4. [Sprint 1: Command System MVP](#sprint-1-command-system-mvp-2-weeks)
5. [Sprint 2: Command Enhancement & Agent Prototype](#sprint-2-command-enhancement--agent-prototype-2-weeks)
6. [Sprint 3: Agent Integration & Validation](#sprint-3-agent-integration--validation-2-weeks)
7. [Sprint 4-5: Core Agents Implementation](#sprint-4-5-core-agents-implementation-4-weeks)
8. [Sprint 6-7: Multi-Agent Orchestration](#sprint-6-7-multi-agent-orchestration-4-weeks)
9. [Risk Management Strategy](#risk-management-strategy)
10. [Quality Assurance & Testing](#quality-assurance--testing)
11. [Team Coordination & Ceremonies](#team-coordination--ceremonies)
12. [Success Metrics & KPIs](#success-metrics--kpis)

---

## Sprint Structure Overview

### Timeline: 14 Weeks Total

```
Sprint 0    Sprint 1-2    Sprint 3      Sprint 4-5        Sprint 6-7
[1 week]    [4 weeks]     [2 weeks]     [4 weeks]         [4 weeks]
────────    ──────────    ─────────     ──────────        ──────────
Foundation  Command MVP   Agent Proto   Core Agents       Orchestration
& Spikes    & Enhance     & Integrate   Implementation    & Multi-Agent

Week:  1     2  3  4  5    6  7          8  9  10  11      12  13  14  15
       │     │     │     │  │  │         │     │      │    │      │      │
       └─────┴─────┴─────┴──┴──┴─────────┴─────┴──────┴────┴──────┴──────┘
       Setup Commands    Agent         5 Agents         Parallel
                         Toolkit       Built            Execution
```

### Sprint Goals Summary

| Sprint | Duration | Primary Goal | Key Deliverable |
|--------|----------|--------------|-----------------|
| **0** | 1 week | Foundation & Risk Mitigation | Architecture approved, spikes complete |
| **1** | 2 weeks | Command System MVP | Basic slash commands working |
| **2** | 2 weeks | Command Enhancement + Agent Prototype | TUI palette + code-reviewer agent |
| **3** | 2 weeks | Agent Integration | Agent toolkit + router functional |
| **4-5** | 4 weeks | Core Agents | 5 specialized agents operational |
| **6-7** | 4 weeks | Multi-Agent Orchestration | Parallel execution + coordination |

---

## Parallel Workstreams

### Overview

To maximize efficiency, development occurs across **three parallel tracks**:

```
┌─────────────────────────────────────────────────────────────┐
│ TRACK 1: Command System (Primary - Critical Path)          │
│ Sprint 0 → Sprint 1 → Sprint 2 → Sprint 3                   │
│ Foundation → MVP → Enhancement → Integration                │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ TRACK 2: Agent System (Parallel - Starts Sprint 1)         │
│           Sprint 1 → Sprint 2 → Sprint 3 → Sprint 4-5       │
│           Design → Prototype → Integration → Full Suite     │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ TRACK 3: Infrastructure (Continuous - All Sprints)         │
│ Testing │ Documentation │ CI/CD │ Performance Benchmarks    │
└─────────────────────────────────────────────────────────────┘
```

### Track Dependencies

```
Command Registry (Sprint 1) ──┐
                              ├──> Agent Toolkit (Sprint 2)
execpolicy Integration (S1) ──┘

Agent Toolkit (Sprint 2) ──┐
                           ├──> Agent Router (Sprint 3)
Agent Trait (Sprint 1) ────┘

Agent Router (Sprint 3) ──┐
                          ├──> Multi-Agent Orchestrator (Sprint 6)
Single Agent (Sprint 2) ──┘
```

---

## Sprint 0: Foundation & Architecture (1 Week)

### Sprint Goal
**Establish architectural foundation, mitigate high-risk items through spikes, and set up development infrastructure.**

### Sprint Objectives
- ✅ Architecture design validated and approved
- ✅ Module structure created in codebase
- ✅ High-risk technical spikes completed
- ✅ Development environment ready

---

### Day 1-2: Architecture Design & Setup

#### Tasks

**Task 0.1: Module Structure Creation** (4 hours)
```bash
# Create core module structure
codex-rs/core/src/
├── commands/
│   ├── mod.rs           # Module exports, public API
│   ├── registry.rs      # Command discovery and management
│   ├── parser.rs        # Markdown + YAML parser
│   ├── expander.rs      # Template expansion engine
│   ├── permissions.rs   # Permission model
│   ├── context.rs       # Context builder for templates
│   ├── builtin/         # Built-in commands
│   │   └── mod.rs
│   └── user/            # User command loading
│       └── loader.rs
│
└── agents/
    ├── mod.rs           # Agent trait definition
    ├── router.rs        # Context-based agent selection
    ├── toolkit.rs       # Agent execution toolkit
    ├── permissions.rs   # Agent permission model
    └── builtin/         # Built-in agents (placeholder)
        └── mod.rs
```

**Acceptance Criteria:**
- [ ] Module structure exists and compiles
- [ ] Each module has basic documentation
- [ ] Public API exports defined

**Task 0.2: Update Cargo.toml Dependencies** (2 hours)
```toml
[dependencies]
# Template engine for command expansion
handlebars = "5.1"

# YAML parsing for frontmatter
serde_yaml = "0.9"

# Markdown parsing
pulldown-cmark = "0.10"

# Async runtime (already present, verify version)
tokio = { version = "1.35", features = ["full"] }

# File watching for hot-reload
notify = "6.1"
```

**Acceptance Criteria:**
- [ ] Dependencies added and build succeeds
- [ ] No version conflicts with existing deps
- [ ] Feature flags configured appropriately

**Task 0.3: Architecture Design Review** (2 hours)
- Review specification with team
- Validate trait designs
- Confirm integration points with existing exec_command
- Document any deviations from spec

**Acceptance Criteria:**
- [ ] Architecture diagram approved
- [ ] Integration points documented
- [ ] Team consensus on approach

---

### Day 3-4: Technical Spikes (Risk Mitigation)

#### Spike 0.1: Permission Model Integration (6 hours)

**Objective:** Validate that agent permissions can integrate with existing execpolicy

**Tasks:**
1. Study existing execpolicy implementation
2. Prototype `AgentPermissions` struct
3. Implement validation function
4. Test with sample commands

**Implementation:**
```rust
// codex-rs/core/src/agents/permissions.rs
use codex_execpolicy::{ExecPolicy, PolicyResult};

#[derive(Debug, Clone)]
pub struct AgentPermissions {
    pub file_access: FileAccessPolicy,
    pub shell_execution: bool,
    pub network_access: bool,
    pub allowed_tools: Vec<String>,
    pub max_iterations: u32,
}

#[derive(Debug, Clone)]
pub enum FileAccessPolicy {
    ReadOnly,
    ReadWrite { allow_patterns: Vec<String>, deny_patterns: Vec<String> },
    NoAccess,
}

pub async fn validate_agent_command(
    agent_perms: &AgentPermissions,
    command: &str,
    args: &[String],
) -> Result<ValidationResult, Error> {
    // 1. Check agent-level permissions
    if !agent_perms.shell_execution {
        return Ok(ValidationResult::Forbidden(
            "Agent not permitted to execute shell commands".into()
        ));
    }

    // 2. Pass through execpolicy
    let policy = ExecPolicy::load_default()?;
    let result = policy.check(command, args)?;

    // 3. Combine results
    match result.result {
        PolicyResult::Safe => Ok(ValidationResult::Allowed),
        PolicyResult::Match => {
            // Check writeable files against agent permissions
            validate_file_access(agent_perms, &result.match_data)
        }
        PolicyResult::Forbidden => {
            Ok(ValidationResult::Forbidden(result.reason))
        }
        PolicyResult::Unverified => {
            Ok(ValidationResult::RequiresApproval)
        }
    }
}
```

**Success Criteria:**
- [ ] Agent permissions integrate with execpolicy
- [ ] Validation function works correctly
- [ ] Tests pass for safe/forbidden cases
- [ ] Performance <5ms per validation

**Deliverable:** `spike-permission-integration.md` with findings

---

#### Spike 0.2: Template Engine Performance (4 hours)

**Objective:** Validate template expansion can meet <50ms performance target

**Tasks:**
1. Prototype Handlebars integration
2. Benchmark simple templates
3. Benchmark complex templates with conditionals
4. Test with real-world command examples

**Implementation:**
```rust
// codex-rs/core/src/commands/expander.rs
use handlebars::Handlebars;
use serde_json::json;

pub struct TemplateExpander {
    handlebars: Handlebars<'static>,
}

impl TemplateExpander {
    pub fn new() -> Self {
        let mut hb = Handlebars::new();
        // Register helpers
        hb.register_helper("default", Box::new(default_helper));
        Self { handlebars: hb }
    }

    pub fn expand(&self, template: &str, context: &CommandContext) -> Result<String> {
        let data = json!({
            "args": context.args,
            "context": {
                "git_diff": context.git_diff,
                "files": context.files,
                "workspace_root": context.workspace_root,
            }
        });

        self.handlebars.render_template(template, &data)
            .map_err(|e| Error::TemplateExpansion(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn benchmark_template_expansion() {
        let expander = TemplateExpander::new();
        let template = "Review {{path}} with {{depth}} analysis...";
        let context = CommandContext { /* ... */ };

        let start = std::time::Instant::now();
        for _ in 0..1000 {
            expander.expand(template, &context).unwrap();
        }
        let duration = start.elapsed();

        // Should be <50ms for 1000 iterations = <0.05ms per expansion
        assert!(duration.as_millis() < 50);
    }
}
```

**Success Criteria:**
- [ ] Simple template expansion <1ms
- [ ] Complex template with conditionals <5ms
- [ ] 1000 expansions <50ms total
- [ ] Memory usage reasonable (<10MB)

**Deliverable:** `spike-template-performance.md` with benchmarks

---

### Day 5: Testing Infrastructure Setup

#### Task 0.4: Test Framework Configuration (4 hours)

**Setup test structure:**
```
codex-rs/core/tests/
├── common/
│   ├── mod.rs
│   ├── fixtures/            # Test fixtures
│   │   ├── commands/        # Sample command files
│   │   └── agents/          # Sample agent configs
│   └── helpers.rs           # Test utilities
│
├── commands/
│   ├── parser_test.rs
│   ├── registry_test.rs
│   └── expander_test.rs
│
└── agents/
    ├── permissions_test.rs
    ├── router_test.rs
    └── toolkit_test.rs
```

**Task 0.5: Snapshot Testing Setup** (2 hours)
```rust
// Add to Cargo.toml
[dev-dependencies]
insta = "1.34"
pretty_assertions = "1.4"

// Example snapshot test
#[test]
fn test_command_parsing() {
    let input = include_str!("../fixtures/commands/review-security.md");
    let parsed = CommandParser::parse(input).unwrap();
    insta::assert_yaml_snapshot!(parsed);
}
```

**Acceptance Criteria:**
- [ ] Test framework configured
- [ ] Snapshot testing working
- [ ] CI/CD runs tests automatically
- [ ] Coverage reporting enabled

---

### Sprint 0 Quality Gates

**Exit Criteria (All Must Pass):**
- ✅ Architecture design document approved by team
- ✅ Module structure created and compiling
- ✅ Permission spike validates integration feasibility
- ✅ Template engine meets <50ms performance target
- ✅ Test infrastructure operational
- ✅ CI/CD pipeline running tests
- ✅ Zero high-risk unknowns remaining

**Deliverables:**
- [ ] `docs/architecture-design.md`
- [ ] `docs/spike-permission-integration.md`
- [ ] `docs/spike-template-performance.md`
- [ ] Module structure in codebase
- [ ] Test framework configured

---

## Sprint 1: Command System MVP (2 Weeks)

### Sprint Goal
**Deliver minimal viable command system: parse Markdown commands, register them, and execute with basic template expansion.**

### Sprint Objectives
- ✅ Parse Markdown command files with YAML frontmatter
- ✅ Command registry discovers and loads commands
- ✅ Template expansion with variable interpolation
- ✅ Integration with exec_command flow
- ✅ 3 example built-in commands working

---

### Week 1: Core Command Implementation

#### Epic 1.1: Command File Format

**Story 1.1.1: Parse Markdown Command Files** (8 hours)

*As a developer, I can parse Markdown files with YAML frontmatter to extract command metadata and prompt templates.*

**Tasks:**

**Task 1.1.1.A: YAML Frontmatter Parser** (3 hours)
```rust
// codex-rs/core/src/commands/parser.rs
use serde::{Deserialize, Serialize};
use pulldown_cmark::{Parser, Event};

#[derive(Debug, Deserialize, Serialize)]
pub struct CommandMetadata {
    pub name: String,
    pub description: String,
    pub category: CommandCategory,
    #[serde(default)]
    pub permissions: CommandPermissions,
    #[serde(default)]
    pub args: Vec<ArgDefinition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CommandCategory {
    Analysis,
    Refactoring,
    Documentation,
    Testing,
    Custom,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArgDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: ArgType,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub default: Option<String>,
}

pub struct CommandParser;

impl CommandParser {
    pub fn parse(content: &str) -> Result<ParsedCommand, Error> {
        // 1. Split frontmatter and body
        let (frontmatter, body) = Self::split_frontmatter(content)?;

        // 2. Parse YAML frontmatter
        let metadata: CommandMetadata = serde_yaml::from_str(&frontmatter)
            .map_err(|e| Error::InvalidFrontmatter(e.to_string()))?;

        // 3. Extract Markdown body as template
        let template = body.trim().to_string();

        Ok(ParsedCommand { metadata, template })
    }

    fn split_frontmatter(content: &str) -> Result<(String, String), Error> {
        let lines: Vec<&str> = content.lines().collect();

        if !lines.first().map_or(false, |l| l.trim() == "---") {
            return Err(Error::MissingFrontmatter);
        }

        let end_idx = lines[1..]
            .iter()
            .position(|l| l.trim() == "---")
            .ok_or(Error::InvalidFrontmatter("No closing ---".into()))?
            + 1;

        let frontmatter = lines[1..end_idx].join("\n");
        let body = lines[end_idx + 1..].join("\n");

        Ok((frontmatter, body))
    }
}
```

**Acceptance Criteria:**
- [ ] Parse valid frontmatter correctly
- [ ] Extract template body
- [ ] Handle missing/invalid frontmatter gracefully
- [ ] Tests cover edge cases

**Task 1.1.1.B: Validation Rules** (2 hours)
```rust
impl CommandMetadata {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Name validation
        if self.name.is_empty() {
            return Err(ValidationError::EmptyName);
        }
        if !self.name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(ValidationError::InvalidName(self.name.clone()));
        }

        // Args validation
        for arg in &self.args {
            if arg.required && arg.default.is_some() {
                return Err(ValidationError::RequiredWithDefault(arg.name.clone()));
            }
        }

        Ok(())
    }
}
```

**Acceptance Criteria:**
- [ ] Validate command name format
- [ ] Validate argument definitions
- [ ] Reject invalid configurations
- [ ] Clear error messages

**Task 1.1.1.C: Unit Tests** (3 hours)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_command() {
        let input = r#"---
name: review-code
description: Code review assistant
category: analysis
---

Review the code at {{path}} for:
- Code quality
- Best practices
"#;
        let parsed = CommandParser::parse(input).unwrap();
        assert_eq!(parsed.metadata.name, "review-code");
        assert!(parsed.template.contains("{{path}}"));
    }

    #[test]
    fn reject_invalid_frontmatter() {
        let input = "No frontmatter here";
        assert!(CommandParser::parse(input).is_err());
    }

    #[test]
    fn validate_command_name() {
        let mut meta = CommandMetadata {
            name: "invalid name!".into(),
            ..Default::default()
        };
        assert!(meta.validate().is_err());
    }
}
```

---

**Story 1.1.2: Template Variable Interpolation** (6 hours)

*As a user, I can use variables in command templates that get replaced with actual values.*

**Task 1.1.2.A: Implement Template Expander** (4 hours)
```rust
// codex-rs/core/src/commands/expander.rs
pub struct TemplateExpander {
    handlebars: Handlebars<'static>,
}

impl TemplateExpander {
    pub fn new() -> Self {
        let mut hb = Handlebars::new();

        // Register custom helpers
        hb.register_helper("default", Box::new(default_helper));
        hb.register_helper("file_content", Box::new(file_content_helper));

        Self { handlebars: hb }
    }

    pub fn expand(&self, template: &str, context: &CommandContext) -> Result<String> {
        let data = json!({
            "args": context.args,
            "context": {
                "git_diff": context.git_diff,
                "files": context.files,
                "workspace_root": context.workspace_root,
            }
        });

        self.handlebars
            .render_template(template, &data)
            .map_err(|e| Error::TemplateExpansion(e.to_string()))
    }
}

// Custom helper: {{arg | default: "value"}}
fn default_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let value = h.param(0).and_then(|v| v.value().as_str());
    let default = h.param(1).and_then(|v| v.value().as_str()).unwrap_or("");

    out.write(value.unwrap_or(default))?;
    Ok(())
}
```

**Task 1.1.2.B: Context Builder** (2 hours)
```rust
// codex-rs/core/src/commands/context.rs
pub struct CommandContext {
    pub args: HashMap<String, String>,
    pub git_diff: Option<String>,
    pub files: Vec<PathBuf>,
    pub workspace_root: PathBuf,
}

impl CommandContext {
    pub async fn build(args: HashMap<String, String>) -> Result<Self> {
        // Get workspace root
        let workspace_root = std::env::current_dir()?;

        // Get git diff if available
        let git_diff = Self::get_git_diff().await.ok();

        // Get current files
        let files = Self::get_current_files(&workspace_root).await?;

        Ok(Self {
            args,
            git_diff,
            files,
            workspace_root,
        })
    }

    async fn get_git_diff() -> Result<String> {
        use tokio::process::Command;

        let output = Command::new("git")
            .args(&["diff", "--cached"])
            .output()
            .await?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
```

**Acceptance Criteria:**
- [ ] Variable interpolation works ({{var}})
- [ ] Default helper works ({{var | default: "val"}})
- [ ] Context includes git_diff, files, workspace
- [ ] Tests cover all helpers

---

#### Epic 1.2: Command Registry

**Story 1.2.1: Command Discovery** (8 hours)

*As a user, commands in ~/.codex/commands/ are automatically discovered and loaded.*

**Task 1.2.1.A: Directory Scanner** (3 hours)
```rust
// codex-rs/core/src/commands/user/loader.rs
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct UserCommandLoader {
    commands_dir: PathBuf,
}

impl UserCommandLoader {
    pub fn new(commands_dir: PathBuf) -> Self {
        Self { commands_dir }
    }

    pub async fn load_all(&self) -> Result<Vec<Box<dyn Command>>> {
        let mut commands = Vec::new();

        // Create directory if it doesn't exist
        fs::create_dir_all(&self.commands_dir).await?;

        // Scan for .md files
        let mut entries = fs::read_dir(&self.commands_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                match self.load_command(&path).await {
                    Ok(cmd) => commands.push(cmd),
                    Err(e) => {
                        eprintln!("Failed to load command from {:?}: {}", path, e);
                        continue;
                    }
                }
            }
        }

        Ok(commands)
    }

    async fn load_command(&self, path: &Path) -> Result<Box<dyn Command>> {
        let content = fs::read_to_string(path).await?;
        let parsed = CommandParser::parse(&content)?;

        // Validate
        parsed.metadata.validate()?;

        // Create command instance
        Ok(Box::new(UserDefinedCommand {
            metadata: parsed.metadata,
            template: parsed.template,
            expander: TemplateExpander::new(),
        }))
    }
}
```

**Task 1.2.1.B: Command Registry** (3 hours)
```rust
// codex-rs/core/src/commands/registry.rs
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CommandRegistry {
    commands: Arc<RwLock<HashMap<String, Box<dyn Command>>>>,
    loader: UserCommandLoader,
}

impl CommandRegistry {
    pub async fn new(commands_dir: PathBuf) -> Result<Self> {
        let loader = UserCommandLoader::new(commands_dir);
        let registry = Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
            loader,
        };

        // Initial load
        registry.reload().await?;

        Ok(registry)
    }

    pub async fn reload(&self) -> Result<()> {
        let commands = self.loader.load_all().await?;

        let mut map = self.commands.write().await;
        map.clear();

        for cmd in commands {
            map.insert(cmd.name().to_string(), cmd);
        }

        Ok(())
    }

    pub async fn get(&self, name: &str) -> Option<Box<dyn Command>> {
        let map = self.commands.read().await;
        map.get(name).map(|cmd| cmd.clone_box())
    }

    pub async fn list(&self) -> Vec<CommandInfo> {
        let map = self.commands.read().await;
        map.values()
            .map(|cmd| CommandInfo {
                name: cmd.name().to_string(),
                description: cmd.description().to_string(),
                category: cmd.category(),
            })
            .collect()
    }
}
```

**Task 1.2.1.C: Command Trait** (2 hours)
```rust
// codex-rs/core/src/commands/mod.rs
#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn category(&self) -> CommandCategory;
    fn permissions(&self) -> &CommandPermissions;
    fn args_schema(&self) -> &[ArgDefinition];

    async fn execute(&self, context: &CommandContext) -> Result<String>;

    // For cloning in registry
    fn clone_box(&self) -> Box<dyn Command>;
}

// User-defined command implementation
pub struct UserDefinedCommand {
    metadata: CommandMetadata,
    template: String,
    expander: TemplateExpander,
}

#[async_trait]
impl Command for UserDefinedCommand {
    fn name(&self) -> &str {
        &self.metadata.name
    }

    async fn execute(&self, context: &CommandContext) -> Result<String> {
        self.expander.expand(&self.template, context)
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(Self {
            metadata: self.metadata.clone(),
            template: self.template.clone(),
            expander: TemplateExpander::new(),
        })
    }
}
```

**Acceptance Criteria:**
- [ ] Discover all .md files in commands directory
- [ ] Parse and validate each command
- [ ] Registry provides get/list operations
- [ ] Handle loading errors gracefully

---

### Week 2: Integration & Built-in Commands

#### Epic 1.3: exec_command Integration

**Story 1.3.1: Slash Command Parsing** (6 hours)

*As a user, typing /command-name executes the command from registry.*

**Task 1.3.1.A: Input Parser** (3 hours)
```rust
// codex-rs/core/src/exec_command/command_input.rs
pub fn parse_input(input: &str) -> InputType {
    let trimmed = input.trim();

    if trimmed.starts_with('/') {
        // Parse slash command
        let parts: Vec<&str> = trimmed[1..].split_whitespace().collect();

        if parts.is_empty() {
            return InputType::Invalid("Empty command".into());
        }

        let command = parts[0].to_string();
        let args = parts[1..].to_vec();

        InputType::SlashCommand { command, args }
    } else {
        InputType::UserMessage(input.to_string())
    }
}

pub enum InputType {
    UserMessage(String),
    SlashCommand {
        command: String,
        args: Vec<String>,
    },
    Invalid(String),
}
```

**Task 1.3.1.B: Argument Parser** (3 hours)
```rust
pub fn parse_args(
    args: &[String],
    schema: &[ArgDefinition],
) -> Result<HashMap<String, String>> {
    let mut parsed = HashMap::new();

    // Parse positional and --flag=value args
    let mut positional_idx = 0;

    for arg in args {
        if arg.starts_with("--") {
            // Flag argument
            let (key, value) = arg[2..]
                .split_once('=')
                .ok_or(Error::InvalidArgFormat(arg.clone()))?;
            parsed.insert(key.to_string(), value.to_string());
        } else {
            // Positional argument
            if let Some(def) = schema.get(positional_idx) {
                parsed.insert(def.name.clone(), arg.clone());
                positional_idx += 1;
            } else {
                return Err(Error::TooManyArgs);
            }
        }
    }

    // Validate required args
    for def in schema {
        if def.required && !parsed.contains_key(&def.name) {
            return Err(Error::MissingRequiredArg(def.name.clone()));
        }

        // Apply defaults
        if !parsed.contains_key(&def.name) {
            if let Some(default) = &def.default {
                parsed.insert(def.name.clone(), default.clone());
            }
        }
    }

    Ok(parsed)
}
```

**Acceptance Criteria:**
- [ ] Parse /command syntax correctly
- [ ] Extract command name and arguments
- [ ] Handle --flag=value arguments
- [ ] Apply defaults for missing optional args

---

**Story 1.3.2: Command Execution Flow** (8 hours)

*As the system, execute commands through the standard exec_command pipeline.*

**Task 1.3.2.A: Modify exec_command_params.rs** (4 hours)
```rust
// codex-rs/core/src/exec_command/exec_command_params.rs
pub struct ExecCommandParams {
    pub message: String,
    pub command_metadata: Option<CommandMetadata>,
    // ... existing fields
}

impl ExecCommandParams {
    pub async fn from_input(
        input: &str,
        registry: &CommandRegistry,
    ) -> Result<Self> {
        match parse_input(input) {
            InputType::UserMessage(msg) => {
                // Standard user message
                Ok(Self {
                    message: msg,
                    command_metadata: None,
                    // ... other fields
                })
            }

            InputType::SlashCommand { command, args } => {
                // Load command from registry
                let cmd = registry
                    .get(&command)
                    .await
                    .ok_or(Error::CommandNotFound(command.clone()))?;

                // Parse arguments
                let parsed_args = parse_args(&args, cmd.args_schema())?;

                // Build context
                let context = CommandContext::build(parsed_args).await?;

                // Execute command (expand template)
                let expanded = cmd.execute(&context).await?;

                // Return as ExecCommandParams
                Ok(Self {
                    message: expanded,
                    command_metadata: Some(CommandMetadata {
                        name: cmd.name().to_string(),
                        permissions: cmd.permissions().clone(),
                    }),
                    // ... other fields
                })
            }

            InputType::Invalid(err) => {
                Err(Error::InvalidInput(err))
            }
        }
    }
}
```

**Task 1.3.2.B: Add Command Execution Path** (2 hours)
```rust
// codex-rs/core/src/executor/runner.rs (modifications)
impl Runner {
    pub async fn execute(&self, params: ExecCommandParams) -> Result<Response> {
        // Check if this came from a command
        if let Some(cmd_meta) = &params.command_metadata {
            // Apply command-specific permissions
            self.validate_command_permissions(cmd_meta)?;
        }

        // Continue with normal execution flow
        self.execute_standard(params).await
    }

    fn validate_command_permissions(&self, meta: &CommandMetadata) -> Result<()> {
        // Ensure command permissions are respected
        if meta.permissions.read_files && !self.config.allow_file_read {
            return Err(Error::PermissionDenied("File read not allowed"));
        }
        Ok(())
    }
}
```

**Task 1.3.2.C: Integration Tests** (2 hours)
```rust
#[tokio::test]
async fn test_command_execution_flow() {
    let registry = CommandRegistry::new(test_commands_dir()).await.unwrap();

    let input = "/review-code src/main.rs --depth=normal";
    let params = ExecCommandParams::from_input(input, &registry).await.unwrap();

    assert!(params.message.contains("Review")); // Template expanded
    assert_eq!(params.command_metadata.unwrap().name, "review-code");
}
```

**Acceptance Criteria:**
- [ ] Slash commands integrate with exec_command
- [ ] Template expansion occurs before execution
- [ ] Command metadata preserved for permissions
- [ ] Integration tests pass

---

#### Epic 1.4: Built-in Commands

**Story 1.4.1: Implement 3 Example Commands** (6 hours)

*As a user, I have 3 useful built-in commands to start with.*

**Task 1.4.1.A: /explain Command** (2 hours)
```rust
// codex-rs/core/src/commands/builtin/explain.rs
pub struct ExplainCommand;

#[async_trait]
impl Command for ExplainCommand {
    fn name(&self) -> &str { "explain" }

    fn description(&self) -> &str {
        "Explain code in beginner-friendly terms"
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Documentation
    }

    fn args_schema(&self) -> &[ArgDefinition] {
        &[
            ArgDefinition {
                name: "file".into(),
                arg_type: ArgType::String,
                required: false,
                description: "File to explain".into(),
                default: None,
            }
        ]
    }

    async fn execute(&self, context: &CommandContext) -> Result<String> {
        let template = if let Some(file) = context.args.get("file") {
            format!(
                "Please explain the code in {} in simple, beginner-friendly terms.\n\n\
                 Focus on:\n\
                 - What the code does\n\
                 - Why it's structured this way\n\
                 - Any important patterns or concepts",
                file
            )
        } else {
            "Please explain the selected code in simple, beginner-friendly terms.".into()
        };

        Ok(template)
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(Self)
    }
}
```

**Task 1.4.1.B: /review Command** (2 hours)
```rust
pub struct ReviewCommand;

#[async_trait]
impl Command for ReviewCommand {
    fn name(&self) -> &str { "review" }

    async fn execute(&self, context: &CommandContext) -> Result<String> {
        let path = context.args.get("path").map(|s| s.as_str()).unwrap_or("current changes");

        let mut prompt = format!(
            "Please perform a comprehensive code review of {}.\n\n\
             Focus on:\n\
             - Code quality and best practices\n\
             - Potential bugs and edge cases\n\
             - Performance considerations\n\
             - Security issues\n\
             - Test coverage\n",
            path
        );

        if let Some(diff) = &context.git_diff {
            prompt.push_str(&format!("\n\nGit diff:\n```\n{}\n```", diff));
        }

        Ok(prompt)
    }
}
```

**Task 1.4.1.C: /test Command** (2 hours)
```rust
pub struct TestCommand;

#[async_trait]
impl Command for TestCommand {
    fn name(&self) -> &str { "test" }

    async fn execute(&self, context: &CommandContext) -> Result<String> {
        let target = context.args.get("target").map(|s| s.as_str()).unwrap_or("this code");

        Ok(format!(
            "Please generate comprehensive tests for {}.\n\n\
             Include:\n\
             - Unit tests for core functionality\n\
             - Edge cases and error conditions\n\
             - Integration tests if applicable\n\
             - Mock setup where needed\n\n\
             Follow the testing patterns used in this project.",
            target
        ))
    }
}
```

**Acceptance Criteria:**
- [ ] All 3 commands implement Command trait
- [ ] Commands generate appropriate prompts
- [ ] Commands handle optional arguments
- [ ] Unit tests for each command

---

### Sprint 1 Quality Gates

**Exit Criteria (All Must Pass):**
- ✅ Parse Markdown command files correctly
- ✅ Command registry discovers ≥3 commands
- ✅ Template expansion with variables works
- ✅ Integration with exec_command complete
- ✅ 3 built-in commands functional
- ✅ Unit test coverage ≥80%
- ✅ Integration test: /command → execution works
- ✅ Performance: Template expansion <50ms

**Deliverables:**
- [ ] Command parser implementation
- [ ] Command registry with loader
- [ ] Template expander
- [ ] 3 built-in commands
- [ ] Integration with exec_command
- [ ] Test suite with ≥80% coverage
- [ ] Performance benchmarks

**Demo:**
- Execute `/explain src/main.rs` end-to-end
- Show template expansion working
- Display commands in registry

---

## Sprint 2: Command System Integration ✅ COMPLETE (Week 1)

### Sprint Status
**Duration**: Days 11-20 (2 weeks)
**Current Status**: ✅ Week 1 complete (Days 11-14) | Week 2 pending (Days 15-20)

### Sprint Goal
**Integrate command system into Codex execution flow with slash command parsing, exec_command hook, hot-reload, and TUI command palette.**

### Sprint Objectives (Revised from Original)

**Week 1: Command Invocation & Integration (Days 11-15)** ✅ COMPLETE
- ✅ Epic 2.1: Slash Command Parser (Days 11-12)
- ✅ Epic 2.2: exec_command Integration (Days 13-15)
  - ✅ Day 13: Command Executor
  - ✅ Day 14: exec_command Hook
  - ⏳ Day 15: Context Enhancement (pending)

**Week 2: Hot-Reload & TUI (Days 16-20)** 📋 PLANNED
- ⏳ Epic 2.3: Hot-Reload System (Days 16-17)
- ⏳ Epic 2.4: TUI Palette Integration (Days 18-20)

---

### Week 1 Progress (Days 11-15) ✅ PARTIAL COMPLETE

#### Epic 2.1: Slash Command Parser (Days 11-12) ✅ COMPLETE

**Implementation**: `core/src/commands/invocation.rs`, `core/src/commands/args.rs`

**Story 2.1.1: Command Line Parsing** ✅ COMPLETE

*As a user, I can type `/command arg1 arg2` and have it parsed correctly.*

**Tasks Completed**:
- ✅ Command line parser (`/command arg1 arg2`) - 120 LOC
- ✅ Argument extraction (positional + named) - 75 LOC
- ✅ Quoted argument handling
- ✅ Key=value parsing
- ✅ Positional → named mapping
- ✅ Argument validation
- ✅ Default value application
- ✅ 26 unit/integration tests (100% coverage)

**Implementation**:
```rust
// InvocationParser struct (120 LOC)
pub struct InvocationParser;
impl InvocationParser {
    pub fn parse(input: &str) -> Result<Invocation>;
}

// ArgumentMapper struct (75 LOC)
pub struct ArgumentMapper;
impl ArgumentMapper {
    pub fn map_args(tokens: Vec<String>, schema: &[ArgDefinition]) -> Result<HashMap<String, String>>;
}
```

**Quality Metrics**:
- ✅ 100% test coverage (exceeded 90% target)
- ✅ <1ms parse time (100x better than 10ms target)
- ✅ Clear error messages with validation

**Deliverables**:
- ✅ `invocation.rs` with InvocationParser
- ✅ `args.rs` with ArgumentMapper
- ✅ 26 comprehensive tests
- ✅ Error handling for all cases

**Report**: `docs/EPIC_2.1_COMPLETION.md`

---

#### Epic 2.2: exec_command Integration (Days 13-15) ✅ PARTIAL COMPLETE

**Day 13 - Command Executor** ✅ COMPLETE

**Implementation**: `core/src/commands/executor.rs`

**Story 2.2.1: Command Execution Pipeline** ✅ COMPLETE

*As the system, execute commands through CommandExecutor with registry lookup and template expansion.*

**Tasks Completed**:
- ✅ Command execution pipeline - 280 LOC
- ✅ Registry lookup integration
- ✅ Context building from exec state
- ✅ Template expansion
- ✅ 5 integration tests (100% coverage)

**Implementation**:
```rust
pub struct CommandExecutor {
    registry: Arc<CommandRegistry>,
}

impl CommandExecutor {
    pub async fn execute(
        &self,
        invocation: Invocation,
        context: &ExecutionContext,
    ) -> Result<String>;
}
```

**Quality Metrics**:
- ✅ 100% test coverage (exceeded 85% target)
- ✅ <10ms execution (10x better than 100ms target)

**Deliverables**:
- ✅ `executor.rs` with CommandExecutor
- ✅ ExecutionContext builder
- ✅ 5 integration tests

**Report**: `docs/EPIC_2.2_DAY13_COMPLETION.md`

---

**Day 14 - exec_command Hook** ✅ COMPLETE

**Implementation**: `core/src/commands/integration.rs`, `core/src/codex.rs`

**Story 2.2.2: Slash Command Detection and Integration** ✅ COMPLETE

*As the system, detect slash commands in user input and transparently replace with expanded prompts.*

**Tasks Completed**:
- ✅ Slash command detection in `Op::UserTurn` handler
- ✅ Command execution integration via `execute_slash_command()`
- ✅ Transparent prompt replacement
- ✅ Feature flag (`experimental_command_system_enabled`)
- ✅ Parallel registry initialization
- ✅ 13 E2E integration tests (100% passing)

**Implementation**:
```rust
// Integration layer (341 LOC)
pub fn detect_slash_command(items: &[InputItem]) -> Option<String>;
pub async fn execute_slash_command(...) -> Result<String>;
pub fn replace_with_expanded_prompt(...) -> Vec<InputItem>;

// codex.rs hook (lines 1279-1317)
Op::UserTurn { mut items, ... } => {
    if config.experimental_command_system_enabled {
        if let Some(cmd_text) = detect_slash_command(&items) {
            let expanded = execute_slash_command(...).await?;
            items = replace_with_expanded_prompt(items, expanded);
        }
    }
    // Continue with normal turn processing
}
```

**Quality Metrics**:
- ✅ 100% test coverage (13/13 tests passing)
- ✅ <10ms execution overhead
- ✅ Backward compatible (feature flag defaults false)
- ✅ Zero impact when disabled

**Deliverables**:
- ✅ `integration.rs` (341 LOC)
- ✅ `integration_tests.rs` (327 LOC, 13 tests)
- ✅ exec_command hook in codex.rs
- ✅ Feature flag in config
- ✅ SessionServices integration

**Reports**:
- `docs/EPIC_2.2_DAY14_COMPLETION.md`
- `docs/SESSION_EPIC_2_2_DAY14.md`

---

**Day 15 - Context Enhancement** ⏳ PENDING

**Planned Implementation**: Enhance `ExecutionContext` with additional context

**Planned Tasks**:
- ⏳ Add git_diff from workspace state
- ⏳ Add current_files from editor context
- ⏳ Include conversation context
- ⏳ Add environment variables
- ⏳ Write 5+ context building tests

**Status**: Not yet started

---

### Week 2: Hot-Reload & TUI (Days 16-20) 📋 PLANNED

#### Epic 2.3: Hot-Reload System (Days 16-17) ⏳ PENDING

**Story 2.3.1: File Watching** (6 hours)

*As a developer, commands reload automatically when I modify files.*

**Task 2.3.1.A: File Watcher Setup** (4 hours)
```rust
// codex-rs/core/src/commands/watcher.rs
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;

pub struct CommandWatcher {
    watcher: RecommendedWatcher,
    registry: Arc<CommandRegistry>,
}

impl CommandWatcher {
    pub async fn start(
        commands_dir: PathBuf,
        registry: Arc<CommandRegistry>,
    ) -> Result<Self> {
        let (tx, mut rx) = mpsc::channel(100);

        let mut watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        })?;

        watcher.watch(&commands_dir, RecursiveMode::NonRecursive)?;

        // Spawn reload handler
        let registry_clone = registry.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                if Self::should_reload(&event) {
                    if let Err(e) = registry_clone.reload().await {
                        eprintln!("Failed to reload commands: {}", e);
                    } else {
                        println!("Commands reloaded");
                    }
                }
            }
        });

        Ok(Self { watcher, registry })
    }

    fn should_reload(event: &notify::Event) -> bool {
        matches!(
            event.kind,
            notify::EventKind::Create(_) |
            notify::EventKind::Modify(_) |
            notify::EventKind::Remove(_)
        )
    }
}
```

**Acceptance Criteria:**
- [ ] Detect file create/modify/delete events
- [ ] Trigger registry reload on changes
- [ ] Debounce rapid changes (avoid reload spam)
- [ ] Log reload events

---

#### Epic 2.4: TUI Command Palette (Days 18-20) ⏳ PENDING

**Story 2.4.1: Command Palette UI** (8 hours)

*As a user, I can press Ctrl+K to open a command palette and select commands.*

**Planned Tasks**:
- [ ] Palette widget implementation
- [ ] Command filtering
- [ ] Keyboard navigation
- [ ] Autocomplete system
- [ ] Fuzzy matching
- [ ] Argument suggestions
- [ ] Keyboard shortcut (Ctrl+P)

**Status**: Not yet started

---

### Sprint 2 Quality Gates (Updated)

**Exit Criteria (All Must Pass)**:

**Week 1 (Days 11-14)**: ✅ ACHIEVED
- ✅ Slash command parsing <10ms (achieved <1ms)
- ✅ exec_command integration complete
- ✅ 100% test coverage for integration (13/13 tests)
- ✅ Backward compatible (feature flag defaults false)
- ✅ Zero impact when disabled

**Week 2 (Days 15-20)**: ⏳ PENDING
- [ ] Context enhancement complete (Day 15)
- [ ] Hot-reload works reliably (Days 16-17)
- [ ] TUI palette responsive (<16ms) (Days 18-20)
- [ ] ≥85% test coverage maintained
- [ ] All performance targets met
- [ ] No memory leaks
- [ ] Zero critical security issues

**Deliverables Summary**:

**Completed** (Days 11-14):
- ✅ `invocation.rs` - InvocationParser (120 LOC)
- ✅ `args.rs` - ArgumentMapper (75 LOC)
- ✅ `executor.rs` - CommandExecutor (280 LOC)
- ✅ `integration.rs` - Integration layer (341 LOC)
- ✅ `integration_tests.rs` - E2E tests (327 LOC, 13 tests)
- ✅ `codex.rs` - exec_command hook integration
- ✅ `config.rs` - Feature flag implementation
- ✅ `service.rs` - SessionServices updates

**Pending** (Days 15-20):
- [ ] Context enhancement (Day 15)
- [ ] Hot-reload system (Days 16-17)
- [ ] TUI command palette (Days 18-20)

**Documentation**:
- ✅ `docs/EPIC_2.1_COMPLETION.md`
- ✅ `docs/EPIC_2.2_DAY13_COMPLETION.md`
- ✅ `docs/EPIC_2.2_DAY14_COMPLETION.md`
- ✅ `docs/SESSION_EPIC_2_2_DAY14.md`

**Next Steps**: Epic 2.2 Day 15 - Context Enhancement for richer command execution! 🚀

---

## Sprint 3: Agent Integration & Validation (2 Weeks) 📋 PLANNED

### Sprint Goal
**Integrate agent system with executor, build agent router for context-based selection, and validate full command-agent workflow.**

### Sprint Objectives
- ⏳ Agent router with context analysis
- ⏳ Integration with executor/runner
- ⏳ Agent results streamed to TUI
- ⏳ Command-agent binding works
- ⏳ Full E2E workflow validated

**Status**: Not yet started - Sprint 2 must complete first

---

### Week 1: Agent Router & Integration (Future)

#### Epic 3.1: Agent Router

**Story 3.1.1: Core Agent Architecture** (8 hours)

*As a developer, I have a clear Agent trait to implement specialized agents.*

**Planned Tasks**:
- [ ] Agent trait definition
- [ ] Permission model
- [ ] Activation scoring system
- [ ] Task context builder
- [ ] Agent toolkit interface

**Status**: Not yet started

---

#### Epic 3.2: Executor Integration (Future)

**Planned Tasks**:
- [ ] Agent runner implementation
- [ ] Task type for agents
- [ ] Modify Runner for agent execution path
- [ ] Agent result to Response conversion
- [ ] Integration tests

**Status**: Not yet started

---

### Week 2: TUI Agent View & E2E Validation (Future)

#### Epic 3.3: TUI Agent Visualization (Future)

**Task 2.4.1.A: Palette Widget** (4 hours)
```rust
// codex-rs/tui/src/commands_palette.rs
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

pub struct CommandPalette {
    commands: Vec<CommandInfo>,
    filter: String,
    selected: usize,
    visible: bool,
}

impl CommandPalette {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            filter: String::new(),
            selected: 0,
            visible: false,
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        if !self.visible {
            self.filter.clear();
            self.selected = 0;
        }
    }

    pub fn update_commands(&mut self, commands: Vec<CommandInfo>) {
        self.commands = commands;
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> Option<CommandAction> {
        match key.code {
            KeyCode::Esc => {
                self.toggle();
                None
            }
            KeyCode::Enter => {
                let filtered = self.filtered_commands();
                if let Some(cmd) = filtered.get(self.selected) {
                    self.toggle();
                    Some(CommandAction::Execute(cmd.name.clone()))
                } else {
                    None
                }
            }
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                None
            }
            KeyCode::Down => {
                let filtered = self.filtered_commands();
                if self.selected < filtered.len().saturating_sub(1) {
                    self.selected += 1;
                }
                None
            }
            KeyCode::Char(c) => {
                self.filter.push(c);
                self.selected = 0; // Reset selection
                None
            }
            KeyCode::Backspace => {
                self.filter.pop();
                self.selected = 0;
                None
            }
            _ => None,
        }
    }

    fn filtered_commands(&self) -> Vec<&CommandInfo> {
        if self.filter.is_empty() {
            self.commands.iter().collect()
        } else {
            self.commands
                .iter()
                .filter(|cmd| {
                    cmd.name.contains(&self.filter) ||
                    cmd.description.contains(&self.filter)
                })
                .collect()
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if !self.visible {
            return;
        }

        // Center the palette
        let popup_area = centered_rect(60, 50, area);

        // Clear background
        Clear.render(popup_area, buf);

        // Render block
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Commands (Ctrl+K to close) ")
            .border_style(Style::default().cyan());

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        // Split into filter and list
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(inner);

        // Render filter input
        let filter_text = format!("> {}", self.filter);
        Paragraph::new(filter_text)
            .style(Style::default().bold())
            .render(chunks[0], buf);

        // Render command list
        let filtered = self.filtered_commands();
        let items: Vec<ListItem> = filtered
            .iter()
            .enumerate()
            .map(|(i, cmd)| {
                let style = if i == self.selected {
                    Style::default().bg(Color::DarkGray).bold()
                } else {
                    Style::default()
                };

                let content = format!(
                    "{:<20} {}",
                    cmd.name.cyan(),
                    cmd.description.dim()
                );

                ListItem::new(content).style(style)
            })
            .collect();

        List::new(items).render(chunks[1], buf);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub enum CommandAction {
    Execute(String),
}
```

**Task 2.4.1.B: Integration with TUI** (2 hours)
```rust
// codex-rs/tui/src/app.rs (modifications)
impl App {
    pub async fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        // Check for Ctrl+K
        if key.code == KeyCode::Char('k') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.command_palette.toggle();

            // Load commands if palette opened
            if self.command_palette.visible {
                let commands = self.registry.list().await;
                self.command_palette.update_commands(commands);
            }

            return Ok(());
        }

        // If palette is visible, route input there
        if self.command_palette.visible {
            if let Some(action) = self.command_palette.handle_input(key) {
                match action {
                    CommandAction::Execute(cmd_name) => {
                        self.execute_command(&cmd_name).await?;
                    }
                }
            }
            return Ok(());
        }

        // Normal key handling...
        Ok(())
    }

    async fn execute_command(&mut self, name: &str) -> Result<()> {
        // Build input
        let input = format!("/{}", name);

        // Execute through standard flow
        self.submit_message(&input).await
    }
}
```

**Task 2.4.1.C: Snapshot Tests** (2 hours)
```rust
#[test]
fn test_palette_rendering() {
    let mut palette = CommandPalette::new();
    palette.update_commands(vec![
        CommandInfo { name: "review".into(), description: "Code review".into(), .. },
        CommandInfo { name: "explain".into(), description: "Explain code".into(), .. },
    ]);
    palette.visible = true;

    let mut buf = Buffer::empty(Rect::new(0, 0, 80, 24));
    palette.render(Rect::new(0, 0, 80, 24), &mut buf);

    insta::assert_snapshot!(buf);
}
```

**Acceptance Criteria:**
- [ ] Ctrl+K opens/closes palette
- [ ] Filter updates results in real-time
- [ ] Arrow keys navigate selection
- [ ] Enter executes selected command
- [ ] Fuzzy search works on name + description

---

#### Epic 2.5: Code Reviewer Agent (Track 2)

**Story 2.5.1: Implement code-reviewer Agent** (10 hours)

*As a user, the code-reviewer agent analyzes my code for quality issues.*

**Task 2.5.1.A: Agent Toolkit** (4 hours)
```rust
// codex-rs/core/src/agents/toolkit.rs
pub struct AgentToolkit {
    agent_id: AgentId,
    permissions: AgentPermissions,
    execpolicy: Arc<ExecPolicy>,
    workspace_root: PathBuf,
}

impl AgentToolkit {
    pub async fn read_file(&self, path: &Path) -> Result<String> {
        // Validate permissions
        if !self.permissions.can_read_file(path) {
            return Err(AgentError::PermissionDenied(
                format!("Cannot read {:?}", path)
            ));
        }

        // Read file
        tokio::fs::read_to_string(path)
            .await
            .map_err(|e| AgentError::FileReadError(e.to_string()))
    }

    pub async fn execute_command(&self, cmd: &str, args: &[String]) -> Result<CommandOutput> {
        // Validate shell execution permission
        if !self.permissions.shell_execution {
            return Err(AgentError::PermissionDenied("Shell execution not allowed".into()));
        }

        // Validate through execpolicy
        let validation = validate_agent_command(&self.permissions, cmd, args).await?;

        match validation {
            ValidationResult::Allowed => {
                // Execute command
                self.execute_shell(cmd, args).await
            }
            ValidationResult::Forbidden(reason) => {
                Err(AgentError::CommandForbidden(reason))
            }
            ValidationResult::RequiresApproval => {
                // For now, deny. Later: prompt user
                Err(AgentError::ApprovalRequired)
            }
        }
    }

    async fn execute_shell(&self, cmd: &str, args: &[String]) -> Result<CommandOutput> {
        use tokio::process::Command;

        let output = Command::new(cmd)
            .args(args)
            .current_dir(&self.workspace_root)
            .output()
            .await?;

        Ok(CommandOutput {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
        })
    }
}
```

**Task 2.5.1.B: code-reviewer Implementation** (4 hours)
```rust
// codex-rs/core/src/agents/builtin/code_reviewer.rs
pub struct CodeReviewerAgent {
    permissions: AgentPermissions,
}

impl CodeReviewerAgent {
    pub fn new() -> Self {
        Self {
            permissions: AgentPermissions {
                file_access: FileAccessPolicy::ReadOnly,
                shell_execution: true,
                network_access: false,
                allowed_tools: vec!["grep".into(), "rg".into(), "git".into()],
                max_iterations: 5,
                can_delegate: false,
            },
        }
    }

    async fn analyze_code(&self, content: &str, path: &Path) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        // Check for common issues
        if content.contains("unwrap()") {
            findings.push(Finding {
                severity: Severity::Warning,
                category: "Error Handling",
                message: "Use of unwrap() - consider proper error handling".into(),
                location: Some(path.to_path_buf()),
            });
        }

        if content.contains("clone()") && content.matches("clone()").count() > 5 {
            findings.push(Finding {
                severity: Severity::Info,
                category: "Performance",
                message: "Frequent use of clone() - consider borrowing".into(),
                location: Some(path.to_path_buf()),
            });
        }

        // More analysis...

        Ok(findings)
    }
}

#[async_trait]
impl Agent for CodeReviewerAgent {
    fn id(&self) -> AgentId {
        AgentId::from("code-reviewer")
    }

    fn name(&self) -> &str {
        "Code Reviewer"
    }

    fn description(&self) -> &str {
        "General code quality and best practices analysis"
    }

    fn can_handle(&self, context: &TaskContext) -> ActivationScore {
        let mut score = 0.0;

        // Check for review keywords
        let keywords = ["review", "analyze", "check", "quality"];
        if keywords.iter().any(|k| context.user_intent.contains(k)) {
            score += 0.5;
        }

        // Check for code files
        if context.file_paths.iter().any(|p| is_code_file(p)) {
            score += 0.3;
        }

        // Check for git context
        if context.git_context.is_some() {
            score += 0.2;
        }

        ActivationScore(score.min(1.0))
    }

    async fn execute(&self, task: Task, toolkit: &AgentToolkit) -> AgentResult {
        let mut all_findings = Vec::new();

        // Analyze each file
        for path in &task.context.file_paths {
            let content = toolkit.read_file(path).await?;
            let findings = self.analyze_code(&content, path).await?;
            all_findings.extend(findings);
        }

        AgentResult::CodeReview {
            findings: all_findings,
        }
    }

    fn permissions(&self) -> &AgentPermissions {
        &self.permissions
    }

    fn system_prompt(&self) -> &str {
        "You are an expert code reviewer focused on code quality, maintainability, \
         and best practices. Analyze code for:\n\
         - Code structure and organization\n\
         - Naming conventions and clarity\n\
         - Potential bugs and edge cases\n\
         - Performance considerations\n\
         - Test coverage gaps\n\
         Provide constructive, actionable feedback."
    }
}

fn is_code_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("rs") | Some("js") | Some("ts") | Some("py") | Some("go")
    )
}
```

**Task 2.5.1.C: Integration Tests** (2 hours)
```rust
#[tokio::test]
async fn test_code_reviewer_agent() {
    let agent = CodeReviewerAgent::new();
    let toolkit = AgentToolkit::new(
        agent.id(),
        agent.permissions().clone(),
        test_workspace(),
    );

    let context = TaskContext {
        file_paths: vec![PathBuf::from("test_files/sample.rs")],
        user_intent: "review this code".into(),
        ..Default::default()
    };

    let task = Task { context, additional_instructions: None };
    let result = agent.execute(task, &toolkit).await.unwrap();

    match result {
        AgentResult::CodeReview { findings } => {
            assert!(!findings.is_empty());
        }
        _ => panic!("Wrong result type"),
    }
}
```

**Planned Tasks**:
- [ ] Agent status display widget
- [ ] Streaming updates from agents
- [ ] Progress bar rendering
- [ ] Findings visualization
- [ ] Integration with main TUI

**Status**: Not yet started

---

#### Epic 3.4: Command-Agent Binding (Future)

**Planned Tasks**:
- [ ] Add agent binding to command metadata
- [ ] Example commands with agent binding
- [ ] Integration tests

**Status**: Not yet started

---

#### Epic 3.5: End-to-End Validation (Future)

**Planned Tasks**:
- [ ] Complete workflow E2E tests
- [ ] Performance benchmarks
- [ ] Edge case validation

**Status**: Not yet started

---

### Sprint 3 Quality Gates (Future)

**Exit Criteria** (All Must Pass - when Sprint 3 begins):
- [ ] Agent router selects correct agent (>80% accuracy in tests)
- [ ] Agent execution integrated with executor
- [ ] TUI displays agent status and findings
- [ ] Command-agent binding works
- [ ] E2E workflow test passes
- [ ] Performance: Agent activation <100ms
- [ ] Performance: Full workflow <500ms
- [ ] No regressions from Sprints 1-2

**Deliverables** (Planned):
- [ ] Agent router implementation
- [ ] AgentRunner in executor
- [ ] TUI agent view widget
- [ ] Command-agent binding system
- [ ] E2E test suite
- [ ] Performance benchmarks
- [ ] Updated documentation

---

## Sprint 4-5: Core Agents Implementation (4 Weeks) 📋 PLANNED

### Sprint Goal
**Implement 5 specialized agents (security-analyst, refactoring-expert, test-engineer, performance-optimizer, rust-expert) with comprehensive testing and validation.**

**Status**: Not yet started - Sprint 3 must complete first

*Detailed tasks would be planned when Sprint 3 completes*

---

## Sprint 6-7: Multi-Agent Orchestration (4 Weeks) 📋 PLANNED

### Sprint Goal
**Enable parallel agent execution, inter-agent communication, task delegation, and MCP integration for advanced workflows.**

**Status**: Not yet started - Sprint 4-5 must complete first

*Detailed tasks would be planned when Sprint 4-5 completes*

---

## Risk Management Strategy

### Current Risk Status (Updated for Sprint 2 Week 1 Completion)

**Mitigated Risks**:
- ✅ Template expansion performance - Sprint 0 spike confirmed <10ms (exceeded <50ms target)
- ✅ Permission model integration - Sprint 0 spike validated execpolicy compatibility

**Active Risks** (Sprint 2 Week 2):
- 🟡 Hot-reload resource usage - Will validate in Days 16-17 with Valgrind tests
- 🟡 TUI performance for palette - Will benchmark in Days 18-20 (target <16ms)
- 🟢 Backward compatibility - Feature flag pattern working well, zero regressions

**Future Risks** (Sprint 3+):
- Agent auto-activation accuracy - Will tune threshold in Sprint 3
- Multi-agent coordination complexity - Starts in Sprint 6-7

---

## Risk Management Strategy (Original from Spec)

### High-Risk Areas & Mitigation

| Risk | Impact | Probability | Mitigation | Contingency |
|------|--------|-------------|------------|-------------|
| **Permission model incompatible with execpolicy** | High | Medium | Sprint 0 spike validates integration | Extend execpolicy to support agent context |
| **Template expansion performance** | Medium | Low | Sprint 0 benchmark, caching | Use simpler template engine |
| **Agent auto-activation low accuracy** | Medium | Medium | Tune threshold, collect metrics | Fall back to explicit selection |
| **Multi-agent coordination complexity** | High | Medium | Start simple, iterate | Sequential execution fallback |
| **TUI integration breaking changes** | Medium | Low | Feature flags, gradual rollout | Separate agent TUI mode |

### Risk Response Plan

**Weekly Risk Review:**
- Identify new risks
- Update mitigation strategies
- Adjust sprint scope if needed

**Risk Escalation:**
- Technical blockers → Team lead
- Architecture changes → Architecture review
- Timeline impacts → Stakeholder notification

---

## Quality Assurance & Testing

### Testing Strategy

**Test Pyramid:**
```
         ┌─────────┐
         │   E2E   │  (10%)
         └─────────┘
      ┌──────────────┐
      │ Integration  │  (30%)
      └──────────────┘
   ┌──────────────────────┐
   │     Unit Tests       │  (60%)
   └──────────────────────┘
```

### Coverage Targets

- Unit tests: ≥80% code coverage
- Integration tests: All critical paths
- E2E tests: User workflows
- Performance tests: <100ms agent activation, <500ms full workflow

### Test Automation

**CI/CD Pipeline:**
```yaml
stages:
  - lint
  - test
  - benchmark
  - integration

lint:
  - cargo fmt --check
  - cargo clippy --all-targets

test:
  - cargo nextest run --no-fail-fast
  - cargo insta review

benchmark:
  - Run performance benchmarks
  - Fail if >10% regression

integration:
  - Run E2E test suite
  - Validate quality gates
```

---

## Team Coordination & Ceremonies

### Sprint Ceremonies

**Sprint Planning (Day 1, 2 hours):**
- Review backlog
- Size stories (Fibonacci: 1, 2, 3, 5, 8)
- Commit to sprint goal
- Assign initial tasks

**Daily Standup (15 min):**
- What did I complete?
- What am I working on?
- Any blockers?

**Mid-Sprint Sync (Day 5, 30 min):**
- Progress check
- Adjust scope if needed
- Technical discussions

**Sprint Review (Last Day, 1 hour):**
- Demo completed features
- Gather feedback
- Update roadmap

**Sprint Retro (Last Day, 30 min):**
- What went well?
- What to improve?
- Action items

### Definition of Done

**Story is "Done" when:**
- [ ] Code implemented and reviewed
- [ ] Unit tests written (≥80% coverage)
- [ ] Integration tests pass
- [ ] Documentation updated
- [ ] No clippy warnings
- [ ] Benchmarks meet targets
- [ ] Demo-able to team

---

## Success Metrics & KPIs

### Sprint-Level Metrics

**Sprint 0:**
- Architecture approved: ✅
- All spikes complete: ✅
- Risk score reduced: ≥50%

**Sprint 1:**
- Commands parsed: ≥3
- Template expansion: <50ms
- Test coverage: ≥80%

**Sprint 2:**
- Hot-reload latency: <1s
- TUI palette functional: ✅
- Agent prototype working: ✅

**Sprint 3:**
- Router accuracy: ≥80%
- E2E workflow: <500ms
- Agent visualization: ✅

**Sprint 4-5:**
- 5 agents operational: ✅
- Agent accuracy: ≥80% each
- Benchmark: <100ms activation

**Sprint 6-7:**
- Multi-agent coordination: ✅
- Parallel execution: ≥2 agents
- Performance: 3 agents <2s

### Overall Project KPIs

**Functionality:**
- Commands: ≥10 (5 built-in + 5 user examples)
- Agents: 5 specialized + extensible framework
- Integration: Seamless with existing Codex

**Performance:**
- Command execution: <100ms
- Agent activation: <100ms
- Multi-agent workflow: <2s

**Quality:**
- Test coverage: ≥80%
- Zero critical bugs
- Documentation complete

**User Experience:**
- TUI responsive (<16ms frame)
- Keyboard shortcuts intuitive
- Error messages helpful

---

## Conclusion

This agile implementation workflow provides a **comprehensive, executable plan** for building the Command & Agent System over **14 weeks across 7 sprints**.

**Key Success Factors:**
- ✅ **Iterative Development**: Incremental value delivery every 2 weeks
- ✅ **Parallel Execution**: Multiple independent workstreams maximize efficiency
- ✅ **Risk Mitigation**: Early spikes and continuous validation reduce unknowns
- ✅ **Quality First**: Quality gates ensure high standards throughout
- ✅ **Clear Ownership**: Well-defined tasks with acceptance criteria
- ✅ **Continuous Integration**: Automated testing and benchmarking

**Next Steps:**
1. **Review & Approve**: Team reviews this workflow and specification
2. **Sprint 0 Start**: Begin foundation and risk mitigation
3. **Iterate & Adapt**: Adjust based on learnings and feedback
4. **Deliver Value**: Ship working features every sprint

The workflow is **ready for execution** - let's build this! 🚀
