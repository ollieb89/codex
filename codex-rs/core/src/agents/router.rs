//! Agent router for context-based agent selection.

use std::collections::HashMap;
use std::sync::Arc;

use super::{Agent, AgentId, TaskContext};

/// Agent router that selects the best agent for a given context.
pub struct AgentRouter {
    agents: HashMap<AgentId, Arc<dyn Agent>>,
    activation_threshold: f64,
}

impl AgentRouter {
    /// Creates a new agent router.
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            activation_threshold: 0.6,
        }
    }

    /// Registers an agent with the router.
    pub fn register_agent(&mut self, agent: Arc<dyn Agent>) {
        self.agents.insert(agent.id(), agent);
    }

    /// Selects the best agent for the given context.
    ///
    /// Returns the highest-scoring agent if above the activation threshold.
    pub async fn select_agent(&self, context: &TaskContext) -> Option<Arc<dyn Agent>> {
        let mut scores: Vec<_> = self
            .agents
            .values()
            .map(|agent| {
                let score = agent.can_handle(context);
                (agent.clone(), score.0)
            })
            .collect();

        // Sort by score descending
        scores.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

        // Return top agent if above threshold
        if let Some((agent, score)) = scores.first() {
            if *score >= self.activation_threshold {
                return Some(agent.clone());
            }
        }

        None
    }

    /// Suggests top-k agents for the given context.
    ///
    /// Returns agents ranked by activation score.
    pub async fn suggest_agents(
        &self,
        context: &TaskContext,
        top_k: usize,
    ) -> Vec<AgentSuggestion> {
        let mut scores: Vec<_> = self
            .agents
            .values()
            .map(|agent| {
                let score = agent.can_handle(context);
                AgentSuggestion {
                    agent_id: agent.id(),
                    name: agent.name().to_string(),
                    description: agent.description().to_string(),
                    score: score.0,
                }
            })
            .collect();

        scores.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scores.truncate(top_k);
        scores
    }

    /// Sets the activation threshold for agent selection.
    pub fn set_activation_threshold(&mut self, threshold: f64) {
        self.activation_threshold = threshold.clamp(0.0, 1.0);
    }
}

impl Default for AgentRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Agent suggestion with score.
#[derive(Debug, Clone)]
pub struct AgentSuggestion {
    pub agent_id: AgentId,
    pub name: String,
    pub description: String,
    pub score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::{ActivationScore, AgentPermissions, Task};
    use async_trait::async_trait;

    struct MockAgent {
        id: AgentId,
        score: f64,
        permissions: AgentPermissions,
    }

    #[async_trait]
    impl Agent for MockAgent {
        fn id(&self) -> AgentId {
            self.id.clone()
        }

        fn name(&self) -> &str {
            "Mock Agent"
        }

        fn description(&self) -> &str {
            "Test agent"
        }

        fn can_handle(&self, _context: &TaskContext) -> ActivationScore {
            ActivationScore::new(self.score)
        }

        async fn execute(
            &self,
            _task: Task,
            _toolkit: &crate::agents::AgentToolkit,
        ) -> anyhow::Result<crate::agents::AgentResult> {
            Ok(crate::agents::AgentResult::Analysis {
                summary: "Mock result".into(),
                details: HashMap::new(),
            })
        }

        fn permissions(&self) -> &AgentPermissions {
            &self.permissions
        }

        fn system_prompt(&self) -> &str {
            "Mock prompt"
        }
    }

    #[tokio::test]
    async fn test_agent_selection() {
        let mut router = AgentRouter::new();

        let agent1 = Arc::new(MockAgent {
            id: AgentId::from("agent1"),
            score: 0.8,
            permissions: AgentPermissions::default(),
        });
        let agent2 = Arc::new(MockAgent {
            id: AgentId::from("agent2"),
            score: 0.5,
            permissions: AgentPermissions::default(),
        });

        router.register_agent(agent1);
        router.register_agent(agent2);

        let context = TaskContext::default();
        let selected = router.select_agent(&context).await.unwrap();

        assert_eq!(selected.id(), AgentId::from("agent1"));
    }

    #[tokio::test]
    async fn test_threshold_filtering() {
        let mut router = AgentRouter::new();
        router.set_activation_threshold(0.7);

        let agent = Arc::new(MockAgent {
            id: AgentId::from("low-score"),
            score: 0.5,
            permissions: AgentPermissions::default(),
        });

        router.register_agent(agent);

        let context = TaskContext::default();
        let selected = router.select_agent(&context).await;

        assert!(selected.is_none());
    }
}
