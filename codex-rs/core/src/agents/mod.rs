//! Agent system module for Codex.
//!
//! This module provides a flexible agent framework that enables context-aware,
//! specialized AI agents to perform targeted tasks like code review, security
//! analysis, and refactoring assistance.
//!
//! # Architecture
//!
//! - `router`: Context-based agent selection and routing
//! - `toolkit`: Agent execution toolkit with permission enforcement
//! - `permissions`: Permission model for agent operations
//! - `builtin`: Built-in agents shipped with Codex
//!
//! # Core Concepts
//!
//! ## Agent Trait
//!
//! All agents implement the `Agent` trait, which defines:
//! - Unique identification
//! - Context analysis for activation scoring
//! - Task execution with toolkit
//! - Permission requirements
//!
//! ## Activation Scoring
//!
//! Agents analyze task context and return an activation score (0.0-1.0)
//! indicating their suitability for the task. The router selects the
//! highest-scoring agent above the activation threshold.
//!
//! ## Toolkit
//!
//! Agents execute tasks using a toolkit that provides:
//! - File system access (with permission validation)
//! - Shell command execution (through execpolicy)
//! - Workspace context and utilities

use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;

pub mod builtin;
pub mod permissions;
pub mod router;
pub mod toolkit;

// Re-export key types
pub use permissions::AgentPermissions;
pub use router::AgentRouter;
pub use toolkit::AgentToolkit;

/// Unique identifier for an agent.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AgentId(String);

impl AgentId {
    /// Creates a new agent ID.
    pub fn from(s: &str) -> Self {
        Self(s.to_string())
    }

    /// Returns the agent ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Activation score indicating agent suitability for a task.
///
/// Scores range from 0.0 (not suitable) to 1.0 (perfect match).
#[derive(Debug, Clone, Copy)]
pub struct ActivationScore(pub f64);

impl ActivationScore {
    /// Creates a new activation score.
    ///
    /// Clamps value to [0.0, 1.0] range.
    pub fn new(score: f64) -> Self {
        Self(score.clamp(0.0, 1.0))
    }
}

/// Core agent trait.
///
/// All agents must implement this trait to participate in the agent system.
#[async_trait]
pub trait Agent: Send + Sync {
    /// Returns the unique agent identifier.
    fn id(&self) -> AgentId;

    /// Returns the human-readable agent name.
    fn name(&self) -> &str;

    /// Returns a description of the agent's expertise.
    fn description(&self) -> &str;

    /// Analyzes context and returns activation score.
    ///
    /// Higher scores indicate better suitability for the task.
    fn can_handle(&self, context: &TaskContext) -> ActivationScore;

    /// Executes a task with the provided toolkit.
    async fn execute(&self, task: Task, toolkit: &AgentToolkit) -> anyhow::Result<AgentResult>;

    /// Returns the agent's permission requirements.
    fn permissions(&self) -> &AgentPermissions;

    /// Returns the system prompt defining agent persona.
    fn system_prompt(&self) -> &str;
}

/// Context for task analysis and execution.
#[derive(Debug, Clone, Default)]
pub struct TaskContext {
    /// File paths relevant to the task.
    pub file_paths: Vec<PathBuf>,
    /// File contents (loaded on demand).
    pub file_contents: Option<HashMap<PathBuf, String>>,
    /// Git context information.
    pub git_context: Option<GitContext>,
    /// Execution mode.
    pub execution_mode: ExecutionMode,
    /// User's intent/instructions.
    pub user_intent: String,
}

/// Git context information.
#[derive(Debug, Clone)]
pub struct GitContext {
    pub diff: String,
    pub branch: String,
    pub changed_files: Vec<PathBuf>,
}

/// Execution mode for agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    /// Interactive mode with user feedback.
    Interactive,
    /// Automated mode without user interaction.
    Automated,
}

impl Default for ExecutionMode {
    fn default() -> Self {
        Self::Interactive
    }
}

/// Task for agent execution.
#[derive(Debug, Clone)]
pub struct Task {
    pub context: TaskContext,
    pub additional_instructions: Option<String>,
}

/// Result from agent execution.
#[derive(Debug, Clone)]
pub enum AgentResult {
    /// Code review findings.
    CodeReview { findings: Vec<CodeReviewFinding> },
    /// Analysis results.
    Analysis {
        summary: String,
        details: HashMap<String, String>,
    },
    /// Suggestions for improvements.
    Suggestions { items: Vec<Suggestion> },
}

/// Code review finding.
#[derive(Debug, Clone)]
pub struct CodeReviewFinding {
    pub severity: Severity,
    pub category: String,
    pub message: String,
    pub location: Option<PathBuf>,
    pub line_number: Option<usize>,
}

/// Severity level for findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Suggestion for improvement.
#[derive(Debug, Clone)]
pub struct Suggestion {
    pub title: String,
    pub description: String,
    pub code_change: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_id() {
        let id = AgentId::from("test-agent");
        assert_eq!(id.as_str(), "test-agent");
    }

    #[test]
    fn test_activation_score_clamping() {
        assert_eq!(ActivationScore::new(-0.5).0, 0.0);
        assert_eq!(ActivationScore::new(1.5).0, 1.0);
        assert_eq!(ActivationScore::new(0.7).0, 0.7);
    }
}
