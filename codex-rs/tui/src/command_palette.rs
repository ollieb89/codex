//! Command palette widget for TUI.
//!
//! Provides a Ctrl+K triggered overlay for discovering and executing commands
//! with fuzzy search and keyboard navigation.
//!
//! # Usage
//!
//! ```ignore
//! use codex_tui::command_palette::{CommandPalette, CommandInfo};
//!
//! let mut palette = CommandPalette::new();
//! palette.load_commands(vec![
//!     CommandInfo {
//!         name: "explain".to_string(),
//!         description: "Explain code".to_string(),
//!         category: "analysis".to_string(),
//!         agent: false,
//!         agent_id: None,
//!     },
//! ]);
//!
//! palette.toggle(); // Show palette
//! ```

use nucleo_matcher::{Config, Matcher, Utf32String};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};

/// Information about a command for display.
#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    #[allow(dead_code)]
    pub category: String,
    /// Whether this command is agent-backed
    pub agent: bool,
    /// Optional agent ID for agent-backed commands
    #[allow(dead_code)]
    pub agent_id: Option<String>,
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
    ///
    /// When closing, resets filter and selection state.
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
    ///
    /// This should be called when the palette opens to get fresh command data.
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
    /// Returns `Some(PaletteAction)` if an action should be taken.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    ///
    /// let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
    /// if let Some(action) = palette.handle_key(key) {
    ///     match action {
    ///         PaletteAction::ExecuteCommand(name) => {
    ///             println!("Execute: {}", name);
    ///         }
    ///     }
    /// }
    /// ```
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
    ///
    /// Uses nucleo-matcher for high-performance fuzzy matching.
    fn update_filtered(&mut self) {
        if self.filter.is_empty() {
            // No filter - show all commands
            self.filtered = (0..self.commands.len()).collect();
        } else {
            // Fuzzy search - convert to Utf32String for nucleo-matcher
            let filter_utf32 = Utf32String::from(self.filter.as_str());
            let filter_slice = filter_utf32.slice(..);

            let mut scored: Vec<(usize, u16)> = self
                .commands
                .iter()
                .enumerate()
                .filter_map(|(idx, cmd)| {
                    // Convert strings to Utf32String
                    let name_utf32 = Utf32String::from(cmd.name.as_str());
                    let desc_utf32 = Utf32String::from(cmd.description.as_str());

                    // Match against name and description
                    let name_score = self.matcher.fuzzy_match(name_utf32.slice(..), filter_slice);
                    let desc_score = self.matcher.fuzzy_match(desc_utf32.slice(..), filter_slice);

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
    ///
    /// Call this LAST in your draw() method to ensure it overlays everything else.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// fn draw(&mut self, frame: &mut Frame) {
    ///     // ... render other widgets
    ///
    ///     // Render palette last (overlay on top)
    ///     self.command_palette.render(frame.size(), frame.buffer_mut());
    /// }
    /// ```
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

                // Show agent icon for agent-backed commands
                let prefix = if cmd.agent { "🤖 " } else { "   " };
                let content = format!(
                    "{}{:<20} {}",
                    prefix,
                    cmd.name.clone().cyan(),
                    cmd.description.clone().dim()
                );

                ListItem::new(content).style(style)
            })
            .collect();

        Widget::render(List::new(items), chunks[1], buf);

        // Render footer
        let footer = format!(
            "{} of {} commands",
            self.filtered.len(),
            self.commands.len()
        );
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
                agent: false,
                agent_id: None,
            },
            CommandInfo {
                name: "review".to_string(),
                description: "Code review assistant".to_string(),
                category: "analysis".to_string(),
                agent: true,
                agent_id: Some("review-agent".to_string()),
            },
            CommandInfo {
                name: "test".to_string(),
                description: "Generate comprehensive tests".to_string(),
                category: "testing".to_string(),
                agent: false,
                agent_id: None,
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

        assert_eq!(
            action,
            Some(PaletteAction::ExecuteCommand("explain".to_string()))
        );
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

    // Agent command display tests

    #[test]
    fn test_agent_command_metadata_preserved() {
        let mut palette = CommandPalette::new();
        palette.load_commands(test_commands());

        // Verify review command is agent-backed
        let review_cmd = &palette.commands[1];
        assert_eq!(review_cmd.name, "review");
        assert!(review_cmd.agent);
        assert_eq!(review_cmd.agent_id, Some("review-agent".to_string()));

        // Verify explain command is not agent-backed
        let explain_cmd = &palette.commands[0];
        assert_eq!(explain_cmd.name, "explain");
        assert!(!explain_cmd.agent);
        assert_eq!(explain_cmd.agent_id, None);
    }

    #[test]
    fn test_agent_commands_in_filtered_list() {
        let mut palette = CommandPalette::new();
        palette.load_commands(test_commands());

        // Filter for "review" (agent command)
        palette.filter = "review".to_string();
        palette.update_filtered();

        assert_eq!(palette.filtered.len(), 1);
        let cmd = &palette.commands[palette.filtered[0]];
        assert_eq!(cmd.name, "review");
        assert!(cmd.agent);
    }

    #[test]
    fn test_mixed_agent_and_normal_commands() {
        let mut palette = CommandPalette::new();
        palette.load_commands(test_commands());

        // Verify we have both types
        let agent_count = palette.commands.iter().filter(|c| c.agent).count();
        let normal_count = palette.commands.iter().filter(|c| !c.agent).count();

        assert_eq!(agent_count, 1); // "review" is agent-backed
        assert_eq!(normal_count, 2); // "explain" and "test" are normal
    }

    #[test]
    fn test_agent_command_selection() {
        let mut palette = CommandPalette::new();
        palette.load_commands(test_commands());
        palette.toggle();

        // Select the agent command (review at index 1)
        palette.selected = 1;
        let selected = palette.selected_command();

        assert_eq!(selected, Some("review"));
        assert!(palette.commands[1].agent);
    }
}
