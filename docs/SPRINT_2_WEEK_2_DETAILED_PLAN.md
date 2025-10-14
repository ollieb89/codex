# Sprint 2 Week 2: Detailed Implementation Plan

**Planning Date**: October 9, 2025
**Sprint**: Sprint 2 (Days 15-20)
**Status**: Ready for Execution
**Dependencies**: Sprint 2 Week 1 Complete ✅

---

## Executive Summary

This document provides a comprehensive, actionable implementation plan for Sprint 2 Week 2, covering **Days 15-20**. The week focuses on three key epics: Context Enhancement, Hot-Reload System, and TUI Command Palette.

### Week 2 Overview

| Epic | Days | Estimated Effort | Complexity | Risk Level |
|------|------|------------------|------------|------------|
| **Day 15: Context Enhancement** | 1 day | 6 hours | 🟢 Low | 🟢 Low |
| **Days 16-17: Hot-Reload System** | 2 days | 12 hours | 🟡 Medium | 🟡 Medium |
| **Days 18-20: TUI Command Palette** | 3 days | 16 hours | 🟡 Medium | 🟡 Medium |
| **Total** | **6 days** | **34 hours** | - | - |

---

## Day 15: Context Enhancement

### Epic Goal

**Enhance ExecutionContext with richer information from Codex's runtime state to enable more powerful command templates.**

### Current State Analysis

**Existing ExecutionContext** (`core/src/commands/executor.rs:161-191`):
```rust
pub struct ExecutionContext {
    pub workspace_root: PathBuf,
    pub git_diff: Option<String>,
    pub current_files: Vec<PathBuf>,
}
```

**Current Usage** (in `core/src/codex.rs:1279-1317`):
```rust
execute_slash_command(
    &command_text,
    Arc::clone(registry),
    cwd.clone(),
    None,      // ← git_diff: Always None
    vec![],    // ← current_files: Always empty
).await
```

**Problem**: Context is minimal - commands cannot access important runtime information.

---

### Implementation Plan

#### Task 15.1: Extract Git Diff from Workspace State (2 hours)

**Objective**: Pass actual git diff to commands instead of None.

**Implementation Steps:**

1. **Reuse Existing git_diff Utilities**

The TUI already has `get_git_diff()` in `tui/src/get_git_diff.rs`. We should either:
- Move to `core/src/git_info.rs` (recommended)
- Or duplicate with simplified version in core

**Recommended Approach** - Move to core module:

```rust
// core/src/commands/git_utils.rs (new file)
use std::io;
use std::process::Stdio;
use tokio::process::Command;

/// Get current git diff (tracked + untracked).
/// Returns (is_git_repo, diff_string).
pub async fn get_git_diff() -> io::Result<(bool, String)> {
    // Check if we're in a git repo
    if !is_git_repo().await? {
        return Ok((false, String::new()));
    }

    // Get tracked diff
    let tracked = Command::new("git")
        .args(["diff"])
        .output()
        .await?;

    if !tracked.status.success() {
        return Ok((true, String::new()));
    }

    Ok((true, String::from_utf8_lossy(&tracked.stdout).into_owned()))
}

async fn is_git_repo() -> io::Result<bool> {
    let status = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?;

    Ok(status.success())
}
```

2. **Modify codex.rs to extract git diff**

```rust
// In core/src/codex.rs, Op::UserTurn handler (line ~1285)
if let Some(command_text) = crate::commands::detect_slash_command(&items) {
    // Extract git diff
    let git_diff = match crate::commands::git_utils::get_git_diff().await {
        Ok((true, diff)) if !diff.is_empty() => Some(diff),
        _ => None,
    };

    // Execute slash command with git diff
    match crate::commands::execute_slash_command(
        &command_text,
        Arc::clone(registry),
        cwd.clone(),
        git_diff, // ← Pass actual git diff
        vec![],   // Still empty for now
    )
    .await
    {
        // ... error handling
    }
}
```

**Acceptance Criteria:**
- [ ] Git diff passed to commands when available
- [ ] No git diff when not in repo (graceful)
- [ ] Performance: <50ms for git diff extraction
- [ ] No impact when git not installed

**Testing:**
```rust
#[tokio::test]
async fn test_git_diff_in_context() {
    // Setup test git repo
    let temp_dir = tempfile::tempdir()?;
    init_test_repo(&temp_dir)?;

    // Make a change
    std::fs::write(temp_dir.path().join("test.txt"), "changed")?;

    // Execute command
    let context = build_exec_context(&temp_dir).await?;

    // Verify git diff present
    assert!(context.git_diff.is_some());
    assert!(context.git_diff.unwrap().contains("test.txt"));
}
```

---

#### Task 15.2: Extract Current Files from Session State (2 hours)

**Objective**: Pass currently open/focused files to command context.

**Challenge**: Codex core doesn't track "open files" concept - this is TUI-specific.

**Solutions**:

**Option A: Environment-Based Extraction** (Recommended)
```rust
// In core/src/codex.rs
let current_files = extract_current_files(&sess, &config).await;

async fn extract_current_files(
    sess: &Session,
    config: &Config,
) -> Vec<PathBuf> {
    // For now, return empty - can be enhanced later
    // Future: Extract from editor state, MCP context, etc.
    vec![]
}
```

**Option B: Pass from Caller**
- TUI can pass open files when calling submit
- Exec mode passes via CLI args
- More explicit but requires more changes

**For Day 15, use Option A** - keep it simple, enhance in future sprints.

**Acceptance Criteria:**
- [ ] Structure in place for current_files
- [ ] Returns empty vec for now (no regression)
- [ ] Easy to enhance in future

---

#### Task 15.3: Add Conversation Context (1.5 hours)

**Objective**: Allow commands to reference recent conversation history.

**Implementation:**

```rust
// Enhance ExecutionContext
pub struct ExecutionContext {
    pub workspace_root: PathBuf,
    pub git_diff: Option<String>,
    pub current_files: Vec<PathBuf>,
    pub conversation_context: Option<ConversationContext>, // ← New
}

#[derive(Debug, Clone)]
pub struct ConversationContext {
    /// Last N messages from conversation
    pub recent_messages: Vec<MessageSummary>,
    /// Current conversation ID
    pub conversation_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MessageSummary {
    pub role: String, // "user" or "assistant"
    pub content: String,
    pub timestamp: Option<String>,
}
```

**Extract from Session:**
```rust
// In core/src/codex.rs
let conversation_context = extract_conversation_context(&sess, 5).await;

async fn extract_conversation_context(
    sess: &Session,
    max_messages: usize,
) -> Option<ConversationContext> {
    // Access message history from sess.conversation
    // Return last N messages
    // For now, return None - enhance in future
    None
}
```

**Acceptance Criteria:**
- [ ] ConversationContext struct defined
- [ ] Integration point added (returns None initially)
- [ ] Template expander can access conversation data
- [ ] No performance impact

---

#### Task 15.4: Add Environment Variables (0.5 hours)

**Objective**: Expose select environment variables to templates.

**Implementation:**

```rust
// Add to ExecutionContext
pub struct ExecutionContext {
    // ... existing fields
    pub env_vars: HashMap<String, String>, // ← New
}

// In execute_slash_command()
let env_vars = collect_safe_env_vars();

fn collect_safe_env_vars() -> HashMap<String, String> {
    let mut vars = HashMap::new();

    // Whitelist of safe env vars
    let safe_vars = [
        "USER",
        "HOME",
        "SHELL",
        "LANG",
        "CODEX_HOME",
        "CODEX_MODEL",
    ];

    for var in safe_vars {
        if let Ok(value) = std::env::var(var) {
            vars.insert(var.to_string(), value);
        }
    }

    vars
}
```

**Security Note**: Only expose whitelisted environment variables - never expose secrets or API keys.

**Acceptance Criteria:**
- [ ] Whitelisted env vars accessible in templates
- [ ] No secrets exposed
- [ ] Performance: <1ms to collect

---

#### Task 15.5: Update Template Expander (1 hour)

**Objective**: Make new context fields available in templates.

**Implementation:**

```rust
// In core/src/commands/expander.rs
pub fn expand(&self, template: &str, context: &CommandContext) -> Result<String> {
    let data = json!({
        "args": context.args,
        "context": {
            "git_diff": context.git_diff,
            "files": context.files,
            "workspace_root": context.workspace_root,
        },
        "conversation": context.conversation, // ← New
        "env": context.env_vars,              // ← New
    });

    self.handlebars.render_template(template, &data)?
}
```

**Template Usage Examples:**

```markdown
---
name: context-aware-review
description: Review with conversation awareness
---

Review the current changes:

{{#if context.git_diff}}
Git diff:
```
{{{context.git_diff}}}
```
{{else}}
No git changes detected.
{{/if}}

{{#if conversation}}
Context from conversation:
{{#each conversation.recent_messages}}
- {{this.role}}: {{this.content}}
{{/each}}
{{/if}}

Environment: {{env.USER}} on {{env.SHELL}}
```

**Acceptance Criteria:**
- [ ] Git diff accessible via `{{context.git_diff}}`
- [ ] Files accessible via `{{context.files}}`
- [ ] Conversation via `{{conversation.recent_messages}}`
- [ ] Env vars via `{{env.VAR_NAME}}`

---

### Day 15 Quality Gates

**Exit Criteria:**
- [ ] Git diff extraction working (with tests)
- [ ] Environment variables exposed (whitelisted only)
- [ ] Template expander supports new fields
- [ ] 5+ new tests for context building
- [ ] No performance degradation (<10ms overhead)
- [ ] All existing tests still pass

**Performance Targets:**
- Git diff extraction: <50ms
- Env var collection: <1ms
- Total overhead: <10ms

**Deliverables:**
- [ ] `core/src/commands/git_utils.rs` (new file, ~150 LOC)
- [ ] Enhanced `ExecutionContext` struct
- [ ] Updated `execute_slash_command()` in integration.rs
- [ ] Updated template expander
- [ ] 5+ context tests in integration_tests.rs
- [ ] Day 15 completion report

---

## Days 16-17: Hot-Reload System

### Epic Goal

**Enable automatic command registry reload when command files change, providing instant feedback during command development.**

### Architecture Design

**Components:**

1. **CommandWatcher** - File system event monitoring
2. **Registry Reload** - Safe async reload mechanism
3. **Debouncing** - Prevent reload spam
4. **Error Handling** - Graceful degradation on errors

**System Diagram:**

```
File System Events
       ↓
   notify Watcher
       ↓
Event Filtering (debounce)
       ↓
Registry Reload
       ↓
Log Success/Failure
```

---

### Implementation Plan

#### Task 16.1: File Watcher Implementation (4 hours)

**Objective**: Monitor `~/.codex/commands/` for file changes.

**Dependencies**: `notify = "6.1"` (already in workspace deps)

**Implementation:**

```rust
// core/src/commands/watcher.rs (new file)
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

use super::registry::CommandRegistry;

/// Watches command directory for changes and reloads registry.
pub struct CommandWatcher {
    _watcher: RecommendedWatcher,
    registry: Arc<CommandRegistry>,
}

impl CommandWatcher {
    /// Starts watching the commands directory.
    pub fn start(
        commands_dir: PathBuf,
        registry: Arc<CommandRegistry>,
    ) -> anyhow::Result<Self> {
        let (tx, mut rx) = mpsc::channel::<Event>(100);

        // Create watcher
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        })?;

        // Watch directory (non-recursive, only .md files)
        watcher.watch(&commands_dir, RecursiveMode::NonRecursive)?;

        // Spawn reload handler task
        let registry_clone = Arc::clone(&registry);
        tokio::spawn(async move {
            let mut debounce_timer: Option<tokio::time::Instant> = None;
            let debounce_duration = Duration::from_millis(500);

            while let Some(event) = rx.recv().await {
                // Only react to relevant events
                if !Self::should_reload(&event) {
                    continue;
                }

                // Debounce: Set timer on first event
                if debounce_timer.is_none() {
                    debounce_timer = Some(tokio::time::Instant::now() + debounce_duration);
                }

                // Check if debounce period expired
                if let Some(timer) = debounce_timer {
                    if tokio::time::Instant::now() >= timer {
                        // Reload registry
                        match registry_clone.reload().await {
                            Ok(count) => {
                                log::info!("Reloaded {count} commands");
                            }
                            Err(e) => {
                                log::warn!("Failed to reload commands: {e:#}");
                            }
                        }
                        debounce_timer = None;
                    }
                }
            }
        });

        Ok(Self {
            _watcher: watcher,
            registry,
        })
    }

    /// Determines if an event should trigger a reload.
    fn should_reload(event: &Event) -> bool {
        use notify::EventKind;

        // Only reload on create/modify/remove
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                // Only .md files
                event.paths.iter().any(|p| {
                    p.extension()
                        .and_then(|e| e.to_str())
                        .map(|e| e == "md")
                        .unwrap_or(false)
                })
            }
            _ => false,
        }
    }
}
```

**Acceptance Criteria:**
- [ ] Watches commands directory
- [ ] Detects create/modify/delete events
- [ ] Only triggers on .md files
- [ ] Graceful handling of watch errors

---

#### Task 16.2: Debouncing Mechanism (2 hours)

**Objective**: Prevent reload spam during rapid file changes (e.g., editor auto-save).

**Strategy**: 500ms debounce window

**Implementation** (already in Task 16.1 above):
- First event starts 500ms timer
- Additional events reset timer
- Reload happens 500ms after last event

**Testing:**

```rust
#[tokio::test]
async fn test_debouncing() {
    let temp_dir = tempfile::tempdir()?;
    let registry = Arc::new(CommandRegistry::new(temp_dir.path()).await?);
    let watcher = CommandWatcher::start(temp_dir.path().to_path_buf(), registry.clone())?;

    // Rapid changes
    for i in 0..10 {
        std::fs::write(temp_dir.path().join("test.md"), format!("v{}", i))?;
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // Wait for debounce
    tokio::time::sleep(Duration::from_millis(600)).await;

    // Verify only one reload happened
    // (would need reload counter in registry for this)
}
```

**Acceptance Criteria:**
- [ ] Multiple rapid changes = single reload
- [ ] Debounce period configurable (500ms default)
- [ ] No reload spam in logs

---

#### Task 16.3: Enhanced Registry Reload (2 hours)

**Objective**: Make registry reload return count and handle errors gracefully.

**Current Implementation** (`core/src/commands/registry.rs`):
```rust
pub async fn reload(&self) -> Result<()> {
    let commands = self.loader.load_all().await?;

    let mut map = self.commands.write().await;
    map.clear();

    for cmd in commands {
        map.insert(cmd.name().to_string(), cmd);
    }

    Ok(())
}
```

**Enhanced Version:**
```rust
pub async fn reload(&self) -> Result<usize> {
    let commands = self.loader.load_all().await?;
    let count = commands.len();

    let mut map = self.commands.write().await;
    map.clear();

    for cmd in commands {
        map.insert(cmd.name().to_string(), cmd);
    }

    log::debug!("Registry reloaded with {} commands", count);
    Ok(count)
}
```

**Error Handling:**
- Continue on individual file parse errors
- Log warnings for failed files
- Never crash the watcher

**Acceptance Criteria:**
- [ ] Returns number of loaded commands
- [ ] Logs reload events
- [ ] Handles individual file errors gracefully
- [ ] Never panics

---

#### Task 16.4: Integration with Codex Initialization (2 hours)

**Objective**: Start watcher when command system enabled.

**Implementation:**

```rust
// In core/src/codex.rs, parallel initialization (line ~388)
let command_registry_fut = async {
    if config.experimental_command_system_enabled {
        let commands_dir = config.codex_home.join("commands");
        match crate::commands::CommandRegistry::new(commands_dir.clone()).await {
            Ok(registry) => {
                let registry = Arc::new(registry);

                // Start watcher
                match crate::commands::CommandWatcher::start(commands_dir, Arc::clone(&registry)) {
                    Ok(watcher) => {
                        log::info!("Command watcher started");
                        Some((Arc::clone(&registry), Some(watcher)))
                    }
                    Err(e) => {
                        log::warn!("Failed to start command watcher: {e:#}");
                        Some((registry, None))
                    }
                }
            }
            Err(e) => {
                warn!("Failed to initialize command registry: {e:#}");
                None
            }
        }
    } else {
        None
    }
};
```

**Update SessionServices:**
```rust
pub(crate) struct SessionServices {
    pub(crate) command_registry: Option<Arc<CommandRegistry>>,
    pub(crate) command_watcher: Option<CommandWatcher>, // ← New
    // ...
}
```

**Acceptance Criteria:**
- [ ] Watcher starts with registry
- [ ] Graceful fallback if watcher fails
- [ ] No impact on startup time (<100ms)
- [ ] Watcher stopped on session end

---

#### Task 16.5: Testing & Validation (2 hours)

**Tests to Write:**

1. **Basic Reload Test**
```rust
#[tokio::test]
async fn test_file_change_triggers_reload() {
    // Setup watcher
    // Modify .md file
    // Wait for reload
    // Verify registry updated
}
```

2. **Debouncing Test**
```rust
#[tokio::test]
async fn test_rapid_changes_debounced() {
    // Rapid file modifications
    // Verify single reload
}
```

3. **Error Handling Test**
```rust
#[tokio::test]
async fn test_invalid_file_handled() {
    // Write invalid .md file
    // Verify watcher continues
    // Verify error logged
}
```

4. **Performance Test**
```rust
#[tokio::test]
async fn test_reload_performance() {
    // Load 100 commands
    // Measure reload time
    // Assert <1s for reload
}
```

**Acceptance Criteria:**
- [ ] 4+ integration tests
- [ ] All tests pass
- [ ] Performance: Reload <1s for 100 commands
- [ ] Resource: <5MB memory overhead

---

### Days 16-17 Quality Gates

**Exit Criteria:**
- [ ] File watcher operational
- [ ] Debouncing prevents spam
- [ ] Registry reload enhanced
- [ ] Integration with initialization
- [ ] 4+ tests passing
- [ ] Performance: <1s reload
- [ ] Memory: <5MB overhead
- [ ] No crashes on file errors

**Deliverables:**
- [ ] `core/src/commands/watcher.rs` (~200 LOC)
- [ ] Enhanced `registry.rs` reload method
- [ ] Updated codex.rs initialization
- [ ] Updated SessionServices struct
- [ ] 4+ watcher tests
- [ ] Days 16-17 completion report

---

## Days 18-20: TUI Command Palette

### Epic Goal

**Build an interactive command palette (Ctrl+K) with fuzzy search, keyboard navigation, and argument suggestions for enhanced command discovery and execution.**

### UX Design

**Interaction Flow:**

1. User presses `Ctrl+K`
2. Palette opens (centered overlay)
3. User types to filter commands (fuzzy search)
4. Arrow keys navigate, Enter executes
5. Esc closes palette

**Visual Layout:**

```
┌────────────────────────────────────────────────────────┐
│  Commands (Ctrl+K to close)                            │
├────────────────────────────────────────────────────────┤
│  > search-text_                                        │
├────────────────────────────────────────────────────────┤
│  > explain                  Explain code in simple...  │
│    review                   Code review assistant      │
│    test                     Generate comprehensive...  │
│    custom-command           User-defined command       │
│                                                        │
│  3 of 4 commands                                       │
└────────────────────────────────────────────────────────┘
```

---

### Implementation Plan

#### Task 18.1: Palette Widget Foundation (4 hours)

**Objective**: Build basic palette widget with Ratatui.

**Implementation:**

```rust
// tui/src/commands_palette.rs (new file)
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Clear};
use std::sync::Arc;
use crate::commands::CommandRegistry;

pub struct CommandPalette {
    /// Whether palette is visible
    visible: bool,
    /// Current filter text
    filter: String,
    /// Selected command index
    selected: usize,
    /// All available commands
    commands: Vec<CommandInfo>,
    /// Filtered commands (cached)
    filtered: Vec<usize>, // Indices into commands vec
    /// Command registry reference
    registry: Option<Arc<CommandRegistry>>,
}

#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub category: String,
}

impl CommandPalette {
    pub fn new() -> Self {
        Self {
            visible: false,
            filter: String::new(),
            selected: 0,
            commands: Vec::new(),
            filtered: Vec::new(),
            registry: None,
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        if !self.visible {
            self.filter.clear();
            self.selected = 0;
        }
    }

    pub fn set_registry(&mut self, registry: Arc<CommandRegistry>) {
        self.registry = Some(registry);
    }

    /// Load commands from registry
    pub async fn load_commands(&mut self) {
        if let Some(registry) = &self.registry {
            self.commands = registry.list().await;
            self.update_filtered();
        }
    }

    /// Update filtered list based on current filter
    fn update_filtered(&mut self) {
        if self.filter.is_empty() {
            self.filtered = (0..self.commands.len()).collect();
        } else {
            self.filtered = self.commands
                .iter()
                .enumerate()
                .filter(|(_, cmd)| {
                    cmd.name.contains(&self.filter) ||
                    cmd.description.to_lowercase().contains(&self.filter.to_lowercase())
                })
                .map(|(i, _)| i)
                .collect();
        }

        // Reset selection if out of bounds
        if self.selected >= self.filtered.len() {
            self.selected = 0;
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        if !self.visible {
            return;
        }

        // Center the palette (60% width, 50% height)
        let popup_area = centered_rect(60, 50, area);

        // Clear background
        Clear.render(popup_area, buf);

        // Render main block
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Commands (Ctrl+K to close) ".bold())
            .border_style(Style::default().cyan());

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        // Split into filter input and command list
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Filter input
                Constraint::Min(0),     // Command list
                Constraint::Length(1),  // Footer
            ])
            .split(inner);

        // Render filter input
        let filter_text = format!("> {}_", self.filter);
        Paragraph::new(filter_text)
            .style(Style::default().bold())
            .render(chunks[0], buf);

        // Render command list
        let items: Vec<ListItem> = self.filtered
            .iter()
            .enumerate()
            .map(|(i, &cmd_idx)| {
                let cmd = &self.commands[cmd_idx];
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

        // Render footer (count)
        let footer = format!(
            "{} of {} commands",
            self.filtered.len(),
            self.commands.len()
        );
        Paragraph::new(footer.dim()).render(chunks[2], buf);
    }
}

/// Helper to create centered rectangle
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
```

**Acceptance Criteria:**
- [ ] Palette renders centered overlay
- [ ] Filter input visible
- [ ] Command list renders
- [ ] Toggle on/off works
- [ ] Compiles and displays correctly

---

#### Task 18.2: Keyboard Navigation (3 hours)

**Objective**: Handle keyboard input for navigation and selection.

**Implementation:**

```rust
impl CommandPalette {
    /// Handle keyboard input
    pub fn handle_key(&mut self, key: KeyEvent) -> Option<PaletteAction> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match (key.code, key.modifiers) {
            // Close palette
            (KeyCode::Esc, _) | (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                self.toggle();
                None
            }

            // Execute selected command
            (KeyCode::Enter, _) => {
                if let Some(&cmd_idx) = self.filtered.get(self.selected) {
                    let cmd_name = self.commands[cmd_idx].name.clone();
                    self.toggle();
                    Some(PaletteAction::Execute(cmd_name))
                } else {
                    None
                }
            }

            // Navigate up
            (KeyCode::Up, _) => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                None
            }

            // Navigate down
            (KeyCode::Down, _) => {
                if self.selected < self.filtered.len().saturating_sub(1) {
                    self.selected += 1;
                }
                None
            }

            // Filter input - character
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                self.filter.push(c);
                self.update_filtered();
                None
            }

            // Filter input - backspace
            (KeyCode::Backspace, _) => {
                self.filter.pop();
                self.update_filtered();
                None
            }

            _ => None,
        }
    }
}

pub enum PaletteAction {
    Execute(String), // Command name to execute
}
```

**Acceptance Criteria:**
- [ ] Up/Down arrows navigate
- [ ] Enter executes command
- [ ] Esc closes palette
- [ ] Text input filters
- [ ] Backspace removes filter chars

---

#### Task 18.3: Fuzzy Search (3 hours)

**Objective**: Implement fuzzy matching for better command discovery.

**Dependency**: `nucleo-matcher = "0.3.1"` (already in workspace)

**Implementation:**

```rust
use nucleo_matcher::{Matcher, Config};

impl CommandPalette {
    fn update_filtered_fuzzy(&mut self) {
        if self.filter.is_empty() {
            self.filtered = (0..self.commands.len()).collect();
            return;
        }

        // Create matcher
        let mut matcher = Matcher::new(Config::DEFAULT);

        // Score and filter
        let mut scored: Vec<(usize, i32)> = self.commands
            .iter()
            .enumerate()
            .filter_map(|(idx, cmd)| {
                // Match against name and description
                let name_score = matcher.fuzzy_match(&cmd.name, &self.filter);
                let desc_score = matcher.fuzzy_match(&cmd.description, &self.filter);

                // Take best score
                let score = name_score.max(desc_score);
                score.map(|s| (idx, s))
            })
            .collect();

        // Sort by score (descending)
        scored.sort_by(|a, b| b.1.cmp(&a.1));

        // Extract indices
        self.filtered = scored.into_iter().map(|(idx, _)| idx).collect();

        // Reset selection
        if self.selected >= self.filtered.len() {
            self.selected = 0;
        }
    }
}
```

**Acceptance Criteria:**
- [ ] Fuzzy matching works (e.g., "rv" matches "review")
- [ ] Results sorted by relevance
- [ ] Performance: <10ms for 100 commands
- [ ] Handles empty input

---

#### Task 18.4: Integration with TUI App (4 hours)

**Objective**: Integrate palette into main TUI app.

**Implementation:**

```rust
// In tui/src/app.rs
pub struct App {
    // ... existing fields
    command_palette: CommandPalette,
}

impl App {
    pub async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        // Check for Ctrl+K to toggle palette
        if matches!(key.code, KeyCode::Char('k')) &&
           key.modifiers.contains(KeyModifiers::CONTROL) {
            self.command_palette.toggle();

            // Load commands when opened
            if self.command_palette.is_visible() {
                self.command_palette.load_commands().await;
            }

            return Ok(());
        }

        // If palette visible, route input there
        if self.command_palette.is_visible() {
            if let Some(action) = self.command_palette.handle_key(key) {
                match action {
                    PaletteAction::Execute(cmd_name) => {
                        // Insert /command into composer
                        self.composer.set_text(format!("/{}", cmd_name));
                        // Or auto-submit
                        self.submit_message().await?;
                    }
                }
            }
            return Ok(());
        }

        // Normal key handling
        // ...
    }

    fn render(&mut self, frame: &mut Frame) {
        // ... normal rendering

        // Render palette on top (if visible)
        self.command_palette.render(frame.size(), frame.buffer_mut());
    }
}
```

**Acceptance Criteria:**
- [ ] Ctrl+K opens palette
- [ ] Palette renders on top of TUI
- [ ] Keyboard input routed to palette when open
- [ ] Command execution inserts into composer
- [ ] Palette closes after selection

---

#### Task 18.5: Argument Suggestions (Optional - 2 hours)

**Objective**: Show argument hints for selected command.

**Implementation:**

```rust
impl CommandPalette {
    fn render_arg_hints(&self, area: Rect, buf: &mut Buffer) {
        if let Some(&cmd_idx) = self.filtered.get(self.selected) {
            if let Some(cmd_meta) = self.get_command_metadata(cmd_idx) {
                let hints = format!(
                    "Arguments: {}",
                    cmd_meta.args.iter()
                        .map(|a| format!("{}{}", a.name, if a.required { "*" } else { "" }))
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                Paragraph::new(hints.dim()).render(area, buf);
            }
        }
    }
}
```

**Acceptance Criteria:**
- [ ] Shows required arguments with *
- [ ] Shows optional arguments
- [ ] Updates as selection changes
- [ ] Handles commands without args

---

### Days 18-20 Quality Gates

**Exit Criteria:**
- [ ] Palette widget operational
- [ ] Fuzzy search working
- [ ] Keyboard navigation smooth
- [ ] Integration with TUI complete
- [ ] Performance: <16ms frame rate
- [ ] No visual glitches
- [ ] 3+ snapshot tests
- [ ] Argument hints (if time permits)

**Performance Targets:**
- Render time: <16ms (60 FPS)
- Fuzzy search: <10ms for 100 commands
- Keyboard responsiveness: Instant (<5ms)

**Deliverables:**
- [ ] `tui/src/commands_palette.rs` (~400 LOC)
- [ ] Updated `tui/src/app.rs` integration (~50 LOC)
- [ ] 3+ snapshot tests
- [ ] Days 18-20 completion report

---

## Dependencies & Integration Points

### Cross-Epic Dependencies

```
Day 15 (Context)
  ↓
Days 16-17 (Hot-Reload) - Independent
  ↓
Days 18-20 (TUI Palette) - Depends on Registry

Sequential execution recommended:
1. Day 15 first (unlocks richer templates)
2. Days 16-17 can run in parallel with Day 15 testing
3. Days 18-20 last (requires stable registry)
```

### External Dependencies

**Rust Crates:**
- `notify = "6.1"` - File watching (already in workspace)
- `nucleo-matcher = "0.3.1"` - Fuzzy search (already in workspace)
- `ratatui = "0.29.0"` - TUI widgets (already in workspace)

**Codex Modules:**
- `core/src/commands/registry.rs` - Command storage
- `core/src/commands/executor.rs` - Execution context
- `tui/src/app.rs` - Main TUI app
- `tui/src/get_git_diff.rs` - Git utilities

### Integration Risks

**🟡 Medium Risk Areas:**

1. **Hot-Reload Resource Usage**
   - Mitigation: Profile with `cargo flamegraph`
   - Target: <5MB memory, <1% CPU idle
   - Validate with 100+ commands

2. **TUI Performance**
   - Mitigation: Benchmark rendering with `insta` snapshots
   - Target: <16ms per frame
   - Test with 1000+ command palette

3. **Git Diff Performance**
   - Mitigation: Timeout after 5s
   - Fallback: Return empty diff on timeout
   - Log warning on slow repos

---

## Risk Mitigation Strategy

### Performance Risks

**Risk**: Hot-reload uses too much memory
- **Mitigation**: Limit watcher to single directory, non-recursive
- **Validation**: Test with 1000 files in directory
- **Threshold**: <10MB total overhead

**Risk**: TUI palette lags with many commands
- **Mitigation**: Virtualized list rendering (only visible items)
- **Validation**: Benchmark with 1000 commands
- **Threshold**: <16ms render time

**Risk**: Git diff extraction slow on large repos
- **Mitigation**: 5s timeout, background execution
- **Validation**: Test on Linux kernel repo
- **Threshold**: <50ms for typical repos, timeout on huge

### Functional Risks

**Risk**: Watcher doesn't detect changes
- **Mitigation**: Fallback to manual reload command
- **Validation**: Test on all platforms
- **Recovery**: Disable watcher, log warning

**Risk**: Fuzzy search produces poor results
- **Mitigation**: Use proven `nucleo-matcher` library
- **Validation**: Test with common search patterns
- **Recovery**: Fall back to substring match

---

## Testing Strategy

### Test Coverage Targets

| Component | Unit Tests | Integration Tests | Snapshot Tests |
|-----------|-----------|------------------|----------------|
| Context Enhancement | 5+ | 3+ | - |
| Hot-Reload | 4+ | 2+ | - |
| TUI Palette | 3+ | 2+ | 3+ |
| **Total** | **12+** | **7+** | **3+** |

### Critical Test Scenarios

**Day 15:**
- [ ] Git diff extraction in/out of repo
- [ ] Env var whitelist (no secrets)
- [ ] Context builder with all fields
- [ ] Template expansion with new fields
- [ ] Performance: <10ms overhead

**Days 16-17:**
- [ ] File change triggers reload
- [ ] Rapid changes debounced
- [ ] Invalid files don't crash
- [ ] Reload performance <1s for 100 commands

**Days 18-20:**
- [ ] Palette toggle
- [ ] Keyboard navigation
- [ ] Fuzzy search accuracy
- [ ] Command execution
- [ ] Rendering performance

---

## Success Criteria

### Week 2 Exit Criteria

**Functional:**
- [ ] Context enhancement complete (Day 15)
- [ ] Hot-reload works reliably (Days 16-17)
- [ ] TUI palette responsive (Days 18-20)
- [ ] All features tested
- [ ] Documentation complete

**Quality:**
- [ ] ≥85% test coverage maintained
- [ ] All performance targets met
- [ ] No memory leaks (validated with Valgrind)
- [ ] Zero critical security issues

**Performance:**
- [ ] Context overhead: <10ms
- [ ] Reload time: <1s for 100 commands
- [ ] Palette render: <16ms
- [ ] Overall: No user-perceivable lag

---

## Recommendations

### Execution Order

**Recommended Sequence:**

1. **Day 15 (Monday)** - Start with context enhancement
   - Low risk, high value
   - Unlocks richer command templates
   - Can be completed in one focused day

2. **Days 16-17 (Tuesday-Wednesday)** - Hot-reload system
   - More complex, allocate 2 days
   - Test thoroughly with profiling
   - Document performance characteristics

3. **Days 18-20 (Thursday-Saturday)** - TUI palette
   - Most visible feature
   - Allocate 3 days for polish
   - Snapshot test heavily

### De-risking Strategies

**Day 15:**
- Start with git diff (highest value)
- Env vars are simple, do last
- Skip conversation context if time tight (future sprint)

**Days 16-17:**
- Profile early and often
- Test on slow file systems (NFS, network drives)
- Have manual reload fallback

**Days 18-20:**
- Build basic palette first (days 18-19)
- Fuzzy search can be day 20
- Argument hints are optional (nice-to-have)

### Contingency Plans

**If Day 15 takes longer:**
- Skip conversation context (future enhancement)
- Focus on git diff + env vars

**If Hot-Reload has performance issues:**
- Disable by default with config flag
- Manual reload command as alternative
- Document in release notes

**If TUI Palette is complex:**
- Ship basic version without fuzzy search
- Add fuzzy search in Sprint 3
- Argument hints → Sprint 4

---

## Conclusion

Sprint 2 Week 2 builds on the solid foundation of Week 1, adding powerful enhancements that make the command system production-ready:

✅ **Day 15** - Richer context enables smarter commands
✅ **Days 16-17** - Hot-reload streamlines development
✅ **Days 18-20** - TUI palette enhances discoverability

**Total Effort:** 34 hours across 6 days
**Complexity:** Medium (manageable with good planning)
**Risk:** Medium (mitigated with profiling and fallbacks)

**Confidence Level:** 🟢 **High** - Clear path, proven patterns

---

**Plan Status:** ✅ Ready for Execution
**Next Action:** Begin Day 15 - Context Enhancement

---

*End of Detailed Plan*
