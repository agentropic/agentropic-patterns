use agentropic_core::AgentId;
use std::collections::HashMap;

/// Resource allocation
#[derive(Debug, Clone)]
pub struct Allocation {
    allocations: HashMap<AgentId, Vec<String>>,
}

impl Allocation {
    /// Create new allocation
    pub fn new() -> Self {
        Self {
            allocations: HashMap::new(),
        }
    }

    /// Allocate resource to agent
    pub fn allocate(&mut self, agent_id: AgentId, resource: impl Into<String>) {
        self.allocations
            .entry(agent_id)
            .or_insert_with(Vec::new)
            .push(resource.into());
    }

    /// Get agent's resources
    pub fn get(&self, agent_id: &AgentId) -> Option<&Vec<String>> {
        self.allocations.get(agent_id)
    }

    /// Get all allocations
    pub fn all(&self) -> &HashMap<AgentId, Vec<String>> {
        &self.allocations
    }
}

impl Default for Allocation {
    fn default() -> Self {
        Self::new()
    }
}
