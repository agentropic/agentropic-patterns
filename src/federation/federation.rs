use super::Policy;
use agentropic_core::AgentId;
use std::collections::{HashMap, HashSet};

/// Federation of autonomous agents
#[derive(Debug, Clone)]
pub struct Federation {
    name: String,
    members: HashSet<AgentId>,
    policies: HashMap<String, Policy>,
    weights: HashMap<AgentId, f64>,
}

impl Federation {
    /// Create a new federation
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            members: HashSet::new(),
            policies: HashMap::new(),
            weights: HashMap::new(),
        }
    }

    /// Add member
    pub fn add_member(&mut self, agent_id: AgentId) -> bool {
        let added = self.members.insert(agent_id.clone());
        if added {
            self.weights.insert(agent_id, 1.0); // Default weight
        }
        added
    }

    /// Remove member
    pub fn remove_member(&mut self, agent_id: &AgentId) -> bool {
        self.weights.remove(agent_id);
        self.members.remove(agent_id)
    }

    /// Set member weight (for weighted voting)
    pub fn set_weight(&mut self, agent_id: AgentId, weight: f64) {
        if self.members.contains(&agent_id) {
            self.weights.insert(agent_id, weight.max(0.0));
        }
    }

    /// Add policy
    pub fn add_policy(&mut self, policy: Policy) {
        self.policies.insert(policy.name().to_string(), policy);
    }

    /// Get policy
    pub fn get_policy(&self, name: &str) -> Option<&Policy> {
        self.policies.get(name)
    }

    /// Get name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get members
    pub fn members(&self) -> impl Iterator<Item = &AgentId> {
        self.members.iter()
    }

    /// Get member weight
    pub fn weight(&self, agent_id: &AgentId) -> Option<f64> {
        self.weights.get(agent_id).copied()
    }

    /// Get size
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Get all policies
    pub fn policies(&self) -> impl Iterator<Item = &Policy> {
        self.policies.values()
    }
}
