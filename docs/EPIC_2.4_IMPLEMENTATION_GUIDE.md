# Epic 2.4: TUI Command Palette - Implementation Guide

**Status**: 📋 **READY FOR IMPLEMENTATION**
**Duration**: Days 18-20 (Sprint 2, Week 2)
**Estimated Effort**: 16 hours over 3 days

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture Decisions](#architecture-decisions)
3. [Dependencies](#dependencies)
4. [Implementation Plan](#implementation-plan)
5. [Code Scaffolding](#code-scaffolding)
6. [Testing Strategy](#testing-strategy)
7. [Performance Requirements](#performance-requirements)
8. [Acceptance Criteria](#acceptance-criteria)

---

## Overview

### Epic Goal

**Build an interactive command palette (Ctrl+K) with fuzzy search, keyboard navigation, and command execution for enhanced command discovery.**

### Key Features

- ✅ Ctrl+K shortcut to toggle palette
- ✅ Fuzzy search with nucleo-matcher
- ✅ Keyboard navigation (↑/↓ arrows, Enter, Esc)
- ✅ Centered overlay UI with Ratatui
- ✅ Real-time command filtering
- ⏳ Argument autocomplete (deferred to Sprint 3)

### UX Flow

```
User presses Ctrl+K
  ↓
Palette opens (centered overlay)
  ↓
User types to filter commands (fuzzy search)
  ↓
Arrow keys navigate, Enter executes
  ↓
Esc closes palette
```

### Visual Design

```
┌─────────────────────────────────────────────────────┐
│  Commands (Ctrl+K to close)                         │
├─────────────────────────────────────────────────────┤
│  > search-text_                                     │
├─────────────────────────────────────────────────────┤
│  > explain                 Explain code in simple..│
│    review                  Code review assistant    │
│    test                    Generate comprehensive..│
│    custom-command          User-defined command     │
│                                                     │
│  3 of 4 commands                                    │
└─────────────────────────────────────────────────────┘
```

---

## Architecture Decisions

### AD-1: Command Source Strategy

**Decision**: Access CommandRegistry via App → Session bridge

**Rationale**:
- TUI crate cannot directly depend on core crate (circular dependency)
- Session already holds CommandRegistry
- Use message passing for command data

**Implementation**:
```rust
// In tui/src/app.rs
pub async fn load_commands(&mut self) -> Vec<CommandInfo> {
    if let Some(session) = &self.chat_widget.session {
        // Access registry via session services
        session.list_commands().await
    } else {
        Vec::new()
    }
}
```

### AD-2: Fuzzy Search Library

**Decision**: Use `nucleo-matcher` crate (already in workspace)

**Rationale**:
- Already available in TUI dependencies
- High-performance fuzzy matching
- Used by neovim and other editors
- <10ms for 100+ commands

**Alternative Considered**: `fuzzy-matcher` (decided against: slower)

### AD-3: Rendering Strategy

**Decision**: Render as centered overlay using Ratatui's `Clear` widget

**Rationale**:
- Non-invasive to existing TUI layout
- Standard pattern for popups in Ratatui
- Clean visual separation

**Implementation**: See scaffold in Section 5.1

### AD-4: Keyboard Handling

**Decision**: Intercept Ctrl+K at app level, delegate other keys when palette is open

**Rationale**:
- Ctrl+K must work from any state
- Palette owns navigation when visible
- Clean separation of concerns

---

## Dependencies

### Required Crates

Add to `tui/Cargo.toml`:

```toml
[dependencies]
# Existing dependencies...

# Fuzzy matching (already present)
nucleo-matcher = "0.3.1"
```

**Verification**:
```bash
cd tui
grep "nucleo-matcher" Cargo.toml
# Should output: nucleo-matcher = "0.3.1"
```

### Core Integration

**No new dependencies required** - CommandRegistry access via Session.

---

## Implementation Plan

### Day 18: Widget Foundation (4 hours)

**Tasks**:
1. Create `tui/src/command_palette.rs` (1 hour)
2. Implement basic rendering (1.5 hours)
3. Add toggle functionality (0.5 hours)
4. Write widget unit tests (1 hour)

**Deliverables**:
- CommandPalette struct
- Basic render() method
- toggle() method
- 3 unit tests

### Day 19: Keyboard & Fuzzy Search (6 hours)

**Tasks**:
1. Implement keyboard navigation (2 hours)
2. Add fuzzy search with nucleo-matcher (2 hours)
3. Write keyboard handling tests (1 hour)
4. Write fuzzy search tests (1 hour)

**Deliverables**:
- handle_key() method
- update_filtered_fuzzy() method
- 6 additional tests

### Day 20: App Integration & Polish (6 hours)

**Tasks**:
1. Integrate with App event loop (2 hours)
2. Add command loading from Session (1.5 hours)
3. Connect command execution (1 hour)
4. Performance testing & optimization (1 hour)
5. Final documentation (0.5 hours)

**Deliverables**:
- Working Ctrl+K in TUI
- Command execution flow
- Performance report
- Completion documentation

---

## Code Scaffolding

### 5.1: CommandPalette Widget

**File**: `tui/src/command_palette.rs`

```rust
//! Command palette widget for TUI.
//!
//! Provides a Ctrl+K triggered overlay for discovering and executing commands
//! with fuzzy search and keyboard navigation.

use nucleo_matcher::{Config, Matcher};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};

/// Information about a command for display.
#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub category: String,
}

/// Command palette state and rendering.
pub struct CommandPalette {
    /// Whether palette is visible
    visible: bool,

    /// Current filter text
    filter: String,

    /// Selected command index (into filtered list)
    selected: usize,

    /// All available commands
    commands: Vec<CommandInfo>,

    /// Filtered command indices (after fuzzy search)
    filtered: Vec<usize>,

    /// Fuzzy matcher instance
    matcher: Matcher,
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandPalette {
    /// Creates a new command palette.
    pub fn new() -> Self {
        Self {
            visible: false,
            filter: String::new(),
            selected: 0,
            commands: Vec::new(),
            filtered: Vec::new(),
            matcher: Matcher::new(Config::DEFAULT),
        }
    }

    /// Toggles palette visibility.
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        if !self.visible {
            // Reset state when closing
            self.filter.clear();
            self.selected = 0;
            self.update_filtered();
        }
    }

    /// Returns true if palette is currently visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Loads commands into the palette.
    pub fn load_commands(&mut self, commands: Vec<CommandInfo>) {
        self.commands = commands;
        self.update_filtered();
    }

    /// Returns the currently selected command name, if any.
    pub fn selected_command(&self) -> Option<&str> {
        self.filtered
            .get(self.selected)
            .and_then(|&idx| self.commands.get(idx))
            .map(|cmd| cmd.name.as_str())
    }

    /// Handles keyboard input.
    ///
    /// Returns Some(PaletteAction) if an action should be taken.
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Option<PaletteAction> {
        use crossterm::event::{KeyCode, KeyModifiers};

        match (key.code, key.modifiers) {
            // Close palette
            (KeyCode::Esc, _) => {
                self.toggle();
                None
            }

            // Execute selected command
            (KeyCode::Enter, _) => {
                if let Some(cmd_name) = self.selected_command() {
                    let cmd = cmd_name.to_string();
                    self.toggle();
                    Some(PaletteAction::ExecuteCommand(cmd))
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

    /// Updates filtered list based on current filter using fuzzy search.
    fn update_filtered(&mut self) {
        if self.filter.is_empty() {
            // No filter - show all commands
            self.filtered = (0..self.commands.len()).collect();
        } else {
            // Fuzzy search
            let mut scored: Vec<(usize, i32)> = self
                .commands
                .iter()
                .enumerate()
                .filter_map(|(idx, cmd)| {
                    // Match against name and description
                    let name_score = self.matcher.fuzzy_match(&cmd.name, &self.filter);
                    let desc_score = self
                        .matcher
                        .fuzzy_match(&cmd.description.to_lowercase(), &self.filter.to_lowercase());

                    // Take best score
                    let score = name_score.max(desc_score);
                    score.map(|s| (idx, s))
                })
                .collect();

            // Sort by score (descending)
            scored.sort_by(|a, b| b.1.cmp(&a.1));

            // Extract indices
            self.filtered = scored.into_iter().map(|(idx, _)| idx).collect();
        }

        // Reset selection if out of bounds
        if self.selected >= self.filtered.len() {
            self.selected = 0;
        }
    }

    /// Renders the palette as a centered overlay.
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

        // Split into sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Filter input
                Constraint::Min(0),    // Command list
                Constraint::Length(1), // Footer
            ])
            .split(inner);

        // Render filter input
        let filter_text = format!("> {}_", self.filter);
        Paragraph::new(filter_text)
            .style(Style::default().bold())
            .render(chunks[0], buf);

        // Render command list
        let items: Vec<ListItem> = self
            .filtered
            .iter()
            .enumerate()
            .map(|(i, &cmd_idx)| {
                let cmd = &self.commands[cmd_idx];
                let style = if i == self.selected {
                    Style::default().bg(Color::DarkGray).bold()
                } else {
                    Style::default()
                };

                let content = format!("{:<20} {}", cmd.name.cyan(), cmd.description.dim());

                ListItem::new(content).style(style)
            })
            .collect();

        List::new(items).render(chunks[1], buf);

        // Render footer
        let footer = format!("{} of {} commands", self.filtered.len(), self.commands.len());
        Paragraph::new(footer.dim()).render(chunks[2], buf);
    }
}

/// Actions that can be triggered by the palette.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaletteAction {
    /// Execute the given command
    ExecuteCommand(String),
}

/// Helper to create a centered rectangle.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn test_commands() -> Vec<CommandInfo> {
        vec![
            CommandInfo {
                name: "explain".to_string(),
                description: "Explain code in simple terms".to_string(),
                category: "analysis".to_string(),
            },
            CommandInfo {
                name: "review".to_string(),
                description: "Code review assistant".to_string(),
                category: "analysis".to_string(),
            },
            CommandInfo {
                name: "test".to_string(),
                description: "Generate comprehensive tests".to_string(),
                category: "testing".to_string(),
            },
        ]
    }

    #[test]
    fn test_palette_toggle() {
        let mut palette = CommandPalette::new();
        assert!(!palette.is_visible());

        palette.toggle();
        assert!(palette.is_visible());

        palette.toggle();
        assert!(!palette.is_visible());
    }

    #[test]
    fn test_load_commands() {
        let mut palette = CommandPalette::new();
        let commands = test_commands();

        palette.load_commands(commands.clone());
        assert_eq!(palette.commands.len(), 3);
        assert_eq!(palette.filtered.len(), 3);
    }

    #[test]
    fn test_fuzzy_search_exact() {
        let mut palette = CommandPalette::new();
        palette.load_commands(test_commands());

        palette.filter = "explain".to_string();
        palette.update_filtered();

        assert_eq!(palette.filtered.len(), 1);
        assert_eq!(palette.commands[palette.filtered[0]].name, "explain");
    }

    #[test]
    fn test_fuzzy_search_partial() {
        let mut palette = CommandPalette::new();
        palette.load_commands(test_commands());

        palette.filter = "rv".to_string();
        palette.update_filtered();

        assert!(palette.filtered.len() >= 1);
        assert_eq!(palette.commands[palette.filtered[0]].name, "review");
    }

    #[test]
    fn test_keyboard_navigation_down() {
        let mut palette = CommandPalette::new();
        palette.load_commands(test_commands());
        palette.toggle();

        assert_eq!(palette.selected, 0);

        let key = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
        palette.handle_key(key);

        assert_eq!(palette.selected, 1);
    }

    #[test]
    fn test_keyboard_navigation_up() {
        let mut palette = CommandPalette::new();
        palette.load_commands(test_commands());
        palette.toggle();
        palette.selected = 1;

        let key = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
        palette.handle_key(key);

        assert_eq!(palette.selected, 0);
    }

    #[test]
    fn test_keyboard_filter_input() {
        let mut palette = CommandPalette::new();
        palette.load_commands(test_commands());
        palette.toggle();

        let key = KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE);
        palette.handle_key(key);

        assert_eq!(palette.filter, "e");
    }

    #[test]
    fn test_keyboard_enter_executes() {
        let mut palette = CommandPalette::new();
        palette.load_commands(test_commands());
        palette.toggle();

        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        let action = palette.handle_key(key);

        assert_eq!(action, Some(PaletteAction::ExecuteCommand("explain".to_string())));
        assert!(!palette.is_visible()); // Palette should close
    }

    #[test]
    fn test_keyboard_esc_closes() {
        let mut palette = CommandPalette::new();
        palette.toggle();
        assert!(palette.is_visible());

        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        palette.handle_key(key);

        assert!(!palette.is_visible());
    }

    #[test]
    fn test_filter_reset_on_close() {
        let mut palette = CommandPalette::new();
        palette.load_commands(test_commands());
        palette.toggle();

        palette.filter = "test".to_string();
        palette.update_filtered();

        palette.toggle(); // Close

        assert_eq!(palette.filter, "");
        assert_eq!(palette.selected, 0);
    }
}
```

---

### 5.2: App Integration

**File**: `tui/src/app.rs` (modifications)

**Step 1**: Add CommandPalette field

```rust
// Near the top of app.rs, add import
mod command_palette;
use command_palette::{CommandPalette, CommandInfo, PaletteAction};

// In struct App
pub(crate) struct App {
    // ... existing fields

    /// Command palette for Ctrl+K
    pub(crate) command_palette: CommandPalette,
}
```

**Step 2**: Initialize in `App::run()`

```rust
// In App::run(), after creating chat_widget
let mut app = Self {
    server: conversation_manager,
    app_event_tx: app_event_tx.clone(),
    chat_widget,
    auth_manager,
    config,
    active_profile,
    file_search: FileSearchManager::new(),
    transcript_cells: Vec::new(),
    overlay: None,
    deferred_history_lines: Vec::new(),
    has_emitted_history_lines: false,
    enhanced_keys_supported,
    commit_anim_running: Arc::new(AtomicBool::new(false)),
    backtrack: BacktrackState::new(),
    command_palette: CommandPalette::new(), // ← NEW
};
```

**Step 3**: Handle Ctrl+K in event loop

```rust
// In the main event loop, BEFORE other keyboard handling
loop {
    select! {
        Some(event) = tui_rx.next() => {
            match event {
                TuiEvent::Key(key) if key.kind == KeyEventKind::Press => {
                    // Check for Ctrl+K FIRST (highest priority)
                    if matches!(key.code, KeyCode::Char('k')) &&
                       key.modifiers.contains(KeyModifiers::CONTROL) {
                        app.command_palette.toggle();

                        // Load commands when opened
                        if app.command_palette.is_visible() {
                            let commands = app.load_commands_for_palette().await;
                            app.command_palette.load_commands(commands);
                        }

                        continue;
                    }

                    // If palette is visible, it gets first priority for keys
                    if app.command_palette.is_visible() {
                        if let Some(action) = app.command_palette.handle_key(key) {
                            match action {
                                PaletteAction::ExecuteCommand(cmd_name) => {
                                    // Insert command into input
                                    let command_text = format!("/{}", cmd_name);
                                    app.chat_widget.insert_text(&command_text);
                                    // Optionally submit immediately:
                                    // app.chat_widget.submit().await?;
                                }
                            }
                        }
                        continue;
                    }

                    // ... rest of keyboard handling
                }
                // ... other events
            }
        }
    }
}
```

**Step 4**: Add command loading method

```rust
impl App {
    /// Loads commands for the palette from the session's registry.
    async fn load_commands_for_palette(&self) -> Vec<CommandInfo> {
        // Access CommandRegistry via Session
        // NOTE: This requires Session to expose a method to list commands

        // Placeholder implementation:
        vec![
            CommandInfo {
                name: "explain".to_string(),
                description: "Explain code in simple terms".to_string(),
                category: "analysis".to_string(),
            },
            CommandInfo {
                name: "review".to_string(),
                description: "Code review assistant".to_string(),
                category: "analysis".to_string(),
            },
            CommandInfo {
                name: "test".to_string(),
                description: "Generate comprehensive tests".to_string(),
                category: "testing".to_string(),
            },
        ]

        // TODO: Replace with actual Session integration:
        // if let Some(session) = &self.chat_widget.session {
        //     session.services.command_registry
        //         .as_ref()
        //         .map(|reg| reg.list().await)
        //         .unwrap_or_default()
        //         .into_iter()
        //         .map(|cmd| CommandInfo {
        //             name: cmd.name().to_string(),
        //             description: cmd.description().to_string(),
        //             category: format!("{:?}", cmd.category()),
        //         })
        //         .collect()
        // } else {
        //     Vec::new()
        // }
    }
}
```

**Step 5**: Add rendering in `App::draw()`

```rust
// In the render/draw method (find where main UI is rendered)
fn draw(&mut self, frame: &mut Frame) {
    // ... existing rendering code

    // Render palette LAST (overlay on top)
    self.command_palette.render(frame.size(), frame.buffer_mut());
}
```

---

### 5.3: Core Integration (Session Access)

**File**: `core/src/lib.rs` (add public export if needed)

```rust
// Make sure CommandRegistry is accessible
pub use commands::CommandRegistry;
```

**File**: `core/src/session.rs` or `core/src/codex.rs`

Add method to Session to expose commands:

```rust
impl Session {
    /// Lists available commands from the registry.
    pub async fn list_commands(&self) -> Vec<CommandInfo> {
        if let Some(ref registry) = self.services.command_registry {
            registry.list().await
                .into_iter()
                .map(|cmd| CommandInfo {
                    name: cmd.name().to_string(),
                    description: cmd.description().to_string(),
                    category: format!("{:?}", cmd.category()),
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

// Define CommandInfo as public type
#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub category: String,
}
```

---

## Testing Strategy

### Unit Tests (8 tests)

**Location**: `tui/src/command_palette.rs::tests`

Tests included in scaffold (Section 5.1):
1. ✅ `test_palette_toggle` - Toggle visibility
2. ✅ `test_load_commands` - Load command data
3. ✅ `test_fuzzy_search_exact` - Exact match search
4. ✅ `test_fuzzy_search_partial` - Fuzzy match (e.g., "rv" → "review")
5. ✅ `test_keyboard_navigation_down` - Down arrow
6. ✅ `test_keyboard_navigation_up` - Up arrow
7. ✅ `test_keyboard_filter_input` - Character input
8. ✅ `test_keyboard_enter_executes` - Enter key execution
9. ✅ `test_keyboard_esc_closes` - Esc key closes
10. ✅ `test_filter_reset_on_close` - State reset on close

**Run Tests**:
```bash
cargo test -p codex-tui command_palette::tests
```

### Integration Tests

**Manual Testing Checklist**:

```bash
# Build and run TUI
cargo run --bin codex

# Test Cases:
1. [ ] Press Ctrl+K → Palette opens
2. [ ] Press Ctrl+K again → Palette closes
3. [ ] Type "exp" → "explain" command highlighted
4. [ ] Press ↓ arrow → Selection moves down
5. [ ] Press ↑ arrow → Selection moves up
6. [ ] Press Enter → Command inserted into input
7. [ ] Press Esc → Palette closes
8. [ ] Type "rv" → "review" appears (fuzzy match)
9. [ ] Press Backspace → Filter updates
10. [ ] With 0 commands → Shows "0 of 0 commands"
```

### Performance Tests

**Fuzzy Search Performance**:

```rust
#[test]
fn bench_fuzzy_search_100_commands() {
    use std::time::Instant;

    let mut palette = CommandPalette::new();
    let commands: Vec<CommandInfo> = (0..100)
        .map(|i| CommandInfo {
            name: format!("command-{}", i),
            description: format!("Description for command {}", i),
            category: "test".to_string(),
        })
        .collect();

    palette.load_commands(commands);
    palette.filter = "cmd".to_string();

    let start = Instant::now();
    palette.update_filtered();
    let elapsed = start.elapsed();

    assert!(elapsed.as_millis() < 10, "Fuzzy search took {}ms, expected <10ms", elapsed.as_millis());
}
```

**Render Performance**:

```rust
#[test]
fn bench_render_time() {
    use std::time::Instant;

    let mut palette = CommandPalette::new();
    palette.load_commands(test_commands());
    palette.toggle();

    let mut buf = Buffer::empty(Rect::new(0, 0, 80, 24));

    let start = Instant::now();
    palette.render(Rect::new(0, 0, 80, 24), &mut buf);
    let elapsed = start.elapsed();

    assert!(elapsed.as_millis() < 16, "Render took {}ms, expected <16ms (60fps)", elapsed.as_millis());
}
```

---

## Performance Requirements

### Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Fuzzy search | <10ms | 100 commands |
| Palette render | <16ms | 60fps (16.67ms frame) |
| Toggle latency | <5ms | Ctrl+K → visible |
| Memory overhead | <1MB | Palette + commands |
| Filter update | <5ms | Per keystroke |

### Optimization Guidelines

**Fuzzy Search**:
- ✅ Use nucleo-matcher (already optimized)
- ✅ Cache filtered results
- ✅ Update only on filter change

**Rendering**:
- ✅ Render only when visible
- ✅ Use Ratatui's efficient buffer updates
- ✅ Limit command list to visible area

**Memory**:
- ✅ CommandInfo is lightweight (3 Strings)
- ✅ Filtered list stores indices, not clones
- ✅ Single Matcher instance

---

## Acceptance Criteria

### Functional Requirements

- [ ] **FR-1**: Ctrl+K toggles palette visibility
- [ ] **FR-2**: Palette renders as centered overlay
- [ ] **FR-3**: Filter input visible with cursor
- [ ] **FR-4**: Command list displays with selection highlight
- [ ] **FR-5**: Up/Down arrows navigate selection
- [ ] **FR-6**: Enter executes selected command
- [ ] **FR-7**: Esc closes palette
- [ ] **FR-8**: Character input updates filter
- [ ] **FR-9**: Backspace removes filter characters
- [ ] **FR-10**: Fuzzy search matches partial strings
- [ ] **FR-11**: Results sorted by relevance
- [ ] **FR-12**: Footer shows "X of Y commands"

### Non-Functional Requirements

- [ ] **NFR-1**: Fuzzy search <10ms for 100 commands
- [ ] **NFR-2**: Render time <16ms (60fps)
- [ ] **NFR-3**: Zero crashes or panics
- [ ] **NFR-4**: Palette state resets on close
- [ ] **NFR-5**: No memory leaks
- [ ] **NFR-6**: Accessible via keyboard only

### Quality Gates

- [ ] **QG-1**: All 10 unit tests pass
- [ ] **QG-2**: Manual test checklist 10/10
- [ ] **QG-3**: Performance tests pass
- [ ] **QG-4**: Code compiles without warnings
- [ ] **QG-5**: Clippy clean
- [ ] **QG-6**: rustfmt formatted

---

## Implementation Steps

### Step 1: Create Widget File

```bash
cd tui/src
touch command_palette.rs
```

Copy scaffold from Section 5.1 into `command_palette.rs`.

### Step 2: Add Module to lib.rs

```rust
// In tui/src/lib.rs
mod command_palette;
```

### Step 3: Verify Tests Pass

```bash
cargo test -p codex-tui command_palette::tests
```

Expected: 10/10 tests pass.

### Step 4: Integrate with App

Follow integration steps from Section 5.2:
1. Add field to App struct
2. Initialize in App::run()
3. Handle Ctrl+K in event loop
4. Add command loading method
5. Add rendering call

### Step 5: Test Compilation

```bash
cargo check -p codex-tui
```

Expected: No errors.

### Step 6: Manual Testing

Run TUI and test all 10 manual test cases.

### Step 7: Performance Testing

Run performance benchmarks and verify targets met.

### Step 8: Documentation

Update completion report and mark Epic 2.4 complete.

---

## Known Limitations & Future Work

### Current Scope (Epic 2.4)

- ✅ Basic command palette with fuzzy search
- ✅ Keyboard navigation
- ✅ Command execution (insert into input)

### Deferred to Sprint 3

- ⏳ Argument autocomplete
- ⏳ Command preview pane
- ⏳ Command history/favorites
- ⏳ Keyboard shortcuts display
- ⏳ Command categories filtering

### Architectural Debt

1. **Session Access Pattern**: Currently uses placeholder. Need proper Session → CommandRegistry bridge.
2. **TUI-Core Coupling**: Consider extracting command types to shared crate.
3. **Command Execution**: Currently inserts text. Consider direct execution API.

---

## Troubleshooting

### Issue: Palette doesn't appear

**Check**:
1. Ctrl+K handling in event loop (before other keys)
2. `command_palette.toggle()` called
3. `command_palette.render()` called LAST in draw()

### Issue: Fuzzy search slow

**Check**:
1. Using `nucleo-matcher` (not string contains)
2. Command list size (<1000 commands)
3. update_filtered() only called on filter change

### Issue: Keyboard navigation broken

**Check**:
1. `handle_key()` called when palette visible
2. Event loop continues after palette handles key
3. Selection bounds checking works

---

## References

### Documentation

- Ratatui Guide: https://ratatui.rs/
- nucleo-matcher: https://docs.rs/nucleo-matcher/
- Codex TUI Architecture: `tui/README.md`

### Related Files

- `tui/src/app.rs` - Main TUI app
- `tui/src/chatwidget.rs` - Chat widget
- `core/src/commands/registry.rs` - Command registry
- `core/src/commands/mod.rs` - Command system

### Prior Art

- VSCode Command Palette (Ctrl+Shift+P)
- Neovim Telescope
- Emacs Helm/Ivy

---

## Completion Checklist

- [ ] Widget created and tested
- [ ] App integration complete
- [ ] All unit tests pass (10/10)
- [ ] Manual tests pass (10/10)
- [ ] Performance tests pass
- [ ] Documentation complete
- [ ] Epic 2.4 completion report created

---

**Guide Version**: 1.0
**Last Updated**: October 9, 2025
**Author**: Claude (AI Development Agent)
**Status**: Ready for implementation
