# Epic 3.1: Agent System Integration - Implementation Workflow

**Status**: 📋 **READY TO IMPLEMENT**
**Sprint**: Sprint 3 (Days 21-25)
**Duration**: 5 days
**Prerequisites**: ✅ Sprint 2 Complete (Command System, TUI Palette)

---

## Executive Summary

Epic 3.1 integrates the existing agent framework (router, toolkit, permissions) with the command system to enable agent invocation via slash commands. This epic focuses on the **coordination layer** between commands and agents.

### Key Integration Points

1. **Command → Agent Bridge**: Commands can invoke agents
2. **Agent Discovery**: Command palette shows available agents
3. **Context Passing**: Task context flows from command execution
4. **Result Handling**: Agent results integrated into conversation

### Current State Analysis

**Existing Infrastructure** (from Sprint 2):
- ✅ Command system with registry and execution
- ✅ Slash command parsing (`/command args`)
- ✅ Template expansion with Handlebars
- ✅ TUI command palette with fuzzy search
- ✅ Hot-reload for command files

**Existing Agent Framework** (from earlier work):
- ✅ Agent trait with `can_handle()` and `execute()`
- ✅ AgentRouter for context-based selection
- ✅ AgentToolkit for file/shell operations
- ✅ AgentPermissions model
- ✅ Core types: AgentId, ActivationScore, TaskContext, Task, AgentResult

**What's Missing** (Epic 3.1 scope):
- ❌ Command-to-Agent invocation mechanism
- ❌ Agent command definitions (`.md` files)
- ❌ Agent result formatting for conversation
- ❌ Agent discovery in command palette
- ❌ Context building from command execution state

---

## Implementation Approach

### Philosophy: **Minimal Integration, Maximum Reuse**

Rather than rebuilding agent infrastructure, Epic 3.1 creates a **thin coordination layer** that:
- Reuses existing Command execution flow
- Delegates to existing AgentRouter for selection
- Passes TaskContext from command arguments
- Returns AgentResult as formatted response

### Architecture Pattern

```
User Input: "/review src/main.rs"
     ↓
Slash Parser (existing) → CommandInvocation
     ↓
Command Executor (existing) → Looks up "review" in registry
     ↓
[NEW] Agent Command Handler → Detects agent: true in frontmatter
     ↓
[NEW] Context Builder → Creates TaskContext from args + git state
     ↓
AgentRouter (existing) → Selects best agent via can_handle()
     ↓
Agent.execute() (existing) → Runs with AgentToolkit
     ↓
[NEW] Result Formatter → Converts AgentResult to markdown
     ↓
Command Response → Returns to conversation
```

---

## Day-by-Day Implementation Plan

### **Day 21: Agent Command Infrastructure**

**Goal**: Enable commands to declare themselves as agent-backed

#### Morning: Command Metadata Enhancement

**Task 1.1**: Extend Command struct for agent metadata
**File**: `core/src/commands/mod.rs`
**Changes**:
```rust
pub struct Command {
    // ... existing fields
    pub agent_backed: bool,
    pub agent_id: Option<String>,
    pub activation_hints: Vec<String>, // Keywords for context matching
}
```

**Task 1.2**: Update YAML frontmatter parsing
**File**: `core/src/commands/frontmatter.rs`
**Changes**:
```yaml
---
name: review
description: Code review with agent
agent: true
agent_id: code-review
activation_hints: ["review", "quality", "bugs"]
---
```

**Task 1.3**: Add agent field validation
**Tests**: 3 unit tests for frontmatter parsing

#### Afternoon: Agent Command Definitions

**Task 1.4**: Create agent command templates
**Files**: `commands/agents/review.md`, `commands/agents/refactor.md`
**Template**:
```markdown
---
name: review
description: Perform code review with AI agent
agent: true
agent_id: code-review
activation_hints: ["review", "quality", "feedback"]
parameters:
  - name: files
    type: path[]
    required: true
    description: Files to review
  - name: focus
    type: string
    description: Review focus (security, performance, readability)
---

# Code Review Agent

This command invokes the code review agent to analyze {{files}}.
{{#if focus}}Focus: {{focus}}{{/if}}
```

**Task 1.5**: Write integration tests
**Tests**: 5 tests for agent command loading

**Deliverables**:
- [ ] Command struct with agent metadata
- [ ] Frontmatter parser supports `agent:` field
- [ ] 2-3 example agent command files
- [ ] 8 tests passing
- [ ] Documentation updated

---

### **Day 22: Context Building & Routing Integration**

**Goal**: Build TaskContext from command execution and route to agents

#### Morning: TaskContext Builder

**Task 2.1**: Implement context builder
**File**: `core/src/commands/agent_context.rs` (new)
**Interface**:
```rust
pub struct AgentContextBuilder {
    exec_state: ExecutionState,
    git_context: Option<GitContext>,
}

impl AgentContextBuilder {
    /// Creates TaskContext from command execution state
    pub fn build_context(
        &self,
        command: &Command,
        args: &HashMap<String, String>,
    ) -> anyhow::Result<TaskContext> {
        // Extract file paths from args
        // Load git diff if applicable
        // Build user_intent from command template
        // Return TaskContext
    }
}
```

**Task 2.2**: File path extraction
**Logic**: Parse `files` parameter, resolve paths, validate existence

**Task 2.3**: Git context integration
**File**: `core/src/commands/git_utils.rs` (enhance existing)
**Add**: Git diff extraction for changed files

**Tests**: 6 unit tests for context building

#### Afternoon: Agent Routing Integration

**Task 2.4**: Implement agent command handler
**File**: `core/src/commands/agent_executor.rs` (new)
**Interface**:
```rust
pub struct AgentCommandExecutor {
    router: Arc<AgentRouter>,
    toolkit: Arc<AgentToolkit>,
}

impl AgentCommandExecutor {
    /// Executes an agent-backed command
    pub async fn execute(
        &self,
        command: &Command,
        context: TaskContext,
    ) -> anyhow::Result<AgentResult> {
        // Use router to select agent
        // Create Task from context
        // Execute agent with toolkit
        // Return result
    }
}
```

**Task 2.5**: Integrate with command executor
**File**: `core/src/commands/executor.rs`
**Changes**:
```rust
// In CommandExecutor::execute_internal()
if command.agent_backed {
    let context = self.context_builder.build_context(command, &args)?;
    let result = self.agent_executor.execute(command, context).await?;
    return Ok(self.format_agent_result(result));
}
```

**Task 2.6**: Write integration tests
**Tests**: 8 tests for routing and execution flow

**Deliverables**:
- [ ] AgentContextBuilder implementation
- [ ] AgentCommandExecutor implementation
- [ ] Integration with CommandExecutor
- [ ] 14 tests passing
- [ ] Git context extraction working

---

### **Day 23: Result Formatting & Response Handling**

**Goal**: Convert AgentResult to formatted conversation responses

#### Morning: Result Formatter

**Task 3.1**: Implement result formatter
**File**: `core/src/commands/agent_formatter.rs` (new)
**Interface**:
```rust
pub struct AgentResultFormatter;

impl AgentResultFormatter {
    /// Converts AgentResult to markdown response
    pub fn format(result: &AgentResult) -> String {
        match result {
            AgentResult::CodeReview { findings } => {
                Self::format_code_review(findings)
            }
            AgentResult::Analysis { summary, details } => {
                Self::format_analysis(summary, details)
            }
            AgentResult::Suggestions { items } => {
                Self::format_suggestions(items)
            }
        }
    }

    fn format_code_review(findings: &[CodeReviewFinding]) -> String {
        // Render as markdown with severity icons
        // Group by file and severity
        // Include line numbers
    }
}
```

**Task 3.2**: Implement formatting for each result type
**Formats**:
- CodeReview: Grouped by severity, with file/line references
- Analysis: Summary + expandable details
- Suggestions: Numbered list with code blocks

**Task 3.3**: Add markdown generation helpers
**File**: `core/src/commands/markdown.rs` (new)
**Helpers**: Code block, table, list, emphasis formatting

**Tests**: 10 tests for formatter (one per result variant + edge cases)

#### Afternoon: Response Integration

**Task 3.4**: Integrate formatter with executor
**File**: `core/src/commands/executor.rs`
**Changes**:
```rust
fn format_agent_result(&self, result: AgentResult) -> String {
    AgentResultFormatter::format(&result)
}
```

**Task 3.5**: Add response metadata
**Enhancement**: Include agent name, execution time, token usage

**Task 3.6**: Write end-to-end tests
**Tests**: 5 E2E tests from command input → formatted output

**Deliverables**:
- [ ] AgentResultFormatter with 3 format methods
- [ ] Markdown generation helpers
- [ ] Response metadata included
- [ ] 15 tests passing
- [ ] Example outputs verified

---

### **Day 24: Agent Discovery & Command Palette Integration**

**Goal**: Show agent commands in TUI palette with metadata

#### Morning: Agent Command Loading

**Task 4.1**: Enhance command palette to show agent status
**File**: `tui/src/command_palette.rs`
**Changes**:
```rust
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub category: String,
    pub is_agent: bool,        // NEW
    pub agent_icon: String,    // NEW: "🤖" or similar
}
```

**Task 4.2**: Update command loading to include agent metadata
**File**: `tui/src/app.rs`
**Changes**:
```rust
fn load_commands_into_palette(&mut self) {
    // TODO from Epic 2.4: Connect to registry
    let commands = self.registry.list_commands()
        .iter()
        .map(|cmd| CommandInfo {
            name: cmd.name.clone(),
            description: cmd.description.clone(),
            category: if cmd.agent_backed { "agents" } else { "commands" },
            is_agent: cmd.agent_backed,
            agent_icon: if cmd.agent_backed { "🤖" } else { "" },
        })
        .collect();
    self.command_palette.load_commands(commands);
}
```

**Task 4.3**: Update palette rendering for agent icons
**File**: `tui/src/command_palette.rs:284`
**Changes**: Include `agent_icon` in formatted output

**Tests**: 4 tests for agent command display

#### Afternoon: Registry Integration

**Task 4.4**: Connect palette to actual CommandRegistry
**File**: `tui/src/app.rs`
**Changes**: Replace placeholder with `Session → CommandRegistry` access

**Task 4.5**: Add registry access to App initialization
**Changes**: Pass `Arc<CommandRegistry>` from Session

**Task 4.6**: Test with real agent command files
**Verification**: Load agent commands, verify in palette

**Deliverables**:
- [ ] CommandInfo enhanced with agent metadata
- [ ] Palette shows agent icons
- [ ] Registry integration complete
- [ ] 4 tests passing
- [ ] Agent commands visible in TUI

---

### **Day 25: Testing, Documentation & Polish**

**Goal**: Comprehensive testing, documentation, and quality validation

#### Morning: Integration Testing

**Task 5.1**: Write comprehensive integration tests
**File**: `core/tests/agent_integration_tests.rs` (new)
**Coverage**:
- Command loading with agent metadata
- Context building from various inputs
- Agent routing and selection
- Result formatting
- End-to-end flow: input → agent → output

**Task 5.2**: Performance testing
**Benchmarks**:
- Context building: <50ms
- Agent routing: <100ms
- Result formatting: <20ms
- Total E2E: <500ms

**Task 5.3**: Error handling validation
**Tests**: Malformed frontmatter, missing agents, permission errors

**Tests**: 15 integration + performance tests

#### Afternoon: Documentation & Polish

**Task 5.4**: Update user documentation
**Files**:
- `docs/AGENT_COMMANDS.md` - How to create agent commands
- `docs/AGENT_INTEGRATION.md` - Architecture documentation
- `README.md` - Update with agent features

**Task 5.5**: Update command examples
**Files**: Add examples in `commands/examples/`

**Task 5.6**: Code review and cleanup
**Actions**:
- Remove TODOs
- Add inline documentation
- Clippy cleanup
- Format with rustfmt

**Task 5.7**: Create completion report
**File**: `docs/EPIC_3.1_COMPLETION.md`

**Deliverables**:
- [ ] 15 integration tests passing
- [ ] Performance benchmarks met
- [ ] User documentation complete
- [ ] Example commands created
- [ ] Code review clean
- [ ] Completion report ready

---

## File Structure Summary

### New Files Created

**Core**:
- `core/src/commands/agent_context.rs` - TaskContext builder
- `core/src/commands/agent_executor.rs` - Agent command executor
- `core/src/commands/agent_formatter.rs` - Result formatter
- `core/src/commands/markdown.rs` - Markdown generation helpers
- `core/tests/agent_integration_tests.rs` - Integration tests

**Command Definitions**:
- `commands/agents/review.md` - Code review agent command
- `commands/agents/refactor.md` - Refactoring agent command
- `commands/agents/security.md` - Security analysis agent command

**Documentation**:
- `docs/AGENT_COMMANDS.md` - User guide for agent commands
- `docs/AGENT_INTEGRATION.md` - Architecture documentation
- `docs/EPIC_3.1_COMPLETION.md` - Completion report

### Modified Files

**Core**:
- `core/src/commands/mod.rs` - Add agent metadata to Command struct
- `core/src/commands/frontmatter.rs` - Parse agent frontmatter
- `core/src/commands/executor.rs` - Integrate agent execution
- `core/src/commands/git_utils.rs` - Enhanced git context

**TUI**:
- `tui/src/command_palette.rs` - Add agent metadata to CommandInfo
- `tui/src/app.rs` - Connect to registry, load agent commands

---

## Acceptance Criteria

### Functional Requirements

- [ ] **FR-1**: Commands can declare `agent: true` in frontmatter
- [ ] **FR-2**: Agent commands build TaskContext from args
- [ ] **FR-3**: AgentRouter selects appropriate agent
- [ ] **FR-4**: Agents execute with AgentToolkit
- [ ] **FR-5**: Results formatted as markdown
- [ ] **FR-6**: Agent commands visible in TUI palette
- [ ] **FR-7**: Registry integration complete

### Non-Functional Requirements

- [ ] **NFR-1**: Context building <50ms
- [ ] **NFR-2**: Agent routing <100ms
- [ ] **NFR-3**: Result formatting <20ms
- [ ] **NFR-4**: E2E execution <500ms
- [ ] **NFR-5**: ≥85% test coverage
- [ ] **NFR-6**: Zero memory leaks
- [ ] **NFR-7**: Backward compatible

### Quality Gates

- [ ] **QG-1**: All tests passing (≥50 new tests)
- [ ] **QG-2**: Performance targets met
- [ ] **QG-3**: Zero clippy warnings
- [ ] **QG-4**: Documentation complete
- [ ] **QG-5**: Example commands working
- [ ] **QG-6**: TUI integration functional
- [ ] **QG-7**: No regressions in existing features

---

## Risk Mitigation

### Risk 1: AgentRouter API Complexity

**Risk**: Existing AgentRouter may not support all needed features
**Mitigation**:
- Review AgentRouter API on Day 22 morning
- Extend if needed (keep changes minimal)
- Fallback: Direct agent instantiation for MVP

### Risk 2: Context Building Edge Cases

**Risk**: Complex argument types may not map cleanly to TaskContext
**Mitigation**:
- Start with simple file path arguments
- Add complexity incrementally
- Document unsupported argument types

### Risk 3: Registry Access from TUI

**Risk**: TUI → Core registry access may have architectural constraints
**Mitigation**:
- Design Session bridge early (Day 24)
- Use message passing if direct access blocked
- Fallback: Continue using placeholder for MVP

### Risk 4: Performance Overhead

**Risk**: Agent routing may add latency to command execution
**Mitigation**:
- Benchmark early (Day 22 afternoon)
- Optimize hot paths (context building)
- Cache agent selection if needed

---

## Dependencies & Prerequisites

### External Dependencies

- ✅ Command system (Epic 2.1, 2.2, 2.3)
- ✅ TUI palette (Epic 2.4)
- ✅ Agent framework (existing)
- ⏳ Session → Registry bridge (design in Epic 3.1)

### Internal Dependencies

**Day-to-Day**:
- Day 22 depends on Day 21 (command metadata)
- Day 23 depends on Day 22 (routing integration)
- Day 24 depends on Day 21-23 (end-to-end flow)
- Day 25 depends on all previous (testing)

### No Blockers

All dependencies are internal or already complete. Epic 3.1 can start immediately.

---

## Success Metrics

### Code Metrics

| Metric | Target | Verification |
|--------|--------|--------------|
| New LOC | ~800-1000 | File size analysis |
| Test LOC | ~600-800 | Test count |
| Test Coverage | ≥85% | cargo tarpaulin |
| Cyclomatic Complexity | <10 per function | cargo clippy |

### Performance Metrics

| Operation | Target | Tool |
|-----------|--------|------|
| Context build | <50ms | criterion benchmark |
| Agent routing | <100ms | criterion benchmark |
| Result format | <20ms | criterion benchmark |
| E2E execution | <500ms | integration test timing |

### Quality Metrics

| Metric | Target | Verification |
|--------|--------|--------------|
| Clippy warnings | 0 | cargo clippy --all-features |
| Failing tests | 0 | cargo test --all-features |
| Documentation | 100% public APIs | cargo doc |
| Example commands | ≥3 | File count |

---

## Next Steps After Epic 3.1

### Epic 3.2: Multi-Agent Coordination (Days 26-28)

**Scope**: Enable multiple agents to collaborate on complex tasks

**Features**:
- Agent chaining (output → input)
- Parallel agent execution
- Result aggregation
- Conflict resolution

### Epic 3.3: Agent Permissions & Safety (Days 29-30)

**Scope**: Enforce permission boundaries for agent operations

**Features**:
- Permission validation before execution
- Sandboxed toolkit operations
- User confirmation for risky actions
- Audit logging

---

## Conclusion

Epic 3.1 delivers the **coordination layer** that bridges commands and agents, enabling:

1. ✅ Agent-backed commands via frontmatter declaration
2. ✅ Automatic agent selection via routing
3. ✅ Context-aware execution with toolkit
4. ✅ Formatted results in conversation
5. ✅ Agent discovery in TUI palette

**Estimated Effort**: 5 days, ~800-1000 LOC, ~50+ tests

**Prerequisites**: ✅ All met (Sprint 2 complete)

**Ready to Start**: ✅ Yes

---

**Document Version**: 1.0
**Date**: October 9, 2025
**Author**: Claude (AI Development Agent)
**Status**: 📋 **APPROVED - READY FOR IMPLEMENTATION**
