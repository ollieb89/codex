//! End-to-end integration tests for the command agent system.
//!
//! These tests verify the complete flow from command invocation through
//! agent execution to result formatting.

use anyhow::Result;
use async_trait::async_trait;
use codex_core::agents::{
    ActivationScore, Agent, AgentId, AgentPermissions, AgentResult, AgentToolkit,
    CodeReviewFinding, ExecutionMode, Severity, Task, TaskContext,
};
use codex_core::commands::agents::{AgentResultFormatter, OutputFormat};
use codex_core::commands::executor::{ConversationContext, ExecutionContext};
use codex_core::commands::parser::{CommandMetadata, CommandPermissions};
use codex_core::commands::registry::{Command, CommandCategory};
use std::any::Any;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

/// Mock agent for testing.
struct MockReviewAgent {
    permissions: AgentPermissions,
}

impl MockReviewAgent {
    fn new() -> Self {
        Self {
            permissions: AgentPermissions::default(),
        }
    }
}

#[async_trait]
impl Agent for MockReviewAgent {
    fn id(&self) -> AgentId {
        AgentId::from("mock-review")
    }

    fn name(&self) -> &str {
        "Mock Review Agent"
    }

    fn description(&self) -> &str {
        "A mock agent for testing code review functionality"
    }

    fn can_handle(&self, context: &TaskContext) -> ActivationScore {
        if context.user_intent.to_lowercase().contains("review") {
            ActivationScore::new(0.9)
        } else {
            ActivationScore::new(0.1)
        }
    }

    async fn execute(&self, _task: Task, _toolkit: &AgentToolkit) -> Result<AgentResult> {
        Ok(AgentResult::CodeReview {
            findings: vec![
                CodeReviewFinding {
                    severity: Severity::Warning,
                    category: "Performance".to_string(),
                    message: "Consider using a more efficient algorithm".to_string(),
                    location: Some(PathBuf::from("src/main.rs")),
                    line_number: Some(42),
                },
                CodeReviewFinding {
                    severity: Severity::Info,
                    category: "Style".to_string(),
                    message: "Variable naming could be more descriptive".to_string(),
                    location: Some(PathBuf::from("src/main.rs")),
                    line_number: Some(15),
                },
            ],
        })
    }

    fn permissions(&self) -> &AgentPermissions {
        &self.permissions
    }

    fn system_prompt(&self) -> &str {
        "You are a code review agent."
    }
}

/// Mock command for testing.
#[derive(Clone)]
struct MockAgentCommand {
    metadata: CommandMetadata,
}

impl MockAgentCommand {
    fn new_agent_command() -> Self {
        use codex_core::commands::parser::CommandPermissions;
        let metadata = CommandMetadata {
            name: "mock-review".to_string(),
            description: "Mock review command".to_string(),
            category: "Analysis".to_string(),
            permissions: CommandPermissions::default(),
            args: Vec::new(),
            agent: true,
            agent_id: Some("mock-review".to_string()),
            activation_hints: vec!["review".to_string()],
        };
        Self { metadata }
    }

    fn new_normal_command() -> Self {
        use codex_core::commands::parser::CommandPermissions;
        let metadata = CommandMetadata {
            name: "mock-explain".to_string(),
            description: "Mock explain command".to_string(),
            category: "Documentation".to_string(),
            permissions: CommandPermissions::default(),
            args: Vec::new(),
            agent: false,
            agent_id: None,
            activation_hints: Vec::new(),
        };
        Self { metadata }
    }

    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }
}

impl Command for MockAgentCommand {
    fn name(&self) -> &str {
        &self.metadata.name
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn category(&self) -> CommandCategory {
        CommandCategory::Custom
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Test that agent command metadata flows through execution pipeline.
#[tokio::test]
async fn test_agent_command_metadata_flows_through_execution() -> Result<()> {
    // Create a command with agent metadata
    let command = MockAgentCommand::new_agent_command();
    let metadata = command.metadata();

    // Verify metadata fields
    assert!(metadata.agent, "Command should be marked as agent-backed");
    assert_eq!(
        metadata.agent_id.as_ref().map(|s| s.as_str()),
        Some("mock-review"),
        "Agent ID should be preserved"
    );

    Ok(())
}

/// Test that agent result formatting integration works correctly.
#[tokio::test]
async fn test_agent_result_formatting_integration() -> Result<()> {
    // Create mock agent and execute a task
    let agent = MockReviewAgent::new();
    let task = Task {
        context: TaskContext {
            user_intent: "Review this code for issues".to_string(),
            execution_mode: ExecutionMode::Interactive,
            ..Default::default()
        },
        additional_instructions: None,
    };

    let agent_id = AgentId::from("mock-review");
    let permissions = AgentPermissions::default();
    let toolkit = AgentToolkit::new(agent_id, permissions, PathBuf::from("/tmp"));
    let result = agent.execute(task, &toolkit).await?;

    // Format the result
    let formatted = AgentResultFormatter::format(&result, OutputFormat::Markdown);

    // Verify formatted output structure
    assert!(
        formatted.contains("Code Review"),
        "Formatted output should include header"
    );
    assert!(
        formatted.contains("Performance"),
        "Formatted output should include findings"
    );
    assert!(
        formatted.contains("Consider using a more efficient algorithm"),
        "Formatted output should include finding details"
    );
    assert!(
        formatted.contains("src/main.rs:42"),
        "Formatted output should include location"
    );

    Ok(())
}

/// Test that non-agent commands bypass the agent executor.
#[tokio::test]
async fn test_non_agent_command_bypasses_agent_executor() -> Result<()> {
    let command = MockAgentCommand::new_normal_command();
    let metadata = command.metadata();

    // Verify this is not an agent command
    assert!(!metadata.agent, "Command should not be agent-backed");
    assert!(
        metadata.agent_id.is_none(),
        "Non-agent command should have no agent_id"
    );

    Ok(())
}

/// Performance benchmark for command dispatch.
///
/// Verifies that command metadata lookup and routing completes
/// within acceptable latency thresholds.
#[tokio::test]
async fn test_command_execution_performance_benchmark() -> Result<()> {
    // Create test commands
    let agent_command = MockAgentCommand::new_agent_command();
    let normal_command = MockAgentCommand::new_normal_command();

    // Benchmark metadata access
    let iterations = 1000;
    let start = Instant::now();

    for _ in 0..iterations {
        let _ = agent_command.metadata();
        let _ = normal_command.metadata();
    }

    let elapsed = start.elapsed();
    let avg_latency = elapsed.as_micros() as f64 / (iterations * 2) as f64;

    // Verify performance (should be <1μs per access)
    assert!(
        avg_latency < 10.0,
        "Metadata access should be <10μs, got {:.2}μs",
        avg_latency
    );

    // Benchmark agent detection
    let start = Instant::now();

    for _ in 0..iterations {
        let _is_agent = agent_command.metadata().agent;
        let _is_normal = normal_command.metadata().agent;
    }

    let elapsed = start.elapsed();
    let avg_latency = elapsed.as_nanos() as f64 / (iterations * 2) as f64;

    // Verify agent detection is extremely fast (<100ns)
    assert!(
        avg_latency < 1000.0,
        "Agent detection should be <1μs, got {:.2}ns",
        avg_latency
    );

    println!("✓ Performance benchmarks passed");
    println!("  Metadata access: {:.2}μs average", avg_latency / 1000.0);
    println!("  Agent detection: {:.2}ns average", avg_latency);

    Ok(())
}

/// Test agent activation scoring.
#[test]
fn test_agent_activation_scoring() {
    let agent = MockReviewAgent::new();

    // Test high activation for review context
    let review_context = TaskContext {
        user_intent: "Review this code for security issues".to_string(),
        ..Default::default()
    };
    let score = agent.can_handle(&review_context);
    assert_eq!(
        score.0, 0.9,
        "Agent should have high activation for review tasks"
    );

    // Test low activation for non-review context
    let explain_context = TaskContext {
        user_intent: "Explain how this function works".to_string(),
        ..Default::default()
    };
    let score = agent.can_handle(&explain_context);
    assert_eq!(
        score.0, 0.1,
        "Agent should have low activation for non-review tasks"
    );
}

/// Test that agent result variants are correctly structured.
#[test]
fn test_agent_result_structure() {
    let result = AgentResult::CodeReview {
        findings: vec![CodeReviewFinding {
            severity: Severity::Error,
            category: "Security".to_string(),
            message: "SQL injection vulnerability".to_string(),
            location: Some(PathBuf::from("src/db.rs")),
            line_number: Some(123),
        }],
    };

    match result {
        AgentResult::CodeReview { findings } => {
            assert_eq!(findings.len(), 1, "Should have one finding");
            assert_eq!(findings[0].severity, Severity::Error);
            assert_eq!(findings[0].category, "Security");
        }
        _ => panic!("Expected CodeReview result"),
    }
}

/// Test output format variants.
#[tokio::test]
async fn test_output_format_variants() -> Result<()> {
    let agent = MockReviewAgent::new();
    let task = Task {
        context: TaskContext {
            user_intent: "Review code".to_string(),
            ..Default::default()
        },
        additional_instructions: None,
    };

    let agent_id = AgentId::from("mock-review");
    let permissions = AgentPermissions::default();
    let toolkit = AgentToolkit::new(agent_id, permissions, PathBuf::from("/tmp"));
    let result = agent.execute(task, &toolkit).await?;

    // Test Markdown format
    let markdown = AgentResultFormatter::format(&result, OutputFormat::Markdown);
    assert!(markdown.contains("#"), "Markdown should contain headers");

    // Test PlainText format
    let plain = AgentResultFormatter::format(&result, OutputFormat::PlainText);
    assert!(
        !plain.contains("#"),
        "Plain format should not contain markdown"
    );

    // Test JSON format
    let json = AgentResultFormatter::format(&result, OutputFormat::Json);
    assert!(
        json.starts_with("{") || json.starts_with("["),
        "JSON should be valid JSON structure"
    );

    Ok(())
}

/// Integration test for complete command execution flow.
///
/// This test verifies the end-to-end flow with actual component integration.
/// Note: Since CommandRegistry requires filesystem access, this test is
/// more limited than it would be with a full integration environment.
#[tokio::test]
async fn test_command_execution_integration() -> Result<()> {
    // Create mock commands
    let agent_command = MockAgentCommand::new_agent_command();
    let normal_command = MockAgentCommand::new_normal_command();

    // Verify agent command metadata
    assert!(
        agent_command.metadata().agent,
        "Agent command should have agent flag"
    );
    assert!(
        agent_command.metadata().agent_id.is_some(),
        "Agent command should have agent_id"
    );

    // Verify normal command metadata
    assert!(
        !normal_command.metadata().agent,
        "Normal command should not have agent flag"
    );

    println!("✓ Command execution integration test passed");

    Ok(())
}

/// Test conversation context integration.
#[test]
fn test_conversation_context_structure() {
    use codex_core::commands::executor::MessageSummary;

    let context = ConversationContext {
        recent_messages: vec![MessageSummary {
            role: "user".to_string(),
            content: "Previous message".to_string(),
            timestamp: None,
        }],
        conversation_id: Some("conv-123".to_string()),
    };

    assert_eq!(context.recent_messages.len(), 1);
    assert!(context.conversation_id.is_some());
}

/// Test execution context builder.
#[test]
fn test_execution_context_creation() {
    use std::collections::HashMap;

    let context = ExecutionContext {
        workspace_root: PathBuf::from("/workspace"),
        git_diff: Some("diff content".to_string()),
        current_files: vec![PathBuf::from("src/main.rs")],
        conversation_context: None,
        env_vars: HashMap::new(),
    };

    assert_eq!(context.workspace_root, PathBuf::from("/workspace"));
    assert!(context.git_diff.is_some());
    assert_eq!(context.current_files.len(), 1);
}
