use agentropic_core::AgentId;
use serde::{Deserialize, Serialize};

/// Task delegation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
    from: AgentId,
    to: AgentId,
    task: String,
    authority_level: u32,
}

impl Delegation {
    /// Create a new delegation
    pub fn new(from: AgentId, to: AgentId, task: impl Into<String>, authority_level: u32) -> Self {
        Self {
            from,
            to,
            task: task.into(),
            authority_level,
        }
    }

    /// Get delegator
    pub fn from(&self) -> &AgentId {
        &self.from
    }

    /// Get delegate
    pub fn to(&self) -> &AgentId {
        &self.to
    }

    /// Get task
    pub fn task(&self) -> &str {
        &self.task
    }

    /// Get authority level
    pub fn authority_level(&self) -> u32 {
        self.authority_level
    }
}
