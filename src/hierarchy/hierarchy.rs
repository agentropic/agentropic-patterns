use super::{Delegation, Level};
use agentropic_core::AgentId;
use std::collections::HashMap;

/// Hierarchical organization structure
#[derive(Debug, Clone)]
pub struct Hierarchy {
    name: String,
    levels: Vec<Level>,
    agents: HashMap<AgentId, Level>,
    delegations: Vec<Delegation>,
}

impl Hierarchy {
    /// Create a new hierarchy
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            levels: Vec::new(),
            agents: HashMap::new(),
            delegations: Vec::new(),
        }
    }

    /// Add a level
    pub fn add_level(&mut self, level: Level) {
        self.levels.push(level);
    }

    /// Assign agent to level
    pub fn assign_agent(&mut self, agent_id: AgentId, level: Level) {
        self.agents.insert(agent_id, level);
    }

    /// Get agent's level
    pub fn get_level(&self, agent_id: &AgentId) -> Option<&Level> {
        self.agents.get(agent_id)
    }

    /// Delegate task
    pub fn delegate(&mut self, delegation: Delegation) {
        self.delegations.push(delegation);
    }

    /// Get all delegations
    pub fn delegations(&self) -> &[Delegation] {
        &self.delegations
    }

    /// Get hierarchy name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get all levels
    pub fn levels(&self) -> &[Level] {
        &self.levels
    }
}
