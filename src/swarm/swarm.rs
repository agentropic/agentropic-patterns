use super::Behavior;
use agentropic_core::AgentId;

/// Swarm of agents
#[derive(Debug, Clone)]
pub struct Swarm {
    name: String,
    members: Vec<AgentId>,
    behavior: Option<Behavior>,
}

impl Swarm {
    /// Create a new swarm
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            members: Vec::new(),
            behavior: None,
        }
    }

    /// Add member
    pub fn add_member(&mut self, agent_id: AgentId) {
        self.members.push(agent_id);
    }

    /// Set behavior
    pub fn set_behavior(&mut self, behavior: Behavior) {
        self.behavior = Some(behavior);
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get members
    pub fn members(&self) -> &[AgentId] {
        &self.members
    }

    /// Get behavior
    pub fn behavior(&self) -> Option<&Behavior> {
        self.behavior.as_ref()
    }

    /// Get size
    pub fn size(&self) -> usize {
        self.members.len()
    }
}
