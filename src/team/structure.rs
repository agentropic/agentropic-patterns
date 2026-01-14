use super::{Coordination, Role};
use agentropic_core::AgentId;
use std::collections::HashMap;

/// Team structure
#[derive(Debug, Clone)]
pub struct Team {
    name: String,
    roles: HashMap<AgentId, Role>,
    coordination: Coordination,
}

impl Team {
    /// Create a new team
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            roles: HashMap::new(),
            coordination: Coordination::new(),
        }
    }

    /// Assign role to agent
    pub fn assign_role(&mut self, agent_id: AgentId, role: Role) {
        self.roles.insert(agent_id, role);
        self.coordination.add_member(agent_id);
    }

    /// Get agent's role
    pub fn get_role(&self, agent_id: &AgentId) -> Option<&Role> {
        self.roles.get(agent_id)
    }

    /// Set team leader
    pub fn set_leader(&mut self, leader: AgentId) {
        self.coordination.set_leader(leader);
    }

    /// Get team name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get coordination
    pub fn coordination(&self) -> &Coordination {
        &self.coordination
    }

    /// Get all members
    pub fn members(&self) -> Vec<&AgentId> {
        self.roles.keys().collect()
    }
}
