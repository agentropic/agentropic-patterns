use agentropic_core::AgentId;
use serde::{Deserialize, Serialize};

/// Type of coalition formation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormationType {
    /// Top-down formation
    TopDown,
    /// Bottom-up formation
    BottomUp,
    /// Negotiation-based
    Negotiation,
    /// Auction-based
    Auction,
}

/// Coalition formation process
#[derive(Debug, Clone)]
pub struct Formation {
    formation_type: FormationType,
    candidates: Vec<AgentId>,
    selected: Vec<AgentId>,
}

impl Formation {
    /// Create a new formation process
    pub fn new(formation_type: FormationType) -> Self {
        Self {
            formation_type,
            candidates: Vec::new(),
            selected: Vec::new(),
        }
    }

    /// Add candidate
    pub fn add_candidate(&mut self, agent_id: AgentId) {
        self.candidates.push(agent_id);
    }

    /// Select agent for coalition
    pub fn select(&mut self, agent_id: AgentId) {
        if self.candidates.contains(&agent_id) {
            self.selected.push(agent_id);
        }
    }

    /// Get formation type
    pub fn formation_type(&self) -> FormationType {
        self.formation_type
    }

    /// Get candidates
    pub fn candidates(&self) -> &[AgentId] {
        &self.candidates
    }

    /// Get selected agents
    pub fn selected(&self) -> &[AgentId] {
        &self.selected
    }

    /// Check if formation is complete
    pub fn is_complete(&self) -> bool {
        !self.selected.is_empty()
    }
}
