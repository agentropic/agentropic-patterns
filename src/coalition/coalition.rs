use super::Strategy;
use agentropic_core::AgentId;
use std::collections::HashSet;

/// Coalition of agents
#[derive(Debug, Clone)]
pub struct Coalition {
    name: String,
    members: HashSet<AgentId>,
    strategy: Option<Strategy>,
    value: f64,
}

impl Coalition {
    /// Create a new coalition
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            members: HashSet::new(),
            strategy: None,
            value: 0.0,
        }
    }

    /// Add member
    pub fn add_member(&mut self, agent_id: AgentId) -> bool {
        self.members.insert(agent_id)
    }

    /// Remove member
    pub fn remove_member(&mut self, agent_id: &AgentId) -> bool {
        self.members.remove(agent_id)
    }

    /// Check if agent is member
    pub fn has_member(&self, agent_id: &AgentId) -> bool {
        self.members.contains(agent_id)
    }

    /// Set strategy
    pub fn set_strategy(&mut self, strategy: Strategy) {
        self.strategy = Some(strategy);
    }

    /// Set coalition value
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get members
    pub fn members(&self) -> impl Iterator<Item = &AgentId> {
        self.members.iter()
    }

    /// Get size
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Get strategy
    pub fn strategy(&self) -> Option<&Strategy> {
        self.strategy.as_ref()
    }

    /// Get value
    pub fn value(&self) -> f64 {
        self.value
    }
}
