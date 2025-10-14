//! Command file watcher for hot-reload functionality.
//!
//! This module provides automatic command registry reload when command files
//! in `~/.codex/commands/` are created, modified, or deleted.

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use super::registry::CommandRegistry;

/// Debounce duration for file system events (milliseconds).
const DEBOUNCE_MS: u64 = 300;

/// Command file watcher that monitors the commands directory for changes
/// and triggers registry reload.
pub struct CommandWatcher {
    _watcher: RecommendedWatcher,
    _shutdown_tx: mpsc::UnboundedSender<()>,
}

impl CommandWatcher {
    /// Creates a new command watcher for the given directory.
    ///
    /// # Arguments
    ///
    /// * `commands_dir` - Directory to watch for command files
    /// * `registry` - Command registry to reload on file changes
    ///
    /// # Returns
    ///
    /// Returns `Ok(CommandWatcher)` on success, or an error if the watcher
    /// cannot be initialized.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::sync::Arc;
    /// # use std::path::PathBuf;
    /// # use codex_core::commands::watcher::CommandWatcher;
    /// # use codex_core::commands::registry::CommandRegistry;
    /// # async fn example() -> anyhow::Result<()> {
    /// let registry = Arc::new(CommandRegistry::new(PathBuf::from("commands")).await?);
    /// let watcher = CommandWatcher::new(PathBuf::from("commands"), registry)?;
    /// // Watcher runs in background, automatically reloading on file changes
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        commands_dir: PathBuf,
        registry: Arc<CommandRegistry>,
    ) -> Result<Self, notify::Error> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let (shutdown_tx, shutdown_rx) = mpsc::unbounded_channel();

        // Create the file system watcher
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    // Only send relevant events
                    if Self::is_relevant_event(&event) {
                        let _ = event_tx.send(event);
                    }
                }
            },
            notify::Config::default(),
        )?;

        // Start watching the commands directory recursively
        watcher.watch(&commands_dir, RecursiveMode::Recursive)?;

        info!(
            "Command watcher started for directory: {}",
            commands_dir.display()
        );

        // Spawn background task to handle events
        tokio::spawn(Self::handle_events(
            event_rx,
            shutdown_rx,
            registry,
            Duration::from_millis(DEBOUNCE_MS),
        ));

        Ok(Self {
            _watcher: watcher,
            _shutdown_tx: shutdown_tx,
        })
    }

    /// Checks if an event is relevant for command reload.
    fn is_relevant_event(event: &Event) -> bool {
        matches!(
            event.kind,
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
        ) && event.paths.iter().any(|p| Self::is_command_file(p))
    }

    /// Checks if a path is a command file (.md extension).
    fn is_command_file(path: &Path) -> bool {
        path.extension().and_then(|s| s.to_str()) == Some("md")
    }

    /// Background task that handles file system events with debouncing.
    async fn handle_events(
        mut event_rx: mpsc::UnboundedReceiver<Event>,
        mut shutdown_rx: mpsc::UnboundedReceiver<()>,
        registry: Arc<CommandRegistry>,
        debounce_duration: Duration,
    ) {
        let mut pending_reloads: HashMap<PathBuf, Instant> = HashMap::new();
        let mut reload_timer = tokio::time::interval(Duration::from_millis(50));

        loop {
            tokio::select! {
                // Shutdown signal received
                _ = shutdown_rx.recv() => {
                    debug!("Command watcher shutting down");
                    break;
                }

                // File system event received
                Some(event) = event_rx.recv() => {
                    let now = Instant::now();
                    for path in event.paths {
                        if Self::is_command_file(&path) {
                            debug!("File event for command file: {}", path.display());
                            pending_reloads.insert(path, now);
                        }
                    }
                }

                // Check for debounced reloads
                _ = reload_timer.tick() => {
                    let now = Instant::now();
                    let mut should_reload = false;

                    // Check if any pending reload has passed the debounce duration
                    pending_reloads.retain(|path, &mut event_time| {
                        if now.duration_since(event_time) >= debounce_duration {
                            debug!("Debounce period elapsed for: {}", path.display());
                            should_reload = true;
                            false // Remove from pending
                        } else {
                            true // Keep in pending
                        }
                    });

                    if should_reload {
                        info!("Reloading command registry due to file changes");
                        match registry.reload().await {
                            Ok(()) => {
                                info!("Successfully reloaded command registry");
                            }
                            Err(e) => {
                                error!("Failed to reload command registry: {}", e);
                            }
                        }
                    }
                }
            }
        }

        debug!("Command watcher event handler terminated");
    }
}

impl Drop for CommandWatcher {
    fn drop(&mut self) {
        debug!("CommandWatcher being dropped, shutdown signal will be sent");
        // Shutdown signal is sent automatically when _shutdown_tx is dropped
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    async fn create_test_registry() -> (Arc<CommandRegistry>, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let commands_dir = temp_dir.path().to_path_buf();
        fs::create_dir_all(&commands_dir).unwrap();

        let registry = Arc::new(CommandRegistry::new(commands_dir.clone()).await.unwrap());

        (registry, temp_dir)
    }

    #[test]
    fn test_is_command_file_true() {
        assert!(CommandWatcher::is_command_file(Path::new("test.md")));
        assert!(CommandWatcher::is_command_file(Path::new(
            "/path/to/command.md"
        )));
    }

    #[test]
    fn test_is_command_file_false() {
        assert!(!CommandWatcher::is_command_file(Path::new("test.txt")));
        assert!(!CommandWatcher::is_command_file(Path::new("test.rs")));
        assert!(!CommandWatcher::is_command_file(Path::new("/path/to/file")));
    }

    #[test]
    fn test_is_relevant_event_create() {
        let event = Event {
            kind: EventKind::Create(notify::event::CreateKind::File),
            paths: vec![PathBuf::from("test.md")],
            attrs: Default::default(),
        };
        assert!(CommandWatcher::is_relevant_event(&event));
    }

    #[test]
    fn test_is_relevant_event_modify() {
        let event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Data(
                notify::event::DataChange::Any,
            )),
            paths: vec![PathBuf::from("test.md")],
            attrs: Default::default(),
        };
        assert!(CommandWatcher::is_relevant_event(&event));
    }

    #[test]
    fn test_is_relevant_event_remove() {
        let event = Event {
            kind: EventKind::Remove(notify::event::RemoveKind::File),
            paths: vec![PathBuf::from("test.md")],
            attrs: Default::default(),
        };
        assert!(CommandWatcher::is_relevant_event(&event));
    }

    #[test]
    fn test_is_relevant_event_non_md_file() {
        let event = Event {
            kind: EventKind::Create(notify::event::CreateKind::File),
            paths: vec![PathBuf::from("test.txt")],
            attrs: Default::default(),
        };
        assert!(!CommandWatcher::is_relevant_event(&event));
    }

    #[test]
    fn test_is_relevant_event_access() {
        let event = Event {
            kind: EventKind::Access(notify::event::AccessKind::Any),
            paths: vec![PathBuf::from("test.md")],
            attrs: Default::default(),
        };
        assert!(!CommandWatcher::is_relevant_event(&event));
    }

    #[tokio::test]
    async fn test_watcher_creation() {
        let (registry, temp_dir) = create_test_registry().await;
        let commands_dir = temp_dir.path().to_path_buf();

        let watcher = CommandWatcher::new(commands_dir, registry);
        assert!(watcher.is_ok());
    }

    #[tokio::test]
    async fn test_watcher_invalid_directory() {
        let (registry, _temp_dir) = create_test_registry().await;
        let invalid_path = PathBuf::from("/nonexistent/path/that/does/not/exist");

        let watcher = CommandWatcher::new(invalid_path, registry);
        assert!(watcher.is_err());
    }
}
