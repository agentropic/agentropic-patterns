use agentropic_core::AgentId;
use std::collections::HashMap;

/// Consensus mechanism for swarm decisions
#[derive(Debug, Clone)]
pub struct Consensus {
    votes: HashMap<AgentId, String>,
    threshold: f64,
}

impl Consensus {
    /// Create new consensus mechanism
    pub fn new(threshold: f64) -> Self {
        Self {
            votes: HashMap::new(),
            threshold: threshold.clamp(0.0, 1.0),
        }
    }

    /// Add a vote
    pub fn vote(&mut self, agent_id: AgentId, choice: impl Into<String>) {
        self.votes.insert(agent_id, choice.into());
    }

    /// Check if consensus reached
    pub fn is_reached(&self) -> bool {
        if self.votes.is_empty() {
            return false;
        }

        let mut counts: HashMap<&String, usize> = HashMap::new();
        for choice in self.votes.values() {
            *counts.entry(choice).or_insert(0) += 1;
        }

        let max_count = counts.values().max().copied().unwrap_or(0);
        let ratio = max_count as f64 / self.votes.len() as f64;

        ratio >= self.threshold
    }

    /// Get winning choice (if consensus reached)
    pub fn winner(&self) -> Option<String> {
        if !self.is_reached() {
            return None;
        }

        let mut counts: HashMap<&String, usize> = HashMap::new();
        for choice in self.votes.values() {
            *counts.entry(choice).or_insert(0) += 1;
        }

        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(choice, _)| choice.clone())
    }

    /// Get all votes
    pub fn votes(&self) -> &HashMap<AgentId, String> {
        &self.votes
    }

    /// Clear votes
    pub fn clear(&mut self) {
        self.votes.clear();
    }
}
