# Sprint 2: Command System Integration - Implementation Plan

## Sprint Goal
**Integrate command system with exec_command flow, implement slash command parsing, and enable hot-reload with comprehensive quality validation.**

## Duration: 2 Weeks (Days 11-20)

## Status: 🎯 READY TO START

---

## Prerequisites (Sprint 1 ✅ Complete)

- ✅ YAML frontmatter parser operational
- ✅ Command registry with directory scanning
- ✅ Handlebars template engine integrated
- ✅ Context system with builder pattern
- ✅ 3 built-in commands (/explain, /review, /test)
- ✅ 42 comprehensive tests, ~90% coverage
- ✅ All Sprint 1 quality gates passed

---

## Sprint 2 Objectives

### Primary Objectives
1. **Slash Command Parsing**: Parse `/command args` syntax
2. **exec_command Integration**: Hook into existing command pipeline
3. **Hot-Reload System**: File watcher with notify crate
4. **TUI Palette Integration**: Command palette with autocomplete

### Quality Objectives
- [ ] ≥85% test coverage for all new code
- [ ] All performance targets met
- [ ] Zero critical security issues
- [ ] Backward compatibility maintained
- [ ] Complete documentation

---

## Week 1: Command Invocation & Integration (Days 11-15)

### Epic 2.1: Slash Command Parser (Days 11-12)

**Goal**: Parse `/command args` syntax and extract command name + arguments

#### Tasks

**Day 11: Command Line Parser**
- [ ] Implement slash command regex parser
- [ ] Extract command name from `/command` syntax
- [ ] Parse space-separated arguments
- [ ] Handle quoted arguments with spaces
- [ ] Parse key=value argument syntax
- [ ] Write 10+ unit tests for parser

**Implementation**: `core/src/commands/invocation.rs`
```rust
pub struct CommandInvocation {
    pub command_name: String,
    pub args: HashMap<String, String>,
    pub raw_args: Vec<String>,
}

pub struct InvocationParser;

impl InvocationParser {
    /// Parses "/command arg1 arg2 key=value" into CommandInvocation
    pub fn parse(input: &str) -> anyhow::Result<CommandInvocation> {
        // TODO: Implement parsing logic
    }
}
```

**Acceptance Criteria**:
- [ ] Parse `/explain src/main.rs` → name="explain", args={"file": "src/main.rs"}
- [ ] Parse `/review --depth=deep src/` → name="review", depth="deep", file="src/"
- [ ] Handle quoted args: `/test "my file.rs"` → preserves spaces
- [ ] Error on invalid syntax with clear messages
- [ ] Performance: <10ms per parse
- [ ] ≥90% test coverage

**Quality Gates**:
- [ ] All unit tests pass
- [ ] Clippy clean
- [ ] Documented with examples
- [ ] Performance benchmark < 10ms

---

**Day 12: Argument Mapping**
- [ ] Implement positional → named argument mapping
- [ ] Support optional arguments with defaults
- [ ] Validate required arguments present
- [ ] Type coercion for arguments (string → bool, number)
- [ ] Write 8+ integration tests

**Implementation**: `core/src/commands/args.rs`
```rust
pub struct ArgumentMapper;

impl ArgumentMapper {
    /// Maps raw arguments to command's expected arguments
    pub fn map_arguments(
        invocation: &CommandInvocation,
        metadata: &CommandMetadata,
    ) -> anyhow::Result<HashMap<String, String>> {
        // TODO: Map positional to named, apply defaults, validate required
    }
}
```

**Acceptance Criteria**:
- [ ] Positional args map to first N named args
- [ ] Named args override positional
- [ ] Required args validated
- [ ] Defaults applied for missing optional args
- [ ] Type validation (if arg type specified)
- [ ] ≥85% test coverage

**Quality Gates**:
- [ ] Integration tests with real command metadata
- [ ] Error cases fully tested
- [ ] Documentation complete

---

### Epic 2.2: exec_command Integration (Days 13-15)

**Goal**: Hook command system into existing exec_command pipeline

#### Tasks

**Day 13: Command Executor**
- [ ] Implement command execution pipeline
- [ ] Integrate with registry for command lookup
- [ ] Build CommandContext from exec state
- [ ] Expand template with context
- [ ] Return expanded prompt to exec_command
- [ ] Write 5+ integration tests

**Implementation**: `core/src/commands/executor.rs`
```rust
pub struct CommandExecutor {
    registry: Arc<CommandRegistry>,
    expander: TemplateExpander,
}

impl CommandExecutor {
    /// Executes a command invocation
    pub async fn execute(
        &self,
        invocation: CommandInvocation,
        exec_state: &ExecState,
    ) -> anyhow::Result<String> {
        // 1. Look up command in registry
        // 2. Map arguments
        // 3. Build context from exec state
        // 4. Expand template
        // 5. Return prompt
    }
}
```

**Acceptance Criteria**:
- [ ] Command lookup from registry
- [ ] Context includes git diff, files, workspace
- [ ] Template expansion produces valid prompt
- [ ] Error handling for missing commands
- [ ] Performance: <100ms end-to-end
- [ ] ≥80% test coverage

**Quality Gates**:
- [ ] Integration tests with real exec state
- [ ] Performance benchmark <100ms
- [ ] Error paths tested

---

**Day 14: exec_command Hook**
- [ ] Add command detection to exec_command.rs
- [ ] Route slash commands to CommandExecutor
- [ ] Preserve existing non-command behavior
- [ ] Add feature flag for gradual rollout
- [ ] Write 6+ E2E tests

**Implementation**: Modify `core/src/exec_command.rs`
```rust
pub async fn execute_command(input: &str, state: &ExecState) -> Result<...> {
    // Check if input starts with '/'
    if input.starts_with('/') {
        if let Ok(invocation) = InvocationParser::parse(input) {
            let executor = CommandExecutor::new(/* ... */);
            let prompt = executor.execute(invocation, state).await?;

            // Continue with expanded prompt
            return execute_with_prompt(prompt, state).await;
        }
    }

    // Existing behavior for non-commands
    // ...
}
```

**Acceptance Criteria**:
- [ ] Slash commands detected and routed
- [ ] Non-slash input works as before (backward compatible)
- [ ] Feature flag `command_system_enabled` controls behavior
- [ ] Built-in commands work end-to-end
- [ ] User commands from `.claude/commands/` work
- [ ] ≥85% test coverage

**Quality Gates**:
- [ ] E2E tests for complete flow
- [ ] Backward compatibility verified
- [ ] Feature flag tested (on/off)
- [ ] No breaking changes

---

**Day 15: Context Enhancement**
- [ ] Enhance CommandContext with exec state data
- [ ] Add current file, cursor position, selection
- [ ] Include recent conversation context
- [ ] Add environment variables
- [ ] Write 5+ context building tests

**Implementation**: Enhance `core/src/commands/context.rs`
```rust
impl CommandContext {
    /// Builds context from exec state
    pub async fn from_exec_state(
        args: HashMap<String, String>,
        state: &ExecState,
    ) -> anyhow::Result<Self> {
        // Extract git diff
        let git_diff = get_git_diff(state.workspace()).await.ok();

        // Get current file/selection
        let files = state.current_files();

        // Build context
        Ok(Self {
            args,
            git_diff,
            files,
            workspace_root: state.workspace().to_path_buf(),
            // Add more from state...
        })
    }
}
```

**Acceptance Criteria**:
- [ ] Context includes all relevant exec state
- [ ] Git diff fetched asynchronously
- [ ] Current file/selection available
- [ ] Environment variables accessible
- [ ] Performance: <50ms context building
- [ ] ≥80% test coverage

**Quality Gates**:
- [ ] Tests cover all context fields
- [ ] Performance benchmark <50ms
- [ ] Error handling for missing state

---

## Week 2: Hot-Reload & TUI Integration (Days 16-20)

### Epic 2.3: Hot-Reload System (Days 16-17)

**Goal**: Automatically reload commands when files change

#### Tasks

**Day 16: File Watcher Setup**
- [ ] Implement notify-based file watcher
- [ ] Watch `.claude/commands/` directory
- [ ] Detect create/modify/delete events
- [ ] Debounce rapid changes (300ms)
- [ ] Write 4+ watcher tests

**Implementation**: `core/src/commands/watcher.rs`
```rust
pub struct CommandWatcher {
    watcher: RecommendedWatcher,
    registry: Arc<CommandRegistry>,
}

impl CommandWatcher {
    /// Starts watching the commands directory
    pub async fn start(
        commands_dir: PathBuf,
        registry: Arc<CommandRegistry>,
    ) -> anyhow::Result<Self> {
        let (tx, rx) = channel();

        let watcher = notify::recommended_watcher(move |res| {
            tx.send(res).unwrap();
        })?;

        watcher.watch(&commands_dir, RecursiveMode::Recursive)?;

        // Spawn task to handle events
        tokio::spawn(async move {
            Self::handle_events(rx, registry).await;
        });

        Ok(Self { watcher, registry })
    }

    async fn handle_events(rx: Receiver<...>, registry: Arc<CommandRegistry>) {
        // TODO: Handle file events, reload registry
    }
}
```

**Acceptance Criteria**:
- [ ] Watches `.claude/commands/` recursively
- [ ] Detects file create/modify/delete
- [ ] Debounces rapid changes
- [ ] No memory leaks (tested with valgrind)
- [ ] Performance: <5ms event handling
- [ ] ≥75% test coverage

**Quality Gates**:
- [ ] File event tests (create, modify, delete)
- [ ] Debouncing tested
- [ ] Memory leak test passes
- [ ] No file descriptor leaks

---

**Day 17: Registry Reload Integration**
- [ ] Trigger registry reload on file events
- [ ] Handle reload errors gracefully
- [ ] Log reload events for debugging
- [ ] Notify TUI of registry changes
- [ ] Write 5+ reload tests

**Implementation**: Enhance `core/src/commands/registry.rs`
```rust
impl CommandRegistry {
    /// Reloads registry and notifies observers
    pub async fn reload_and_notify(&self) -> anyhow::Result<()> {
        match self.reload().await {
            Ok(()) => {
                tracing::info!("Command registry reloaded successfully");
                self.notify_observers().await;
                Ok(())
            }
            Err(e) => {
                tracing::error!("Command registry reload failed: {}", e);
                Err(e)
            }
        }
    }

    async fn notify_observers(&self) {
        // Notify TUI, etc.
    }
}
```

**Acceptance Criteria**:
- [ ] Registry reloads on file changes
- [ ] Errors logged but don't crash
- [ ] TUI notified of changes
- [ ] Performance: reload <100ms for 100 commands
- [ ] ≥80% test coverage

**Quality Gates**:
- [ ] Reload tests with real file changes
- [ ] Error handling tested
- [ ] Performance benchmark <100ms
- [ ] TUI notification verified

---

### Epic 2.4: TUI Palette Integration (Days 18-20)

**Goal**: Add command palette to TUI with autocomplete

#### Tasks

**Day 18: Command Palette Widget**
- [ ] Create TUI command palette widget
- [ ] List all available commands
- [ ] Show command descriptions
- [ ] Filter by category
- [ ] Follow TUI style guidelines
- [ ] Write widget tests

**Implementation**: `tui/src/widgets/command_palette.rs`
```rust
pub struct CommandPalette {
    commands: Vec<CommandInfo>,
    filter: String,
    selected: usize,
}

impl CommandPalette {
    pub fn new(registry: &CommandRegistry) -> Self {
        let commands = registry.list().await;
        Self {
            commands,
            filter: String::new(),
            selected: 0,
        }
    }

    pub fn filter(&mut self, text: impl Into<String>) {
        self.filter = text.into();
        // Filter commands by name/description
    }
}

impl Widget for CommandPalette {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Render command list with styling
    }
}
```

**Acceptance Criteria**:
- [ ] Shows all commands with descriptions
- [ ] Filter updates in real-time
- [ ] Keyboard navigation (up/down/enter)
- [ ] Follows `tui/styles.md` guidelines
- [ ] Performance: <16ms render (60fps)
- [ ] Widget tests pass

**Quality Gates**:
- [ ] Visual tests with snapshots
- [ ] Keyboard navigation tested
- [ ] Style compliance verified
- [ ] Performance benchmark <16ms

---

**Day 19: Autocomplete System**
- [ ] Implement fuzzy command matching
- [ ] Show argument suggestions
- [ ] Display argument types and defaults
- [ ] Highlight matched characters
- [ ] Write autocomplete tests

**Implementation**: `tui/src/autocomplete.rs`
```rust
pub struct Autocomplete {
    registry: Arc<CommandRegistry>,
}

impl Autocomplete {
    /// Provides autocomplete suggestions for input
    pub async fn suggest(&self, input: &str) -> Vec<Suggestion> {
        let invocation = InvocationParser::parse(input).ok();

        if let Some(inv) = invocation {
            // Command is complete, suggest arguments
            self.suggest_arguments(&inv).await
        } else {
            // Suggest command names
            self.suggest_commands(input).await
        }
    }
}
```

**Acceptance Criteria**:
- [ ] Fuzzy matches command names
- [ ] Suggests arguments after command
- [ ] Shows argument types/defaults
- [ ] Highlights matched characters
- [ ] Performance: <10ms per suggestion
- [ ] ≥80% test coverage

**Quality Gates**:
- [ ] Fuzzy matching tests
- [ ] Argument suggestion tests
- [ ] Performance benchmark <10ms
- [ ] Visual rendering verified

---

**Day 20: Integration & Polish**
- [ ] Integrate palette into main TUI
- [ ] Add keyboard shortcut (Ctrl+P)
- [ ] Implement command execution from palette
- [ ] Add command history
- [ ] Final testing and bug fixes

**Implementation**: Modify `tui/src/app.rs`
```rust
impl App {
    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.show_command_palette();
            }
            // ...
        }
    }

    fn show_command_palette(&mut self) {
        let palette = CommandPalette::new(&self.command_registry);
        self.overlay = Some(Overlay::CommandPalette(palette));
    }
}
```

**Acceptance Criteria**:
- [ ] Ctrl+P opens command palette
- [ ] Command executes from palette
- [ ] History tracks recent commands
- [ ] Smooth animations (fade in/out)
- [ ] All E2E tests pass

**Quality Gates**:
- [ ] E2E TUI tests
- [ ] Keyboard shortcuts tested
- [ ] Animation performance <16ms
- [ ] User experience validated

---

## Sprint 2 Quality Gates

### Exit Criteria (All Must Pass)
- [ ] Slash command parsing <10ms
- [ ] exec_command integration complete
- [ ] Hot-reload works reliably (no crashes)
- [ ] TUI palette responsive (<16ms render)
- [ ] ≥85% test coverage for new code
- [ ] All performance targets met
- [ ] No memory leaks in file watcher
- [ ] Backward compatibility maintained
- [ ] Zero critical security issues
- [ ] Complete documentation

### Performance Targets

| Component | Target | Measurement |
|-----------|--------|-------------|
| Slash command parse | <10ms | Per parse |
| Command execution | <100ms | End-to-end |
| Context building | <50ms | Per build |
| Registry reload | <100ms | Per 100 cmds |
| File event handling | <5ms | Per event |
| Palette render | <16ms | 60fps |
| Autocomplete | <10ms | Per suggest |

### Test Coverage Goals

| Module | Target | Tests |
|--------|--------|-------|
| Invocation parser | ≥90% | 10+ unit |
| Argument mapper | ≥85% | 8+ integration |
| Command executor | ≥80% | 5+ integration |
| exec_command hook | ≥85% | 6+ E2E |
| File watcher | ≥75% | 4+ system |
| Registry reload | ≥80% | 5+ integration |
| Command palette | ≥80% | Widget tests |
| Autocomplete | ≥80% | 6+ tests |

---

## Risk Mitigation

### Identified Risks

1. **exec_command Integration Complexity**
   - **Risk**: Breaking existing functionality
   - **Mitigation**: Feature flag, comprehensive tests, gradual rollout
   - **Owner**: Engineering lead

2. **File Watcher Resource Usage**
   - **Risk**: Memory leaks, file descriptor leaks
   - **Mitigation**: Valgrind tests, resource monitoring, cleanup on drop
   - **Owner**: Performance engineer

3. **TUI Performance**
   - **Risk**: Palette rendering impacts frame rate
   - **Mitigation**: Render optimization, lazy loading, benchmarks
   - **Owner**: Frontend engineer

4. **Backward Compatibility**
   - **Risk**: Breaking existing commands or workflows
   - **Mitigation**: Comprehensive regression tests, compatibility layer
   - **Owner**: QA lead

---

## Dependencies

### Internal Dependencies
- ✅ Sprint 1 complete (all foundations in place)
- [ ] exec_command API understanding (Day 13)
- [ ] TUI widget framework (Day 18)

### External Dependencies
- ✅ `notify` crate (already in Cargo.toml)
- ✅ `tokio` for async (already in use)
- ✅ `ratatui` for TUI (already in use)

---

## Deliverables

### Code Deliverables
1. **Invocation System** (`commands/invocation.rs`, `commands/args.rs`)
   - Slash command parser
   - Argument mapper
   - 18+ tests

2. **Execution Pipeline** (`commands/executor.rs`)
   - Command executor
   - exec_command integration
   - Context enhancement
   - 16+ tests

3. **Hot-Reload System** (`commands/watcher.rs`)
   - File watcher
   - Registry reload
   - 9+ tests

4. **TUI Integration** (`tui/widgets/command_palette.rs`, `tui/autocomplete.rs`)
   - Command palette widget
   - Autocomplete system
   - Widget tests

### Documentation Deliverables
1. **Quality Gates** (`docs/QUALITY_GATES.md`) ✅
2. **Sprint 2 Plan** (`docs/SPRINT_2_PLAN.md`) ✅
3. **User Guide** (`docs/COMMAND_USAGE.md`) - To be created
4. **API Documentation** (rustdoc) - Generated

---

## Success Criteria

Sprint 2 is successful when:
- ✅ Users can execute commands with `/command args` syntax
- ✅ Commands integrate seamlessly with existing exec_command flow
- ✅ Hot-reload updates commands without restart
- ✅ TUI palette provides smooth command discovery
- ✅ All quality gates pass
- ✅ Performance targets met
- ✅ ≥85% test coverage
- ✅ Zero critical issues

---

## Next Steps: Sprint 3 (Weeks 6-7)

### Sprint 3 Preview: Agent System Integration
- Agent routing and selection
- Multi-agent coordination
- Permission enforcement
- Agent toolkit implementation
- Agent result handling

---

## Conclusion

Sprint 2 builds on the solid Sprint 1 foundation to create a fully integrated command system. Quality and performance are prioritized throughout, with comprehensive testing and validation at every step.

**Quality-First. Integration-Ready. User-Focused.**

🚀 **Let's integrate this system!**
